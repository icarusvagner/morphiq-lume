use iced::{
	Background,
	Border,
	Color,
	border::Radius,
	widget::text_input::{
		Catalog,
		Status,
		Style,
	},
};

use crate::gui::styles::{
	style_constant::{
		BORDER_RADIUS,
		BORDER_WIDTH,
	},
	types::style_type::StyleType,
};

#[derive(Debug, Default, Clone, PartialEq, Copy, Eq)]
#[allow(clippy::large_enum_variant)]
pub enum TextInputType {
	Base100,
	Base200,
	Base300,
	#[default]
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

#[allow(
	clippy::unused_self,
	clippy::use_self,
	clippy::trivially_copy_pass_by_ref
)]
impl TextInputType {
	#[allow(clippy::unused_self)]
	const fn selection_color(&self, style: &StyleType) -> Color {
		style.get_palette().primary
	}

	#[allow(clippy::unused_self)]
	fn lighten_color(&self, color: Color) -> Color {
		Color { a: color.a - 0.20, ..color }
	}

	#[allow(clippy::unused_self)]
	fn placeholder_color(&self, style: &StyleType) -> Color {
		self.lighten_color(style.get_palette().base_content)
	}

	#[allow(clippy::unused_self)]
	const fn value_color(&self, style: &StyleType) -> Color {
		style.get_palette().base_content
	}
	fn active(&self, style: &StyleType) -> Style {
		let colors = style.get_palette();

		Style {
			background: Background::Color(Color::TRANSPARENT),
			border: match self {
				TextInputType::Ghost => Border::default(),
				_ => Border {
					radius: BORDER_RADIUS.into(),
					width: BORDER_WIDTH,
					color: match self {
						TextInputType::Primary => colors.primary,
						TextInputType::Secondary => colors.secondary,
						TextInputType::Accent => colors.accent,
						TextInputType::Neutral => colors.neutral,
						TextInputType::Info => colors.info,
						TextInputType::Success => colors.success,
						TextInputType::Warning => colors.warning,
						TextInputType::Error => colors.error,
						_ => colors.base_300,
					},
				},
			},
			icon: match self {
				TextInputType::Primary => colors.primary,
				TextInputType::Secondary => colors.secondary,
				TextInputType::Accent => colors.accent,
				TextInputType::Neutral => colors.neutral,
				TextInputType::Info => colors.info,
				TextInputType::Success => colors.success,
				TextInputType::Warning => colors.warning,
				TextInputType::Error => colors.error,
				_ => colors.base_300,
			},
			placeholder: self.placeholder_color(style),
			value: self.value_color(style),
			selection: self.selection_color(style),
		}
	}

	fn hovered(&self, style: &StyleType) -> Style {
		let colors = style.get_palette();

		Style {
			background: Background::Color(Color::TRANSPARENT),
			border: Border {
				color: match self {
					TextInputType::Base100
					| TextInputType::Base200
					| TextInputType::Base300 => self.lighten_color(colors.base_100),
					TextInputType::Primary => colors.primary,
					TextInputType::Secondary => colors.secondary,
					TextInputType::Accent => colors.accent,
					TextInputType::Neutral => colors.neutral,
					TextInputType::Info => colors.info,
					TextInputType::Success => colors.success,
					TextInputType::Warning => colors.warning,
					TextInputType::Error => colors.error,
					TextInputType::Ghost => Color::TRANSPARENT,
				},
				width: BORDER_WIDTH,
				radius: BORDER_RADIUS.into(),
			},
			icon: colors.primary_content,
			placeholder: self.placeholder_color(style),
			value: self.value_color(style),
			selection: self.selection_color(style),
		}
	}

	fn disabled(&self, style: &StyleType) -> Style {
		let colors = style.get_palette();

		Style {
			background: Background::Color(Color::TRANSPARENT),
			border: match self {
				TextInputType::Ghost => Border {
					color: Color::TRANSPARENT,
					width: 0.0,
					radius: Radius::default(),
				},
				_ => Border {
					color: self.lighten_color(colors.base_100),
					width: BORDER_WIDTH,
					radius: BORDER_RADIUS.into(),
				},
			},
			icon: colors.primary_content,
			placeholder: self.placeholder_color(style),
			value: self.value_color(style),
			selection: self.selection_color(style),
		}
	}

	fn focused(&self, style: &StyleType) -> Style {
		let colors = style.get_palette();

		Style {
			background: Background::Color(Color::TRANSPARENT),
			border: match self {
				TextInputType::Ghost => Border::default(),
				_ => Border {
					color: match self {
						TextInputType::Primary => {
							self.lighten_color(colors.primary)
						}
						TextInputType::Secondary => {
							self.lighten_color(colors.secondary)
						}
						TextInputType::Accent => {
							self.lighten_color(colors.accent)
						}
						TextInputType::Neutral => {
							self.lighten_color(colors.neutral)
						}
						TextInputType::Info => self.lighten_color(colors.info),
						TextInputType::Success => {
							self.lighten_color(colors.success)
						}
						TextInputType::Warning => {
							self.lighten_color(colors.warning)
						}
						TextInputType::Error => {
							self.lighten_color(colors.error)
						}
						_ => self.lighten_color(colors.base_300),
					},
					width: BORDER_WIDTH,
					radius: BORDER_RADIUS.into(),
				},
			},
			icon: colors.base_content,
			placeholder: self.placeholder_color(style),
			value: self.value_color(style),
			selection: self.selection_color(style),
		}
	}
}

impl Catalog for StyleType {
	type Class<'a> = TextInputType;

	fn default<'a>() -> Self::Class<'a> {
		Self::Class::default()
	}

	fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
		match status {
			Status::Active => class.active(self),
			Status::Hovered => class.hovered(self),
			Status::Focused => class.focused(self),
			Status::Disabled => class.disabled(self),
		}
	}
}
