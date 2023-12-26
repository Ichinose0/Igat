use acure::Command;

use crate::{Color, Rect};

use super::{Component, Data, Widget, WidgetMessage};

pub struct Panel<D>
where
    D: Data,
{
    inner: Vec<Box<dyn Widget<D>>>,
    data: D,
}

impl<D> Panel<D>
where
    D: Data,
{
    pub fn new(data: D) -> Self {
        Self {
            inner: vec![],
            data,
        }
    }

    pub fn child<T>(mut self, widget: T) -> Self
    where
        T: Widget<D> + 'static,
    {
        self.inner.push(Box::new(widget));
        self
    }

    pub fn into_component(self) -> Component<D> {
        Component {
            inner: self.inner,
            static_data: self.data,
        }
    }
}
