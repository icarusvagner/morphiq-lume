use iced::{Element, widget::text};

use crate::{core::utils::messages::Message, styles::style_type::StyleType};

#[derive(Debug, Clone, Default)]
pub struct LoadingContainer;

impl LoadingContainer {
	#[allow(clippy::unused_self)]
	pub fn view<'a>(&self) -> Element<'a, Message, StyleType> {
		text("Testing").size(24).into()
	}
}
