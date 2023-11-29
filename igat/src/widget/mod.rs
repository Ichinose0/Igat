mod button;

pub use button::*;

use crate::Color;

pub enum ClientMessage {
    OnClick
}

pub enum WidgetType {
    Rectangle,
    Circle,
    Text,
}

pub struct Component<M> {
    pub(crate) inner: Box<dyn Widget<M>>
}

pub trait Widget<M> 
where
    M: Send + std::fmt::Debug
{
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn x(&self) -> u32;
    fn y(&self) -> u32;
    fn color(&self) -> Color;
    fn background_color(&self) -> Color;
    fn shadow(&self) -> Shadow {
        Shadow {
            color: Color::ARGB(255, 128, 128, 128),
            border: 2,
        }
    }
    fn widget_type(&self) -> WidgetType;
    fn title(&self) -> &str;

    fn on_click(&self) -> &Option<M>;

    fn message(&self,msg: ClientMessage);
}

pub struct Shadow {
    pub(crate) color: Color,
    pub(crate) border: u32,
}


pub fn build_component<T,M>(widget: T) -> Component<T>
where
    M: Send + std::fmt::Debug,
    T: Widget<M>
{
    Component {
        inner: widget
    }
}