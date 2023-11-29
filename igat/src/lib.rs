#[macro_use]
extern crate log;

pub(crate) mod cde;
pub mod frame;
pub mod widget;

use std::{fmt::Debug, marker::PhantomData};

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

    fn route(&mut self,event: ApplicationEvent) -> &dyn Frame<Message = M> {
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
                    let frame = ctx.route(ApplicationEvent::RedrawRequested);
                    cde.bgr(frame.bgr());
                    match frame.ui() {
                        Some(component) => {
                            cde.draw(&component.inner);
                        },
                        None => {},
                    };
                    cde.write();
                    self.window.set_title(&frame.title());
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

    fn route(&self, event: ApplicationEvent) -> &dyn Frame<Message = M>;

    fn on_close(&self);
}
