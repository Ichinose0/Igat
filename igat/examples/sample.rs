use igat::{
    menu::Menubar,
    widget::{build_component, Button, Component, Text},
    Application, ApplicationEvent, Color, Executable, Theme,
};

#[derive(Clone, Copy, Debug)]
pub enum Message {
    ButtonClicked,
}

pub struct Poweredit {
    theme: Theme,
    menu: Menubar,
}

impl Application<Message> for Poweredit {
    type Message = Message;

    fn on_close(&self) {}

    fn set_up(&mut self) {}
    
    fn message(&mut self, event: ApplicationEvent,_message: Option<M>) -> Option<ApplicationResponse> {
        None
    }

    fn menu(&self) -> Option<&igat::menu::Menubar> {
        None
    }

    fn ui(&mut self) -> Option<Component<Message>> {
        let button = Button::new()
            .width(70)
            .height(30)
            .text("Count up".to_string())
            .x(10)
            .y(10)
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
        menu: Menubar::new(),
    };
    exe.run(app);
}
