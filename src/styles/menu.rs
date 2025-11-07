use iced::{Background, Border, Color, Padding, Shadow};
use iced_aw::{
	card::Status, menu::{Catalog, Style}
};

use crate::gui::styles::{
	style_constant::{BORDER_RADIUS, BORDER_WIDTH}, types::style_type::StyleType
};

#[derive(Default)]
pub enum MenuType {
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

#[allow(clippy::use_self, clippy::trivially_copy_pass_by_ref)]
impl MenuType {
	#[allow(clippy::unused_self)]
	fn lighten_color(&self, color: Color) -> Color {
		Color { a: color.a - 0.05, ..color }
	}

	fn active_focused(&self, style: &StyleType) -> Style {
		let colors = style.get_palette();

		Style {
			bar_background: match self {
				MenuType::Base100 => Background::Color(colors.base_100),
				MenuType::Base200 => Background::Color(colors.base_200),
				MenuType::Base300 => Background::Color(colors.base_300),
				MenuType::Primary => Background::Color(colors.primary),
				MenuType::Secondary => Background::Color(colors.secondary),
				MenuType::Accent => Background::Color(colors.accent),
				MenuType::Neutral => Background::Color(colors.neutral),
				MenuType::Info => Background::Color(colors.info),
				MenuType::Success => Background::Color(colors.success),
				MenuType::Warning => Background::Color(colors.warning),
				MenuType::Error => Background::Color(colors.error),
				MenuType::Ghost => Background::Color(Color::TRANSPARENT),
			},
			bar_border: match self {
				MenuType::Ghost => Border::default(),
				_ => Border {
					color: match self {
						MenuType::Primary => colors.primary,
						MenuType::Secondary => colors.secondary,
						MenuType::Accent => colors.accent,
						MenuType::Neutral => colors.neutral,
						MenuType::Info => colors.info,
						MenuType::Success => colors.success,
						MenuType::Warning => colors.warning,
						MenuType::Error => colors.error,
						_ => colors.base_200,
					},
					width: BORDER_WIDTH,
					radius: BORDER_RADIUS.into(),
				},
			},
			bar_shadow: Shadow::default(),
			bar_background_expand: Padding::default(),
			menu_background: match self {
				MenuType::Base100 => Background::Color(colors.base_100),
				MenuType::Base200 => Background::Color(colors.base_200),
				MenuType::Base300 => Background::Color(colors.base_300),
				MenuType::Primary => Background::Color(colors.primary),
				MenuType::Secondary => Background::Color(colors.secondary),
				MenuType::Accent => Background::Color(colors.accent),
				MenuType::Neutral => Background::Color(colors.neutral),
				MenuType::Info => Background::Color(colors.info),
				MenuType::Success => Background::Color(colors.success),
				MenuType::Warning => Background::Color(colors.warning),
				MenuType::Error => Background::Color(colors.error),
				MenuType::Ghost => Background::Color(Color::TRANSPARENT),
			},
			menu_border: match self {
				MenuType::Ghost => Border::default(),
				_ => Border {
					color: match self {
						MenuType::Primary => colors.primary,
						MenuType::Secondary => colors.secondary,
						MenuType::Accent => colors.accent,
						MenuType::Neutral => colors.neutral,
						MenuType::Info => colors.info,
						MenuType::Success => colors.success,
						MenuType::Warning => colors.warning,
						MenuType::Error => colors.error,
						_ => colors.base_200,
					},
					width: BORDER_WIDTH,
					radius: BORDER_RADIUS.into(),
				},
			},
			menu_shadow: Shadow::default(),
			menu_background_expand: Padding::from(5.0),
			path: match self {
				MenuType::Base100 => Background::Color(colors.base_100),
				MenuType::Base200 => Background::Color(colors.base_200),
				MenuType::Base300 => Background::Color(colors.base_300),
				MenuType::Primary => Background::Color(colors.primary),
				MenuType::Secondary => Background::Color(colors.secondary),
				MenuType::Accent => Background::Color(colors.accent),
				MenuType::Neutral => Background::Color(colors.neutral),
				MenuType::Info => Background::Color(colors.info),
				MenuType::Success => Background::Color(colors.success),
				MenuType::Warning => Background::Color(colors.warning),
				MenuType::Error => Background::Color(colors.error),
				MenuType::Ghost => Background::Color(Color::TRANSPARENT),
			},
			path_border: Border::default(),
		}
	}

