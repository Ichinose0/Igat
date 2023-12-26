use igat::{
    widget::{Component, Data, Panel},
    IApplicationBuilder, Theme, Window,
};

pub struct MyApp;

impl Data for MyApp {}

fn main() {
    let counter = MyApp;
    let window = Window::new(ui(counter));
    let app = IApplicationBuilder::new()
        .with(window)
        .theme(Theme::LIGHT)
        .build();
    app.run(|event| match event {
        _ => {}
    });
}

fn ui(app: MyApp) -> Component<MyApp> {
    let panel = Panel::new(app);
    panel.into_component()
}
