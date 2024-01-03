use igat::{
    widget::{Align, Button, Data, Label, Layout, StackPanel, WidgetMessage},
    ApplicationBuilder, Theme, Window,
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
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    let counter = Counter::new();
    let window = Window::new(ui(&counter));
    let app = ApplicationBuilder::new()
        .with(window)
        .theme(Theme::LIGHT)
        .build(counter);
    app.run(|event| match event {
        _ => {}
    });
}

fn ui(counter: &Counter) -> StackPanel<Counter> {
    let mut countup = Button::new();
    countup.width(240);
    countup.height(80);
    countup.x(10);
    countup.y(100);
    countup.text("Up".to_owned());
    countup.on_message(|msg, _, counter: &mut Counter| match msg {
        WidgetMessage::OnClick => counter.count += 1,

        _ => {}
    });
    let mut countdown = Button::new();
    countdown.width(240);
    countdown.height(80);
    countdown.x(280);
    countdown.y(100);
    countdown.text("Down".to_owned());
    countdown.on_message(|msg, _, counter: &mut Counter| match msg {
        WidgetMessage::OnClick => counter.count -= 1,

        _ => {}
    });
    let mut count = Label::new();
    count.width(190);
    count.height(40);
    count.x(180);
    count.y(20);
    count.text(counter.count.to_string());
    count.on_message(|_, prop, counter: &mut Counter| {
        prop.text = counter.count.to_string();
    });
    let panel = StackPanel::new(None, Align::Vertical)
        .child(count)
        .child(countup)
        .child(countdown);

    panel
}
