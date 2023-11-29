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

impl<M> NewButton<M>
where
    M: Send + std::fmt::Debug
{
    pub fn new() -> Self {
        Self {
            width: 1,
            height: 1,
            x: 1,
            y: 1,
            text: String::new(),
            on_click: None
        }
    }

    pub fn width(mut self,width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self,height: u32) -> Self {
        self.height = height;
        self
    }

    pub fn x(mut self,x: u32) -> Self {
        self.x = x;
        self
    }

    pub fn y(mut self,y: u32) -> Self {
        self.y = y;
        self
    }
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