use acure::Command;

use crate::{Color, Rect};

use super::{ClientMessage, Widget};

#[derive(Debug)]
pub struct Text<M>
where
    M: Send + std::fmt::Debug,
{
    width: u32,
    height: u32,
    menu_height: u32,
    x: u32,
    y: u32,
    color: Color,
    background_color: Color,
    text: String,
    on_click: Option<M>,
}

impl<M> Text<M>
where
    M: Send + std::fmt::Debug,
{
    pub fn new() -> Self {
        Self {
            width: 30,
            height: 80,
            menu_height: 0,
            x: 0,
            y: 0,
            text: String::new(),
            on_click: None,
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

    pub fn on_click(mut self, on_click: M) -> Self {
        self.on_click = Some(on_click);
        self
    }
}

impl<M> Widget for Text<M>
where
    M: Send + Copy + std::fmt::Debug,
{
    fn area(&self) -> Vec<Rect> {
        vec![Rect {
            left: self.x,
            top: self.y + self.menu_height,
            right: self.x + self.width,
            bottom: self.y + self.menu_height + self.height,
        }]
    }

    fn on_click(&self) -> Option<M> {
        self.on_click
    }

    fn message(&mut self, _msg: ClientMessage) {}

    fn is_capture_event(&self) -> bool {
        false
    }

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
