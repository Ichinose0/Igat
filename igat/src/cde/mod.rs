#[cfg(target_os = "windows")]
pub mod windows;
use acure::Command;
use raw_window_handle::HasWindowHandle;

use crate::{menu::Menubar, Color, Frame, Theme};

#[cfg(target_os = "windows")]
pub use self::windows::*;

pub struct RenderManager<M>
where
    M: Send + std::fmt::Debug,
{
    cde: Cde<M>,
    pub(crate) frame: Frame,
    theme: Theme,
}

impl<M> RenderManager<M>
where
    M: Send + std::fmt::Debug,
{
    pub fn new(frame: Frame, theme: Theme) -> Self {
        Self {
            cde: Cde::new(&frame),
            frame,
            theme,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.cde.resize(width, height);
    }

    pub fn frame(&self) -> &Frame {
        &self.frame
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
