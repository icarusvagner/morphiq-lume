//! Text style

#[allow(clippy::module_name_repetitions)]
use iced::{
    widget::text::{Catalog, Style},
    Color,
};

use crate::gui::styles::types::style_type::StyleType;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum TextType {
    #[default]
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
}

impl TextType {
    // pub fn highlighted_subtitle_with_desc<'a>() {}

    fn appearance(self, style: &StyleType) -> Style {
        let colors = style.get_palette();

        Style {
            color: Some(match self {
                TextType::Primary => colors.primary,
                TextType::Secondary => colors.secondary,
                TextType::Accent => colors.accent,
                TextType::Neutral => colors.neutral,
                TextType::Info => colors.info,
                TextType::Success => colors.success,
                TextType::Warning => colors.warning,
                TextType::Error => colors.error,
                TextType::Base100 => colors.base_100,
                TextType::Base200 => colors.base_200,
                _ => colors.base_content,
            }),
        }
    }
}

pub fn highlight(style: &StyleType, element: TextType) -> Color {
    let colors = style.get_palette();

    match element {
        TextType::Primary => colors.secondary,
        TextType::Secondary => colors.accent,
        TextType::Accent => colors.neutral,
        TextType::Neutral => colors.info,
        TextType::Info => colors.success,
        TextType::Success => colors.warning,
        TextType::Warning => colors.error,
        TextType::Error => colors.base_100,
        _ => colors.primary,
    }
}

impl Catalog for StyleType {
    type Class<'a> = TextType;

    fn default<'a>() -> Self::Class<'a> {
        Self::Class::default()
    }

    fn style(&self, item: &Self::Class<'_>) -> iced::widget::text::Style {
        item.appearance(self)
    }
}
