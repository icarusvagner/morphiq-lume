use iced::{widget::container::Style, Background, Border, Color, Shadow};
use iced_table::Catalog;

use crate::gui::styles::{style_constant::BORDER_RADIUS, types::style_type::StyleType};

#[derive(Default, Clone)]
pub enum TableType {
	#[default]
	Base,
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

impl TableType {
	fn lighten_color(&self, color: Color) -> Color {
		Color {
			a: color.a - 0.50,
			..color
		}
	}

	fn header(&self, style: StyleType) -> Style {
		let colors = style.get_palette();

		Style {
			text_color: Some(match self {
				TableType::Base => colors.base_content,
				TableType::Primary
				| TableType::Secondary
				| TableType::Accent
				| TableType::Neutral => colors.primary_content,
				TableType::Info => colors.info_content,
				TableType::Success => colors.success_content,
				TableType::Warning => colors.warning_content,
				TableType::Error => colors.error_content,
				_ => colors.base_content,
			}),
			background: Some(Background::Color(match self {
				TableType::Base => colors.base_200,
				TableType::Primary => colors.primary,
				TableType::Secondary => colors.secondary,
				TableType::Accent => colors.accent,
				TableType::Neutral => colors.neutral,
				TableType::Info => colors.info,
				TableType::Success => colors.success,
				TableType::Warning => colors.warning,
				TableType::Error => colors.error,
				_ => colors.base_200,
			})),
			border: Border {
				color: Color::TRANSPARENT,
				width: 0.0,
				radius: BORDER_RADIUS.into(),
			},
			shadow: Shadow::default(),
		}
	}

	fn footer(&self, style: StyleType) -> Style {
		let colors = style.get_palette();

		Style {
			text_color: Some(match self {
				TableType::Base => colors.base_content,
				TableType::Primary
				| TableType::Secondary
				| TableType::Accent
				| TableType::Neutral => colors.primary_content,
				TableType::Info => colors.info_content,
				TableType::Success => colors.success_content,
				TableType::Warning => colors.warning_content,
				TableType::Error => colors.error_content,
				_ => colors.base_content,
			}),
			background: Some(Background::Color(match self {
				TableType::Base => colors.base_200,
				TableType::Primary => colors.primary,
				TableType::Secondary => colors.secondary,
				TableType::Accent => colors.accent,
				TableType::Neutral => colors.neutral,
				TableType::Info => colors.info,
				TableType::Success => colors.success,
				TableType::Warning => colors.warning,
				TableType::Error => colors.error,
				_ => colors.base_200,
			})),
			border: Border {
				color: Color::TRANSPARENT,
				width: 0.0,
				radius: BORDER_RADIUS.into(),
			},
			shadow: Shadow::default(),
		}
	}

	fn row(&self, style: StyleType, index: usize) -> Style {
		let colors = style.get_palette();
		let idx = index % 2 == 0;

		Style {
			text_color: Some(match self {
				TableType::Base | TableType::Ghost => colors.base_content,
				TableType::Primary
				| TableType::Secondary
				| TableType::Accent
				| TableType::Neutral => colors.primary_content,
				TableType::Info => colors.info_content,
				TableType::Success => colors.success,
				TableType::Warning => colors.warning,
				TableType::Error => colors.error,
			}),
			background: Some(Background::Color(match self {
				TableType::Base => {
					if idx {
						self.lighten_color(colors.base_200)
					} else {
						colors.base_200
					}
				}
				TableType::Primary => {
					if idx {
						self.lighten_color(colors.primary)
					} else {
						colors.primary
					}
				}
				TableType::Secondary => {
					if idx {
						self.lighten_color(colors.secondary)
					} else {
						colors.secondary
					}
				}
				TableType::Accent => {
					if idx {
						self.lighten_color(colors.accent)
					} else {
						colors.accent
					}
				}
				TableType::Neutral => {
					if idx {
						self.lighten_color(colors.neutral)
					} else {
						colors.neutral
					}
				}
				TableType::Info => {
					if idx {
						self.lighten_color(colors.info)
					} else {
						colors.info
					}
				}
				TableType::Success => {
					if idx {
						self.lighten_color(colors.success)
					} else {
						colors.success
					}
				}
				TableType::Warning => {
					if idx {
						self.lighten_color(colors.warning)
					} else {
						colors.warning
					}
				}
				TableType::Error => {
					if idx {
						self.lighten_color(colors.error)
					} else {
						colors.error
					}
				}
				_ => {
					if idx {
						self.lighten_color(colors.base_200)
					} else {
						colors.base_200
					}
				}
			})),
			border: Border {
				color: Color::TRANSPARENT,
				width: 0.0,
				radius: BORDER_RADIUS.into(),
			},
			shadow: Shadow::default(),
		}
	}

	fn divider(&self, style: StyleType, hovered: bool) -> Style {
		let colors = style.get_palette();

		Style {
			text_color: Some(match self {
				TableType::Base | TableType::Ghost => colors.base_content,
				TableType::Primary
				| TableType::Secondary
				| TableType::Accent
				| TableType::Neutral => colors.primary_content,
				TableType::Info => colors.info_content,
				TableType::Success => colors.success,
				TableType::Warning => colors.warning,
				TableType::Error => colors.error,
			}),
			background: if hovered {
				Some(Background::Color(match self {
					TableType::Base => self.lighten_color(colors.base_200),
					TableType::Primary => self.lighten_color(colors.primary),
					TableType::Secondary => {
						self.lighten_color(colors.secondary)
					}
					TableType::Accent => self.lighten_color(colors.accent),
					TableType::Neutral => self.lighten_color(colors.neutral),
					TableType::Info => self.lighten_color(colors.info),
					TableType::Success => self.lighten_color(colors.success),
					TableType::Warning => self.lighten_color(colors.warning),
					TableType::Error => self.lighten_color(colors.error),
					_ => self.lighten_color(colors.base_200),
				}))
			} else {
				Some(Background::Color(match self {
					TableType::Base => colors.base_200,
					TableType::Primary => colors.primary,
					TableType::Secondary => colors.secondary,
					TableType::Accent => colors.accent,
					TableType::Neutral => colors.neutral,
					TableType::Info => colors.info,
					TableType::Success => colors.success,
					TableType::Warning => colors.warning,
					TableType::Error => colors.error,
					_ => colors.base_200,
				}))
			},
			border: Border {
				color: Color::TRANSPARENT,
				width: 0.0,
				radius: BORDER_RADIUS.into(),
			},
			shadow: Shadow::default(),
		}
	}
}

impl Catalog for StyleType {
	type Style = TableType;

	fn header(&self, style: &Self::Style) -> Style {
		style.header(*self)
	}
	fn footer(&self, style: &Self::Style) -> Style {
		style.footer(*self)
	}
	fn row(&self, style: &Self::Style, index: usize) -> Style {
		style.row(*self, index)
	}
	fn divider(&self, style: &Self::Style, hovered: bool) -> Style {
		style.divider(*self, hovered)
	}
}
