#![allow(clippy::unreadable_literal)]

//! Retro Theme
use iced::color;

use crate::gui::styles::types::{palette::Palette, palette_extension::PaletteExtension};

pub static SYNTHWAVE_DARK_PALETTE: std::sync::LazyLock<Palette> =
    std::sync::LazyLock::new(|| Palette {
        base_100: color!(0x1A1033),
        base_200: color!(0x26144A),
        base_300: color!(0x321960),
        base_content: color!(0xC2B2FF),
        primary: color!(0xFF4D8C),
        primary_content: color!(0x5A1C1D),
        secondary: color!(0x9BD7FF),
        secondary_content: color!(0x4A5E8C),
        accent: color!(0xFFD54F),
        accent_content: color!(0x5A3C29),
        neutral: color!(0x6A2C91),
        neutral_content: color!(0xD9CFFF),
        info: color!(0x6BCBFF),
        info_content: color!(0x4A5E8C),
        success: color!(0x62E67E),
        success_content: color!(0x3C5F48),
        warning: color!(0xFFD54D),
        warning_content: color!(0x6A4F2E),
        error: color!(0xE24B4B),
        error_content: color!(0xE24B4B),
    });

pub static SYNTHWAVE_DARK_PALETTE_EXTENSION: std::sync::LazyLock<PaletteExtension> =
    std::sync::LazyLock::new(|| SYNTHWAVE_DARK_PALETTE.generate_palette_extension());
