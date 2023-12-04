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

pub enum WidgetType {
    Rectangle,
    Circle,
    Text,
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
    #[deprecated(since = "0.0.3", note = "Widgets should be rendered in view()")]
    fn color(&self) -> Color;
    #[deprecated(since = "0.0.3", note = "Widgets should be rendered in view()")]
    fn background_color(&self) -> Color;
    #[deprecated(since = "0.0.3", note = "Widgets should be rendered in view()")]
    fn shadow(&self) -> Shadow {
        Shadow {
            color: Color::ARGB(255, 128, 128, 128),
            border: 2,
        }
    }
    #[deprecated(since = "0.0.3", note = "Widgets should be rendered in view()")]
    fn widget_type(&self) -> WidgetType;
    #[deprecated(since = "0.0.3", note = "Widgets should be rendered in view()")]
    fn title(&self) -> &str;

    fn view(&self) -> Vec<acure::Command>;

    fn on_click(&self) -> Option<M>;

    fn message(&mut self, msg: ClientMessage);

    fn is_capture_event(&self) -> bool;
}

#[deprecated(since = "0.0.3", note = "Widgets should be rendered in view()")]
pub struct Shadow {
    pub(crate) color: Color,
    pub(crate) border: u32,
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
