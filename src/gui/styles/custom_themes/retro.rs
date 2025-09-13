#![allow(clippy::unreadable_literal)]

//! Retro Theme
use iced::color;

use crate::gui::styles::types::{palette::Palette, palette_extension::PaletteExtension};

pub static RETRO_LIGHT_PALETTE: std::sync::LazyLock<Palette> =
    std::sync::LazyLock::new(|| Palette {
        base_100: color!(0xF7F6E4),
        base_200: color!(0xEFEBCB),
        base_300: color!(0xE2DEA5),
        base_content: color!(0x6E5B2E),
        primary: color!(0xF2B27A),
        primary_content: color!(0x6B4023),
        secondary: color!(0xE5FAD7),
        secondary_content: color!(0x4F7F55),
        accent: color!(0xD6A756),
        accent_content: color!(0x6E5B2E),
        neutral: color!(0x6E706B),
        neutral_content: color!(0xDADADA),
        info: color!(0x4C7DFF),
        info_content: color!(0xF9FCEB),
        success: color!(0x3F9E6F),
        success_content: color!(0xF9FCEB),
        warning: color!(0xD88829),
        warning_content: color!(0xF9FCEB),
        error: color!(0xE05C3C),
        error_content: color!(0x6A3A1C),
    });

pub static RETRO_LIGHT_PALETTE_EXTENSION: std::sync::LazyLock<PaletteExtension> =
    std::sync::LazyLock::new(|| RETRO_LIGHT_PALETTE.generate_palette_extension());
