#[cfg(target_os = "windows")]
pub mod windows;
use acure::Command;
use raw_window_handle::HasWindowHandle;

use crate::{Theme, menu::Menubar, Frame};

#[cfg(target_os = "windows")]
pub use self::windows::*;

pub struct RenderManager<M>
where
    M: Send + std::fmt::Debug,
{
    cde: Cde<M>,
    pub(crate) frame: Frame,
    theme: Theme
}

impl<M> RenderManager<M> 
where
    M: Send + std::fmt::Debug
{
    pub fn new(frame: Frame,theme: Theme) -> Self {
        Self {
            cde: Cde::new(&frame.window),
            frame,
            theme
        }
    }

    pub fn frame(&self) -> &Frame {
        &self.frame
    }

    pub fn write(&self) {
        let mut commands = vec![Command::Clear(self.theme.bgr.into())];
        self.cde.draw(commands);
        self.cde.write();
    }
}