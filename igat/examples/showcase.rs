use igat::{IApplicationBuilder,Window, Event, Theme};

#[derive(Clone,Copy,Debug)]
pub enum Message {
    Resized
}

fn main() {
    let window = Window::new(Event::new(Message::Resized));
    let app = IApplicationBuilder::new().with(window).theme(Theme::DARK).build();
    app.run(|event| {
        
    });
}