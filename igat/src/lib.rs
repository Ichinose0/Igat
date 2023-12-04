#[macro_use]
extern crate log;

pub(crate) mod cde;
pub mod menu;
pub mod widget;

use std::{fmt::Debug, marker::PhantomData};

use menu::Menubar;
use widget::Component;
use winapi::um::winuser::{GetAsyncKeyState, VK_LBUTTON};
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

impl Into<acure::Color> for Color {
    fn into(self) -> acure::Color {
        match self {
            Color::Black => acure::Color::ARGB(255, 0, 0, 0),
            Color::White => acure::Color::ARGB(255, 255, 255, 255),
            Color::ARGB(a, r, g, b) => acure::Color::ARGB(a, r, g, b),
        }
    }
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

        let _result = self.event_loop.run(move |event, elwt| {
            cde.bgr(theme.bgr);
            match ctx.app.menu() {
                Some(menu) => {
                    cde.draw_menu(&self.window,menu);
                }
                None => {}
            }
            match event {
                Event::WindowEvent { event, window_id } if window_id == self.window.id() => {
                    match event {
                        WindowEvent::CloseRequested => elwt.exit(),
                        WindowEvent::RedrawRequested => {
                            match &mut ui {
                                Some(ref mut component) => {
                                    cde.draw(&component.inner);
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
                                            component.inner.message(widget::ClientMessage::OnHover);
                                            if unsafe { GetAsyncKeyState(VK_LBUTTON) != 0 } {
                                                println!("On click");
                                                component
                                                    .inner
                                                    .message(widget::ClientMessage::OnClick);
                                                match component.inner.on_click() {
                                                    Some(e) => {
                                                        ctx.app.message(e);
                                                    }
                                                    None => todo!(),
                                                }
                                                cde.draw(&component.inner);
                                            }
                                        }
                                    } else {
                                        println!("unfocus");
                                        component.inner.message(widget::ClientMessage::Unfocus)
                                    }
                                }
                            }
                            None => {}
                        },

                        _ => {}
                    }
                }

                Event::AboutToWait => {
                    self.window.request_redraw();
                }

                _ => {}
            }
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
        Self { bgr: Color::ARGB(255,240,240,240) }
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

    fn menu(&self) -> Option<&Menubar> {
        None
    }

    fn ui(&mut self) -> Option<Component<M>>;

    fn on_close(&self);
}
