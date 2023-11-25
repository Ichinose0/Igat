use crate::Color;

use super::{Element, Widget, WidgetType, Target};

pub struct Text<T> 
where
    T: Send + std::fmt::Debug
{
    pub(crate) inner: _Text<T>
}

impl<T> Text<T>
where
    T: Send + std::fmt::Debug + 'static
{
    pub fn new() -> Self {
        Text::default()
    }

    pub fn text(mut self,text: String) -> Self {
        self.inner.text = text;
        self
    }

    pub fn width(mut self,width: u32) -> Self {
        self.inner.width = width;
        self
    }

    pub fn height(mut self,height: u32) -> Self {
        self.inner.height = height;
        self
    }

    pub fn x(mut self,x: u32) -> Self {
        self.inner.x = x;
        self
    }

    pub fn y(mut self,y: u32) -> Self {
        self.inner.y = y;
        self
    }
    
    pub fn element(self) -> Element<T> {
        Element {
            widget: Box::new(self.inner),
            msg: None
        }
    }

    pub fn build(self) -> Target<T> {
        Target {
            inner: vec![
                Element {
                    widget: Box::new(self.inner),
                    msg: None
                }
            ]
        }
    }
}

impl<T> Default for Text<T> 
where
    T: Send + std::fmt::Debug
{
    fn default() -> Self {
        Self {
            inner: _Text { on_click: Default::default(), text: "".to_owned(), width: 120, height: 40, color: Color::Black, background_color: Color::White, x: 30, y: 30 },
        }
    }
}

struct _Text<T> 
where
    T: Send + std::fmt::Debug 
{
    text: String,
    width: u32,
    height: u32,
    x: u32,
    y: u32,
    color: Color,
    background_color: Color,
    on_click: Option<T>
}

impl<T> Widget for _Text<T> 
where
    T: Send + std::fmt::Debug
{
    fn width(&self) -> u32 {
        self.width
    }

    fn x(&self) -> u32 {
        self.x
    }

    fn y(&self) -> u32 {
        self.y
    }

    fn color(&self) -> Color {
        self.color
    }

    fn background_color(&self) -> Color {
        self.background_color
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn widget_type(&self) -> WidgetType {
        WidgetType::Text
    }

    fn title(&self) -> &str {
        &self.text
    }
}