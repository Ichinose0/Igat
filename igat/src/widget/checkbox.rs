use std::{cell::RefCell, marker::PhantomData, time::Duration};

use acure::Command;

use crate::{cursor::Cursor, Color, CursorIcon, Rect};

use super::{ColorPair, Data, Layout, Property, Widget, WidgetMessage};

const HOVERED_COLOR: ColorPair = ColorPair {
    color: Color::Black,
    bgr: Color::White,
    shadow: Color::ARGB(255, 0, 170, 204),
};

const CLICKED_COLOR: ColorPair = ColorPair {
    color: Color::Black,
    bgr: Color::ARGB(255, 200, 200, 200),
    shadow: Color::ARGB(255, 0, 70, 204),
};

const NORMAL_COLOR: ColorPair = ColorPair {
    color: Color::Black,
    bgr: Color::White,
    shadow: Color::ARGB(255, 128, 128, 128),
};

#[derive(Debug)]
pub struct CheckboxProperty {
    pub hovered_color: ColorPair,
    pub clicked_color: ColorPair,
    pub color: ColorPair,
    pub width: u32,
    pub height: u32,
    pub x: u32,
    pub y: u32,

    pub is_check: bool,
}

#[derive(Debug)]
pub struct Checkbox<E, D>
where
    E: Fn(WidgetMessage, &mut CheckboxProperty, &mut D),
    D: Data,
{
    on_message: RefCell<Option<E>>,
    current_color: ColorPair,
    property: CheckboxProperty,
    phantom: PhantomData<D>,
}

impl<E, D> Checkbox<E, D>
where
    E: Fn(WidgetMessage, &mut CheckboxProperty, &mut D),
    D: Data,
{
    pub fn new() -> Self {
        Self {
            on_message: RefCell::new(None),
            current_color: NORMAL_COLOR,
            property: CheckboxProperty {
                hovered_color: HOVERED_COLOR,
                clicked_color: CLICKED_COLOR,
                color: NORMAL_COLOR,
                width: 80,
                height: 30,
                x: 0,
                y: 0,
                is_check: false,
            },

            phantom: PhantomData,
        }
    }

    pub fn width(mut self, width: u32) -> Self {
        self.property.width = width;
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.property.height = height;
        self
    }

    pub fn x(mut self, x: u32) -> Self {
        self.property.x = x;
        self
    }

    pub fn y(mut self, y: u32) -> Self {
        self.property.y = y;
        self
    }

    pub fn on_message(mut self, on_message: E) -> Self {
        self.on_message = RefCell::new(Some(on_message));
        self
    }
}

impl<E, D> Layout for Checkbox<E, D>
where
    E: Fn(WidgetMessage, &mut CheckboxProperty, &mut D),
    D: Data,
{
    fn area(&self) -> Vec<Rect> {
        vec![Rect {
            left: self.property.x,
            top: self.property.y,
            right: self.property.x + self.property.width,
            bottom: self.property.y + self.property.height,
        }]
    }

    fn is_capture_event(&self) -> bool {
        true
    }

    fn theme(&mut self, theme: crate::Theme) {
        self.property.clicked_color = theme.click;
        self.property.hovered_color = theme.hover;
        self.property.color = theme.normal;
    }
}

impl<E, D> Widget<D> for Checkbox<E, D>
where
    E: Fn(WidgetMessage, &mut CheckboxProperty, &mut D),
    D: Data,
{
    fn view(&self) -> Vec<acure::Command> {
        let text = if self.property.is_check { "✔" } else { " " };
        let y = self.property.y;
        vec![
            Command::FillRectangle(
                self.property.x,
                y,
                self.property.width,
                self.property.height,
                4.2,
                self.current_color.shadow.into(),
            ),
            Command::FillRectangle(
                self.property.x + 1,
                y + 1,
                self.property.width - 2,
                self.property.height - 2,
                4.2,
                self.current_color.bgr.into(),
            ),
            Command::WriteString(
                self.property.x,
                y,
                self.property.width,
                self.property.height,
                self.current_color.color.into(),
                text.to_owned(),
            ),
        ]
    }

    fn message(&mut self, msg: WidgetMessage, data: &mut D) {
        match msg {
            WidgetMessage::OnClick => {
                self.current_color = self.property.clicked_color;
                if self.property.is_check {
                    self.property.is_check = false;
                } else {
                    self.property.is_check = true;
                }
            }
            WidgetMessage::OnHover => {
                self.current_color = self.property.hovered_color;
            }
            WidgetMessage::Unfocus => {
                self.current_color = self.property.color;
            }
        }
        let mut e = self.on_message.borrow_mut();
        e.as_mut().unwrap()(msg, &mut self.property, data);
    }
}
