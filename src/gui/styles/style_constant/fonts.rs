use iced::{
	Font,
	font::{
		Family,
		Stretch,
		Style,
		Weight,
	},
};

use crate::gui::morphiq::ICON_FONT_FAMILY_NAME;

/// fonts
// Outfit Font (Bytes)
pub const OUTFIT_EXTRALIGHT_BYTES: &[u8] =
	include_bytes!("../../../../assets/fonts/Outfit/Outfit-ExtraLight.ttf");
pub const OUTFIT_LIGHT_BYTES: &[u8] =
	include_bytes!("../../../../assets/fonts/Outfit/Outfit-Light.ttf");
pub const OUTFIT_THIN_BYTES: &[u8] =
	include_bytes!("../../../../assets/fonts/Outfit/Outfit-Thin.ttf");
pub const OUTFIT_REGULAR_BYTES: &[u8] =
	include_bytes!("../../../../assets/fonts/Outfit/Outfit-Regular.ttf");
pub const OUTFIT_MEDIUM_BYTES: &[u8] =
	include_bytes!("../../../../assets/fonts/Outfit/Outfit-Medium.ttf");
pub const OUTFIT_SEMIBOLD_BYTES: &[u8] =
	include_bytes!("../../../../assets/fonts/Outfit/Outfit-SemiBold.ttf");
pub const OUTFIT_BOLD_BYTES: &[u8] =
	include_bytes!("../../../../assets/fonts/Outfit/Outfit-Bold.ttf");
pub const OUTFIT_EXTRABOLD_BYTES: &[u8] =
	include_bytes!("../../../../assets/fonts/Outfit/Outfit-ExtraBold.ttf");
pub const OUTFIT_BLACK_BYTES: &[u8] =
	include_bytes!("../../../../assets/fonts/Outfit/Outfit-Black.ttf");

// Outfit Font (Font objects)
pub const OUTFIT_EXTRALIGHT: Font = Font {
	family: Family::Name("Outfit"),
	weight: Weight::ExtraLight,
	stretch: Stretch::Normal,
	style: Style::Normal,
};
pub const OUTFIT_LIGHT: Font = Font {
	family: Family::Name("Outfit"),
	weight: Weight::Light,
	stretch: Stretch::Normal,
	style: Style::Normal,
};
pub const OUTFIT_THIN: Font = Font {
	family: Family::Name("Outfit"),
	weight: Weight::Thin,
	stretch: Stretch::Normal,
	style: Style::Normal,
};
pub const OUTFIT_REGULAR: Font = Font {
	family: Family::Name("Outfit"),
	weight: Weight::Normal,
	stretch: Stretch::Normal,
	style: Style::Normal,
};
pub const OUTFIT_MEDIUM: Font = Font {
	family: Family::Name("Outfit"),
	weight: Weight::Medium,
	stretch: Stretch::Normal,
	style: Style::Normal,
};
pub const OUTFIT_SEMIBOLD: Font = Font {
	family: Family::Name("Outfit"),
	weight: Weight::Normal,
	stretch: Stretch::Normal,
	style: Style::Normal,
};
pub const OUTFIT_BOLD: Font = Font {
	family: Family::Name("Outfit"),
	weight: Weight::Bold,
	stretch: Stretch::Normal,
	style: Style::Normal,
};
pub const OUTFIT_EXTRABOLD: Font = Font {
	family: Family::Name("Outfit"),
	weight: Weight::ExtraBold,
	stretch: Stretch::Normal,
	style: Style::Normal,
};
pub const OUTFIT_BLACK: Font = Font {
	family: Family::Name("Outfit"),
	weight: Weight::Black,
	stretch: Stretch::Normal,
	style: Style::Normal,
};

// Raleway Font (Bytes)
pub const RALEWAY_REGULAR_BYTES: &[u8] =
	include_bytes!("../../../../assets/fonts/Raleway/Raleway-Regular.ttf");
pub const RALEWAY_BLACK_ITALIC_BYTES: &[u8] =
	include_bytes!("../../../../assets/fonts/Raleway/Raleway-BlackItalic.ttf");
pub const RALEWAY_BLACK_BYTES: &[u8] =
	include_bytes!("../../../../assets/fonts/Raleway/Raleway-Black.ttf");
pub const RALEWAY_BOLD_ITALIC_BYTES: &[u8] =
	include_bytes!("../../../../assets/fonts/Raleway/Raleway-BoldItalic.ttf");
pub const RALEWAY_BOLD_BYTES: &[u8] =
	include_bytes!("../../../../assets/fonts/Raleway/Raleway-Bold.ttf");
pub const RALEWAY_EXTRA_BOLD_ITALIC_BYTES: &[u8] = include_bytes!(
	"../../../../assets/fonts/Raleway/Raleway-ExtraBoldItalic.ttf"
);
pub const RALEWAY_EXTRA_BOLD_BYTES: &[u8] =
	include_bytes!("../../../../assets/fonts/Raleway/Raleway-ExtraBold.ttf");
pub const RALEWAY_EXTRA_LIGHT_ITALIC_BYTES: &[u8] = include_bytes!(
	"../../../../assets/fonts/Raleway/Raleway-ExtraLightItalic.ttf"
);
pub const RALEWAY_EXTRA_LIGHT_BYTES: &[u8] =
	include_bytes!("../../../../assets/fonts/Raleway/Raleway-ExtraLight.ttf");
pub const RALEWAY_ITALIC_BYTES: &[u8] =
	include_bytes!("../../../../assets/fonts/Raleway/Raleway-Italic.ttf");
pub const RALEWAY_LIGHT_ITALIC_BYTES: &[u8] =
	include_bytes!("../../../../assets/fonts/Raleway/Raleway-LightItalic.ttf");
