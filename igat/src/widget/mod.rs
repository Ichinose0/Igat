mod button;
mod text;

pub use button::*;
pub use text::*;

use crate::Color;

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
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn x(&self) -> u32;
    fn y(&self) -> u32;

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
