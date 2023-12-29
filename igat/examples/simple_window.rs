use igat::{
    widget::{Checkbox, Component, Data, Panel},
    ApplicationBuilder, Theme, Window,
};

pub struct MyApp;

impl Data for MyApp {}

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    let counter = MyApp;
    let window = Window::new(ui());
    let app = ApplicationBuilder::new()
        .with(window)
        .theme(Theme::LIGHT)
        .build(counter);
    app.run(|event| match event {
        _ => {}
    });
}

fn ui() -> Panel<MyApp> {
    let panel = Panel::new();
    panel
}
