use iced::{
    widget::{container, Text},
    Element,
};

use crate::gui::{
    styles::{text::TextType, types::style_type::StyleType},
    types::message::Message,
};

#[derive(Debug, Clone, Default, Copy)]
pub struct DashboardView;

impl DashboardView {
    pub(crate) fn view<'a>() -> Element<'a, Message, StyleType> {
        container(
            Text::new("Dashboard view")
                .size(42)
                .class(TextType::Neutral),
        )
        .into()
    }
}
