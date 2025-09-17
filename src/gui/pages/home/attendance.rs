use iced::{
    widget::{container, Text},
    Element,
};

use crate::gui::{styles::types::style_type::StyleType, types::message::Message};

#[derive(Default, Clone, Debug, Copy)]
pub struct AttendanceView;

impl AttendanceView {
    pub(crate) fn view<'a>() -> Element<'a, Message, StyleType> {
        container(Text::new("Attendance view").size(42)).into()
    }
}
