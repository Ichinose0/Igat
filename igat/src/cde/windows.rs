use std::marker::PhantomData;

use acure::{Acure, Command};
use raw_window_handle::HasWindowHandle;
use winit::window::Window;

use crate::{menu::Menubar, widget::Widget, Frame};

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
    pub fn new(handle: &Frame) -> Self {
        let size = handle.window.inner_size();
        let handle = handle.window.window_handle().unwrap();
        match handle.as_raw() {
            raw_window_handle::RawWindowHandle::Win32(handle) => {
                let acure = Acure::new();
                Self {
                    acure,
                    surface: acure::d2d1::D2D1Surface::new(isize::from(handle.hwnd),size.width,size.height),
                    phantom: PhantomData,
                }
            }
            _ => panic!("Error"),
        }
    }

    pub fn resize(&mut self,width: u32,height: u32) {
        self.surface.resize(width, height);
    }

    pub fn begin(&mut self) {
        self.acure.begin(&mut self.surface);
    }

    pub fn bgr(&mut self, color: crate::Color) {
        self.acure.set_background_color(color.into());
    }

    pub fn register(&mut self,cmds: Vec<Command>) {
        for c in cmds {
            self.acure.push(c);
        }
    }

    pub fn draw_menu(&mut self, window: &Window, menu: &Menubar) {
        for i in menu.view(window) {
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
