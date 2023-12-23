use acure::Command;

use crate::{Color, Frame, Rect};

use super::{ClientMessage, Component, Widget};

#[derive(Debug)]
pub struct Panel<M>
where
    M: Send + std::fmt::Debug,
{
    inner: Vec<Box<dyn Widget<M>>>,
}

impl<M> Panel<M>
where
    M: Send + std::fmt::Debug,
{
    pub fn new() -> Self {
        Self { inner: vec![] }
    }

    pub fn child<T>(mut self, widget: T) -> Self
    where
        T: Widget<M> + 'static,
    {
        self.inner.push(Box::new(widget));
        self
    }
}

impl<M> Into<Component<M>> for Panel<M>
where
    M: Send + std::fmt::Debug,
{
    fn into(self) -> Component<M> {
        Component { inner: self.inner }
    }
}
