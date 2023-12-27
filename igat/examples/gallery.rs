use igat::{
    widget::{Checkbox, Component, Data, Label, Panel},
    IApplicationBuilder, Theme, Window,
};

pub struct Gallery {
    check: bool,
}

impl Data for Gallery {}

fn main() {
    let counter = Gallery { check: false };
    let window = Window::new(ui(counter));
    let app = IApplicationBuilder::new()
        .with(window)
        .theme(Theme::DARK)
        .build();
    app.run(|event| match event {
        _ => {}
    });
}

fn ui(gallery: Gallery) -> Component<Gallery> {
    let check =
        Checkbox::new()
            .width(40)
            .height(40)
            .on_message(|msg, prop, gallery: &mut Gallery| {
                gallery.check = prop.is_check;
            });
    let text =
        Label::new()
            .x(50)
            .width(240)
            .height(40)
            .on_message(|msg, prop, gallery: &mut Gallery| {
                if gallery.check {
                    prop.text = "Checked!".to_owned();
                } else {
                    prop.text = "Unchecked!".to_owned();
                }
            });
    let panel = Panel::new(gallery).child(check).child(text);
    panel.into_component()
}
