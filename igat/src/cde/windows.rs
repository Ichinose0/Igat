use std::marker::PhantomData;

use acure::{Acure, Command};
use raw_window_handle::HasWindowHandle;

use crate::{menu::Menubar, widget::Widget, Window};

pub struct Cde
{
    acure: Acure,
    surface: acure::d2d1::D2D1Surface,
}

impl Cde
{
    pub fn new(handle: &Window) -> Self {
        let size = handle.inner.inner_size();
        let handle = handle.inner.window_handle().unwrap();
        match handle.as_raw() {
            raw_window_handle::RawWindowHandle::Win32(handle) => {
                let acure = Acure::new();
                Self {
                    acure,
                    surface: acure::d2d1::D2D1Surface::new(isize::from(handle.hwnd)),
                }
            }
            _ => panic!("Error"),
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface.resize();
    }

    pub fn begin(&mut self) {
        self.acure.begin(&mut self.surface);
    }

    pub fn bgr(&mut self, color: crate::Color) {
        self.acure.set_background_color(color.into());
    }

    pub fn register(&mut self, cmds: Vec<Command>) {
        for c in cmds {
            self.acure.push(c);
        }
    }

    pub fn draw_menu(&mut self, window: &Window, menu: &Menubar) {
        for i in menu.view(&window.inner) {
            self.acure.push(i);
        }
    }

    pub fn draw(&mut self, commands: Vec<Command>) {
        for c in commands {
            self.acure.push(c);
        }
    }

    pub fn write(&mut self) {
        self.acure.write(&mut self.surface);
        self.acure.clear();
    }
}

fn color_to_acure_color(color: crate::Color) -> acure::Color {
    match color {
        crate::Color::Black => acure::Color::ARGB(255, 0, 0, 0),
        crate::Color::White => acure::Color::ARGB(255, 255, 255, 255),
        crate::Color::ARGB(a, r, g, b) => acure::Color::ARGB(a, r, g, b),
    }
}
