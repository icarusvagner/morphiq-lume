use iced::{
    widget::button::{Catalog, Status, Style},
    Background, Border, Color, Shadow,
};

use crate::gui::styles::{
    style_constant::{BORDER_RADIUS, BORDER_WIDTH},
    types::style_type::StyleType,
};

#[derive(Default)]
pub enum ButtonType {
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

impl ButtonType {
    #[allow(clippy::unused_self)]
    fn lighten_color(&self, color: Color) -> Color {
        Color {
            a: color.a - 0.05,
            ..color
        }
    }

    #[allow(clippy::unused_self)]
    fn active(&self, style: &StyleType) -> Style {
        let colors = style.get_palette();

        Style {
            background: Some(match self {
                ButtonType::Base100 => Background::Color(colors.base_100),
                ButtonType::Base200 => Background::Color(colors.base_200),
                ButtonType::Base300 => Background::Color(colors.base_300),
                ButtonType::Primary => Background::Color(colors.primary),
                ButtonType::Secondary => Background::Color(colors.secondary),
                ButtonType::Accent => Background::Color(colors.accent),
                ButtonType::Neutral => Background::Color(colors.neutral),
                ButtonType::Info => Background::Color(colors.info),
                ButtonType::Success => Background::Color(colors.success),
                ButtonType::Warning => Background::Color(colors.warning),
                ButtonType::Error => Background::Color(colors.error),
                ButtonType::Ghost => Background::Color(Color::TRANSPARENT),
            }),
            text_color: match self {
                ButtonType::Primary
                | ButtonType::Secondary
                | ButtonType::Accent
                | ButtonType::Neutral
                | ButtonType::Info
                | ButtonType::Success
                | ButtonType::Warning
                | ButtonType::Error => colors.primary_content,
                ButtonType::Base100
                | ButtonType::Base200
                | ButtonType::Base300
                | ButtonType::Ghost => colors.base_content,
            },
            border: match self {
                ButtonType::Ghost => Border::default(),
                _ => Border {
                    color: match self {
                        ButtonType::Primary => colors.primary,
                        ButtonType::Secondary => colors.secondary,
                        ButtonType::Accent => colors.accent,
                        ButtonType::Neutral => colors.neutral,
                        ButtonType::Info => colors.info,
                        ButtonType::Success => colors.success,
                        ButtonType::Warning => colors.warning,
                        ButtonType::Error => colors.error,
                        _ => colors.base_200,
                    },
                    width: BORDER_WIDTH,
                    radius: BORDER_RADIUS.into(),
                },
            },
            shadow: Shadow::default(),
        }
    }

    #[allow(clippy::unused_self)]
    fn hovered(&self, style: &StyleType) -> Style {
        let colors = style.get_palette();

        Style {
            background: Some(match self {
                ButtonType::Primary => Background::Color(self.lighten_color(colors.primary)),
                ButtonType::Secondary => Background::Color(self.lighten_color(colors.secondary)),
                ButtonType::Accent => Background::Color(self.lighten_color(colors.accent)),
                ButtonType::Neutral => Background::Color(self.lighten_color(colors.neutral)),
                ButtonType::Info => Background::Color(self.lighten_color(colors.info)),
                ButtonType::Success => Background::Color(self.lighten_color(colors.success)),
                ButtonType::Warning => Background::Color(self.lighten_color(colors.warning)),
                ButtonType::Error => Background::Color(self.lighten_color(colors.error)),
                _ => Background::Color(self.lighten_color(colors.base_300)),
            }),
            text_color: match self {
                ButtonType::Primary => colors.primary_content,
                ButtonType::Secondary => colors.secondary_content,
                ButtonType::Accent => colors.accent_content,
                ButtonType::Neutral => colors.neutral_content,
                ButtonType::Info => colors.info_content,
                ButtonType::Success => colors.success_content,
                ButtonType::Warning => colors.warning_content,
                ButtonType::Error => colors.error_content,
                ButtonType::Base100 | ButtonType::Base200 | ButtonType::Base300 => {
                    colors.base_content
                }
                _ => colors.base_content,
            },
            border: match self {
                ButtonType::Ghost => Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: BORDER_RADIUS.into(),
                },
                _ => Border {
                    color: match self {
                        ButtonType::Primary => self.lighten_color(colors.primary),
                        ButtonType::Secondary => self.lighten_color(colors.secondary),
                        ButtonType::Accent => self.lighten_color(colors.accent),
                        ButtonType::Neutral => self.lighten_color(colors.neutral),
                        ButtonType::Info => self.lighten_color(colors.info),
                        ButtonType::Success => self.lighten_color(colors.success),
                        ButtonType::Warning => self.lighten_color(colors.warning),
                        ButtonType::Error => self.lighten_color(colors.error),
                        _ => Color::TRANSPARENT,
                    },
                    width: match self {
                        ButtonType::Base100 | ButtonType::Base200 | ButtonType::Base300 => 0.0,
                        _ => BORDER_WIDTH,
                    },
                    radius: BORDER_RADIUS.into(),
                },
            },
            shadow: Shadow::default(),
        }
    }

