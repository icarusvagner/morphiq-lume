use iced::{
	Element, widget::{Text, container}
};

use crate::{core::utils::messages::Message, styles::style_type::StyleType};

#[derive(Default, Clone, Debug, Copy)]
pub struct EditProfileView;

impl EditProfileView {
	pub(crate) fn view<'a>() -> Element<'a, Message, StyleType> {
		container(Text::new("EditProfile view").size(42)).into()
	}
}
