mod button;
mod text;

pub use button::*;
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
    pub(crate) inner: Box<dyn Widget<M>>,
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

pub fn build_component<M, T>(widget: T) -> Component<M>
where
    M: Send + Copy + std::fmt::Debug,
    T: Widget<M> + 'static,
{
    Component {
        inner: Box::new(widget),
    }
}
