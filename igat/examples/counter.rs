use igat::{
    widget::{Button, Component, Data, Panel, WidgetMessage, Label},
    IApplicationBuilder, Theme, Window, WindowEvent,
};

pub struct Counter {
    pub count: i32,
}

impl Counter {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}

impl Data for Counter {}

fn main() {
    let counter = Counter::new();
    let window = Window::new(ui(counter));
    let app = IApplicationBuilder::new()
        .with(window)
        .theme(Theme::DARK)
        .build();
    app.run(|event| match event {
        _ => {}
    });
}

fn ui(counter: Counter) -> Component<Counter> {
    let countup = Button::new()
        .width(240)
        .height(80)
        .x(10)
        .y(100)
        .text("Up".to_owned())
        .on_message(|msg, prop, counter: &mut Counter| match msg {
            WidgetMessage::OnClick => counter.count += 1,

            _ => {}
        });
    let countdown = Button::new()
        .width(240)
        .height(80)
        .x(280)
        .y(100)
        .text("Down".to_owned())
        .on_message(|msg, prop, counter: &mut Counter| match msg {
            WidgetMessage::OnClick => counter.count -= 1,

            _ => {}
        });
    let count = Label::new()
        .width(190)
        .height(40)
        .x(180)
        .y(20)
        .text(counter.count.to_string())
        .on_message(|msg, prop, counter: &mut Counter| {
            prop.text = counter.count.to_string();
        });
    let panel = Panel::new(counter)
        .child(countup)
        .child(countdown)
        .child(count);
    panel.into_component()
}
