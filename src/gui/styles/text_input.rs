//! Text Input style
#[allow(clippy::module_name_repetitions)]
use iced::{
    widget::text_input::{Catalog, Status, Style},
    Background, Border, Color,
};

use crate::gui::styles::{style_constant::BORDER_WIDTH, types::style_type::StyleType};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum TextInputType {
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

impl TextInputType {
    fn active(&self, style: &StyleType) -> Style {
        let colors = style.get_palette();
        let ext = style.get_extension();

        Style {
            background: Background::Color(Color::TRANSPARENT),
            border: match self {
                TextInputType::Ghost => Border::default(),
                _ => Border {
                    radius: ext.radius_boxes.into(),
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

    #[allow(clippy::unused_self)]
    fn selection_color(&self, style: &StyleType) -> Color {
        style.get_palette().primary
    }

    #[allow(clippy::unused_self)]
    fn lighted_color(&self, color: Color) -> Color {
        Color {
            a: color.a - 0.20,
            ..color
        }
    }

    #[allow(clippy::unused_self)]
    fn placeholder_color(&self, style: &StyleType) -> Color {
        let colors = style.get_palette();

        match self {
            TextInputType::Primary => self.lighted_color(colors.primary),
            TextInputType::Secondary => self.lighted_color(colors.secondary),
            TextInputType::Accent => self.lighted_color(colors.accent),
            TextInputType::Neutral => self.lighted_color(colors.neutral),
            TextInputType::Info => self.lighted_color(colors.info),
            TextInputType::Success => self.lighted_color(colors.success),
            TextInputType::Warning => self.lighted_color(colors.warning),
            TextInputType::Error => self.lighted_color(colors.error),
            _ => self.lighted_color(colors.base_content),
        }
    }

    #[allow(clippy::unused_self)]
    fn value_color(&self, style: &StyleType) -> Color {
        style.get_palette().neutral
    }

    #[allow(clippy::unused_self)]
    fn disabled_color(&self, style: &StyleType) -> Color {
        let color = style.get_palette().base_200;
        let is_nightly = style.get_extension().is_nightly;

        Color {
            a: if is_nightly { 0.2 } else { 0.7 },
            ..color
        }
    }

    fn hovered(&self, style: &StyleType) -> Style {
        let colors = style.get_palette();
        let ext = style.get_extension();

        Style {
            background: Background::Color(Color::TRANSPARENT),
            border: Border {
                radius: ext.radius_boxes.into(),
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
                    TextInputType::Ghost => Color::TRANSPARENT,
                    _ => colors.base_300,
                },
            },
            icon: Color {
                a: if ext.is_nightly { 0.2 } else { 0.7 },
                ..colors.base_content
            },
            placeholder: self.placeholder_color(style),
            value: self.value_color(style),
            selection: self.selection_color(style),
        }
    }

    fn disabled(&self, style: &StyleType) -> Style {
        let colors = style.get_palette();
        let ext = style.get_extension();

        Style {
            background: Background::Color(Color::TRANSPARENT),
            border: match self {
                TextInputType::Ghost => Border::default(),
                _ => Border {
                    color: match self {
                        TextInputType::Primary => colors.primary,
                        TextInputType::Secondary => colors.secondary,
                        TextInputType::Accent => colors.accent,
                        TextInputType::Neutral => colors.neutral,
                        TextInputType::Info => colors.info,
                        TextInputType::Success => colors.success,
                        TextInputType::Warning => colors.warning,
                        TextInputType::Error => colors.error,
                        _ => colors.base_100,
                    },
                    width: BORDER_WIDTH,
                    radius: ext.radius_boxes.into(),
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

    fn focused(&self, style: &StyleType) -> Style {
        let colors = style.get_palette();
        let ext = style.get_extension();

        Style {
            background: Background::Color(Color::TRANSPARENT),
            border: match self {
                TextInputType::Ghost => Border::default(),
                _ => Border {
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
                    width: BORDER_WIDTH,
                    radius: ext.radius_boxes.into(),
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
            Status::Disabled => class.disabled(self),
            Status::Focused => class.focused(self),
        }
    }
}
