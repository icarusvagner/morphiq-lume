#![allow(clippy::unreadable_literal)]

//! Corporate Theme
use iced::color;

use crate::gui::styles::types::{palette::Palette, palette_extension::PaletteExtension};

pub static CORPORATE_LIGHT_PALETTE: std::sync::LazyLock<Palette> =
    std::sync::LazyLock::new(|| Palette {
        base_100: color!(0xFFFFFF),
        base_200: color!(0xECECEC),
        base_300: color!(0xDBDBDB),
        base_content: color!(0x262633),
        primary: color!(0x2563EB),
        primary_content: color!(0xFFFFFF),
        secondary: color!(0x5B5F8C),
        secondary_content: color!(0xFFFFFF),
        accent: color!(0x2DD4BF),
        accent_content: color!(0xFFFFFF),
        neutral: color!(0x000000),
        neutral_content: color!(0xFFFFFF),
        info: color!(0x3B82F6),
        info_content: color!(0xFFFFFF),
        success: color!(0x22C55E),
        success_content: color!(0xFFFFFF),
        warning: color!(0xFACC15),
        warning_content: color!(0xFFFFFF),
        error: color!(0xDC2626),
        error_content: color!(0xFFFFFF),
    });

pub static COPORATE_LIGHT_PALETTE_EXTENSION: std::sync::LazyLock<PaletteExtension> =
    std::sync::LazyLock::new(|| CORPORATE_LIGHT_PALETTE.generate_palette_extension());
