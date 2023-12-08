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

pub struct SampleApp {
    menu: Menubar,
    num: u32
}

impl Application<Message> for SampleApp {
    type Message = Message;

    fn on_close(&self) {}

    fn set_up(&mut self, frame: &Frame) {}

    fn message(
        &mut self,
        event: ApplicationEvent,
        message: Option<Message>,
        frame: &Frame,
    ) -> Option<ApplicationResponse> {
        match event {
            ApplicationEvent::RedrawRequested => {},
            ApplicationEvent::WidgetEvent => {
                match message.unwrap() {
                    Message::ButtonClicked => {self.num+=1},
                }
            },
            ApplicationEvent::KeyboardInput(_) => {},
        }
        None
    }

    fn menu(&self) -> Option<&igat::menu::Menubar> {
        Some(&self.menu)
    }

    fn ui(&mut self, frame: &Frame) -> Option<Component<Message>> {
        let button = Button::new(frame)
            .width(70)
            .height(30)
            .text(self.num.to_string())
            .on_click(Message::ButtonClicked);
        Some(build_component(button))
    }
}

fn main() {
    let exe = Executable::new();

    let mut menubar = Menubar::new();
    menubar.add(Menu::new("File".to_string()));
    menubar.add(Menu::new("Edit".to_string()));
    let app = SampleApp { menu: menubar,num:0 };
    exe.run(app);
}
