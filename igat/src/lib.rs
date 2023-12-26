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
use widget::{Button, ColorPair, Component, Data, RenderConfig};
use winapi::um::winuser::{GetAsyncKeyState, VK_LBUTTON};

//use cde::{Cde, RenderManager};
use winit::{event_loop::EventLoop, window::WindowBuilder};

pub type CursorIcon = winit::window::CursorIcon;
pub type WindowTheme = winit::window::Theme;

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

#[derive(Clone, Copy, Debug)]
pub struct Theme {
    hover: ColorPair,
    click: ColorPair,
    normal: ColorPair,
    window: WindowTheme,
    bgr: Color,
}

impl Theme {
    pub const LIGHT: Theme = Theme {
        hover: ColorPair {
            color: Color::Black,
            bgr: Color::White,
            shadow: Color::ARGB(255, 0, 170, 204),
        },
        click: ColorPair {
            color: Color::Black,
            bgr: Color::ARGB(255, 200, 200, 200),
            shadow: Color::ARGB(255, 0, 70, 204),
        },
        normal: ColorPair {
            color: Color::Black,
            bgr: Color::White,
            shadow: Color::ARGB(255, 128, 128, 128),
        },
        window: WindowTheme::Light,
        bgr: Color::ARGB(255, 240, 240, 240),
    };
    pub const DARK: Theme = Theme {
        hover: ColorPair {
            color: Color::White,
            bgr: Color::ARGB(255, 180, 180, 180),
            shadow: Color::ARGB(255, 200, 200, 200),
        },
        click: ColorPair {
            color: Color::White,
            bgr: Color::ARGB(255, 144, 144, 144),
            shadow: Color::White,
        },
        normal: ColorPair {
            color: Color::ARGB(255, 220, 220, 220),
            bgr: Color::ARGB(255, 72, 72, 72),
            shadow: Color::ARGB(255, 200, 200, 200),
        },
        window: WindowTheme::Dark,
        bgr: Color::ARGB(255, 72, 72, 72),
    };
}

impl Theme {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::LIGHT
    }
}

pub struct Window<D>
where
    D: Data,
{
    pub(crate) inner: winit::window::Window,
    event_loop: Option<EventLoop<()>>,
    comp: Component<D>,
}

impl<D> Window<D>
where
    D: Data,
{
    pub fn new(ui: Component<D>) -> Self {
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

pub struct IApplicationBuilder<D>
where
    D: Data,
{
    window: Option<Window<D>>,
    theme: Option<Theme>,
}

impl<D> IApplicationBuilder<D>
where
    D: Data,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with(mut self, window: crate::Window<D>) -> Self {
        self.window = Some(window);
        self
    }

    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = Some(theme);
        self
    }

    pub fn build(self) -> IApplication<D> {
        let window = self.window.unwrap();
        let theme = match self.theme {
            Some(t) => t,
            None => Theme::default(),
        };
        let render_manager = RenderManager::new(&window, theme);
        IApplication {
            window,
            theme,
            render_manager,
        }
    }
}

impl<D> Default for IApplicationBuilder<D>
where
    D: Data,
{
    fn default() -> Self {
        Self {
            window: None,
            theme: None,
        }
    }
}

pub enum WindowEvent {
    Resized,
}

pub struct IApplication<D>
where
    D: Data,
{
    window: Window<D>,
    theme: Theme,
    render_manager: RenderManager,
}

impl<D> IApplication<D>
where
    D: Data,
{
    pub fn run<F>(mut self, mut callback: F)
    where
        F: FnMut(crate::WindowEvent),
    {
        let event_loop = self.window.event_loop.unwrap();

        self.render_manager.set_background_color();

        let mut is_enter_cursor = false;

        let mut component = self.window.comp;
        self.window.inner.set_theme(Some(self.theme.window));

        for comp in &mut component.inner {
            comp.theme(self.theme);
        }

        let mut data = &mut component.static_data;
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
                                            comp.message(widget::WidgetMessage::OnHover, data);
                                            self.window.inner.set_cursor_icon(comp.cursor());
                                            if unsafe { GetAsyncKeyState(VK_LBUTTON) != 0 } {
                                                comp.message(widget::WidgetMessage::OnClick, data);
                                                std::thread::sleep(Duration::from_millis(130));
                                            }
                                        } else {
                                            comp.message(widget::WidgetMessage::Unfocus, data);
                                        }
                                    } else {
                                        comp.message(widget::WidgetMessage::Unfocus, data);
                                    }
                                }
                            }
                            // Cursor is out of window range
                            else {
                                comp.message(widget::WidgetMessage::Unfocus, data);
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

                            self.window.inner.set_cursor_icon(CursorIcon::Default);

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
