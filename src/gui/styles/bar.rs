use iced::Color;

use crate::gui::styles::types::style_type::StyleType;

#[derive(Default)]
pub enum BarType {
	#[default]
	Standard,
}

impl BarType {
	#[allow(clippy::unused_self, clippy::trivially_copy_pass_by_ref)]
	const fn active(&self, style: &StyleType) -> Style {
		let colors = style.get_palette();

		Style {
			background: colors.base_300,
			primary: colors.primary,
			secondary: colors.secondary,
			accent: colors.accent,
			text_color: colors.base_content,
		}
	}
}

impl Catalog for StyleType {
	type Class<'a> = BarType;

	fn default<'a>() -> Self::Class<'a> {
		Self::Class::default()
	}

	fn style(&self, class: &Self::Class<'_>) -> Style {
		class.active(self)
	}
}

pub struct Style {
	pub(crate) background: Color,
	pub(crate) primary: Color,
	pub(crate) secondary: Color,
	pub(crate) accent: Color,
	pub(crate) text_color: Color,
}

pub trait Catalog {
	type Class<'a>;

	fn default<'a>() -> Self::Class<'a>;

	fn style(&self, class: &Self::Class<'_>) -> Style;
}