pub const RALEWAY_LIGHT_BYTES: &[u8] =
	include_bytes!("../../../../assets/fonts/Raleway/Raleway-Light.ttf");
pub const RALEWAY_MEDIUM_ITALIC_BYTES: &[u8] =
	include_bytes!("../../../../assets/fonts/Raleway/Raleway-MediumItalic.ttf");
pub const RALEWAY_MEDIUM_BYTES: &[u8] =
	include_bytes!("../../../../assets/fonts/Raleway/Raleway-Medium.ttf");
pub const RALEWAY_SEMI_BOLD_ITALIC_BYTES: &[u8] = include_bytes!(
	"../../../../assets/fonts/Raleway/Raleway-SemiBoldItalic.ttf"
);
pub const RALEWAY_SEMI_BOLD_BYTES: &[u8] =
	include_bytes!("../../../../assets/fonts/Raleway/Raleway-SemiBold.ttf");
pub const RALEWAY_THIN_ITALIC_BYTES: &[u8] =
	include_bytes!("../../../../assets/fonts/Raleway/Raleway-ThinItalic.ttf");
pub const RALEWAY_THIN_BYTES: &[u8] =
	include_bytes!("../../../../assets/fonts/Raleway/Raleway-Thin.ttf");

// Raleway Font (Font objects)
pub const RALEWAY_REGULAR: Font = Font {
	family: Family::Name("Raleway"),
	weight: Weight::Normal,
	stretch: Stretch::Normal,
	style: Style::Normal,
};
pub const RALEWAY_BLACK: Font = Font {
	family: Family::Name("Raleway"),
	weight: Weight::Black,
	stretch: Stretch::Normal,
	style: Style::Normal,
};
pub const RALEWAY_BLACK_ITALIC: Font = Font {
	family: Family::Name("Raleway"),
	weight: Weight::Black,
	stretch: Stretch::Normal,
	style: Style::Italic,
};
pub const RALEWAY_BOLD: Font = Font {
	family: Family::Name("Raleway"),
	weight: Weight::Bold,
	stretch: Stretch::Normal,
	style: Style::Normal,
};
pub const RALEWAY_BOLD_ITALIC: Font = Font {
	family: Family::Name("Raleway"),
	weight: Weight::Bold,
	stretch: Stretch::Normal,
	style: Style::Italic,
};
pub const RALEWAY_EXTRA_BOLD: Font = Font {
	family: Family::Name("Raleway"),
	weight: Weight::ExtraBold,
	stretch: Stretch::Normal,
	style: Style::Normal,
};
pub const RALEWAY_EXTRA_BOLD_ITALIC: Font = Font {
	family: Family::Name("Raleway"),
	weight: Weight::ExtraBold,
	stretch: Stretch::Normal,
	style: Style::Italic,
};
pub const RALEWAY_EXTRA_LIGHT: Font = Font {
	family: Family::Name("Raleway"),
	weight: Weight::ExtraLight,
	stretch: Stretch::Normal,
	style: Style::Normal,
};
pub const RALEWAY_EXTRA_LIGHT_ITALIC: Font = Font {
	family: Family::Name("Raleway"),
	weight: Weight::ExtraLight,
	stretch: Stretch::Normal,
	style: Style::Italic,
};
pub const RALEWAY_LIGHT: Font = Font {
	family: Family::Name("Raleway"),
	weight: Weight::Light,
	stretch: Stretch::Normal,
	style: Style::Normal,
};
pub const RALEWAY_LIGHT_ITALIC: Font = Font {
	family: Family::Name("Raleway"),
	weight: Weight::Light,
	stretch: Stretch::Normal,
	style: Style::Italic,
};
pub const RALEWAY_MEDIUM: Font = Font {
	family: Family::Name("Raleway"),
	weight: Weight::Medium,
	stretch: Stretch::Normal,
	style: Style::Normal,
};
pub const RALEWAY_MEDIUM_ITALIC: Font = Font {
	family: Family::Name("Raleway"),
	weight: Weight::Medium,
	stretch: Stretch::Normal,
	style: Style::Italic,
};
pub const RALEWAY_ITALIC: Font = Font {
	family: Family::Name("Raleway"),
	weight: Weight::Normal,
	stretch: Stretch::Normal,
	style: Style::Italic,
};
pub const RALEWAY_SEMI_BOLD: Font = Font {
	family: Family::Name("Raleway"),
	weight: Weight::Semibold,
	stretch: Stretch::Normal,
	style: Style::Normal,
};
pub const RALEWAY_SEMI_BOLD_ITALIC: Font = Font {
	family: Family::Name("Raleway"),
	weight: Weight::Semibold,
	stretch: Stretch::Normal,
	style: Style::Italic,
};
pub const RALEWAY_THIN: Font = Font {
	family: Family::Name("Raleway"),
	weight: Weight::Thin,
	stretch: Stretch::Normal,
	style: Style::Normal,
};
pub const RALEWAY_THIN_ITALIC: Font = Font {
	family: Family::Name("Raleway"),
	weight: Weight::Thin,
	stretch: Stretch::Normal,
	style: Style::Italic,
};

// font to display icons
pub const ICONS_BYTES: &[u8] =
	include_bytes!("../../../../assets/fonts/icons/font/icons.ttf");
pub const ICONS: Font = Font::with_name(ICON_FONT_FAMILY_NAME);

// font sizes

pub const FONT_SIZE_FOOTER: f32 = 14.3;
pub const FONT_SIZE_BODY: f32 = 16.8;
pub const FONT_SIZE_SUBTITLE: f32 = 18.3;
pub const FONT_SIZE_TITLE: f32 = 19.9;
