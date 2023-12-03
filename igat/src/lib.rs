#[macro_use]
extern crate log;

pub(crate) mod cde;
pub mod frame;
pub mod widget;

use std::{fmt::Debug, marker::PhantomData};

use frame::Frame;

use widget::Component;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::Window,
    window::WindowBuilder,
};

use cde::Cde;

#[derive(Clone, Copy, Debug)]
pub enum Color {
    Black,
    White,
    ARGB(u8, u8, u8, u8),
}

pub enum ApplicationEvent {
    RedrawRequested,
    KeyboardInput(char),
}

pub struct Executable {
    window: Window,
    event_loop: EventLoop<()>,
}

pub struct ApplicationContext<A, M>
where
    A: Application<M>,
    M: Send + std::fmt::Debug,
{
    app: A,
    phantom: PhantomData<M>,
}

impl<A, M> ApplicationContext<A, M>
where
    A: Application<M>,
    M: Send + std::fmt::Debug,
{
    fn new(app: A) -> Self {
        Self {
            app,
            phantom: PhantomData,
        }
    }

    fn set_up(&mut self) {
        self.app.set_up();
    }

    fn route(&mut self, event: ApplicationEvent) -> &mut dyn Frame<Message = M> {
        self.app.route(event)
    }

    pub fn dispatch_message(&mut self, message: M) {
        self.app.message(message);
    }
}

impl Executable {
    pub fn new() -> Self {
        let event_loop = EventLoop::new().unwrap();

        let window = WindowBuilder::new()
            .with_title("A fantastic window!")
            .with_inner_size(winit::dpi::LogicalSize::new(128.0, 128.0))
            .build(&event_loop)
            .unwrap();
        Self { window, event_loop }
    }

    pub fn run<T, M>(self, app: T)
    where
        T: Application<M>,
        M: Send + std::fmt::Debug,
    {
        let mut ctx = ApplicationContext::new(app);
        ctx.set_up();

        let cde: Cde<M> = Cde::new(&self.window);
        let theme = ctx.app.theme();
        let mut ui = ctx.app.ui();

        let _result = self.event_loop.run(move |event, elwt| match event {
            Event::WindowEvent { event, window_id } if window_id == self.window.id() => match event
            {
                WindowEvent::CloseRequested => elwt.exit(),
                WindowEvent::RedrawRequested => {
                    let frame: &mut dyn Frame<Message = M> =
                        ctx.route(ApplicationEvent::RedrawRequested);
                    let title = &frame.title();
                    self.window.set_title(title);

                    cde.bgr(theme.bgr);
                    match &mut ui {
                        Some(ref mut component) => {
                            if let Some(message) = cde.draw(&component.inner) {
                                ctx.dispatch_message(message);
                            }
                        }
                        None => {}
                    };
                    cde.write();
                    self.window.pre_present_notify();
                }

                WindowEvent::CursorMoved {
                    device_id: _,
                    position,
                } => match &mut ui {
                    Some(component) => {
                        if component.inner.is_capture_event() {
                            if (position.x as u32) > component.inner.x()
                                && (position.x as u32)
                                    < component.inner.x() + component.inner.width()
                            {
                                if (position.y as u32) > component.inner.y()
                                    && (position.y as u32)
                                        < component.inner.y() + component.inner.height()
                                {
                                    component.inner.message(widget::ClientMessage::OnHover)
                                }
                            } else {
                                component.inner.message(widget::ClientMessage::Unfocus)
                            }
                        }
                    }
                    None => {}
                },
                _ => (),
            },

            Event::AboutToWait => {
                self.window.request_redraw();
            }

            _ => (),
        });
    }
}

impl Default for Executable {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Theme {
    bgr: Color,
}

impl Theme {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn bgr(mut self, bgr: Color) -> Self {
        self.bgr = bgr;
        self
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self { bgr: Color::White }
    }
}

pub trait Application<M>: Sized
where
    M: Send + std::fmt::Debug,
{
    type Message: Send + Debug;

    fn theme(&self) -> Theme;

    fn set_up(&mut self) {}

    fn message(&mut self, _event: M) {}

    fn ui(&mut self) -> Option<Component<M>>;

    #[deprecated(
        since = "0.0.2",
        note = "UI construction using Frames will be discontinued. Please use the ui method instead"
    )]
    fn route(&mut self, event: ApplicationEvent) -> &mut dyn Frame<Message = M>;

    fn on_close(&self);
}
