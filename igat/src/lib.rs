//! # Igat - Expressive GUI library
//!
//! <p style="display: inline">
//! <img src="https://img.shields.io/badge/-Rust-000000.svg?logo=rust&style=for-the-badge">
//! <img src="https://img.shields.io/badge/-githubactions-FFFFFF.svg?logo=github-actions&style=for-the-badge">
//! </p>

#[macro_use]
extern crate log;

#[doc(hidden)]
pub(crate) mod cde;
#[doc(hidden)]
pub(crate) mod cursor;
pub mod menu;
pub mod widget;

use std::{fmt::Debug, marker::PhantomData, time::Duration};

use cde::RenderManager;
use cursor::Cursor;
use menu::Menubar;
use widget::{Button, Component, ContentPanel, RenderConfig};
use winapi::um::winuser::{GetAsyncKeyState, VK_LBUTTON};

//use cde::{Cde, RenderManager};
use winit::{event_loop::EventLoop, window::WindowBuilder};

pub type CursorIcon = winit::window::CursorIcon;

/// Represents an area on the screen
///
/// This structure represents an area (rectangle) by the coordinates of the upper left and lower right corners
/// # Members
/// * `left` - X coordinate of upper left corner
/// * `top` - Y coordinate of upper left corner
/// * `right` - X coordinate of lower right corner
/// * `bottom` - Y coordinate of lower right corner
#[derive(Clone, Copy, Debug)]
pub struct Rect {
    left: u32,
    top: u32,
    right: u32,
    bottom: u32,
}

impl Rect {
    /// Obtains the X coordinate
    pub fn x(&self) -> u32 {
        self.left
    }

    /// Obtains the Y coordinate
    pub fn y(&self) -> u32 {
        self.top
    }

    /// Get width
    pub fn width(&self) -> u32 {
        self.right - self.left
    }

    /// Get height
    pub fn height(&self) -> u32 {
        self.bottom - self.top
    }
}

/// Represents a color
///
/// Initialization with ARGB allows you to create your own colors
#[derive(Clone, Copy, Debug)]
pub enum Color {
    Black,
    White,
    /// Initialization with ARGB allows you to create your own colors
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
    event_loop: EventLoop<()>,
}

/// Represents an application window handle
///
/// This structure can be used to change window properties
// pub struct Frame {
//     window: Window,
//     menu_height: u32,
// }

// impl Frame {
//     pub fn set_title(&self, title: &str) {
//         self.window.set_title(title);
//     }

//     pub fn set_resizable(&self, resizable: bool) {
//         self.window.set_resizable(resizable);
//     }

//     pub fn set_minimized(&self, minimized: bool) {
//         self.window.set_minimized(minimized);
//     }

//     pub fn set_maximized(&self, maximized: bool) {
//         self.window.set_maximized(maximized);
//     }

//     pub fn set_decorations(&self, decorations: bool) {
//         self.window.set_decorations(decorations);
//     }

//     pub fn set_cursor_visible(&self, visible: bool) {
//         self.window.set_cursor_visible(visible);
//     }

//     pub fn set_cursor_icon(&self, cursor: CursorIcon) {
//         self.window.set_cursor_icon(cursor);
//     }

//     pub fn get_rect(&self) -> Rect {
//         let size = self.window.inner_size();
//         Rect {
//             left: 0,
//             top: self.menu_height,
//             right: size.width,
//             bottom: size.height + self.menu_height,
//         }
//     }
// }

// pub struct ApplicationContext<A, M>
// where
//     A: Application<M>,
//     M: Send + std::fmt::Debug,
// {
//     app: A,
//     phantom: PhantomData<M>,
// }

// impl<A, M> ApplicationContext<A, M>
// where
//     A: Application<M>,
//     M: Send + std::fmt::Debug,
// {
//     fn new(app: A) -> Self {
//         Self {
//             app,
//             phantom: PhantomData,
//         }
//     }

//     pub fn dispatch_message(&mut self, message: M, frame: &Frame) {
//         self.app
//             .message(ApplicationEvent::WidgetEvent, Some(message), frame);
//     }
// }

// impl Executable {
//     pub fn new() -> Self {
//         let event_loop = EventLoop::new().unwrap();

