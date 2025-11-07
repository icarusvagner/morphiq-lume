use iced::{
	Element, widget::{Text, container}
};

use crate::{core::utils::message::Message, styles::style_type::StyleType};

#[derive(Default, Clone, Debug, Copy)]
pub struct AttendanceView;

impl AttendanceView {
	pub(crate) fn view<'a>() -> Element<'a, Message, StyleType> {
		container(Text::new("Attendance view").size(42)).into()
	}
}
