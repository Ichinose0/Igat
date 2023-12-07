#[macro_use]
extern crate log;

pub(crate) mod cde;
mod cursor;
pub mod menu;
pub mod widget;

use std::{fmt::Debug, marker::PhantomData};

use cursor::Cursor;
use menu::Menubar;
use widget::{Component, RenderConfig};
use winapi::um::winuser::{GetAsyncKeyState, VK_LBUTTON};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::Window,
    window::WindowBuilder,
};

use cde::Cde;

pub type CursorIcon = winit::window::CursorIcon;

#[derive(Clone,Copy,Debug)]
pub struct Rect {
    left: u32,
    top: u32,
    right: u32,
    bottom: u32,
}

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
    WidgetEvent,
    KeyboardInput(char),
}

pub enum ApplicationResponse {
    ReloadUi,
}

pub struct Executable {
    window: Window,
    event_loop: EventLoop<()>,
}

pub struct Frame {
    window: Window,
    menu_height: u32,
}

impl Frame {
    pub fn set_title(&self,title: &str) {
        self.window.set_title(title);
    }

    pub fn set_resizable(&self,resizable: bool) {
        self.window.set_resizable(resizable);
    }

    pub fn set_minimized(&self,minimized: bool) {
        self.window.set_minimized(minimized);
    }

    pub fn set_maximized(&self, maximized: bool) {
        self.window.set_maximized(maximized);
    } 

    pub fn set_decorations(&self, decorations: bool) {
        self.window.set_decorations(decorations);
    }

    pub fn set_cursor_visible(&self, visible: bool) {
        self.window.set_cursor_visible(visible);
    }

    pub fn set_cursor_icon(&self, cursor: CursorIcon) {
        self.window.set_cursor_icon(cursor);
    }

    pub fn get_rect(&self) -> Rect {
        let size = self.window.inner_size();
        Rect {
            left: 0,
            top: self.menu_height,
            right: size.width,
            bottom: size.height+self.menu_height,
        }
    }
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

    pub fn dispatch_message(&mut self, message: M,frame: &Frame) {
        self.app.message(ApplicationEvent::WidgetEvent,Some(message),frame);
    }
}

impl Executable {
    pub fn new() -> Self {
        let event_loop = EventLoop::new().unwrap();

        let window = WindowBuilder::new()
            .with_title("A fantastic window!")
            .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0))
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
        

        let mut config = RenderConfig {
            thickness: 0,
            border_radius: 0.0,
        };

        let height = match ctx.app.menu() {
            Some(m) => m.height(),
            None => 0,
        };

        let frame = Frame {
            window: self.window,
            menu_height: height
        };

        let mut ui = ctx.app.ui(&frame);

        let _result = self.event_loop.run(move |event, elwt| {
            cde.bgr(theme.bgr);
            
            match event {
                Event::WindowEvent { event, window_id } if window_id == frame.window.id() => {
                    let cursor = Cursor::get();
                    match &mut ui {
                        Some(component) => {
                            if component.inner.is_capture_event() {
                                let area = component.area();
                                for a in area {
                                    match cursor.window_x(&frame.window) {
                                        Some(x) => {
                                            let x = x - component.inner.x() as i32;
                                            if (x as u32) > component.inner.x()
                                                && (x as u32)
                                                    < component.inner.x() + component.inner.width()
                                            {
                                                match cursor.window_y(&frame.window) {
                                                    Some(y) => {
                                                        let y = y - component.inner.y() as i32;
                                                        if (y as u32) > component.inner.y()
                                                            && (y as u32)
                                                                < component.inner.y()
                                                                    + component.inner.height()
                                                        {
                                                            component.inner.message(
                                                                widget::ClientMessage::OnHover,
                                                            );
                                                            if unsafe {
                                                                GetAsyncKeyState(VK_LBUTTON) != 0
                                                            } {
                                                                component.inner.message(
                                                                    widget::ClientMessage::OnClick,
                                                                );
                                                                match component.inner.on_click() {
                                                                    Some(e) => {
                                                                        match ctx.app.message(ApplicationEvent::WidgetEvent,Some(e),&frame) {
                                                                            Some(e) => {
    
                                                                            }
    
                                                                            None => {}
                                                                        }
                                                                    }
                                                                    None => todo!(),
                                                                }
                                                                cde.draw(&component.inner);
                                                            }
                                                        }
                                                    }
    
                                                    None => {}
                                                }
                                            } else {
                                                component.inner.message(widget::ClientMessage::Unfocus)
                                            }
                                        }
                                        None => {}
                                    }
                                }
                            }
                        }
                        None => todo!(),
                    }

                    match event {
                        WindowEvent::CloseRequested => elwt.exit(),
                        WindowEvent::RedrawRequested => {
                            match ctx.app.message(ApplicationEvent::RedrawRequested,None,&frame) {
                                Some(e) => {

                                }

                                None => {}
                            }
                            match &mut ui {
                                Some(ref mut component) => {
                                    cde.draw(&component.inner);
                                }
                                None => {}
                            };
                            match ctx.app.menu() {
                                Some(menu) => {
                                    cde.draw_menu(&frame.window, menu);
                                }
                                None => {}
                            }

                            cde.write();

                            frame.window.pre_present_notify();
                        }

                        _ => {}
                    }
                }

                Event::AboutToWait => {
                    frame.window.request_redraw();
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
        Self {
            bgr: Color::ARGB(255, 240, 240, 240),
        }
    }
}

pub trait Application<M>: Sized
where
    M: Send + std::fmt::Debug,
{
    type Message: Send + Debug;

    fn theme(&self) -> Theme;

    fn set_up(&mut self) {}

    fn message(
        &mut self,
        event: ApplicationEvent,
        _message: Option<M>,
        frame: &Frame
    ) -> Option<ApplicationResponse> {
        None
    }

    fn menu(&self) -> Option<&Menubar> {
        None
    }

    fn ui(&mut self,frame: &Frame) -> Option<Component<M>>;

    fn on_close(&self);
}