//         let window = WindowBuilder::new()
//             .with_title("None")
//             .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0))
//             .build(&event_loop)
//             .unwrap();

//         Self { window, event_loop }
//     }

//     pub fn run<T, M>(self, app: T)
//     where
//         T: Application<M>,
//         M: Send + std::fmt::Debug,
//     {
//         let mut ctx = ApplicationContext::new(app);

//         let theme = ctx.app.theme();

//         let config = RenderConfig {
//             thickness: 0,
//             border_radius: 0.0,
//         };

//         let height = match ctx.app.menu() {
//             Some(m) => m.height(),
//             None => 0,
//         };

//         let frame = Frame {
//             window: self.window,
//             menu_height: height,
//         };

//         let mut render_manager: RenderManager<M> = RenderManager::new(frame, theme);

//         ctx.app.set_up(render_manager.frame());

//         render_manager.set_background_color();

//         let mut clicked = false;
//         let mut resizing = false;

//         let _result = self.event_loop.run(move |event, elwt| {
// let mut ui = ctx.app.ui(render_manager.frame());
// match event {
//     Event::WindowEvent { event, window_id } if window_id == render_manager.frame().window.id() => {
//         let cursor = Cursor::get(&render_manager.frame().window);
//         match &mut ui {
//             Some(component) => {

//             }
//             None => todo!(),
//         }

//                     match event {
//                         WindowEvent::Resized(size) => {
//                             resizing = true;
//                             render_manager.resize(size.width,size.height);
//                         }
//                         WindowEvent::CloseRequested => elwt.exit(),
//                         WindowEvent::RedrawRequested => {
//                             render_manager.begin();
//                             if let Some(e) =
//                                 ctx.app
//                                     .message(ApplicationEvent::RedrawRequested, None, &render_manager.frame())
//                             {
//                             }
//                             match &mut ui {
//                                 Some(ref mut component) => {
//                                     for comp in &component.inner {
//                                         render_manager.register(&comp.view());
//                                     }
//                                 }
//                                 None => {}
//                             };
//                             if let Some(menu) = ctx.app.menu() {
//                                 render_manager.register(&menu.view(&render_manager.frame.window));
//                                 //cde.draw_menu(&frame.window, menu);
//                             }
//                             render_manager.write();

//                             if !resizing {
//                                 std::thread::sleep(Duration::from_millis(130));
//                             }

//                             resizing = false;
//                             clicked = false;

//                             render_manager.frame().window.pre_present_notify();
//                         }

//                         _ => {}
//                     }
//                 }

//                 Event::AboutToWait => {
//                     render_manager.frame().window.request_redraw();
//                 }

//                 _ => {}
//             }
//         });
//     }
// }

// impl Default for Executable {
//     fn default() -> Self {
//         Self::new()
//     }
// }

#[derive(Clone, Copy, Debug)]
pub struct Theme {
    bgr: Color,
}

impl Theme {
    pub const ORIGINAL: Theme = Theme { bgr: Color::White };
    pub const DARK: Theme = Theme {
        bgr: Color::ARGB(255, 72, 72, 72),
    };
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

pub struct Window<M>
where
    M: Send + std::fmt::Debug,
{
    pub(crate) inner: winit::window::Window,
    event_loop: Option<EventLoop<()>>,
    comp: Component<M>,
}

impl<M> Window<M>
where
    M: Send + std::fmt::Debug,
{
    pub fn new(ui: Component<M>) -> Self {
        let event_loop = EventLoop::new().unwrap();

        let inner = WindowBuilder::new()
            .with_title("None")
            .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0))
            .build(&event_loop)
            .unwrap();

        Self {
            inner,
            event_loop: Some(event_loop),
            comp: ui,
        }
    }
}

pub struct IApplicationBuilder<M>
where
    M: Send + Copy + std::fmt::Debug,
{
    window: Option<Window<M>>,
    theme: Option<Theme>,
}

impl<M> IApplicationBuilder<M>
where
    M: Send + Copy + std::fmt::Debug,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with(mut self, window: crate::Window<M>) -> Self {
        self.window = Some(window);
        self
    }

    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = Some(theme);
        self
    }

    pub fn build(self) -> IApplication<M> {
        let window = self.window.unwrap();
        let theme = match self.theme {
            Some(t) => t,
            None => Theme::default(),
        };
        let render_manager = RenderManager::new(&window, theme);
        IApplication {
            window,
            render_manager,
        }
    }
}

