use iced::{Background, Color};
use iced_aw::{
    card::Status,
    context_menu::{Catalog, Style},
};

use crate::gui::styles::types::style_type::StyleType;

#[derive(Default)]
pub enum ContextMenuType {
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
    Ghost,
}

impl ContextMenuType {
    fn lighted_color(&self, color: Color) -> Color {
        Color {
            a: color.a - 0.01,
            ..color
        }
    }

    fn active(&self, style: &StyleType) -> Style {
        let colors = style.get_palette();

        Style {
            background: match self {
                ContextMenuType::Base100 => Background::Color(colors.base_100),
                ContextMenuType::Base200 => Background::Color(colors.base_200),
                ContextMenuType::Base300 => Background::Color(colors.base_300),
                ContextMenuType::Primary => Background::Color(colors.primary),
                ContextMenuType::Secondary => Background::Color(colors.secondary),
                ContextMenuType::Accent => Background::Color(colors.accent),
                ContextMenuType::Neutral => Background::Color(colors.neutral),
                ContextMenuType::Info => Background::Color(colors.info),
                ContextMenuType::Success => Background::Color(colors.success),
                ContextMenuType::Warning => Background::Color(colors.warning),
                ContextMenuType::Error => Background::Color(colors.error),
                _ => Background::Color(Color::TRANSPARENT),
            },
        }
    }

    fn hovered(&self, style: &StyleType) -> Style {
        let colors = style.get_palette();

        Style {
            background: match self {
                ContextMenuType::Base100 => Background::Color(self.lighted_color(colors.base_100)),
                ContextMenuType::Base200 => Background::Color(self.lighted_color(colors.base_200)),
                ContextMenuType::Base300 => Background::Color(self.lighted_color(colors.base_300)),
                ContextMenuType::Primary => Background::Color(self.lighted_color(colors.primary)),
                ContextMenuType::Secondary => {
                    Background::Color(self.lighted_color(colors.secondary))
                }
                ContextMenuType::Accent => Background::Color(self.lighted_color(colors.accent)),
                ContextMenuType::Neutral => Background::Color(self.lighted_color(colors.neutral)),
                ContextMenuType::Info => Background::Color(self.lighted_color(colors.info)),
                ContextMenuType::Success => Background::Color(self.lighted_color(colors.success)),
                ContextMenuType::Warning => Background::Color(self.lighted_color(colors.warning)),
                ContextMenuType::Error => Background::Color(self.lighted_color(colors.error)),
                _ => Background::Color(Color::TRANSPARENT),
            },
        }
    }

    fn disabled(&self, style: &StyleType) -> Style {
        let colors = style.get_palette();

        Style {
            background: Background::Color(colors.base_300),
        }
    }

    fn pressed(&self, style: &StyleType) -> Style {
        let colors = style.get_palette();

        Style {
            background: match self {
                ContextMenuType::Base100 => Background::Color(colors.base_100),
                ContextMenuType::Base200 => Background::Color(colors.base_200),
                ContextMenuType::Base300 => Background::Color(colors.base_300),
                ContextMenuType::Primary => Background::Color(colors.primary),
                ContextMenuType::Secondary => Background::Color(colors.secondary),
                ContextMenuType::Accent => Background::Color(colors.accent),
                ContextMenuType::Neutral => Background::Color(colors.neutral),
                ContextMenuType::Info => Background::Color(colors.info),
                ContextMenuType::Success => Background::Color(colors.success),
                ContextMenuType::Warning => Background::Color(colors.warning),
                ContextMenuType::Error => Background::Color(colors.error),
                _ => Background::Color(Color::TRANSPARENT),
            },
        }
    }

    fn focused(&self, style: &StyleType) -> Style {
        let colors = style.get_palette();

        Style {
            background: match self {
                ContextMenuType::Base100 => Background::Color(colors.base_100),
                ContextMenuType::Base200 => Background::Color(colors.base_200),
                ContextMenuType::Base300 => Background::Color(colors.base_300),
                ContextMenuType::Primary => Background::Color(colors.primary),
                ContextMenuType::Secondary => Background::Color(colors.secondary),
                ContextMenuType::Accent => Background::Color(colors.accent),
                ContextMenuType::Neutral => Background::Color(colors.neutral),
                ContextMenuType::Info => Background::Color(colors.info),
                ContextMenuType::Success => Background::Color(colors.success),
                ContextMenuType::Warning => Background::Color(colors.warning),
                ContextMenuType::Error => Background::Color(colors.error),
                _ => Background::Color(Color::TRANSPARENT),
            },
        }
    }

    fn selected(&self, style: &StyleType) -> Style {
        let colors = style.get_palette();

        Style {
            background: match self {
                ContextMenuType::Base100 => Background::Color(self.lighted_color(colors.base_100)),
                ContextMenuType::Base200 => Background::Color(self.lighted_color(colors.base_200)),
                ContextMenuType::Base300 => Background::Color(self.lighted_color(colors.base_300)),
                ContextMenuType::Primary => Background::Color(self.lighted_color(colors.primary)),
                ContextMenuType::Secondary => {
                    Background::Color(self.lighted_color(colors.secondary))
                }
                ContextMenuType::Accent => Background::Color(self.lighted_color(colors.accent)),
                ContextMenuType::Neutral => Background::Color(self.lighted_color(colors.neutral)),
                ContextMenuType::Info => Background::Color(self.lighted_color(colors.info)),
                ContextMenuType::Success => Background::Color(self.lighted_color(colors.success)),
                ContextMenuType::Warning => Background::Color(self.lighted_color(colors.warning)),
                ContextMenuType::Error => Background::Color(self.lighted_color(colors.error)),
                _ => Background::Color(Color::TRANSPARENT),
            },
        }
    }
}

impl Catalog for StyleType {
    type Class<'a> = ContextMenuType;

    fn default<'a>() -> Self::Class<'a> {
        Self::Class::default()
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        match status {
            Status::Active => class.active(self),
            Status::Hovered => class.hovered(self),
            Status::Disabled => class.disabled(self),
            Status::Pressed => class.pressed(self),
            Status::Focused => class.focused(self),
            Status::Selected => class.selected(self),
        }
    }
}
