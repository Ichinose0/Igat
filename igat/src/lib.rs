#[macro_use]
extern crate log;

pub(crate) mod cde;
mod cursor;
pub mod menu;
pub mod widget;

use std::{fmt::Debug, marker::PhantomData, time::Duration};

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

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    left: u32,
    top: u32,
    right: u32,
    bottom: u32,
}

impl Rect {
    pub fn x(&self) -> u32 {
        self.left
    }

    pub fn y(&self) -> u32 {
        self.top
    }

    pub fn width(&self) -> u32 {
        self.right - self.left
    }

    pub fn height(&self) -> u32 {
        self.bottom - self.top
    }
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
    pub fn set_title(&self, title: &str) {
        self.window.set_title(title);
    }

    pub fn set_resizable(&self, resizable: bool) {
        self.window.set_resizable(resizable);
    }

    pub fn set_minimized(&self, minimized: bool) {
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
            bottom: size.height + self.menu_height,
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

    pub fn dispatch_message(&mut self, message: M, frame: &Frame) {
        self.app
            .message(ApplicationEvent::WidgetEvent, Some(message), frame);
    }
}

impl Executable {
    pub fn new() -> Self {
        let event_loop = EventLoop::new().unwrap();

        let window = WindowBuilder::new()
            .with_title("None")
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

        let cde: Cde<M> = Cde::new(&self.window);
        let theme = ctx.app.theme();

        let config = RenderConfig {
            thickness: 0,
            border_radius: 0.0,
        };

        let height = match ctx.app.menu() {
            Some(m) => m.height(),
            None => 0,
        };

        let frame = Frame {
            window: self.window,
            menu_height: height,
        };

        ctx.app.set_up(&frame);

        let mut clicked = false;

        let _result = self.event_loop.run(move |event, elwt| {
            cde.bgr(theme.bgr);
            let mut ui = ctx.app.ui(&frame);

            match event {
                Event::WindowEvent { event, window_id } if window_id == frame.window.id() => {
                    let cursor = Cursor::get(&frame.window);
                    match &mut ui {
                        Some(component) => {
                            if component.inner.is_capture_event() {
                                let x = cursor.x();
                                let y = cursor.y();
                                if x > 0 && y > 0 {
                                    let area = component.inner.area();
                                    for area in area {
                                        let cx = (area.left) as i32;
                                        let cy = (area.top) as i32;
                                        let width = (area.right - area.left) as i32;
                                        let height = (area.bottom - area.top) as i32;
                                        if x >= cx && x <= cx + width {
                                            if y >= cy && y <= cy + height {
                                                component
                                                    .inner
                                                    .message(widget::ClientMessage::OnHover);
                                                if unsafe { GetAsyncKeyState(VK_LBUTTON) != 0 } {
                                                    component
                                                        .inner
                                                        .message(widget::ClientMessage::OnClick);

                                                    if clicked == false {
                                                        match component.inner.on_click() {
                                                            Some(e) => {
                                                                ctx.app.message(
                                                                    ApplicationEvent::WidgetEvent,
                                                                    Some(e),
                                                                    &frame,
                                                                );
                                                                clicked = true;
                                                            }
                                                            None => {}
                                                        }
                                                    }
                                                    cde.draw(&component.inner);
                                                }
                                            }
                                        } else {
                                            component.inner.message(widget::ClientMessage::Unfocus);
                                        }
                                    }
                                }
                                // Cursor is out of window range
                                else {
                                    component.inner.message(widget::ClientMessage::Unfocus);
                                }
                            }
                        }
                        None => todo!(),
                    }

                    match event {
                        WindowEvent::CloseRequested => elwt.exit(),
                        WindowEvent::RedrawRequested => {
                            if let Some(e) =
                                ctx.app
                                    .message(ApplicationEvent::RedrawRequested, None, &frame)
                            {
                            }
                            match &mut ui {
                                Some(ref mut component) => {
                                    cde.draw(&component.inner);
                                }
                                None => {}
                            };
                            if let Some(menu) = ctx.app.menu() {
                                cde.draw_menu(&frame.window, menu);
                            }

                            cde.write();
                            std::thread::sleep(Duration::from_millis(130));
                            clicked = false;

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

    fn theme(&self) -> Theme {
        Theme::default()
    }

    fn set_up(&mut self, _frame: &Frame) {}

    fn message(
        &mut self,
        _event: ApplicationEvent,
        _message: Option<M>,
        _frame: &Frame,
    ) -> Option<ApplicationResponse> {
        None
    }

    fn menu(&self) -> Option<&Menubar> {
        None
    }

    fn ui(&mut self, frame: &Frame) -> Option<Component<M>>;

    #[deprecated(since = "0.0.4", note = "This method is not planned to be used")]
    fn on_close(&self);
}
