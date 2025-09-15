use iced::{
    widget::{container, text},
    Element,
};

use crate::gui::{
    styles::{text::TextType, types::style_type::StyleType},
    types::message::Message,
};

#[derive(Default, Debug, Clone)]
pub struct ViewLanguages;

impl ViewLanguages {
    pub(crate) fn view<'a>(&'a self) -> Element<'a, Message, StyleType> {
        container(
            text("Languages Settings view")
                .size(42)
                .class(TextType::Neutral),
        )
        .into()
    }
}
