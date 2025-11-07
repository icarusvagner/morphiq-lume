use iced::{
	Element, widget::{Text, container}
};

use crate::{core::utils::messages::Message, styles::style_type::StyleType};

#[derive(Default, Clone, Debug, Copy)]
pub struct PayrollView;

impl PayrollView {
	pub(crate) fn view<'a>() -> Element<'a, Message, StyleType> {
		container(Text::new("Payroll view").size(42)).into()
	}
}
