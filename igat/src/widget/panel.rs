use crate::Rect;

use super::{Container, Data, Layout, Widget};

pub struct Panel<D>
where
    D: Data,
{
    inner: Vec<Box<dyn Widget<D>>>,
    auto_resize: bool,
    rect: Rect,
}

impl<D> Panel<D>
where
    D: Data,
{
    pub fn new(rect: Option<Rect>) -> Self {
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

impl<D> Layout for Panel<D>
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
}

impl<D> Container<D> for Panel<D>
where
    D: Data,
{
    fn format(&mut self, window_rect: Rect) {}

    fn childrens(&mut self) -> &mut [Box<dyn Widget<D>>] {
        &mut self.inner
    }
}
