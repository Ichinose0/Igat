mod button;
mod label;
mod panel;
mod text;

pub use button::*;
pub use label::*;
pub use panel::*;
pub use text::*;

use crate::{Color, Rect, Theme};

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

#[derive(Clone,Copy,Debug)]
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

pub struct Component {
    pub(crate) inner: Vec<Box<dyn Widget>>,
}

pub trait Layout {
    fn area(&self) -> Vec<Rect>;

    fn theme(&mut self, theme: Theme);

    fn is_capture_event(&self) -> bool;
}

pub trait Widget: Layout {
    fn view(&self) -> Vec<acure::Command>;

    fn message(&mut self, msg: WidgetMessage);
}

pub struct ContentPanel {
    pub(crate) widgets: Vec<Box<dyn Widget>>,
}
