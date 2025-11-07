use iced::{
	Color,
	widget::text::{
		Catalog,
		Style,
	},
};

use crate::gui::styles::types::style_type::StyleType;

#[derive(Debug, Default, Clone, PartialEq, Copy, Eq)]
#[allow(clippy::large_enum_variant)]
pub enum TextType {
	#[default]
	BaseContent,
	Content,
	Standard,
	Info,
	Success,
	Warning,
	Error,
}

#[allow(clippy::trivially_copy_pass_by_ref, clippy::use_self)]
impl TextType {
	fn appearance(self, style: &StyleType) -> Style {
		let color = style.get_palette();

		Style {
			color: if self == TextType::Standard {
				None
			} else {
				Some(match self {
					TextType::BaseContent => color.base_content,
					TextType::Content => color.primary_content,
					TextType::Standard => Color::default(),
					TextType::Info => color.info,
					TextType::Success => color.success,
					TextType::Warning => color.warning,
					TextType::Error => color.error,
				})
			},
		}
	}
}

impl Catalog for StyleType {
	type Class<'a> = TextType;

	fn default<'a>() -> Self::Class<'a> {
		Self::Class::default()
	}

	fn style(&self, item: &Self::Class<'_>) -> Style {
		item.appearance(self)
	}
}
