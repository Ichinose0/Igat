use acure::Command;

use crate::Color;

use super::{ClientMessage, Widget, WidgetType};

#[derive(Debug)]
pub struct Button<M>
where
    M: Send + std::fmt::Debug,
{
    width: u32,
    height: u32,
    x: u32,
    y: u32,
    color: Color,
    background_color: Color,
    shadow_color: Color,
    text: String,
    on_click: Option<M>,
}

impl<M> Button<M>
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

    pub fn text(mut self, text: String) -> Self {
        self.text = text;
        self
    }

    pub fn on_click(mut self, on_click: M) -> Self {
        self.on_click = Some(on_click);
        self
    }
}

impl<M> Default for Button<M>
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
            color: Color::Black,
            background_color: Color::White,
            shadow_color: Color::ARGB(255, 128, 128, 128),
        }
    }
}

impl<M> Widget<M> for Button<M>
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
        WidgetType::Rectangle
    }

    fn title(&self) -> &str {
        &self.text
    }

    fn view(&self) -> Vec<acure::Command> {
        vec![
            Command::FillRectangle(
                self.x,
                self.y,
                self.width,
                self.height,
                self.shadow_color.into(),
            ),
            Command::FillRectangle(
                self.x + 1,
                self.y + 1,
                self.width - 2,
                self.height - 2,
                self.background_color.into(),
            ),
            Command::WriteString(
                self.x,
                self.y,
                self.width,
                self.height,
                self.color.into(),
                self.text.clone(),
            ),
        ]
    }

    fn on_click(&self) -> Option<M> {
        self.on_click
    }

    fn message(&mut self, msg: ClientMessage) {
        self.background_color = Color::White;
        match msg {
            ClientMessage::OnClick => {
                self.background_color = Color::ARGB(255, 200, 200, 200);
                self.shadow_color = Color::ARGB(255,0, 70, 204);
            }
            ClientMessage::OnHover => {
                self.background_color = Color::ARGB(255,255, 255, 255);
                self.shadow_color = Color::ARGB(255,0, 170, 204);
            }
            ClientMessage::Unfocus => {
                self.background_color = Color::ARGB(255, 255, 255, 255);
                self.shadow_color = Color::ARGB(255, 128, 128, 128);
            }
        }
    }

    fn is_capture_event(&self) -> bool {
        true
    }
}
