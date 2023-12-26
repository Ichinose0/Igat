// use igat::widget::Panel;
// use igat::ApplicationResponse;
// use igat::{
//     menu::Menu,
//     menu::Menubar,
//     widget::{build_component, Button, Component, Text},
//     Application, ApplicationEvent, Color, Executable, Frame, Theme,
// };

// #[derive(Clone, Copy, Debug)]
// pub enum AppMessage {
//     CountUp,
//     CountDown,
// }

// pub struct Counter {
//     count: i32,
// }

// impl Application<AppMessage> for Counter {
//     fn set_up(&mut self, frame: &Frame) {
//         //frame.set_resizable(false);
//     }

//     fn message(
//         &mut self,
//         event: ApplicationEvent,
//         message: Option<AppMessage>,
//         frame: &Frame,
//     ) -> Option<ApplicationResponse> {
//         match event {
//             ApplicationEvent::RedrawRequested => {}
//             ApplicationEvent::WidgetEvent => match message.unwrap() {
//                 AppMessage::CountUp => self.count += 1,
//                 AppMessage::CountDown => self.count -= 1,
//             },
//             ApplicationEvent::KeyboardInput(_) => {}
//         }
//         None
//     }

//     fn menu(&self) -> Option<&igat::menu::Menubar> {
//         None
//     }

//     fn ui(&mut self, frame: &Frame) -> Option<Component<AppMessage>> {
//         let countup = Button::new(frame)
//             .width(240)
//             .height(80)
//             .x(10)
//             .y(100)
//             .text("Up".to_owned())
//             .on_click(AppMessage::CountUp);
//         let countdown = Button::new(frame)
//             .width(240)
//             .height(80)
//             .x(280)
//             .y(100)
//             .text("Down".to_owned())
//             .on_click(AppMessage::CountDown);
//         let count = Text::new(frame)
//             .width(190)
//             .height(40)
//             .x(180)
//             .y(20)
//             .text(self.count.to_string())
//             .on_click(AppMessage::CountDown);
//         let panel = Panel::new().child(countup).child(countdown).child(count);
//         Some(panel.into())
//     }
// }

// fn main() {
//     let exe = Executable::new();

//     let app = Counter { count: 0 };
//     exe.run(app);
// }

use std::str::FromStr;

use igat::{
    widget::{Button, Component, Panel, WidgetMessage, Data},
    IApplicationBuilder, Theme, Window, WindowEvent,
};

#[derive(Clone, Copy, Debug)]
pub enum Message {
    ButtonClicked,
}

pub struct Counter {
    pub count: i32
}

impl Counter {
    pub fn new() -> Self {
        Self {
            count: 0
        }
    }
}

impl Data for Counter {

}

fn main() {
    let counter = Counter::new();
    let window = Window::new(ui(counter));
    let mut app = IApplicationBuilder::new()
        .with(window)
        .theme(Theme::DARK)
        .build();
    app.run(|event| match event {
        _ => {}
    });
}

fn ui(counter: Counter) -> Component<Counter> {
    let button = Button::new()
        .width(240)
        .height(40)
        .text("Click me".to_owned())
        .on_message(|message, prop,counter: &mut Counter| match message {
            WidgetMessage::OnClick => {
                counter.count += 1;
                prop.text = counter.count.to_string();
            }

            _ => {}
        });
    let panel = Panel::new(counter).child(button);
    panel.into_component()
}
