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
pub struct ViewOrganization;

#[allow(clippy::unused_self)]
impl ViewOrganization {
	pub(crate) fn view(&self) -> Element<'_, Message, StyleType> {
		container(text("Organization Settings view").size(42)).into()
	}
}
