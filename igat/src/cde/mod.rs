#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "windows")]
pub mod windows;
use acure::Command;

use crate::{
    widget::{Container, Data},
    Theme, Window,
};

#[cfg(target_os = "linux")]
pub use self::linux::*;
#[cfg(target_os = "windows")]
pub use self::windows::*;

pub struct RenderManager {
    cde: Cde,
    theme: Theme,
}

impl RenderManager {
    pub fn new<C, D>(window: &Window<C, D>, theme: Theme) -> Self
    where
        C: Container<D>,
        D: Data,
    {
        Self {
            cde: Cde::new(window),
            theme,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.cde.resize(width, height);
    }

    pub fn set_background_color(&mut self) {
        self.cde.bgr(self.theme.bgr);
    }

    pub fn register(&mut self, cmds: &[Command]) {
        self.cde.register(cmds.to_vec());
    }

    pub fn begin(&mut self) {
        self.cde.begin();
    }

    pub fn write(&mut self) {
        self.cde.write();
    }
}
