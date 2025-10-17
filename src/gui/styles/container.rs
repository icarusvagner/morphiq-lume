use iced::{
	Background,
	Border,
	Color,
	Shadow,
	widget::container::{
		Catalog,
		Style,
	},
};

use crate::gui::styles::{
	style_constant::{
		BORDER_RADIUS,
		StandardNames,
	},
	types::style_type::StyleType,
};

#[allow(clippy::enum_variant_names, clippy::large_enum_variant, dead_code)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerType {
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
	Overlay,
	Icon(StandardNames),
	#[default]
	Standard,
	Bordered,
	Ghost,
}

impl ContainerType {
	#[allow(clippy::unused_self, clippy::trivially_copy_pass_by_ref)]
	fn lighten_color(&self, color: Color) -> Color {
		Color { a: color.a - 0.20, ..color }
	}

	#[allow(clippy::trivially_copy_pass_by_ref, clippy::use_self)]
	fn appearance(self, style: &StyleType) -> Style {
		let colors = style.get_palette();

		Style {
			text_color: match self {
				ContainerType::Overlay => None,
				ContainerType::Icon(icon_name) => Some(match icon_name {
					StandardNames::Base100
					| StandardNames::Base200
					| StandardNames::Base300 => colors.base_content,
					StandardNames::Primary
					| StandardNames::Secondary
					| StandardNames::Accent
					| StandardNames::Neutral => colors.primary_content,
					StandardNames::Info => colors.info_content,
					StandardNames::Success => colors.success_content,
					StandardNames::Warning => colors.warning_content,
					StandardNames::Error => colors.error_content,
				}),
				_ => Some(match self {
					ContainerType::Standard | ContainerType::Bordered => {
						colors.base_content
					}
					_ => colors.primary_content,
				}),
			},
			background: match self {
				ContainerType::Standard => {
					Some(Background::Color(colors.base_200))
				}
				ContainerType::Icon(icon_name) => {
					Some(Background::Color(match icon_name {
						StandardNames::Base100 => colors.base_100,
						StandardNames::Base200 => colors.base_200,
						StandardNames::Base300 => colors.base_300,
						StandardNames::Primary => colors.primary,
						StandardNames::Secondary => colors.secondary,
						StandardNames::Accent => colors.accent,
						StandardNames::Neutral => colors.neutral,
						StandardNames::Info => colors.info,
						StandardNames::Success => colors.success,
						StandardNames::Warning => colors.warning,
						StandardNames::Error => colors.error,
					}))
				}
				_ => Some(match self {
					ContainerType::Primary => Background::Color(colors.primary),
					ContainerType::Secondary => {
						Background::Color(colors.secondary)
					}
					ContainerType::Accent => Background::Color(colors.accent),
					ContainerType::Neutral => Background::Color(colors.neutral),
					ContainerType::Info => Background::Color(colors.info),
					ContainerType::Error => Background::Color(colors.error),
					ContainerType::Base100 => {
						Background::Color(colors.base_100)
					}
					ContainerType::Base200 => {
						Background::Color(colors.base_200)
					}
					ContainerType::Base300 => {
						Background::Color(colors.base_300)
					}
					ContainerType::Overlay => Background::Color(Color {
						a: colors.neutral.a - 0.60,
						..colors.neutral
					}),
					_ => Background::Color(Color::TRANSPARENT),
				}),
			},
			border: match self {
				ContainerType::Standard | ContainerType::Overlay => {
					Border::default()
				}
				ContainerType::Ghost => Border::default(),
				ContainerType::Icon(_) => Border {
					width: 0.0,
					radius: 1000.0.into(),
					..Default::default()
				},
				_ => Border {
					color: self.lighten_color(colors.base_300),
					width: 1.0,
					radius: BORDER_RADIUS.into(),
				},
			},
			shadow: Shadow::default(),
		}
	}
}

impl Catalog for StyleType {
	type Class<'a> = ContainerType;

	fn default<'a>() -> Self::Class<'a> {
		Self::Class::default()
	}

	fn style(&self, class: &Self::Class<'_>) -> Style {
		class.appearance(self)
	}
}
