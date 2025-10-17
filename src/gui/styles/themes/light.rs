use iced::color;

use crate::gui::styles::types::palette::Palette;

pub static LIGHT_PALETTE: std::sync::LazyLock<Palette> =
	std::sync::LazyLock::new(|| Palette {
		base_100: color!(0xFFF_FFF),
		base_200: color!(0xF8_F8F8),
		base_300: color!(0xEEE_EEE),
		base_content: color!(0x181_81B),
		primary: color!(0x0D9_488),
		primary_content: color!(0xFFF_FFF),
		secondary: color!(0xFB9_23C),
		secondary_content: color!(0xFFF_FFF),
		accent: color!(0x3B8_2F6),
		accent_content: color!(0xFFF_FFF),
		neutral: color!(0x089_1B2),
		neutral_content: color!(0xFFF_FFF),
		info: color!(0x00B_AFE),
		info_content: color!(0xFFF_FFF),
		warning: color!(0xFCB_700),
		warning_content: color!(0xFFF_FFF),
		success: color!(0x00D_390),
		success_content: color!(0xFFF_FFF),
		error: color!(0xFF6_37D),
		error_content: color!(0xFFF_FFF),
	});
