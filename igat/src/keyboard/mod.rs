#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub(crate) use self::linux::*;
#[cfg(target_os = "windows")]
pub(crate) use self::windows::*;

pub type KeyId = u32;

pub const VK_NULL: KeyId = 0x0;

pub const VK_LBUTTON: KeyId = _VK_LBUTTON;

pub const VK_A: KeyId = _VK_A;
pub const VK_B: KeyId = _VK_B;
pub const VK_C: KeyId = _VK_C;
pub const VK_D: KeyId = _VK_D;
pub const VK_E: KeyId = _VK_E;
pub const VK_F: KeyId = _VK_F;
pub const VK_G: KeyId = _VK_G;
pub const VK_H: KeyId = _VK_H;
pub const VK_I: KeyId = _VK_I;
pub const VK_J: KeyId = _VK_J;
pub const VK_K: KeyId = _VK_K;
pub const VK_L: KeyId = _VK_L;
pub const VK_M: KeyId = _VK_M;
pub const VK_N: KeyId = _VK_N;
pub const VK_O: KeyId = _VK_O;
pub const VK_P: KeyId = _VK_P;
pub const VK_Q: KeyId = _VK_Q;
pub const VK_R: KeyId = _VK_R;
pub const VK_S: KeyId = _VK_S;
pub const VK_T: KeyId = _VK_T;
pub const VK_U: KeyId = _VK_U;
pub const VK_V: KeyId = _VK_V;
pub const VK_W: KeyId = _VK_W;
pub const VK_X: KeyId = _VK_X;
pub const VK_Y: KeyId = _VK_Y;
pub const VK_Z: KeyId = _VK_Z;

pub const VK_BACKSPACE: KeyId = _VK_BACKSPACE;
pub const VK_TAB: KeyId = _VK_TAB;
pub const VK_SHIFT: KeyId = _VK_SHIFT;

const VK_ARRAY: [KeyId; 26] = [
    VK_A, VK_B, VK_C, VK_D, VK_E, VK_F, VK_G, VK_H, VK_I, VK_J, VK_K, VK_L, VK_M, VK_N, VK_O, VK_P,
    VK_Q, VK_R, VK_S, VK_T, VK_U, VK_V, VK_W, VK_X, VK_Y, VK_Z,
];

pub struct Keyboard {
    inner: Vec<u8>,
}

impl Keyboard {
    pub fn get() -> Self {
        Self {
            inner: _get_keyboard_state(),
        }
    }

    pub fn get_key_state(&self, id: KeyId) -> bool {
        self.inner[id as usize] != 0
    }

    pub fn extract(&self) -> KeyId {
        let mut code = VK_NULL;

        for i in VK_ARRAY {
            if self.inner[i as usize] & 0x80 == 128 {
                code += i;
            }
        }

        code
    }
}

pub fn get_key_state(id: KeyId) -> bool {
    _get_key_state(id)
}
