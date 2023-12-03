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
    frame: MyFrame,
}

impl Application<Message> for Poweredit {
    type Message = Message;

    fn route(&mut self, event: ApplicationEvent) -> &mut dyn Frame<Message = Self::Message> {
        &mut self.frame
    }

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

pub struct MyFrame {}

impl Frame for MyFrame {
    type Message = Message;

    fn bgr(&self) -> igat::Color {
        igat::Color::ARGB(255, 125, 0, 255)
    }

    fn title(&self) -> String {
        "サンプルフレーム".to_owned()
    }

    fn ui(&self) -> Option<Component<Message>> {
        let button = Button::new()
            .width(240)
            .height(80)
            .x(20)
            .y(20)
            .on_click(Message::ButtonClicked);
        Some(build_component(button))
    }

    fn resizable(&self) -> bool {
        true
    }
}

fn main() {
    let exe = Executable::new();

    let app = Poweredit {
        theme: Theme::default(),
        frame: MyFrame {},
    };
    exe.run(app);
}
