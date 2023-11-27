use igat::{frame::Frame, Application, ApplicationEvent, Executable};

#[derive(Debug)]
pub enum Message {}

pub struct Poweredit {
    frame: MyFrame,
}

impl Application for Poweredit {
    type Message = Message;

    fn init(&mut self, loader: &igat::plugin::PluginLoader) {}

    fn route(&mut self, event: ApplicationEvent) -> &dyn Frame<Message = Self::Message> {
        &self.frame
    }

    fn on_close(&mut self) {}
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

    fn ui(&self) -> igat::widget::Target<Self::Message> {
        let text = igat::widget::Button::new()
            .width(120)
            .height(60)
            .text(String::from("ボタン"));

        text.build()
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
