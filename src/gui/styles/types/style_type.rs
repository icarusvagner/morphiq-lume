use iced::application::{Appearance, DefaultStyle};
use plotters::prelude::FontStyle;
use serde::{Deserialize, Serialize};

use crate::gui::styles::{
    style_constant::{
        colors::{
            ABYSS_PALETTE, ABYSS_PALETTE_EXTENSION, NIGHT_PALETTE, NIGHT_PALETTE_EXTENSION,
            NORD_PALETTE, NORD_PALETTE_EXTENSION, SILK_PALETTE, SILK_PALETTE_EXTENSION,
        },
        fonts::OUTFIT_BOLD,
    },
    types::{custom_palette::ExtraStyles, palette::Palette, palette_extension::PaletteExtension},
};

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Hash, PartialEq)]
#[serde(tag = "style", content = "name")]
#[allow(clippy::large_enum_variant)]
pub enum StyleType {
    Nord,
    Night,
    Silk,
    Abyss,
    Custom(ExtraStyles),
}

impl Default for StyleType {
    fn default() -> Self {
        Self::Custom(ExtraStyles::Corporate)
    }
}

impl DefaultStyle for StyleType {
    fn default_style(&self) -> Appearance {
        let colors = self.get_palette();

        Appearance {
            background_color: colors.base_300,
            text_color: colors.base_content,
        }
    }
}

impl StyleType {
    pub fn get_palette(self) -> Palette {
        match self {
            StyleType::Nord => NORD_PALETTE,
            StyleType::Night => NIGHT_PALETTE,
            StyleType::Silk => SILK_PALETTE,
            StyleType::Abyss => ABYSS_PALETTE,
            StyleType::Custom(style) => style.get_palette(),
        }
    }

    pub fn get_extension(self) -> PaletteExtension {
        match self {
            StyleType::Nord => NORD_PALETTE_EXTENSION,
            StyleType::Night => NIGHT_PALETTE_EXTENSION,
            StyleType::Silk => SILK_PALETTE_EXTENSION,
            StyleType::Abyss => ABYSS_PALETTE_EXTENSION,
            StyleType::Custom(style) => style.get_extension(),
        }
    }

    pub fn get_font_weight(self) -> FontStyle {
        if self.get_extension().font.eq(&OUTFIT_BOLD) {
            FontStyle::Bold
        } else {
            FontStyle::Normal
        }
    }
}
