use iced::application::{
	Appearance,
	DefaultStyle,
};
use serde::{
	Deserialize,
	Serialize,
};

use crate::gui::styles::{
	style_constant::colors::{
		DARK_COLOR,
		LIGHT_COLOR,
	},
	types::palette::Palette,
};

#[derive(
	Default, Clone, Copy, Serialize, Deserialize, Debug, Hash, PartialEq, Eq,
)]
#[serde(tag = "style", content = "name")]
#[allow(clippy::large_enum_variant)]
pub enum StyleType {
	#[default]
	Light,
	Dark,
}

impl DefaultStyle for StyleType {
	fn default_style(&self) -> Appearance {
		let color = self.get_palette();

		Appearance {
			background_color: color.base_200,
			text_color: color.base_content,
		}
	}
}

#[allow(clippy::use_self)]
impl StyleType {
	pub const fn get_palette(self) -> Palette {
		match self {
			StyleType::Light => LIGHT_COLOR,
			StyleType::Dark => DARK_COLOR,
		}
	}
}
