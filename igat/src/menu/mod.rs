use winit::window::Window;

use crate::Color;

pub struct Menubar {
    background_color: Color,
    children: Vec<Menu>,
}

impl Menubar {
    pub fn new() -> Self {
        Self {
            background_color: Color::White,
            children: vec![],
        }
    }

    pub fn add(&mut self, mut menu: Menu) {
        menu.height = self.height();
        self.children.push(menu)
    }

    pub fn height(&self) -> u32 {
        19
    }

    pub(crate) fn view(&self, window: &Window) -> Vec<acure::Command> {
        let size = window.inner_size();
        let mut x = 0;
        let mut view = vec![
            acure::Command::FillRectangle(
                0,
                0,
                size.width,
                19,
                0.0,
                Color::ARGB(255, 128, 128, 128).into(),
            ),
            acure::Command::FillRectangle(0, 0, size.width, 18, 0.0, self.background_color.into()),
        ];
        for i in &self.children {
            view.push(acure::Command::FillRectangle(
                x,
                i.y,
                i.width,
                i.height,
                2.0,
                i.background_color.into(),
            ));
            view.push(acure::Command::WriteString(
                x,
                i.y,
                i.width,
                i.height + 2,
                Color::Black.into(),
                i.text.clone(),
            ));
            x = x + i.width + 2;
        }
        view
    }
}

impl Default for Menubar {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Menu {
    background_color: Color,
    text: String,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl Menu {
    pub fn new(text: String) -> Self {
        let len = text.len() as u32;
        Self {
            background_color: Color::ARGB(255, 200, 200, 200),
            text,
            x: 0,
            y: 0,
            width: len * 10,
            height: 0,
        }
    }
}
