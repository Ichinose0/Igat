use igat::{
    frame::Frame,
    widget::{build_component, Button, Component, Text},
    Application, ApplicationEvent, Color, Executable, Theme,
};

#[derive(Clone, Copy, Debug)]
pub enum Message {
    ButtonClicked,
}

pub struct Poweredit {
    theme: Theme,
}

impl Application<Message> for Poweredit {
    type Message = Message;

    fn on_close(&self) {}

    fn set_up(&mut self) {}

    fn message(&mut self, event: Message) {}

    fn ui(&mut self) -> Option<Component<Message>> {
        let button = Text::new()
            .width(240)
            .height(80)
            .x(20)
            .y(20)
            .on_click(Message::ButtonClicked);
        Some(build_component(button))
    }

    fn theme(&self) -> igat::Theme {
        self.theme
    }
}

fn main() {
    let exe = Executable::new();

    let app = Poweredit {
        theme: Theme::default(),
    };
    exe.run(app);
}
