#[cfg(target_os = "windows")]
pub mod windows;
use raw_window_handle::HasWindowHandle;

use crate::Rect;

#[cfg(target_os = "windows")]
pub use self::windows::*;


// pub fn get_suitable_rect(handle: &impl HasWindowHandle) -> Rect {
    
// }