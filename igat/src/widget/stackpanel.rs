use std::cell::RefCell;

use acure::Command;

use crate::{Color, Rect};

use super::{Align, Component, Container, Data, Layout, Widget, WidgetMessage};

pub struct StackPanel<D>
where
    D: Data,
{
    inner: Vec<Box<dyn Widget<D>>>,
    auto_resize: bool,
    orientation: Align,
    rect: Rect,
}

impl<D> StackPanel<D>
where
    D: Data,
{
    pub fn new(rect: Option<Rect>, orientation: Align) -> Self {
        let mut auto_resize = false;
        let rect = match rect {
            Some(r) => r,
            None => {
                auto_resize = true;
                Rect {
                    left: 0,
                    top: 0,
                    right: 0,
                    bottom: 0,
                }
            }
        };

        Self {
            inner: vec![],
            rect,
            auto_resize,
            orientation,
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
    fn format(&mut self, window_rect: Rect) {
        if self.auto_resize {
            self.rect = window_rect;
        }

        match self.orientation {
            Align::Vertical => {
                let mut y = 0;
                let size = self.rect.height() / self.inner.len() as u32;
                for i in &mut self.inner {
                    i.width(self.rect.width());
                    i.height(size);
                    i.x(0);
                    i.y(y);
                    y += size;
                }
            }
            Align::Horizontal => {
                let mut x = 0;
                let size = self.rect.width() / self.inner.len() as u32;
                for i in &mut self.inner {
                    i.width(size);
                    i.height(self.rect.height());
                    i.x(x);
                    i.y(0);
                    x += size;
                }
            }
        }
    }

    fn childrens(&mut self) -> &mut [Box<dyn Widget<D>>] {
        &mut self.inner
    }
}
