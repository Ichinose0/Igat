use igat::ApplicationResponse;
use igat::{
    menu::Menu,
    menu::Menubar,
    widget::{build_component, Button, Component, Text},
    Application, ApplicationEvent, Color, Executable, Frame, Theme,
};

#[derive(Clone, Copy, Debug)]
pub enum AppMessage {
    CountUp,
    CountDown,
}

pub struct Counter {
    count: i32,
}

impl Application<AppMessage> for Counter {

    fn set_up(&mut self, frame: &Frame) {
        //frame.set_resizable(false);
    }

    fn message(
        &mut self,
        event: ApplicationEvent,
        message: Option<AppMessage>,
        frame: &Frame,
    ) -> Option<ApplicationResponse> {
        match event {
            ApplicationEvent::RedrawRequested => {}
            ApplicationEvent::WidgetEvent => match message.unwrap() {
                AppMessage::CountUp => self.count += 1,
                AppMessage::CountDown => self.count -= 1,
            },
            ApplicationEvent::KeyboardInput(_) => {}
        }
        None
    }

    fn menu(&self) -> Option<&igat::menu::Menubar> {
        None
    }

    fn ui(&mut self, frame: &Frame) -> Option<Component<AppMessage>> {
        let button = Button::new(frame)
            .width(800)
            .height(600)
            .text(self.count.to_string())
            .on_click(AppMessage::CountUp);
        Some(build_component(button))
    }
}

fn main() {
    let exe = Executable::new();

    let app = Counter { count: 0 };
    exe.run(app);
}
