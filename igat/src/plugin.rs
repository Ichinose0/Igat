use raw_window_handle::RawWindowHandle;

pub trait Plugin {
    fn name(&self) -> String;
    fn version(&self) -> String;
    fn set_up(&mut self,handle: RawWindowHandle);
}

pub struct PluginLoader {
    plugins: Vec<Box<dyn Plugin>>
}

impl PluginLoader {
    pub fn new() -> Self {
        Self {
            plugins: vec![]
        }
    }

    pub fn join(&mut self,plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }

    pub(crate) fn load(self) {
        for p in self.plugins {
            
        }
    }
}