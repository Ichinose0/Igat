use acure::Command;

use crate::{Color, Rect};

use super::{Component, Container, Data, Layout, Widget, WidgetMessage};

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

impl<D> Layout for Panel<D>
where
    D: Data,
{
    fn area(&self) -> Vec<Rect> {
        todo!()
    }

    fn theme(&mut self, theme: crate::Theme) {
        todo!()
    }

    fn is_capture_event(&self) -> bool {
        todo!()
    }

    fn x(&mut self, x: u32) {
        //self.property.x = x;
    }

    fn y(&mut self, y: u32) {
        //self.property.y = y;
    }

    fn width(&mut self, width: u32) {
        //self.property.width = width;
    }

    fn height(&mut self, height: u32) {
        //self.property.height = height;
    }
}

impl<D> Container<D> for Panel<D>
where
    D: Data,
{
    fn format(&mut self) {}

    fn childrens(&mut self) -> &mut [Box<dyn Widget<D>>] {
        &mut self.inner
    }
}
