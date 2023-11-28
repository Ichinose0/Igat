use crate::widget::Target;
use acure::{Acure, AlignMode};
use raw_window_handle::HasWindowHandle;

pub struct Cde<T>
where
    T: Send + std::fmt::Debug,
{
    acure: Acure,
    surface: acure::gdi::GDISurface,
    msg: Option<T>,
}

impl<T> Cde<T>
where
    T: Send + std::fmt::Debug,
{
    pub fn new(handle: &impl HasWindowHandle) -> Self {
        let handle = handle.window_handle().unwrap();
        match handle.as_raw() {
            raw_window_handle::RawWindowHandle::Win32(handle) => Self {
                acure: Acure::new(),
                surface: acure::gdi::GDISurface::new(isize::from(handle.hwnd)),
                msg: None,
            },
            _ => panic!("Error"),
        }
    }

    pub fn draw(&self, color: crate::Color, target: &Target<T>) {
        self.acure
            .push(acure::Command::Clear(color_to_acure_color(color)));

        for i in target.get() {
            match i.widget.widget_type() {
                crate::widget::WidgetType::Rectangle => {
                    let shadow = i.widget.shadow();
                    let border = shadow.border;
                    self.acure.set_align_mode(AlignMode::CenterAligned);
                    self.acure.push(acure::Command::FillRectangle(
                        i.widget.x(),
                        i.widget.y(),
                        i.widget.width() + border * 2,
                        i.widget.height() + border * 2,
                        color_to_acure_color(shadow.color),
                    ));
                    self.acure.push(acure::Command::FillRectangle(
                        i.widget.x() + border,
                        i.widget.y() + border,
                        i.widget.width(),
                        i.widget.height(),
                        color_to_acure_color(i.widget.background_color()),
                    ));
                }
                crate::widget::WidgetType::Circle => todo!(),
                crate::widget::WidgetType::Text => {
                    self.acure.push(acure::Command::WriteString(
                        i.widget.x(),
                        i.widget.y(),
                        i.widget.width(),
                        i.widget.height(),
                        color_to_acure_color(i.widget.color()),
                        i.widget.title().to_owned(),
                    ));
                }
            }
        }
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
