use std::ffi::c_int;

use winapi::um::winuser::*;

use super::KeyId;

pub const _VK_LBUTTON: KeyId = VK_LBUTTON as KeyId;

pub const _VK_A: KeyId = 0x41;
pub const _VK_B: KeyId = 0x42;
pub const _VK_C: KeyId = 0x43;
pub const _VK_D: KeyId = 0x44;
pub const _VK_E: KeyId = 0x45;
pub const _VK_F: KeyId = 0x46;
pub const _VK_G: KeyId = 0x47;
pub const _VK_H: KeyId = 0x48;
pub const _VK_I: KeyId = 0x49;
pub const _VK_J: KeyId = 0x4A;
pub const _VK_K: KeyId = 0x4B;
pub const _VK_L: KeyId = 0x4C;
pub const _VK_M: KeyId = 0x4D;
pub const _VK_N: KeyId = 0x4E;
pub const _VK_O: KeyId = 0x4F;
pub const _VK_P: KeyId = 0x50;
pub const _VK_Q: KeyId = 0x51;
pub const _VK_R: KeyId = 0x52;
pub const _VK_S: KeyId = 0x53;
pub const _VK_T: KeyId = 0x54;
pub const _VK_U: KeyId = 0x55;
pub const _VK_V: KeyId = 0x56;
pub const _VK_W: KeyId = 0x57;
pub const _VK_X: KeyId = 0x58;
pub const _VK_Y: KeyId = 0x59;
pub const _VK_Z: KeyId = 0x5A;

pub const _VK_BACKSPACE: KeyId = VK_BACK as KeyId;
pub const _VK_TAB: KeyId = VK_TAB as KeyId;
pub const _VK_SHIFT: KeyId = VK_SHIFT as KeyId;

pub fn _get_key_state(id: KeyId) -> bool {
    unsafe { GetAsyncKeyState(id as i32) != 0 }
}

pub fn _get_keyboard_state() -> Vec<u8> {
    let mut state = vec![0; 256];
    unsafe {
        GetKeyboardState(state.as_mut_ptr());
    }
    state
}
