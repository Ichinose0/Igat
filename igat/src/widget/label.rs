use std::{cell::RefCell, marker::PhantomData};

use acure::Command;

use crate::{Color, CursorIcon, Rect};

use super::{ColorPair, Data, Layout, Property, Widget, WidgetMessage};

const NORMAL_COLOR: ColorPair = ColorPair {
    color: Color::Black,
    bgr: Color::White,
    shadow: Color::ARGB(255, 128, 128, 128),
};

#[derive(Debug)]
pub struct Label<E, D>
where
    E: Fn(WidgetMessage, &mut Property, &mut D),
    D: Data,
{
    on_message: RefCell<Option<E>>,
    current_color: ColorPair,
    property: Property,
    phantom: PhantomData<D>,
}

impl<E, D> Label<E, D>
where
    E: Fn(WidgetMessage, &mut Property, &mut D),
    D: Data,
{
    pub fn new() -> Self {
        Self {
            property: Property {
                hovered_color: NORMAL_COLOR,
                clicked_color: NORMAL_COLOR,
                color: NORMAL_COLOR,
                width: 0,
                height: 0,
                x: 0,
                y: 0,
                text: String::new(),
            },
            current_color: NORMAL_COLOR,
            on_message: RefCell::new(None),
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

impl<E, D> Layout for Label<E, D>
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

    fn theme(&mut self, theme: crate::Theme) {
        self.current_color = theme.normal;
    }

    fn is_capture_event(&self) -> bool {
        true
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

impl<E, D> Widget<D> for Label<E, D>
where
    E: Fn(WidgetMessage, &mut Property, &mut D),
    D: Data,
{
    fn message(&mut self, msg: WidgetMessage, data: &mut D) {
        let mut e = self.on_message.borrow_mut();
        e.as_mut().unwrap()(msg, &mut self.property, data);
    }

    fn view(&self) -> Vec<acure::Command> {
        vec![Command::WriteString(
            self.property.x,
            self.property.y,
            self.property.width,
            self.property.height,
            self.current_color.color.into(),
            self.property.text.clone(),
        )]
    }
}
