use std::ptr::null;

use raw_window_handle::HasWindowHandle;
use x11::xlib::{XDefaultRootWindow, XOpenDisplay, XQueryPointer};

pub struct Cursor {
    root_x: i32,
    root_y: i32,
    win_x: i32,
    win_y: i32,
}

impl Cursor {
    pub fn get(window: &impl HasWindowHandle) -> Self {
        match window.window_handle().unwrap().as_raw() {
            raw_window_handle::RawWindowHandle::Xlib(mut handle) => unsafe {
                let display = XOpenDisplay(null());
                let mut mask = 0;
                let mut root_x = 0;
                let mut root_y = 0;
                let mut win_x = 0;
                let mut win_y = 0;
                XQueryPointer(
                    display,
                    XDefaultRootWindow(display),
                    &mut handle.window,
                    &mut handle.window,
                    &mut root_x,
                    &mut root_y,
                    &mut win_x,
                    &mut win_y,
                    &mut mask,
                );
                Self {
                    root_x,
                    root_y,
                    win_x,
                    win_y,
                }
            },
            _ => panic!("Unknown Error."),
        }
    }

    pub fn x(&self) -> i32 {
        self.win_x - self.root_x
    }

    pub fn y(&self) -> i32 {
        self.win_y - self.root_y
    }
}
