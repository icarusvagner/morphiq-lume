use iced::{
	Background, Border, Color, widget::pick_list::{Catalog, Status, Style}
};

use crate::gui::styles::{
	style_constant::BORDER_RADIUS, types::style_type::StyleType
};

#[derive(Default)]
pub enum PicklistType {
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

#[allow(clippy::trivially_copy_pass_by_ref, clippy::unused_self)]
impl PicklistType {
	fn appearance(&self, style: &StyleType) -> iced::overlay::menu::Style {
		let colors = style.get_palette();

		iced::overlay::menu::Style {
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
			border: Border {
				color: Color { a: 0.50, ..Color::BLACK },
				width: 1.0,
				radius: BORDER_RADIUS.into(),
			},
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
			selected_text_color: colors.primary_content,
			selected_background: Background::Color(colors.primary),
		}
	}
}

#[allow(clippy::trivially_copy_pass_by_ref)]
impl PicklistType {
	fn active_opened(&self, style: &StyleType) -> Style {
		let colors = style.get_palette();

		Style {
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
			placeholder_color: match self {
				Self::Primary => lighten_color(colors.primary_content),
				Self::Secondary => lighten_color(colors.secondary_content),
				Self::Accent => lighten_color(colors.accent_content),
				Self::Neutral => lighten_color(colors.neutral_content),
				Self::Info => lighten_color(colors.info_content),
				Self::Success => lighten_color(colors.success_content),
				Self::Warning => lighten_color(colors.warning_content),
				Self::Error => lighten_color(colors.error_content),
				_ => lighten_color(colors.base_content),
			},
			handle_color: colors.primary,
			background: Background::Color(match self {
				Self::Primary => colors.primary,
				Self::Secondary => colors.secondary,
				Self::Accent => colors.accent,
				Self::Neutral => colors.neutral,
				Self::Info => colors.info,
				Self::Success => colors.success,
				Self::Warning => colors.warning,
				Self::Error => colors.error,
				Self::Base100 => colors.base_100,
				Self::Base200 => colors.base_200,
				Self::Base300 => colors.base_300,
				Self::Ghost => Color::TRANSPARENT,
			}),
			border: Border {
				color: Color::TRANSPARENT,
				width: 0.0,
				radius: BORDER_RADIUS.into(),
			},
		}
	}

	fn hovered(&self, style: &StyleType) -> Style {
		let colors = style.get_palette();

		Style {
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
			placeholder_color: match self {
				Self::Primary => lighten_color(colors.primary_content),
				Self::Secondary => lighten_color(colors.secondary_content),
				Self::Accent => lighten_color(colors.accent_content),
				Self::Neutral => lighten_color(colors.neutral_content),
				Self::Info => lighten_color(colors.info_content),
				Self::Success => lighten_color(colors.success_content),
				Self::Warning => lighten_color(colors.warning_content),
				Self::Error => lighten_color(colors.error_content),
				_ => lighten_color(colors.base_content),
			},
			handle_color: lighten_color(colors.primary),
			background: Background::Color(match self {
				Self::Primary => lighten_color(colors.primary),
				Self::Secondary => lighten_color(colors.secondary),
				Self::Accent => lighten_color(colors.accent),
				Self::Neutral => lighten_color(colors.neutral),
				Self::Info => lighten_color(colors.info),
				Self::Success => lighten_color(colors.success),
				Self::Warning => lighten_color(colors.warning),
				Self::Error => lighten_color(colors.error),
				Self::Base100 => lighten_color(colors.base_100),
				Self::Base200 => lighten_color(colors.base_200),
				Self::Base300 | Self::Ghost => lighten_color(colors.base_300),
			}),
			border: Border {
				color: Color::TRANSPARENT,
				width: 0.0,
				radius: BORDER_RADIUS.into(),
			},
		}
	}
}

impl iced::overlay::menu::Catalog for StyleType {
	type Class<'a> = PicklistType;

	fn default<'a>() -> <Self as iced::overlay::menu::Catalog>::Class<'a> {
		<Self as iced::overlay::menu::Catalog>::Class::default()
	}

	fn style(
		&self,
		class: &<Self as iced::overlay::menu::Catalog>::Class<'_>,
	) -> iced::overlay::menu::Style {
		class.appearance(self)
	}
}

impl Catalog for StyleType {
	type Class<'a> = PicklistType;

	fn default<'a>() -> <Self as Catalog>::Class<'a> {
		<Self as Catalog>::Class::default()
	}

	fn style(
		&self,
		class: &<Self as Catalog>::Class<'_>,
		status: Status,
	) -> Style {
		match status {
			Status::Active | Status::Opened => class.active_opened(self),
			Status::Hovered => class.hovered(self),
		}
	}
}
