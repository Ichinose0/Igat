use igat::{
    widget::{Button, Component, Panel},
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
        WindowEvent::WidgetEvent(mes) => match mes {
            Message::ButtonClicked => {
                println!("Clicked!");
            }
        },

        _ => {}
    });
}

fn ui() -> Component<Message> {
    let button = Button::new()
        .width(240)
        .height(40)
        .text("Click me".to_owned())
        .on_click(Message::ButtonClicked);
    let panel = Panel::new().child(button);
    panel.into()
}
