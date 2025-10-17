//! Rule Style

use iced::Color;
#[allow(clippy::module_name_repetitions)]
use iced::widget::rule::{
	Catalog,
	FillMode,
	Style,
};

use crate::gui::styles::types::style_type::StyleType;

#[derive(Default)]
pub enum RuleType {
	#[default]
	Base100,
	Base200,
	Base300,
	Primary,
	Secondary,
	Accent,
	Neutral,
	Info,
	Success,
	Warning,
	Error,

	PaletteColor(Color, u16),
}

impl RuleType {
	#[allow(clippy::trivially_copy_pass_by_ref, clippy::use_self)]
	fn appearance(&self, style: &StyleType) -> Style {
		let colors = style.get_palette();

		Style {
			color: match self {
				RuleType::Base100 => colors.base_100,
				RuleType::Base200 => colors.base_200,
				RuleType::Base300 => colors.base_300,
				RuleType::Primary => colors.primary,
				RuleType::Secondary => colors.secondary,
				RuleType::Accent => colors.accent,
				RuleType::Neutral => colors.neutral,
				RuleType::Info => colors.info,
				RuleType::Success => colors.success,
				RuleType::Warning => colors.warning,
				RuleType::Error => colors.error,

				RuleType::PaletteColor(color, _) => *color,
			},
			width: match self {
				RuleType::PaletteColor(_, width) => *width,
				_ => 1,
			},
			radius: 0.0.into(),
			fill_mode: FillMode::Full,
		}
	}
}

impl Catalog for StyleType {
	type Class<'a> = RuleType;

	fn default<'a>() -> Self::Class<'a> {
		Self::Class::default()
	}

	fn style(&self, class: &Self::Class<'_>) -> Style {
		class.appearance(self)
	}
}
