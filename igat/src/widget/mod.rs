mod button;
mod text;

pub use button::*;
pub use text::*;

use crate::Color;

pub enum WidgetType {
    Rectangle,
    Circle,
    Text
}

pub struct Target<T>
{
    pub(crate) inner: Vec<Element<T>>
}

impl<T> Target<T>
{
    pub fn get(&self) -> &[Element<T>] {
        &self.inner
    }
}

pub struct Element<T> {
    pub(crate)widget: Box<dyn Widget>,
    pub(crate)msg: Option<T>
}

impl<T> Element<T> {
    pub fn get(&self) -> Vec<&Box<dyn Widget>> {
        vec![&self.widget]
    }
}

pub trait Widget {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn x(&self) -> u32;
    fn y(&self) -> u32;
    fn color(&self) -> Color;
    fn background_color(&self) -> Color;
    fn shadow(&self) -> Shadow {
        Shadow {
            color: Color::ARGB(255,128,128,128),
            border: 1
        }
    }
    fn widget_type(&self) -> WidgetType;
    fn title(&self) -> &str;
}

pub struct Shadow {
    pub(crate) color: Color,
    pub(crate) border: u32
}

// pub struct Container<T> 
// where
//     T: Send + std::fmt::Debug
// {
//     on_click: Option<T>
// }

// impl<T> Container<T>
// where
//     T: Send + std::fmt::Debug
// {
//     pub fn new() -> Self {
//         Self {
//             on_click: None
//         }
//     }

//     pub fn build(self) -> Element<T> {
//         Element {
//             widget: Box::new(_Container {}),
//             msg: None,
//         }
//     }
// }

// pub struct _Container {}

// impl Widget<T> for _Container {
//     fn width(&self) -> u32 {
//         30
//     }

//     fn height(&self) -> u32 {
//         30
//     }

//     fn color(&self) -> Color {
//         Color::White
//     }

//     fn widget_type(&self) -> WidgetType {
//         WidgetType::Rectangle
//     }
// }

