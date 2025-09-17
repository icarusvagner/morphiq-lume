use iced::{
    widget::container::{Catalog, Style},
    Background, Border, Color, Shadow,
};

use crate::gui::styles::{style_constant::BORDER_RADIUS, types::style_type::StyleType};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
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
    Icon,
    #[default]
    Standard,
    Bordered,
    Ghost,
}

impl ContainerType {
    #[allow(clippy::unused_self)]
    fn lighten_color(&self, color: Color) -> Color {
        Color {
            a: color.a - 0.20,
            ..color
        }
    }

    fn appearance(self, style: &StyleType) -> Style {
        let colors = style.get_palette();

        Style {
            text_color: match self {
                ContainerType::Overlay | ContainerType::Icon => None,
                _ => Some(match self {
                    ContainerType::Standard | ContainerType::Bordered => colors.base_content,
                    _ => colors.primary_content,
                }),
            },
            background: if self == ContainerType::Standard {
                Some(Background::Color(colors.base_200))
            } else {
                Some(match self {
                    ContainerType::Primary => Background::Color(colors.primary),
                    ContainerType::Secondary => Background::Color(colors.secondary),
                    ContainerType::Accent => Background::Color(colors.accent),
                    ContainerType::Neutral => Background::Color(colors.neutral),
                    ContainerType::Info => Background::Color(colors.info),
                    ContainerType::Error => Background::Color(colors.error),
                    ContainerType::Base100 => Background::Color(colors.base_100),
                    ContainerType::Base200 => Background::Color(colors.base_200),
                    ContainerType::Base300 => Background::Color(colors.base_300),
                    ContainerType::Overlay => Background::Color(Color {
                        a: colors.neutral.a - 0.60,
                        ..colors.neutral
                    }),
                    _ => Background::Color(Color::TRANSPARENT),
                })
            },
            border: match self {
                ContainerType::Standard | ContainerType::Overlay | ContainerType::Icon => {
                    Border::default()
                }
                ContainerType::Ghost => Border::default(),
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
