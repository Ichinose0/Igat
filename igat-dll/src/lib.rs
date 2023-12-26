#![no_main]

use std::ptr::{null, null_mut};

use igat::{IApplication, IApplicationBuilder, widget::Panel};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
}

pub enum Menubar {}

pub enum Menu {}

pub enum Theme {}

pub enum Window {}

pub enum Application {}

#[repr(C)]
pub struct IgatApplicationCreateInfo {
    window: *mut Window,
    theme: *mut Theme
}

#[no_mangle]
pub extern "C" fn IgatCreateApplication() -> *mut Application {
    let app = IApplicationBuilder::new().with(igat::Window::new(Panel::new().into())).theme(igat::Theme::ORIGINAL).build();
    let app = Box::new(app);
    Box::into_raw(app) as *mut Application
}

#[no_mangle]
pub extern "C" fn IgatRunApplication(application: *mut Application) {
    let app = unsafe { Box::from_raw(application as *mut IApplication) };
    app.run(|event| {

    })
}

#[no_mangle]
pub extern "C" fn IgatGetDarkTheme() -> *mut Theme {
    Box::into_raw(Box::new(igat::Theme::DARK)) as *mut Theme
}