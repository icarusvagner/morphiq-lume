#![allow(clippy::unreadable_literal)]

//! Business Theme
use iced::color;

use crate::gui::styles::types::{palette::Palette, palette_extension::PaletteExtension};

pub static BUSINESS_DARK_PALETTE: std::sync::LazyLock<Palette> =
    std::sync::LazyLock::new(|| Palette {
        base_100: color!(0x3E3E3E),
        base_200: color!(0x3A3A3A),
        base_300: color!(0x353535),
        base_content: color!(0xD8D8D8),
        primary: color!(0x4962B0),
        primary_content: color!(0xE1E6F4),
        secondary: color!(0x9FB1C4),
        secondary_content: color!(0x1F2226),
        accent: color!(0xD59A4A),
        accent_content: color!(0x2A2013),
        neutral: color!(0x44464C),
        neutral_content: color!(0xDADBDC),
        info: color!(0x6A94D9),
        info_content: color!(0x1F2735),
        success: color!(0x6BBE86),
        success_content: color!(0x1F2A24),
        warning: color!(0xE3C26A),
        warning_content: color!(0x2E2615),
        error: color!(0xB94D2F),
        error_content: color!(0xF0E0DB),
    });

pub static BUSINESS_DARK_PALETTE_EXTENSION: std::sync::LazyLock<PaletteExtension> =
    std::sync::LazyLock::new(|| BUSINESS_DARK_PALETTE.generate_palette_extension());
