use std::marker::PhantomData;

use acure::{Acure, AlignMode};
use raw_window_handle::HasWindowHandle;
use winit::window::Window;

use crate::{menu::Menubar, widget::Widget};

pub struct Cde<M>
where
    M: Send + std::fmt::Debug,
{
    acure: Acure,
    surface: acure::d2d1::D2D1Surface,
    phantom: PhantomData<M>,
}

impl<M> Cde<M>
where
    M: Send + std::fmt::Debug,
{
    pub fn new(handle: &impl HasWindowHandle) -> Self {
        let handle = handle.window_handle().unwrap();
        match handle.as_raw() {
            raw_window_handle::RawWindowHandle::Win32(handle) => Self {
                acure: Acure::new(),
                surface: acure::d2d1::D2D1Surface::new(isize::from(handle.hwnd)),
                phantom: PhantomData,
            },
            _ => panic!("Error"),
        }
    }

    pub fn bgr(&self, color: crate::Color) {
        self.acure
            .push(acure::Command::Clear(color_to_acure_color(color)));
    }

    pub fn draw_menu(&self, window: &Window, menu: &Menubar) {
        for i in menu.view(window) {
            self.acure.push(i)
        }
    }

    pub fn draw(&self, widget: &Box<dyn Widget<M>>) {
        for c in widget.view() {
            self.acure.push(c);
        }
    }

    pub fn write(&self) {
        self.acure.write(&self.surface);
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
