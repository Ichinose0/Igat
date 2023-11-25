use crate::{Color, widget::{Element, Target}};
use std::fmt::Debug;

pub trait Frame {
    type Message: Send + Debug;

    fn resizable(&self) -> bool;

    fn bgr(&self) -> Color;
    fn title(&self) -> String;
    fn ui(&self) -> Target<Self::Message>;
}