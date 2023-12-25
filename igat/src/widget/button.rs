use std::cell::RefCell;

use acure::Command;

use crate::{Color, Rect};

use super::{ClientMessage, Layout, Widget};

#[derive(Debug)]
pub struct Button<E>
where
    E: Fn(ClientMessage)
{
    width: u32,
    height: u32,
    menu_height: u32,
    x: u32,
    y: u32,
    color: Color,
    background_color: Color,
    shadow_color: Color,
    text: String,
    on_message: RefCell<Option<E>>
}

impl<E> Button<E>
where
    E: Fn(ClientMessage)
{
    pub fn new() -> Self {
        Self {
            width: 30,
            height: 80,
            menu_height: 0,
            x: 0,
            y: 0,
            text: String::new(),
            on_message: RefCell::new(None),
            color: Color::Black,
            background_color: Color::White,
            shadow_color: Color::ARGB(255, 128, 128, 128),
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
        self.on_message = RefCell::new(Some(on_message));
        self
    }
}

impl<E> Layout for Button<E>
where
    E: Fn(ClientMessage)
{
    fn area(&self) -> Vec<Rect> {
        vec![Rect {
            left: self.x,
            top: self.y + self.menu_height,
            right: self.x + self.width,
            bottom: self.y + self.menu_height + self.height,
        }]
    }

    fn is_capture_event(&self) -> bool {
        true
    }

    fn theme(&mut self, theme: crate::Theme) {}
}

impl<E> Widget for Button<E>
where
    E: Fn(ClientMessage)
{
    fn view(&self) -> Vec<acure::Command> {
        let y = self.y + self.menu_height;
        vec![
            Command::FillRectangle(
                self.x,
                y,
                self.width,
                self.height,
                4.2,
                self.shadow_color.into(),
            ),
            Command::FillRectangle(
                self.x + 1,
                y + 1,
                self.width - 2,
                self.height - 2,
                4.2,
                self.background_color.into(),
            ),
            Command::WriteString(
                self.x,
                y,
                self.width,
                self.height,
                self.color.into(),
                self.text.clone(),
            ),
        ]
    }

    fn message(&mut self, msg: ClientMessage) {
        self.background_color = Color::White;
        match msg {
            ClientMessage::OnClick => {
                self.background_color = Color::ARGB(255, 200, 200, 200);
                self.shadow_color = Color::ARGB(255, 0, 70, 204);
                let mut e = self.on_message.borrow_mut();
                e.as_mut().unwrap()(msg);
            }
            ClientMessage::OnHover => {
                self.background_color = Color::ARGB(255, 255, 255, 255);
                self.shadow_color = Color::ARGB(255, 0, 170, 204);
            }
            ClientMessage::Unfocus => {
                self.background_color = Color::ARGB(255, 255, 255, 255);
                self.shadow_color = Color::ARGB(255, 128, 128, 128);
            }
        }
    }
}
