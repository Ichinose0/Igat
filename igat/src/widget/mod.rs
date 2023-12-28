mod button;
mod checkbox;
mod label;
mod panel;

pub use button::*;
pub use checkbox::*;
pub use label::*;
pub use panel::*;
use winit::window::CursorIcon;

use crate::{cursor::Cursor, Color, Rect, Theme};

#[derive(Debug)]
pub struct Property {
    pub hovered_color: ColorPair,
    pub clicked_color: ColorPair,
    pub color: ColorPair,
    pub width: u32,
    pub height: u32,
    pub x: u32,
    pub y: u32,

    pub text: String,
}

#[derive(Clone, Copy, Debug)]
pub struct ColorPair {
    pub color: Color,
    pub bgr: Color,
    pub shadow: Color,
}

impl ColorPair {
    pub fn new(color: Color, bgr: Color, shadow: Color) -> Self {
        Self { color, bgr, shadow }
    }
}

pub struct RenderConfig {
    pub thickness: u32,
    pub border_radius: f64,
}

#[derive(Debug)]
pub enum WidgetMessage {
    OnClick,
    OnHover,
    Unfocus,
}

pub struct Component<D>
where
    D: Data,
{
    pub(crate) inner: Vec<Box<dyn Widget<D>>>,
    pub(crate) static_data: D,
}

pub trait Container<D>: Layout
where
    D: Data,
{
    fn format(&mut self);
    fn childrens(&mut self) -> &mut [Box<dyn Widget<D>>];
}

pub trait Layout {
    fn area(&self) -> Vec<Rect>;

    fn theme(&mut self, theme: Theme);

    fn cursor(&self) -> CursorIcon {
        CursorIcon::default()
    }

    fn is_capture_event(&self) -> bool;

    fn x(&mut self, x: u32);
    fn y(&mut self, y: u32);
    fn width(&mut self, width: u32);
    fn height(&mut self, height: u32);
}

pub trait Widget<D>: Layout
where
    D: Data,
{
    fn view(&self) -> Vec<acure::Command>;

    fn message(&mut self, msg: WidgetMessage, data: &mut D);
}

pub trait Data {}
