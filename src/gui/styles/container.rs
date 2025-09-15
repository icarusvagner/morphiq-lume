//! Container style
#[allow(clippy::module_name_repetitions)]
use iced::{
    widget::container::{Catalog, Style},
    Background, Border, Color, Shadow,
};

use crate::gui::styles::{style_constant::BORDER_WIDTH, types::style_type::StyleType};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum ContainerType {
    Base100,
    Base100NoBorder,
    Base200,
    Base200NoBorder,
    Base300,
    #[default]
    Base300NoBorder,
    Primary,
    PrimaryNoBorder,
    Secondary,
    SecondaryNoBorder,
    Accent,
    AccentNoBorder,
    Neutral,
    NeutralNoBorder,
    Info,
    InfoNoBorder,
    Success,
    SuccessNoBorder,
    Warning,
    WarningNoBorder,
    Error,
    ErrorNoBorder,

    // --- For modal overlay ---
    Overlay,
    Bordered,
    Ghost,
}

impl ContainerType {
    fn appearance(&self, style: &StyleType) -> Style {
        let colors = style.get_palette();
        let ext = style.get_extension();

        Style {
            text_color: Some(match self {
                ContainerType::Primary | ContainerType::PrimaryNoBorder => colors.primary_content,
                ContainerType::Secondary | ContainerType::SecondaryNoBorder => {
                    colors.secondary_content
                }
                ContainerType::Accent | ContainerType::AccentNoBorder => colors.accent_content,
                ContainerType::Neutral | ContainerType::NeutralNoBorder => colors.neutral_content,
                ContainerType::Info | ContainerType::InfoNoBorder => colors.info_content,
                ContainerType::Success | ContainerType::SuccessNoBorder => colors.success_content,
                ContainerType::Warning | ContainerType::WarningNoBorder => colors.warning_content,
                ContainerType::Error | ContainerType::ErrorNoBorder => colors.error_content,
                _ => colors.base_content,
            }),
            background: Some(match self {
                ContainerType::Primary | ContainerType::PrimaryNoBorder => {
                    Background::Color(colors.primary)
                }
                ContainerType::Secondary | ContainerType::SecondaryNoBorder => {
                    Background::Color(colors.secondary)
                }
                ContainerType::Accent | ContainerType::AccentNoBorder => {
                    Background::Color(colors.accent)
                }
                ContainerType::Neutral | ContainerType::NeutralNoBorder => {
                    Background::Color(colors.neutral)
                }
                ContainerType::Info | ContainerType::InfoNoBorder => Background::Color(colors.info),
                ContainerType::Error | ContainerType::ErrorNoBorder => {
                    Background::Color(colors.error)
                }
                ContainerType::Base100 | ContainerType::Base100NoBorder => {
                    Background::Color(colors.base_100)
                }
                ContainerType::Base200 | ContainerType::Base200NoBorder => {
                    Background::Color(colors.base_200)
                }
                ContainerType::Base300 | ContainerType::Base300NoBorder => {
                    Background::Color(colors.base_300)
                }
                ContainerType::Overlay => Background::Color(Color {
                    a: colors.neutral.a - 0.60,
                    ..colors.neutral
                }),
                _ => Background::Color(Color::TRANSPARENT),
            }),
            border: match self {
                ContainerType::Overlay
                | ContainerType::Base300
                | ContainerType::Ghost
                | ContainerType::PrimaryNoBorder
                | ContainerType::SecondaryNoBorder
                | ContainerType::AccentNoBorder
                | ContainerType::NeutralNoBorder
                | ContainerType::InfoNoBorder
                | ContainerType::SuccessNoBorder
                | ContainerType::WarningNoBorder
                | ContainerType::ErrorNoBorder
                | ContainerType::Base100NoBorder
                | ContainerType::Base200NoBorder
                | ContainerType::Base300NoBorder => Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: BORDER_WIDTH.into(),
                },
                _ => Border {
                    color: match self {
                        ContainerType::Success => colors.success,
                        ContainerType::Warning => colors.warning,
                        ContainerType::Bordered => colors.base_300,
                        _ => Color::TRANSPARENT,
                    },
                    width: BORDER_WIDTH,
                    radius: ext.radius_boxes.into(),
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
