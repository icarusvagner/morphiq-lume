use iced::{Background, Color};
use iced_aw::{card::Status, date_picker::Style, style::date_picker::Catalog};

use crate::gui::styles::{
	style_constant::BORDER_RADIUS, types::style_type::StyleType
};

#[derive(Default)]
pub enum DatePickerType {
	Base100,
	#[default]
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
	Ghost,
}

fn lighten_color(color: Color) -> Color {
	Color { a: color.a - 0.05, ..color }
}

#[allow(clippy::unused_self, clippy::trivially_copy_pass_by_ref)]
impl DatePickerType {
	const fn active(&self, style: &StyleType) -> Style {
		let colors = style.get_palette();

		Style {
			background: Background::Color(match self {
				Self::Base100 => colors.base_100,
				Self::Base200 => colors.base_200,
				Self::Base300 => colors.base_300,
				Self::Primary => colors.primary,
				Self::Secondary => colors.secondary,
				Self::Accent => colors.accent,
				Self::Neutral => colors.neutral,
				Self::Info => colors.info,
				Self::Success => colors.success,
				Self::Warning => colors.warning,
				Self::Error => colors.error,
				Self::Ghost => Color { a: 0.60, ..Color::BLACK },
			}),
			border_radius: 0.0,
			border_width: 0.0,
			border_color: Color { a: 0.50, ..Color::BLACK },
			text_color: match self {
				Self::Primary => colors.primary_content,
				Self::Secondary => colors.secondary_content,
				Self::Accent => colors.accent_content,
				Self::Neutral => colors.neutral_content,
				Self::Info => colors.info_content,
				Self::Success => colors.success_content,
				Self::Warning => colors.warning_content,
				Self::Error => colors.error_content,
				_ => colors.base_content,
			},
			text_attenuated_color: colors.info,
			day_background: Background::Color(match self {
				Self::Base100 => colors.base_100,
				Self::Base200 => colors.base_200,
				Self::Base300 => colors.base_300,
				Self::Primary => colors.primary,
				Self::Secondary => colors.secondary,
				Self::Accent => colors.accent,
				Self::Neutral => colors.neutral,
				Self::Info => colors.info,
				Self::Success => colors.success,
				Self::Warning => colors.warning,
				Self::Error => colors.error,
				Self::Ghost => Color { a: 0.60, ..Color::BLACK },
			}),
		}
	}

	fn hovered(&self, style: &StyleType) -> Style {
		let colors = style.get_palette();

		Style {
			background: Background::Color(match self {
				Self::Base100 => lighten_color(colors.base_100),
				Self::Base200 => lighten_color(colors.base_200),
				Self::Base300 => lighten_color(colors.base_300),
				Self::Primary => lighten_color(colors.primary),
				Self::Secondary => lighten_color(colors.secondary),
				Self::Accent => lighten_color(colors.accent),
				Self::Neutral => lighten_color(colors.neutral),
				Self::Info => lighten_color(colors.info),
				Self::Success => lighten_color(colors.success),
				Self::Warning => lighten_color(colors.warning),
				Self::Error => lighten_color(colors.error),
				Self::Ghost => Color { a: 0.60, ..Color::BLACK },
			}),
			border_radius: BORDER_RADIUS,
			border_width: 1.0,
			border_color: Color { a: 0.50, ..Color::BLACK },
			text_color: colors.primary_content,
			text_attenuated_color: lighten_color(Color::BLACK),
			day_background: Background::Color(colors.primary),
		}
	}

	const fn disabled(&self, style: &StyleType) -> Style {
		let colors = style.get_palette();

		Style {
			background: Background::Color(Color { a: 0.50, ..colors.info }),
			border_radius: 0.0,
			border_width: 1.0,
			border_color: colors.neutral,
			text_color: Color { a: 0.50, ..colors.info_content },
			text_attenuated_color: colors.primary_content,
			day_background: Background::Color(colors.info),
		}
	}

	fn focused(&self, style: &StyleType) -> Style {
		let colors = style.get_palette();

		Style {
			background: Background::Color(colors.primary),
			border_radius: 0.0,
			border_width: 1.0,
			border_color: lighten_color(colors.neutral),
			text_color: colors.primary_content,
			text_attenuated_color: colors.base_content,
			day_background: Background::Color(colors.secondary),
		}
	}
}

impl Catalog for StyleType {
	type Class<'a> = DatePickerType;

	fn default<'a>() -> Self::Class<'a> {
		Self::Class::default()
	}

	fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
		match status {
			Status::Active | Status::Selected => class.active(self),
			Status::Hovered => class.hovered(self),
			Status::Pressed | Status::Focused => class.focused(self),
			Status::Disabled => class.disabled(self),
		}
	}
}
