use std::ptr::{null,null_mut};

use gdiplus_sys2::*;
use raw_window_handle::HasWindowHandle;
use winapi::{um::{winuser::{GetDC, DrawTextW, DT_LEFT, DT_WORDBREAK}, wingdi::{CreateSolidBrush, RGB, SetBkColor, SetTextColor, TextOutW, CreateFontW, FW_BOLD, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS, DEFAULT_QUALITY, VARIABLE_PITCH, SHIFTJIS_CHARSET, FF_ROMAN, SelectObject, DeleteObject, FF_DONTCARE, FF_MODERN, FW_SEMIBOLD, SetBkMode, TRANSPARENT}}, shared::{minwindef::FALSE, windef::RECT}};

use crate::widget::{Element, Shadow, Target};

#[derive(Debug)]
pub struct CDE<T> 
where
    T: Send + std::fmt::Debug
{
    token: usize,
    input: GdiplusStartupInput,
    hwnd: isize,
    msg: Option<T>
}

impl<T> CDE<T> 
where
    T: Send + std::fmt::Debug
{
    pub fn new(handle: &impl HasWindowHandle) -> Self {
        let handle = handle.window_handle().unwrap();
        match handle.as_raw() {
            raw_window_handle::RawWindowHandle::Win32(handle) => {
                unsafe {
                    let mut token = 0;
                    let input = GdiplusStartupInput {
                        GdiplusVersion: 1,
                        DebugEventCallback: None,
                        SuppressBackgroundThread: FALSE,
                        SuppressExternalCodecs: FALSE,
                    };
                    let status = GdiplusStartup(&mut token, &input, null_mut());
                    println!("{}",status);
                    if status != Status_Ok {
                        panic!("Can't startup GDI+");
                    }
        
                    Self {
                        token,
                        input,
                        hwnd:handle.hwnd.into(),
                        msg: None
                    }
                }
            },
            _ => panic!("Error"),
        }    
    }

    pub fn draw(&self,color: crate::Color,target: &Target<T>) {
        unsafe {
            let mut ps: winapi::um::winuser::PAINTSTRUCT = std::mem::zeroed();
            let hdc = winapi::um::winuser::BeginPaint(self.hwnd as HWND, &mut ps);
            let mut graphics = null_mut();
            let status = gdiplus_sys2::GdipCreateFromHDC(hdc, &mut graphics);

            //Drawing start

            self.draw_background(color,&ps,graphics);

            for i in target.get() {
                match i.widget.widget_type() {
                    crate::widget::WidgetType::Rectangle => {
                        self.draw_rectangle(graphics,i.widget.background_color(),i.widget.shadow(),i.widget.x() as i32,i.widget.y() as i32,i.widget.width() as i32,i.widget.height() as i32);
                    },
                    crate::widget::WidgetType::Circle => todo!(),
                    crate::widget::WidgetType::Text => {
                        let s = i.widget.title();
                        let mut v: Vec<u16> = s.encode_utf16().collect();
                        v.push(0);
                        let f = "Elite";
                        let mut font_name: Vec<u16> = s.encode_utf16().collect();
                        font_name.push(0);
                        //SetBkColor(hdc,color_to_rgb(color));
                        SetBkMode(hdc, TRANSPARENT as i32);
                        SetTextColor(hdc, color_to_rgb(i.widget.color()));

                        let font = CreateFontW(i.widget.height() as i32,0,0,0,0,0,0,0,SHIFTJIS_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS, DEFAULT_QUALITY,VARIABLE_PITCH | FF_MODERN, font_name.as_ptr());

                        SelectObject(hdc, font as *mut c_void);
                        TextOutW(hdc,i.widget.x() as i32, i.widget.y() as i32,v.as_ptr(),v.len() as i32);
                        DeleteObject(font as *mut c_void);
                    },
                }

                let shadow = i.widget.shadow();
                
            }
            GdipDeleteGraphics(graphics);
            winapi::um::winuser::EndPaint(self.hwnd as HWND,&ps);
        }
    }

    fn draw_background(&self,color: crate::Color,ps: &winapi::um::winuser::PAINTSTRUCT,graphics: *mut GpGraphics) {
        unsafe {
            let mut brush = null_mut();
            
            gdiplus_sys2::GdipCreateSolidFill(color_to_argb(color),&mut brush);
            GdipFillRectangleI(graphics,brush as *mut GpBrush,ps.rcPaint.left as i32,
                ps.rcPaint.top as i32,
                (ps.rcPaint.right - ps.rcPaint.left) as i32,
                (ps.rcPaint.bottom - ps.rcPaint.top) as i32);
                GdipDeleteBrush(brush as *mut GpBrush);
        }
    }

    fn draw_rectangle(&self,graphics: *mut GpGraphics,color: crate::Color,shadow: Shadow,x:i32,y:i32,width:i32,height:i32) {
        unsafe {
            let mut blur = null_mut();
                let mut brush = null_mut();
                gdiplus_sys2::GdipCreateSolidFill(color_to_argb(color),&mut brush);
                gdiplus_sys2::GdipCreateSolidFill(color_to_argb(shadow.color),&mut blur);
                GdipFillRectangleI(graphics,blur as *mut GpBrush,x-shadow.border as i32,
                    y-shadow.border as i32,
                width+(shadow.border*2) as i32,
                height+(shadow.border*2) as i32);
                GdipFillRectangleI(graphics,brush as *mut GpBrush,30,
                    30,
                    width,
                    height);
                    GdipDeleteBrush(blur as *mut GpBrush);
                    GdipDeleteBrush(brush as *mut GpBrush);
        }
    }
}

impl<T> Drop for CDE<T> 
where
    T: Send + std::fmt::Debug
{
    fn drop(&mut self) {
        unsafe {
            GdiplusShutdown(self.token);
        }
    }
}

fn rgb_to_argb(alpha: u8, red: u8, green: u8, blue: u8) -> u32 {
    ((alpha as u32) << 24) | ((red as u32) << 16) | ((green as u32) << 8) | (blue as u32)
}

fn color_to_rgb(color: crate::Color) -> u32 {
    match color {
        crate::Color::Black => RGB(0,0,0),
        crate::Color::White => RGB(255,255,255),
        crate::Color::ARGB(a, r, g, b) => RGB(r,g,b),
    }
}

fn color_to_argb(color: crate::Color) -> u32 {
    match color {
        crate::Color::Black => rgb_to_argb(255,0,0,0),
        crate::Color::White => rgb_to_argb(255,255,255,255),
        crate::Color::ARGB(a, r, g, b) => rgb_to_argb(a,r,g,b),
    }
}