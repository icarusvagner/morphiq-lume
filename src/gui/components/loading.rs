use iced::{widget::text, Element};

use crate::gui::{styles::types::style_type::StyleType, types::message::Message};

#[derive(Debug, Clone, Default)]
pub struct LoadingContainer;

impl LoadingContainer {
    pub fn view<'a>(&self) -> Element<'a, Message, StyleType> {
        text("Testing").size(24).into()
    }
}
