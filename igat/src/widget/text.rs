use crate::Color;

use super::{ClientMessage, Widget, WidgetType};

#[derive(Debug)]
pub struct Text<M>
where
    M: Send + std::fmt::Debug,
{
    width: u32,
    height: u32,
    x: u32,
    y: u32,
    background_color: Color,
    text: String,
    on_click: Option<M>,
}

impl<M> Text<M>
where
    M: Send + std::fmt::Debug,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    pub fn x(mut self, x: u32) -> Self {
        self.x = x;
        self
    }

    pub fn y(mut self, y: u32) -> Self {
        self.y = y;
        self
    }

    pub fn on_click(mut self, on_click: M) -> Self {
        self.on_click = Some(on_click);
        self
    }
}

impl<M> Default for Text<M>
where
    M: Send + std::fmt::Debug,
{
    fn default() -> Self {
        Self {
            width: 1,
            height: 1,
            x: 1,
            y: 1,
            text: String::new(),
            on_click: None,
            background_color: Color::White,
        }
    }
}

impl<M> Widget<M> for Text<M>
where
    M: Send + Copy + std::fmt::Debug,
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
        self.background_color
    }

    fn widget_type(&self) -> WidgetType {
        WidgetType::Text
    }

    fn title(&self) -> &str {
        &self.text
    }

    fn on_click(&self) -> Option<M> {
        self.on_click
    }

    fn message(&mut self, msg: ClientMessage) { }

    fn is_capture_event(&self) -> bool {
        false
    }
}
