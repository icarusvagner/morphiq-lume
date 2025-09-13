use iced::{
    widget::{container, Text},
    Element,
};

use crate::gui::{
    styles::{text::TextType, types::style_type::StyleType},
    types::message::Message,
};

#[derive(Default, Clone, Debug, Copy)]
pub struct EditProfileView;

impl EditProfileView {
    pub(crate) fn view<'a>() -> Element<'a, Message, StyleType> {
        container(
            Text::new("EditProfile view")
                .size(42)
                .class(TextType::Neutral),
        )
        .into()
    }
}
