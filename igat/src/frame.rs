use crate::{widget::Target, Color};
use crate::widget::Component;
use std::fmt::Debug;

pub trait Frame {
    type Message: Send + Debug;

    fn resizable(&self) -> bool;

    fn bgr(&self) -> Color;
    fn title(&self) -> String;
    fn ui(&self) -> Target<Self::Message>;
    fn new_ui(&self) -> Component<Self::Message>;
}
