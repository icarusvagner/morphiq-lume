use iced::{
    widget::text::{Catalog, Style},
    Color,
};

use crate::gui::styles::types::style_type::StyleType;

#[derive(Debug, Default, Clone, PartialEq, Copy)]
#[allow(clippy::large_enum_variant)]
pub enum TextType {
    #[default]
    BaseContent,
    Content,
    Standard,
}

impl TextType {
    fn appearance(self, style: &StyleType) -> Style {
        let color = style.get_palette();

        Style {
            color: if self == TextType::Standard {
                None
            } else {
                Some(match self {
                    TextType::BaseContent => color.base_content,
                    TextType::Content => color.primary_content,
                    _ => Color::default(),
                })
            },
        }
    }
}

impl Catalog for StyleType {
    type Class<'a> = TextType;

    fn default<'a>() -> Self::Class<'a> {
        Self::Class::default()
    }

    fn style(&self, item: &Self::Class<'_>) -> Style {
        item.appearance(self)
    }
}