impl<M> Default for IApplicationBuilder<M>
where
    M: Send + Copy + std::fmt::Debug,
{
    fn default() -> Self {
        Self {
            window: None,
            theme: None,
        }
    }
}

pub enum WindowEvent<M>
where
    M: Send + Copy + std::fmt::Debug,
{
    Resized,
    WidgetEvent(M),
}

pub struct IApplication<M>
where
    M: Send + Copy + std::fmt::Debug,
{
    window: Window<M>,
    render_manager: RenderManager<M>,
}

impl<M> IApplication<M>
where
    M: Send + Copy + std::fmt::Debug + 'static,
{
    pub fn run<F>(mut self, mut callback: F)
    where
        F: FnMut(crate::WindowEvent<M>),
    {
        let event_loop = self.window.event_loop.unwrap();

        self.render_manager.set_background_color();

        let mut is_enter_cursor = false;

        let mut component = self.window.comp;

        event_loop
            .run(move |event, elwt| {
                // Check if the cursor is over the widget
                if is_enter_cursor {
                    for comp in &mut component.inner {
                        if comp.is_capture_event() {
                            let cursor = Cursor::get(&self.window.inner);
                            let x = cursor.x();
                            let y = cursor.y();
                            if x > 0 && y > 0 {
                                let area = comp.area();
                                for area in area {
                                    let cx = (area.left) as i32;
                                    let cy = (area.top) as i32;
                                    let width = (area.right - area.left) as i32;
                                    let height = (area.bottom - area.top) as i32;
                                    if x >= cx && x <= cx + width {
                                        if y >= cy && y <= cy + height {
                                            comp.message(widget::ClientMessage::OnHover);
                                            if unsafe { GetAsyncKeyState(VK_LBUTTON) != 0 } {
                                                comp.message(widget::ClientMessage::OnClick);
                                                match comp.on_click() {
                                                    Some(e) => {
                                                        callback(WindowEvent::WidgetEvent(e));
                                                    }
                                                    None => {}
                                                }
                                            }
                                        }
                                    } else {
                                        comp.message(widget::ClientMessage::Unfocus);
                                    }
                                }
                            }
                            // Cursor is out of window range
                            else {
                                //(*state).event = WidgetEvent::None;
                                comp.message(widget::ClientMessage::Unfocus);
                            }
                        }
                    }
                }

                match event {
                    winit::event::Event::WindowEvent {
                        window_id: _,
                        event,
                    } => match event {
                        winit::event::WindowEvent::RedrawRequested => {
                            self.render_manager.begin();
                            for i in &component.inner {
                                self.render_manager.register(&i.view());
                            }
                            self.render_manager.write();
                            self.window.inner.pre_present_notify();
                        }

                        winit::event::WindowEvent::Resized(_) => {
                            self.render_manager.resize(0, 0);
                            callback(WindowEvent::Resized);
                        }

                        winit::event::WindowEvent::CursorLeft { device_id } => {
                            is_enter_cursor = false;
                        }

                        winit::event::WindowEvent::CursorEntered { device_id } => {
                            is_enter_cursor = true;
                        }

                        _ => {}
                    },

                    winit::event::Event::AboutToWait => {
                        self.window.inner.request_redraw();
                    }

                    _ => {}
                }
            })
            .unwrap();
    }
}

// pub trait Application<M>: Sized
// where
//     M: Send + std::fmt::Debug,
// {
//     fn theme(&self) -> Theme {
//         Theme::default()
//     }

//     fn set_up(&mut self, _frame: &Frame) {}

//     fn message(
//         &mut self,
//         _event: ApplicationEvent,
//         _message: Option<M>,
//         _frame: &Frame,
//     ) -> Option<ApplicationResponse> {
//         None
//     }

//     /// Specify the application menu.
//     /// By default, None is returned and no menu is added.
//     fn menu(&self) -> Option<&Menubar> {
//         None
//     }

//     /// Passes the UI to be displayed in the application.
//     fn ui(&mut self, frame: &Frame) -> Option<Component<M>>;
// }
