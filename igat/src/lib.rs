#[macro_use]
extern crate log;

pub(crate) mod cde;
pub mod frame;
#[deprecated(
    since = "0.0.1",
    note = "Plugin function will be deprecated. Use the set_up method instead"
)]
pub mod plugin;
#[deprecated(
    since = "0.0.1",
    note = "UI module is obsolete as it is no longer needed"
)]
pub mod ui;
pub mod widget;

use std::fmt::Debug;

use frame::Frame;
use plugin::PluginLoader;

use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::Window,
    window::WindowBuilder,
};

use cde::CDE;

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
        info!("Loading plugins...");
        let mut loader = PluginLoader::new();
        app.init(&loader);

        loader.load();

        let cde = CDE::new(&self.window);

        self.event_loop.run(move |event, elwt| {
            match event {
                Event::WindowEvent { event, window_id } if window_id == self.window.id() => {
                    match event {
                        WindowEvent::CloseRequested => elwt.exit(),
                        WindowEvent::RedrawRequested => {
                            let mut frame = app.route(ApplicationEvent::RedrawRequested);
                            let Element = frame.ui();
                            cde.draw(frame.bgr(), &Element);
                            self.window.set_title(&frame.title());
                            //self.window.set_resizable(*&frame.resizable());
                            // Notify the windowing system that we'll be presenting to the window.
                            self.window.pre_present_notify();
                        }
                        _ => (),
                    }
                }
                Event::AboutToWait => {
                    self.window.request_redraw();
                }

                _ => (),
            }
        });
    }
}

pub trait Application: Sized {
    type Message: Send + Debug;

    #[deprecated(
        since = "0.0.1",
        note = "Plugin function will be deprecated. Use the set_up method instead"
    )]
    fn init(&mut self, loader: &PluginLoader);

    fn set_up(&mut self) {}

    fn route(&self, event: ApplicationEvent) -> &dyn Frame<Message = Self::Message>;

    fn on_close(&self);
}
