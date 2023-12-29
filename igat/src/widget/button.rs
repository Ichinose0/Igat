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
pub struct Button<E, D>
where
    E: Fn(WidgetMessage, &mut Property, &mut D),
    D: Data,
{
    on_message: RefCell<Option<E>>,
    current_color: ColorPair,
    property: Property,
    phantom: PhantomData<D>,
}

impl<E, D> Button<E, D>
where
    E: Fn(WidgetMessage, &mut Property, &mut D),
    D: Data,
{
    pub fn new() -> Self {
        Self {
            on_message: RefCell::new(None),
            current_color: NORMAL_COLOR,
            property: Property {
                hovered_color: HOVERED_COLOR,
                clicked_color: CLICKED_COLOR,
                color: NORMAL_COLOR,
                width: 80,
                height: 30,
                x: 0,
                y: 0,
                text: Default::default(),
            },
            phantom: PhantomData,
        }
    }

    pub fn text(&mut self, text: String) {
        self.property.text = text;
    }

    pub fn on_message(&mut self, on_message: E) {
        self.on_message = RefCell::new(Some(on_message));
    }
}

impl<E, D> Layout for Button<E, D>
where
    E: Fn(WidgetMessage, &mut Property, &mut D),
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

    fn x(&mut self, x: u32) {
        self.property.x = x;
    }

    fn y(&mut self, y: u32) {
        self.property.y = y;
    }

    fn width(&mut self, width: u32) {
        self.property.width = width;
    }

    fn height(&mut self, height: u32) {
        self.property.height = height;
    }
}

impl<E, D> Widget<D> for Button<E, D>
where
    E: Fn(WidgetMessage, &mut Property, &mut D),
    D: Data,
{
    fn view(&self) -> Vec<acure::Command> {
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
                self.property.text.clone(),
            ),
        ]
    }

    fn message(&mut self, msg: WidgetMessage, data: &mut D) {
        match msg {
            WidgetMessage::OnClick => {
                self.current_color = self.property.clicked_color;
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
