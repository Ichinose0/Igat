use igat::ApplicationResponse;
use igat::{
    menu::Menu,
    menu::Menubar,
    widget::{build_component, Button, Component, Text},
    Application, ApplicationEvent, Color, Executable, Frame, Theme,
};

#[derive(Clone, Copy, Debug)]
pub enum Message {
    ButtonClicked,
}

pub struct SimpleApp {
    theme: Theme,
    menu: Menubar,
}

impl Application<Message> for SimpleApp {
    fn set_up(&mut self, frame: &Frame) {}

    fn message(
        &mut self,
        event: ApplicationEvent,
        _message: Option<Message>,
        frame: &Frame,
    ) -> Option<ApplicationResponse> {
        None
    }

    fn menu(&self) -> Option<&igat::menu::Menubar> {
        Some(&self.menu)
    }

    fn ui(&mut self, frame: &Frame) -> Option<Component<Message>> {
        let button = Button::new(frame)
            .width(70)
            .height(30)
            .text("Button".to_string())
            .on_click(Message::ButtonClicked);
        Some(build_component(button))
    }

    fn theme(&self) -> igat::Theme {
        self.theme
    }
}

fn main() {
    let exe = Executable::new();

    let mut menubar = Menubar::new();
    menubar.add(Menu::new("File".to_string()));
    menubar.add(Menu::new("Edit".to_string()));
    let app = SimpleApp {
        theme: Theme::default(),
        menu: menubar,
    };
    exe.run(app);
}
