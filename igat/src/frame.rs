use crate::widget::Component;
use crate::Color;
use std::fmt::Debug;

#[deprecated(
    since = "0.0.2",
    note = "UI construction using Frames will be discontinued."
)]
pub trait Frame {
    type Message: Send + Debug;

    fn resizable(&self) -> bool;

    fn bgr(&self) -> Color;
    fn title(&self) -> String;
    fn ui(&self) -> Option<Component<Self::Message>>;
}
