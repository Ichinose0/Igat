use acure::{Acure, Command};
use raw_window_handle::HasWindowHandle;

use crate::{
    menu::Menubar,
    widget::{Container, Data},
    Window,
};

pub struct Cde {
    acure: Acure,
    surface: acure::d2d1::D2D1Surface,
}

impl Cde {
    pub fn new<C, D>(handle: &Window<C, D>) -> Self
    where
        C: Container<D>,
        D: Data,
    {
        let handle = handle.inner.window_handle().unwrap();
        match handle.as_raw() {
            raw_window_handle::RawWindowHandle::Win32(handle) => {
                let acure = Acure::new();
                Self {
                    acure,
                    surface: unsafe {
                        acure::d2d1::D2D1Surface::new(isize::from(handle.hwnd)).unwrap()
                    },
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

    pub fn draw_menu<C, D>(&mut self, window: &Window<C, D>, menu: &Menubar)
    where
        C: Container<D>,
        D: Data,
    {
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
        self.acure.write(&mut self.surface).unwrap();
        self.acure.clear();
    }
}