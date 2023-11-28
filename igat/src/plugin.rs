use raw_window_handle::RawWindowHandle;

#[deprecated(since = "0.0.1", note = "Plugin function will be deprecated.")]
pub trait Plugin {
    fn name(&self) -> String;
    fn version(&self) -> String;
    fn set_up(&mut self, handle: RawWindowHandle);
}

#[deprecated(since = "0.0.1", note = "Plugin function will be deprecated.")]
pub struct PluginLoader {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginLoader {
    #[deprecated(since = "0.0.1", note = "Plugin function will be deprecated.")]
    pub fn new() -> Self {
        Self { plugins: vec![] }
    }

    #[deprecated(since = "0.0.1", note = "Plugin function will be deprecated.")]
    pub fn join(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }

    #[deprecated(since = "0.0.1", note = "Plugin function will be deprecated.")]
    pub(crate) fn load(self) {
        for p in self.plugins {}
    }
}
