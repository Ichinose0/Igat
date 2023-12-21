#![no_main]

use std::ptr::{null,null_mut};

pub type MESSEAGEPROCPTR = fn();
pub type UIPROCPTR = fn();
pub type SETUPPROCPTR = fn();
pub type THEMEPROCPTR = fn(theme: Theme);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
}

#[repr(C)]
pub enum Menubar {

}

#[repr(C)]
pub enum Menu {
    
}

#[repr(C)]
pub enum Theme {

}

#[repr(C)]
pub enum Frame {

}

#[repr(C)]
pub enum Application {

}

pub struct DefaultApplication {
    ui: UIPROCPTR
}


#[no_mangle]
pub extern "C" fn CreateApplication() -> *mut Application {
    null_mut();
}
