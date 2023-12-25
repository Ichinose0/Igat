mod button;
mod label;
mod panel;
mod text;

pub use button::*;
pub use label::*;
pub use panel::*;
pub use text::*;

use crate::{Rect, Theme};

pub struct RenderConfig {
    pub thickness: u32,
    pub border_radius: f64,
}

#[derive(Debug)]
pub enum ClientMessage {
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

    fn message(&mut self, msg: ClientMessage);
}

pub struct ContentPanel {
    pub(crate) widgets: Vec<Box<dyn Widget>>,
}
