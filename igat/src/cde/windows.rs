use std::marker::PhantomData;

use acure::{Acure, AlignMode};
use raw_window_handle::HasWindowHandle;

use crate::widget::{Component, Widget};

pub struct Cde<M>
where
    M: Send + std::fmt::Debug,
{
    acure: Acure,
    surface: acure::gdi::GDISurface,
    phantom: PhantomData<M>
}

impl<M> Cde<M>
where
    M: Send + std::fmt::Debug,
{
    pub fn new(handle: &impl HasWindowHandle) -> Self {
        let handle = handle.window_handle().unwrap();
        match handle.as_raw() {
            raw_window_handle::RawWindowHandle::Win32(handle) => Self {
                acure: Acure::new(),
                surface: acure::gdi::GDISurface::new(isize::from(handle.hwnd)),
                phantom: PhantomData,
            },
            _ => panic!("Error"),
        }
    }

    pub fn bgr(&self, color: crate::Color) {
        self.acure
            .push(acure::Command::Clear(color_to_acure_color(color)));
    }

    pub fn draw(&self, widget: &Box<dyn Widget<M>>) {
        match widget.widget_type() {
            crate::widget::WidgetType::Rectangle => {
                let shadow = widget.shadow();
                let border = shadow.border;
                self.acure.set_align_mode(AlignMode::CenterAligned);
                self.acure.push(acure::Command::FillRectangle(
                    widget.x(),
                    widget.y(),
                    widget.width() + border * 2,
                    widget.height() + border * 2,
                    color_to_acure_color(shadow.color),
                ));
                self.acure.push(acure::Command::FillRectangle(
                    widget.x() + border,
                    widget.y() + border,
                    widget.width(),
                    widget.height(),
                    color_to_acure_color(widget.background_color()),
                ));
            }
            crate::widget::WidgetType::Circle => todo!(),
            crate::widget::WidgetType::Text => {
                self.acure.push(acure::Command::WriteString(
                    widget.x(),
                    widget.y(),
                    widget.width(),
                    widget.height(),
                    color_to_acure_color(widget.color()),
                    widget.title().to_owned(),
                ));
            }
        }
    }

    pub fn write(&self) {
        self.acure.write(&self.surface);
        self.acure.clear();
    }
}

fn color_to_acure_color(color: crate::Color) -> acure::Color {
    match color {
        crate::Color::Black => acure::Color::ARGB(255, 0, 0, 0),
        crate::Color::White => acure::Color::ARGB(255, 255, 255, 255),
        crate::Color::ARGB(a, r, g, b) => acure::Color::ARGB(a, r, g, b),
    }
}
