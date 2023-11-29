#[macro_use]
extern crate log;

pub(crate) mod cde;
pub mod frame;
pub mod widget;

use std::fmt::Debug;

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

    pub fn run<T>(self, mut app: T)
    where
        T: Application,
    {
        app.set_up();

        let cde = Cde::new(&self.window);

        let _result = self.event_loop.run(move |event, elwt| match event {
            Event::WindowEvent { event, window_id } if window_id == self.window.id() => match event
            {
                WindowEvent::CloseRequested => elwt.exit(),
                WindowEvent::RedrawRequested => {
                    let frame = app.route(ApplicationEvent::RedrawRequested);
                    let element = frame.ui();
                    cde.bgr(frame.bgr());
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

pub trait Application: Sized {
    type Message: Send + Debug;

    fn set_up(&mut self) {}

    fn route(&self, event: ApplicationEvent) -> &dyn Frame<Message = Self::Message>;

    fn on_close(&self);
}
