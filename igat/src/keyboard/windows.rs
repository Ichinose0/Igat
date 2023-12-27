use std::ffi::c_int;

use winapi::um::winuser::*;

use super::KeyId;

pub const _VK_LBUTTON: c_int = VK_LBUTTON;

pub const _VK_A: c_int = 0x41;
pub const _VK_B: c_int = 0x42;
pub const _VK_C: c_int = 0x43;
pub const _VK_D: c_int = 0x44;
pub const _VK_E: c_int = 0x45;
pub const _VK_F: c_int = 0x46;
pub const _VK_G: c_int = 0x47;
pub const _VK_H: c_int = 0x48;
pub const _VK_I: c_int = 0x49;
pub const _VK_J: c_int = 0x4A;
pub const _VK_K: c_int = 0x4B;
pub const _VK_L: c_int = 0x4C;
pub const _VK_M: c_int = 0x4D;
pub const _VK_N: c_int = 0x4E;
pub const _VK_O: c_int = 0x4F;
pub const _VK_P: c_int = 0x50;
pub const _VK_Q: c_int = 0x51;
pub const _VK_R: c_int = 0x52;
pub const _VK_S: c_int = 0x53;
pub const _VK_T: c_int = 0x54;
pub const _VK_U: c_int = 0x55;
pub const _VK_V: c_int = 0x56;
pub const _VK_W: c_int = 0x57;
pub const _VK_X: c_int = 0x58;
pub const _VK_Y: c_int = 0x59;
pub const _VK_Z: c_int = 0x5A;

pub const _VK_BACKSPACE: KeyId = VK_BACK;
pub const _VK_TAB: KeyId = VK_TAB;
pub const _VK_SHIFT: KeyId = VK_SHIFT;

pub fn _get_key_state(id: KeyId) -> bool {
    unsafe { GetAsyncKeyState(id) != 0 }
}

pub fn _get_keyboard_state() -> Vec<u8> {
    let mut state = vec![0; 256];
    unsafe {
        GetKeyboardState(state.as_mut_ptr());
    }
    state
}
