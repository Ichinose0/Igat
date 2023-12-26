use std::str::FromStr;

use igat::{
    widget::{Button, Component, Panel, WidgetMessage},
    IApplicationBuilder, Theme, Window, WindowEvent,
};

#[derive(Clone, Copy, Debug)]
pub enum Message {
    ButtonClicked,
}

fn main() {
    let window = Window::new(ui());
    let mut app = IApplicationBuilder::new()
        .with(window)
        .theme(Theme::DARK)
        .build();
    app.run(|event| match event {
        _ => {}
    });
}

fn ui() -> Component {
    let button = Button::new()
        .width(240)
        .height(40)
        .text("Click me".to_owned())
        .on_message(|message,prop| match message {
            WidgetMessage::OnClick => {
                prop.text = String::from_str("Clicked!").unwrap();
            }

            _ => {}
        });
    let panel = Panel::new().child(button);
    panel.into()
}
