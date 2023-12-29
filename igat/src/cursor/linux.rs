use std::ptr::null;

use raw_window_handle::HasWindowHandle;
use x11::xlib::{XCloseDisplay, XDefaultRootWindow, XOpenDisplay, XQueryPointer, XWindowAttributes, XGetWindowAttributes};

use crate::{Window, widget::{Container, Data}};

pub struct Cursor {
    root_x: i32,
    root_y: i32,
    win_x: i32,
    win_y: i32,
    attributes: XWindowAttributes,
}

impl Cursor {
    pub fn get(window: &impl HasWindowHandle) -> Self 
    
    {
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
                let attributes = get_window_attributes(display, handle.window);
                XCloseDisplay(display);
                Self {
                    root_x,
                    root_y,
                    win_x,
                    win_y,
                    attributes,
                }
            },
            _ => panic!("Unknown Error."),
        }
    }

    pub fn x(&self) -> i32 {
        self.root_x-self.attributes.x
    }

    pub fn y(&self) -> i32 {
        self.root_y-self.attributes.y
    }
}

fn get_window_attributes(display: *mut _XDisplay, window: c_ulong) -> XWindowAttributes {
    let mut attributes = unsafe { MaybeUninit::uninit().assume_init() };
    unsafe { XGetWindowAttributes(display, window, &mut attributes) };
    attributes
}