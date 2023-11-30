#[macro_use]
extern crate log;

pub(crate) mod cde;
pub mod frame;
pub mod widget;
pub mod cursor;

use std::{fmt::Debug, marker::PhantomData};

use cursor::Cursor;
use frame::Frame;

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

pub struct ApplicationContext<A,M>
where
    A: Application<M>,
    M: Send + std::fmt::Debug
{
    app: A,
    phantom: PhantomData<M>
}

impl<A,M> ApplicationContext<A,M>
where
    A: Application<M>,
    M: Send + std::fmt::Debug
{
    fn new(app: A) -> Self {
        Self {
            app,
            phantom: PhantomData
        }
    }

    fn set_up(&mut self) {
        self.app.set_up();
    }

    fn route(&mut self,event: ApplicationEvent) -> &mut dyn Frame<Message = M> {
        self.app.route(event)
    }

    pub fn dispatch_message(&mut self,message: M) {
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

    pub fn run<T,M>(self, app: T)
    where
        T: Application<M>,
        M: Send + std::fmt::Debug
    {
        let mut ctx = ApplicationContext::new(app);
        ctx.set_up();

        let cde:Cde<M> = Cde::new(&self.window);

        let _result = self.event_loop.run(move |event, elwt| match event {
            Event::WindowEvent { event, window_id } if window_id == self.window.id() => match event
            {
                WindowEvent::CloseRequested => elwt.exit(),
                WindowEvent::RedrawRequested => {
                    let frame: &mut dyn Frame<Message = M> = ctx.route(ApplicationEvent::RedrawRequested);
                    let title = &frame.title();
                    self.window.set_title(title);
                    let cursor = Cursor::get();
                    
                    cde.bgr(frame.bgr());
                    match frame.ui() {
                        Some(mut component) => {
                            match cursor.window_x(&self.window) {
                                Some(x) => {
                                    match cursor.window_y(&self.window) {
                                        // Determine if the cursor is at the widget position
                                        Some(y) => {
                                            if (x as u32) > component.inner.x() && (x as u32) < component.inner.x()+component.inner.width() {
                                                if (y as u32) > component.inner.y() && (y as u32) < component.inner.y()+component.inner.height() {
                                                    component.inner.message(widget::ClientMessage::OnHover)
                                                }
                                            }
                                            println!("{}",y);
                                        }
                                        None => {}
                                    }
                                }
                                None => {}
                            }
                            match cde.draw(&component.inner) {
                                Some(message) => {
                                    ctx.dispatch_message(message);
                                }

                                None => {

                                }
                            }
                        },
                        None => {},
                    };
                    cde.write();
                    self.window.pre_present_notify();
                }
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

pub trait Application<M>: Sized 
where
    M: Send + std::fmt::Debug
{
    type Message: Send + Debug;

    fn set_up(&mut self) {}

    fn message(&mut self,event: M) {

    }

    fn route(&mut self, event: ApplicationEvent) -> &mut dyn Frame<Message = M>;

    fn on_close(&self);
}
