use igat::{
    menu::Menubar,
    menu::Menu,
    widget::{build_component, Button, Component, Text},
    Application, ApplicationEvent, Color, Executable, Theme, Frame,
};
use igat::ApplicationResponse;

#[derive(Clone, Copy, Debug)]
pub enum Message {
    ButtonClicked,
}

pub struct SampleApp {
    menu: Menubar,
}

impl Application<Message> for SampleApp {
    type Message = Message;

    fn on_close(&self) {}

    fn set_up(&mut self) {}

    fn message(
        &mut self,
        event: ApplicationEvent,
        _message: Option<Message>,
        frame: &Frame
    ) -> Option<ApplicationResponse> {
        None
    }

    fn menu(&self) -> Option<&igat::menu::Menubar> {
        Some(&self.menu)
    }

    fn ui(&mut self,frame: &Frame) -> Option<Component<Message>> {
        let button = Button::new(frame)
            .width(70)
            .height(30)
            .text("Button".to_string())
            .on_click(Message::ButtonClicked);
        Some(build_component(button))
    }
}

fn main() {
    let exe = Executable::new();

    let mut menubar = Menubar::new();
    menubar.add(Menu::new("File".to_string()));
    menubar.add(Menu::new("Edit".to_string()));
    let app = SampleApp {
        menu: menubar,
    };
    exe.run(app);
}
