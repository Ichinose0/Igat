use igat::{IApplicationBuilder,Window, Event};

#[derive(Debug)]
pub enum Message {
    Resized
}

fn main() {
    let window = Window::new(Event::new(Message::Resized));
    let app = IApplicationBuilder::new().with(window).build();
    app.run();
}