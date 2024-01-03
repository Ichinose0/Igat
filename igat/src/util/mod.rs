#[cfg(target_os = "windows")]
pub mod windows;
use raw_window_handle::HasWindowHandle;

use crate::Rect;

#[cfg(target_os = "windows")]
pub use self::windows::*;