    #[allow(clippy::unused_self)]
    fn disabled(&self, style: &StyleType) -> Style {
        let colors = style.get_palette();

        Style {
            background: Some(match self {
                ButtonType::Primary => Background::Color(colors.primary),
                ButtonType::Secondary => Background::Color(colors.secondary),
                ButtonType::Accent => Background::Color(colors.accent),
                ButtonType::Neutral => Background::Color(colors.neutral),
                ButtonType::Info => Background::Color(colors.info),
                ButtonType::Success => Background::Color(colors.success),
                ButtonType::Warning => Background::Color(colors.warning),
                ButtonType::Error => Background::Color(colors.error),
                ButtonType::Ghost => Background::Color(Color::TRANSPARENT),
                _ => Background::Color(colors.base_300),
            }),
            text_color: match self {
                ButtonType::Primary => colors.primary_content,
                ButtonType::Secondary => colors.secondary_content,
                ButtonType::Accent => colors.accent_content,
                ButtonType::Neutral => colors.neutral_content,
                ButtonType::Info => colors.info_content,
                ButtonType::Success => colors.success_content,
                ButtonType::Warning => colors.warning_content,
                ButtonType::Error => colors.error_content,
                _ => colors.base_content,
            },
            border: match self {
                ButtonType::Ghost => Border::default(),
                _ => Border {
                    color: colors.base_300,
                    width: BORDER_WIDTH,
                    radius: BORDER_RADIUS.into(),
                },
            },
            shadow: Shadow::default(),
        }
    }

    #[allow(clippy::unused_self)]
    fn pressed(&self, style: &StyleType) -> Style {
        let colors = style.get_palette();

        Style {
            background: Some(match self {
                ButtonType::Base100 => Background::Color(self.lighten_color(colors.base_100)),
                ButtonType::Base200 => Background::Color(self.lighten_color(colors.base_200)),
                ButtonType::Base300 => Background::Color(self.lighten_color(colors.base_300)),
                ButtonType::Primary => Background::Color(colors.primary),
                ButtonType::Secondary => Background::Color(colors.secondary),
                ButtonType::Accent => Background::Color(colors.accent),
                ButtonType::Neutral => Background::Color(colors.neutral),
                ButtonType::Info => Background::Color(colors.info),
                ButtonType::Success => Background::Color(colors.success),
                ButtonType::Warning => Background::Color(colors.warning),
                ButtonType::Error => Background::Color(colors.error),
                _ => Background::Color(Color::TRANSPARENT),
            }),
            text_color: match self {
                ButtonType::Primary => colors.primary_content,
                ButtonType::Secondary => colors.secondary_content,
                ButtonType::Accent => colors.accent_content,
                ButtonType::Neutral => colors.neutral_content,
                ButtonType::Info => colors.info_content,
                ButtonType::Success => colors.success_content,
                ButtonType::Warning => colors.warning_content,
                ButtonType::Error => colors.error_content,
                _ => colors.base_content,
            },
            border: match self {
                ButtonType::Ghost => Border::default(),
                _ => Border {
                    color: colors.base_100,
                    width: match self {
                        ButtonType::Primary
                        | ButtonType::Secondary
                        | ButtonType::Accent
                        | ButtonType::Info
                        | ButtonType::Success
                        | ButtonType::Warning
                        | ButtonType::Error => 0.0,
                        _ => BORDER_WIDTH,
                    },
                    radius: BORDER_RADIUS.into(),
                },
            },
            shadow: Shadow::default(),
        }
    }
}

impl Catalog for StyleType {
    type Class<'a> = ButtonType;

    fn default<'a>() -> Self::Class<'a> {
        Self::Class::default()
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        match status {
            Status::Active => class.active(self),
            Status::Hovered => class.hovered(self),
            Status::Pressed => class.pressed(self),
            Status::Disabled => class.pressed(self),
        }
    }
}
