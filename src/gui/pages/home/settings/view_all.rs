use iced::{
	Element,
	widget::{
		container,
		text,
	},
};

use crate::gui::{
	styles::types::style_type::StyleType,
	types::message::Message,
};

#[derive(Default, Debug, Clone)]
pub struct ViewAll;

#[allow(clippy::unused_self)]
impl ViewAll {
	pub(crate) fn view(&self) -> Element<'_, Message, StyleType> {
		container(text("Settings view").size(42)).into()
	}
}
