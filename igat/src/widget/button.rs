use crate::Color;

use super::{Widget, WidgetType, ClientMessage};

#[derive(Debug)]
pub struct NewButton<M> 
where
    M: Send + std::fmt::Debug
{
    width: u32,
    height: u32,
    x: u32,
    y: u32,
    text: String,
    on_click: Option<M>
}

impl<M> Widget<M> for NewButton<M> 
where
    M: Send + std::fmt::Debug
{
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn x(&self) -> u32 {
        self.x
    }

    fn y(&self) -> u32 {
        self.y
    }

    fn color(&self) -> Color {
        Color::Black
    }

    fn background_color(&self) -> Color {
        Color::White
    }

    fn widget_type(&self) -> WidgetType {
        WidgetType::Rectangle
    }

    fn title(&self) -> &str {
        &self.text
    }

    fn on_click(&self) -> &Option<M> {
        &self.on_click
    }

    fn message(&self,msg: ClientMessage) {

    }
}