#![allow(clippy::unreadable_literal)]

//! Black Theme
use iced::color;

use crate::gui::styles::types::{palette::Palette, palette_extension::PaletteExtension};

pub static BLACK_DARK_PALETTE: std::sync::LazyLock<Palette> =
    std::sync::LazyLock::new(|| Palette {
        base_100: color!(0x000000),
        base_200: color!(0x303030),
        base_300: color!(0x383838),
        base_content: color!(0xE0E0E0),
        primary: color!(0x595959),
        primary_content: color!(0xFFFFFF),
        secondary: color!(0x595959),
        secondary_content: color!(0xFFFFFF),
        accent: color!(0x595959),
        accent_content: color!(0xFFFFFF),
        neutral: color!(0x595959),
        neutral_content: color!(0xFFFFFF),
        info: color!(0x6C63FF),
        info_content: color!(0xE1E0F8),
        success: color!(0x4CAF50),
        success_content: color!(0xE6F6E6),
        warning: color!(0xFFF176),
        warning_content: color!(0x2F2B0F),
        error: color!(0xE53935),
        error_content: color!(0x220605),
    });

pub static BLACK_DARK_PALETTE_EXTENSION: std::sync::LazyLock<PaletteExtension> =
    std::sync::LazyLock::new(|| BLACK_DARK_PALETTE.generate_palette_extension());
