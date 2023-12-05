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

    pub fn height(&self) -> u32 {
        19
    }

    pub(crate) fn view(&self, window: &Window) -> Vec<acure::Command> {
        let size = window.inner_size();
        vec![
            acure::Command::FillRectangle(
                0,
                0,
                size.width,
                19,
                0.0,
                Color::ARGB(255,128,128,128).into(),
            ),
            acure::Command::FillRectangle(
            0,
            0,
            size.width,
            18,
            0.0,
            self.background_color.into(),
        ),]
    }
}

pub struct Menu {
    background_color: Color,
}
