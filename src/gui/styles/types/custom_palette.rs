use serde::{Deserialize, Serialize};

use crate::gui::styles::{
    custom_themes::{
        autumn::{AUTUMN_LIGHT_PALETTE, AUTUMN_LIGHT_PALETTE_EXTENSION},
        black::{BLACK_DARK_PALETTE, BLACK_DARK_PALETTE_EXTENSION},
        business::{BUSINESS_DARK_PALETTE, BUSINESS_DARK_PALETTE_EXTENSION},
        corporate::{COPORATE_LIGHT_PALETTE_EXTENSION, CORPORATE_LIGHT_PALETTE},
        retro::{RETRO_LIGHT_PALETTE, RETRO_LIGHT_PALETTE_EXTENSION},
        synthwave::{SYNTHWAVE_DARK_PALETTE, SYNTHWAVE_DARK_PALETTE_EXTENSION},
    },
    types::{palette::Palette, palette_extension::PaletteExtension},
};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Hash, PartialEq)]
pub struct CustomPalette {
    #[serde(flatten)]
    pub(crate) palette: Palette,
    #[serde(flatten)]
    pub(crate) extension: PaletteExtension,
}

impl CustomPalette {
    pub fn from_palette(palette: Palette) -> Self {
        Self {
            palette,
            extension: palette.generate_palette_extension(),
        }
    }
}

/// Built in extra styles
#[derive(Clone, Copy, Debug, Hash, PartialEq, Serialize, Deserialize)]
#[serde(tag = "style", content = "attributes")]
#[allow(clippy::large_enum_variant)]
pub enum ExtraStyles {
    Corporate,
    Synthwave,
    Retro,
    Black,
    Autumn,
    Business,
    CustomToml(CustomPalette),
}

impl ExtraStyles {
    /// [`Palette`] fo the [`ExtraStyles`] variant
    pub fn get_palette(self) -> Palette {
        match self {
            ExtraStyles::Corporate => *CORPORATE_LIGHT_PALETTE,
            ExtraStyles::Synthwave => *SYNTHWAVE_DARK_PALETTE,
            ExtraStyles::Retro => *RETRO_LIGHT_PALETTE,
            ExtraStyles::Black => *BLACK_DARK_PALETTE,
            ExtraStyles::Autumn => *AUTUMN_LIGHT_PALETTE,
            ExtraStyles::Business => *BUSINESS_DARK_PALETTE,
            ExtraStyles::CustomToml(custom_palette) => custom_palette.palette,
        }
    }

    /// [`PaletteExtension`] of the [`ExtraStyles`] variant
    pub fn get_extension(self) -> PaletteExtension {
        match self {
            ExtraStyles::Corporate => *COPORATE_LIGHT_PALETTE_EXTENSION,
            ExtraStyles::Synthwave => *SYNTHWAVE_DARK_PALETTE_EXTENSION,
            ExtraStyles::Retro => *RETRO_LIGHT_PALETTE_EXTENSION,
            ExtraStyles::Black => *BLACK_DARK_PALETTE_EXTENSION,
            ExtraStyles::Autumn => *AUTUMN_LIGHT_PALETTE_EXTENSION,
            ExtraStyles::Business => *BUSINESS_DARK_PALETTE_EXTENSION,
            ExtraStyles::CustomToml(custom_palette) => custom_palette.extension,
        }
    }

    /// Slice of all implemented custom styles
    pub fn all_styles() -> &'static [Self] {
        &[
            ExtraStyles::Corporate,
            ExtraStyles::Synthwave,
            ExtraStyles::Retro,
            ExtraStyles::Black,
            ExtraStyles::Autumn,
            ExtraStyles::Business,
        ]
    }
}

impl std::fmt::Display for ExtraStyles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ExtraStyles::Corporate => write!(f, "Corporate"),
            ExtraStyles::Synthwave => write!(f, "Synthwave"),
            ExtraStyles::Retro => write!(f, "Retro"),
            ExtraStyles::Black => write!(f, "Black"),
            ExtraStyles::Autumn => write!(f, "Autumn"),
            ExtraStyles::Business => write!(f, "Business"),
            ExtraStyles::CustomToml(_) => unreachable!(),
        }
    }
}