	fn hovered_pressed(&self, style: &StyleType) -> Style {
		let colors = style.get_palette();

		Style {
			bar_background: match self {
				MenuType::Base100 => {
					Background::Color(self.lighten_color(colors.base_100))
				}
				MenuType::Base200 => {
					Background::Color(self.lighten_color(colors.base_200))
				}
				MenuType::Base300 => {
					Background::Color(self.lighten_color(colors.base_300))
				}
				MenuType::Primary => {
					Background::Color(self.lighten_color(colors.primary))
				}
				MenuType::Secondary => {
					Background::Color(self.lighten_color(colors.secondary))
				}
				MenuType::Accent => {
					Background::Color(self.lighten_color(colors.accent))
				}
				MenuType::Neutral => {
					Background::Color(self.lighten_color(colors.neutral))
				}
				MenuType::Info => {
					Background::Color(self.lighten_color(colors.info))
				}
				MenuType::Success => {
					Background::Color(self.lighten_color(colors.success))
				}
				MenuType::Warning => {
					Background::Color(self.lighten_color(colors.warning))
				}
				MenuType::Error => {
					Background::Color(self.lighten_color(colors.error))
				}
				MenuType::Ghost => Background::Color(Color::TRANSPARENT),
			},
			bar_border: match self {
				MenuType::Ghost => Border::default(),
				_ => Border {
					color: match self {
						MenuType::Primary => self.lighten_color(colors.primary),
						MenuType::Secondary => {
							self.lighten_color(colors.secondary)
						}
						MenuType::Accent => self.lighten_color(colors.accent),
						MenuType::Neutral => self.lighten_color(colors.neutral),
						MenuType::Info => self.lighten_color(colors.info),
						MenuType::Success => self.lighten_color(colors.success),
						MenuType::Warning => self.lighten_color(colors.warning),
						MenuType::Error => self.lighten_color(colors.error),
						_ => self.lighten_color(colors.base_200),
					},
					width: BORDER_WIDTH,
					radius: BORDER_RADIUS.into(),
				},
			},
			bar_shadow: Shadow::default(),
			bar_background_expand: Padding::from(5.0),
			menu_background: match self {
				MenuType::Base100 => {
					Background::Color(self.lighten_color(colors.base_100))
				}
				MenuType::Base200 => {
					Background::Color(self.lighten_color(colors.base_200))
				}
				MenuType::Base300 => {
					Background::Color(self.lighten_color(colors.base_300))
				}
				MenuType::Primary => {
					Background::Color(self.lighten_color(colors.primary))
				}
				MenuType::Secondary => {
					Background::Color(self.lighten_color(colors.secondary))
				}
				MenuType::Accent => {
					Background::Color(self.lighten_color(colors.accent))
				}
				MenuType::Neutral => {
					Background::Color(self.lighten_color(colors.neutral))
				}
				MenuType::Info => {
					Background::Color(self.lighten_color(colors.info))
				}
				MenuType::Success => {
					Background::Color(self.lighten_color(colors.success))
				}
				MenuType::Warning => {
					Background::Color(self.lighten_color(colors.warning))
				}
				MenuType::Error => {
					Background::Color(self.lighten_color(colors.error))
				}
				MenuType::Ghost => Background::Color(Color::TRANSPARENT),
			},
			menu_border: match self {
				MenuType::Ghost => Border::default(),
				_ => Border {
					color: match self {
						MenuType::Primary => colors.primary,
						MenuType::Secondary => colors.secondary,
						MenuType::Accent => colors.accent,
						MenuType::Neutral => colors.neutral,
						MenuType::Info => colors.info,
						MenuType::Success => colors.success,
						MenuType::Warning => colors.warning,
						MenuType::Error => colors.error,
						_ => colors.base_200,
					},
					width: BORDER_WIDTH,
					radius: BORDER_RADIUS.into(),
				},
			},
			menu_shadow: Shadow::default(),
			menu_background_expand: Padding::from(5.0),
			path: match self {
				MenuType::Base100 => {
					Background::Color(self.lighten_color(colors.base_100))
				}
				MenuType::Base200 => {
					Background::Color(self.lighten_color(colors.base_200))
				}
				MenuType::Base300 => {
					Background::Color(self.lighten_color(colors.base_300))
				}
				MenuType::Primary => {
					Background::Color(self.lighten_color(colors.primary))
				}
				MenuType::Secondary => {
					Background::Color(self.lighten_color(colors.secondary))
				}
				MenuType::Accent => {
					Background::Color(self.lighten_color(colors.accent))
				}
				MenuType::Neutral => {
					Background::Color(self.lighten_color(colors.neutral))
				}
				MenuType::Info => {
					Background::Color(self.lighten_color(colors.info))
				}
				MenuType::Success => {
					Background::Color(self.lighten_color(colors.success))
				}
				MenuType::Warning => {
					Background::Color(self.lighten_color(colors.warning))
				}
				MenuType::Error => {
					Background::Color(self.lighten_color(colors.error))
				}
				MenuType::Ghost => Background::Color(Color::TRANSPARENT),
			},
			path_border: Border::default(),
		}
	}

	fn disabled(&self, style: &StyleType) -> Style {
		let colors = style.get_palette();

		Style {
			bar_background: Background::Color(colors.base_200),
			bar_border: Border::default(),
			bar_shadow: Shadow::default(),
			bar_background_expand: Padding::from(5.0),
			menu_background: Background::Color(
				self.lighten_color(colors.base_300),
			),
			menu_border: Border::default(),
			menu_shadow: Shadow::default(),
			menu_background_expand: Padding::from(5.0),
			path: Background::Color(colors.base_200),
			path_border: Border::default(),
		}
	}
}

impl Catalog for StyleType {
	type Class<'a> = MenuType;

	fn default<'a>() -> Self::Class<'a> {
		Self::Class::default()
	}

	fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
		match status {
			Status::Focused | Status::Selected | Status::Active => {
				class.active_focused(self)
			}
			Status::Hovered | Status::Pressed => class.hovered_pressed(self),
			Status::Disabled => class.disabled(self),
		}
	}
}
