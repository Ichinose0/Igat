use acure::Command;

use crate::{Color, Rect};

use super::{Component, Widget, WidgetMessage};

pub struct Panel {
    inner: Vec<Box<dyn Widget>>,
}

impl Panel {
    pub fn new() -> Self {
        Self { inner: vec![] }
    }

    pub fn child<T>(mut self, widget: T) -> Self
    where
        T: Widget + 'static,
    {
        self.inner.push(Box::new(widget));
        self
    }
}

impl Into<Component> for Panel {
    fn into(self) -> Component {
        Component { inner: self.inner }
    }
}
