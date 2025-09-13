#![allow(clippy::unreadable_literal)]

//! Autumn Theme
use iced::color;

use crate::gui::styles::types::{palette::Palette, palette_extension::PaletteExtension};

pub static AUTUMN_LIGHT_PALETTE: std::sync::LazyLock<Palette> =
    std::sync::LazyLock::new(|| Palette {
        base_100: color!(0xF5F5F5),
        base_200: color!(0xE3E3E3),
        base_300: color!(0xD1D1D1),
        base_content: color!(0x313131),
        primary: color!(0x8B3A1C),
        primary_content: color!(0xF0D7CF),
        secondary: color!(0xD26939),
        secondary_content: color!(0x25120B),
        accent: color!(0xE6B873),
        accent_content: color!(0x2C2312),
        neutral: color!(0x8B7C6C),
        neutral_content: color!(0xE8E5E1),
        info: color!(0x5EAAD8),
        info_content: color!(0x1C2933),
        success: color!(0x4DAF87),
        success_content: color!(0x19231D),
        warning: color!(0xE5A84C),
        warning_content: color!(0x2B1E11),
        error: color!(0xC6461D),
        error_content: color!(0xF2DAD3),
    });

pub static AUTUMN_LIGHT_PALETTE_EXTENSION: std::sync::LazyLock<PaletteExtension> =
    std::sync::LazyLock::new(|| AUTUMN_LIGHT_PALETTE.generate_palette_extension());
