use iced::{
	Element, widget::{Text, container}
};

use crate::{core::utils::messages::Message, styles::style_type::StyleType};

#[derive(Default, Clone, Debug, Copy)]
pub struct LeavesView;

impl LeavesView {
	pub(crate) fn view<'a>() -> Element<'a, Message, StyleType> {
		container(Text::new("Leaves view").size(42)).into()
	}
}
