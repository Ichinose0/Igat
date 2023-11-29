use igat::{frame::Frame, Application, ApplicationEvent, Executable, widget::{Component, build_component, NewButton}};

#[derive(Debug)]
pub enum Message {}

pub struct Poweredit {
    frame: MyFrame,
}

impl Application<Message> for Poweredit {
    type Message = Message;

    fn route(&self, event: ApplicationEvent) -> &dyn Frame<Message = Self::Message> {
        &self.frame
    }

    fn on_close(&self) {}

    fn set_up(&mut self) {}

    fn message(&mut self,event: Message) {
        
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
        let button = NewButton::new()
                                            .width(240)
                                            .height(80)
                                            .x(20)
                                            .y(20);
        Some(build_component(button))
    }

    fn resizable(&self) -> bool {
        true
    }
}

fn main() {
    let exe = Executable::new();
    let app = Poweredit { frame: MyFrame {} };
    exe.run(app);
}
