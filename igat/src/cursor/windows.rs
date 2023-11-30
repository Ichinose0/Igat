use raw_window_handle::HasWindowHandle;
use winapi::um::winuser::{GetCursorPos, GetWindowRect, GetClientRect};
use winapi::shared::windef::{POINT, RECT, HWND};

pub struct Cursor {
    point: POINT
}

impl Cursor {
    pub fn get() -> Self {
        let mut point = POINT { x: 0, y: 0 };
        unsafe {
            GetCursorPos(&mut point);
        }
        Self {
            point
        }
    }

    pub fn x(&self) -> i32 {
        self.point.x
    }

    pub fn y(&self) -> i32 {
        self.point.y
    }

    fn get_window_rect(&self,handle: &impl HasWindowHandle) -> RECT {
        match handle.window_handle().unwrap().as_raw() {
            raw_window_handle::RawWindowHandle::Win32(handle) => {
                let mut rect = RECT {
                    left: 0,
                    top: 0,
                    right: 0,
                    bottom: 0,
                };
                unsafe {
                    GetWindowRect(isize::from(handle.hwnd) as HWND, &mut rect);
                    rect
                }
            },
            _ => panic!("Unknown Error."),
        }
    }


    fn is_window_area(&self,rect: &RECT) -> bool {
        let x = rect.left;
        let y = rect.top;
        let x2 = rect.right;
        let y2 = rect.bottom;
        if self.point.x > x2 {
            return false;
        } else if self.point.x < x {
            return false;
        }

        if self.point.y < y {
            return false;
        } else if self.point.y > y2 {
            return false;
        }

        true
    }

    pub fn window_x(&self,handle: &impl HasWindowHandle) -> Option<i32> {
        let rect = self.get_window_rect(handle);
        if self.is_window_area(&rect) {
            Some(self.point.x-rect.left)
        } else {
            None
        }
    }

    pub fn window_y(&self,handle: &impl HasWindowHandle) -> Option<i32> {
        let rect = self.get_window_rect(handle);
        if self.is_window_area(&rect) {
            let y = (self.point.y-rect.top)-36;
            if y < 0 {
                return None;
            }
            Some(y)
        } else {
            None
        }
    }
}