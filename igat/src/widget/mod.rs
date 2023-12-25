mod button;
mod label;
mod panel;
mod text;

pub use button::*;
pub use label::*;
pub use panel::*;
pub use text::*;

use crate::Rect;

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

pub struct Component<M>
where
    M: Send + std::fmt::Debug,
{
    pub(crate) inner: Vec<Box<dyn Widget<M>>>,
}

pub trait Widget<M>: Send + std::fmt::Debug
where
    M: Send + std::fmt::Debug,
{
    fn area(&self) -> Vec<Rect>;

    fn view(&self) -> Vec<acure::Command>;

    fn on_click(&self) -> Option<M>;

    fn message(&mut self, msg: ClientMessage);

    fn is_capture_event(&self) -> bool;
}

#[deprecated(since = "0.0.4", note = "Make sure to use the Panel widget")]
pub fn build_component<M, T>(widget: T) -> Component<M>
where
    M: Send + Copy + std::fmt::Debug,
    T: Widget<M> + 'static,
{
    Component {
        inner: vec![Box::new(widget)],
    }
}

pub struct ContentPanel<M>
where
    M: Send + std::fmt::Debug,
{
    pub(crate) widgets: Vec<Box<dyn Widget<M>>>,
}

impl<M> ContentPanel<M> where M: Send + Copy + std::fmt::Debug {}
