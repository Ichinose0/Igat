use std::ptr::null;
use x11::{
    keysym::*,
    xlib::{
        AnyModifier, Button1, ButtonPressMask, False, XCloseDisplay, XDefaultRootWindow,
        XGrabButton, XNextEvent, XOpenDisplay, XQueryKeymap,
    },
};

use super::KeyId;

pub const _VK_LBUTTON: KeyId = 800;

pub const _VK_A: KeyId = XK_A;
pub const _VK_B: KeyId = XK_B;
pub const _VK_C: KeyId = XK_C;
pub const _VK_D: KeyId = XK_D;
pub const _VK_E: KeyId = XK_E;
pub const _VK_F: KeyId = XK_F;
pub const _VK_G: KeyId = XK_G;
pub const _VK_H: KeyId = XK_H;
pub const _VK_I: KeyId = XK_I;
pub const _VK_J: KeyId = XK_J;
pub const _VK_K: KeyId = XK_K;
pub const _VK_L: KeyId = XK_L;
pub const _VK_M: KeyId = XK_M;
pub const _VK_N: KeyId = XK_N;
pub const _VK_O: KeyId = XK_O;
pub const _VK_P: KeyId = XK_P;
pub const _VK_Q: KeyId = XK_Q;
pub const _VK_R: KeyId = XK_R;
pub const _VK_S: KeyId = XK_S;
pub const _VK_T: KeyId = XK_T;
pub const _VK_U: KeyId = XK_U;
pub const _VK_V: KeyId = XK_V;
pub const _VK_W: KeyId = XK_W;
pub const _VK_X: KeyId = XK_X;
pub const _VK_Y: KeyId = XK_Y;
pub const _VK_Z: KeyId = XK_Z;

pub const _VK_BACKSPACE: KeyId = XK_BackSpace;
pub const _VK_TAB: KeyId = XK_Tab;
pub const _VK_SHIFT: KeyId = XK_Shift_L;

fn convert_lower_case(id: KeyId) -> KeyId {
    id + 32
}

fn conver_upper_case(id: KeyId) -> KeyId {
    id - 32
}

pub fn _get_key_state(id: KeyId) -> bool {
    if id == _VK_LBUTTON {
        unsafe {
            let display = XOpenDisplay(null());
            let root = XDefaultRootWindow(display);
            let mut ev = std::mem::MaybeUninit::uninit().assume_init();
            XGrabButton(
                display,
                Button1,
                AnyModifier,
                root,
                False,
                ButtonPressMask as i32,
                GrabModeAsync,
                GrabModeAsync,
                0,
                0,
            );
            XNextEvent(display, &mut ev);
            XCloseDisplay(display);
            return ev.button.button == Button1;
        }
    } else {
        let display = unsafe { XOpenDisplay(null()) };
        if display.is_null() {
            panic!("Can't open display.");
        }
        let mut state = vec![0; 256];
        unsafe {
            XQueryKeymap(display, state.as_mut_ptr());
        }
        state[id as usize] != 0
    }
}

pub fn _get_keyboard_state() -> Vec<u8> {
    let display = unsafe { XOpenDisplay(null()) };
    if display.is_null() {
        panic!("Can't open display.");
    }
    let mut state: Vec<u8> = vec![0; 256];
    unsafe {
        XQueryKeymap(display, state.as_mut_ptr() as *mut i8);
    }
    state
}
