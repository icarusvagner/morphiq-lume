use iced::color;

use crate::gui::styles::types::palette::Palette;

pub static DARK_PALETTE: std::sync::LazyLock<Palette> = std::sync::LazyLock::new(|| Palette {
    base_100: color!(0x1F2937),
    base_200: color!(0x111827),
    base_300: color!(0x020817),
    base_content: color!(0xFFFFFF),
    primary: color!(0x0D9488),
    primary_content: color!(0xFFFFFF),
    secondary: color!(0xFB923C),
    secondary_content: color!(0xFFFFFF),
    accent: color!(0x3B82F6),
    accent_content: color!(0xFFFFFF),
    neutral: color!(0x0891B2),
    neutral_content: color!(0xFFFFFF),
    info: color!(0x00BAFE),
    info_content: color!(0xFFFFFF),
    warning: color!(0xFCB700),
    warning_content: color!(0xFFFFFF),
    success: color!(0x00D390),
    success_content: color!(0xFFFFFF),
    error: color!(0xFF637D),
    error_content: color!(0xFFFFFF),
});
