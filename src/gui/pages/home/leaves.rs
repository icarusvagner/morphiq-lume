use iced::{
    widget::{container, Text},
    Element,
};

use crate::gui::{styles::types::style_type::StyleType, types::message::Message};

#[derive(Default, Clone, Debug, Copy)]
pub struct LeavesView;

impl LeavesView {
    pub(crate) fn view<'a>() -> Element<'a, Message, StyleType> {
        container(Text::new("Leaves view").size(42)).into()
    }
}
