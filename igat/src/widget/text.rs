use acure::Command;

use crate::{Color, Rect};

use super::{Layout, Widget, WidgetMessage};

#[derive(Debug)]
pub struct Text<E>
where
    E: Fn(WidgetMessage),
{
    width: u32,
    height: u32,
    menu_height: u32,
    x: u32,
    y: u32,
    color: Color,
    background_color: Color,
    text: String,
    on_message: Option<E>,
}

impl<E> Text<E>
where
    E: Fn(WidgetMessage),
{
    pub fn new() -> Self {
        Self {
            width: 30,
            height: 80,
            menu_height: 0,
            x: 0,
            y: 0,
            text: String::new(),
            on_message: None,
            color: Color::Black,
            background_color: Color::White,
        }
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

    pub fn text(mut self, text: String) -> Self {
        self.text = text;
        self
    }

    pub fn on_message(mut self, on_message: E) -> Self {
        self.on_message = Some(on_message);
        self
    }
}

impl<E> Layout for Text<E>
where
    E: Fn(WidgetMessage),
{
    fn area(&self) -> Vec<Rect> {
        vec![Rect {
            left: self.x,
            top: self.y + self.menu_height,
            right: self.x + self.width,
            bottom: self.y + self.menu_height + self.height,
        }]
    }

    fn theme(&mut self, theme: crate::Theme) {}

    fn is_capture_event(&self) -> bool {
        false
    }
}

impl<E> Widget for Text<E>
where
    E: Fn(WidgetMessage),
{
    fn message(&mut self, _msg: WidgetMessage) {}

    fn view(&self) -> Vec<acure::Command> {
        vec![Command::WriteString(
            self.x,
            self.y,
            self.width,
            self.height,
            self.color.into(),
            self.text.clone(),
        )]
    }
}
