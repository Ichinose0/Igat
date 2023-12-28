use std::cell::RefCell;

use acure::Command;

use crate::{Color, Rect};

use super::{Align, Component, Container, Data, Layout, Widget, WidgetMessage};

pub struct StackPanel<D>
where
    D: Data,
{
    inner: Vec<Box<dyn Widget<D>>>,
    orientation: Align,
    data: RefCell<D>,
}

impl<D> StackPanel<D>
where
    D: Data,
{
    pub fn new(data: D, align: Align) -> Self {
        Self {
            inner: vec![],
            orientation: align,
            data: RefCell::new(data),
        }
    }

    pub fn child<T>(mut self, widget: T) -> Self
    where
        T: Widget<D> + 'static,
    {
        self.inner.push(Box::new(widget));
        self
    }
}

impl<D> Layout for StackPanel<D>
where
    D: Data,
{
    fn area(&self) -> Vec<Rect> {
        vec![]
    }

    fn theme(&mut self, theme: crate::Theme) {}

    fn is_capture_event(&self) -> bool {
        true
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

    fn cursor(&self) -> winit::window::CursorIcon {
        winit::window::CursorIcon::default()
    }
}

impl<D> Container<D> for StackPanel<D>
where
    D: Data,
{
    fn format(&mut self) {}

    fn childrens(&mut self) -> &mut [Box<dyn Widget<D>>] {
        &mut self.inner
    }
}
