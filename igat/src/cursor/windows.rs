use raw_window_handle::HasWindowHandle;
use winapi::shared::windef::{HWND, POINT, RECT};
use winapi::um::winuser::{
    GetClientRect, GetCursorPos, GetSystemMetrics, ScreenToClient, GetWindowRect, SM_CXSIZEFRAME, SM_CYCAPTION,
};

pub struct Cursor {
    point: POINT,
}

impl Cursor {
    pub fn get(window: &impl HasWindowHandle) -> Self {
        match window.window_handle().unwrap().as_raw() {
            raw_window_handle::RawWindowHandle::Win32(handle) => {
                let mut point = POINT { x: 0, y: 0 };
                unsafe {
                    GetCursorPos(&mut point);
                    ScreenToClient(isize::from(handle.hwnd) as HWND,&mut point);
                }
                Self { point }
            }
            _ => panic!("Unknown Error."),
        }
    }

    pub fn x(&self) -> i32 {
        self.point.x
    }

    pub fn y(&self) -> i32 {
        self.point.y
    }
}
