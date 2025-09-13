use iced::Color;

use crate::gui::styles::{
    style_constant::fonts::{OUTFIT_REGULAR, RALEWAY_BOLD},
    types::{palette::Palette, palette_extension::PaletteExtension},
};

#[rustfmt::skip]
pub const NORD_PALETTE: Palette = Palette {
    base_100: Color { r: 0.951, g: 0.953, b: 0.965, a: 1.0, }, // oklch(95.127%...)
    base_200: Color { r: 0.933, g: 0.937, b: 0.957, a: 1.0, }, // oklch(93.299%...)
    base_300: Color { r: 0.902, g: 0.910, b: 0.953, a: 1.0, }, // oklch(89.925%...)
    base_content: Color { r: 0.322, g: 0.333, b: 0.376, a: 1.0, }, // oklch(32.437%...)

    primary: Color { r: 0.467, g: 0.525, b: 0.714, a: 1.0, }, // oklch(59.435%...)
    primary_content: Color { r: 0.118, g: 0.129, b: 0.176, a: 1.0, }, // oklch(11.887%...)

    secondary: Color { r: 0.545, g: 0.600, b: 0.769, a: 1.0, }, // oklch(69.651%...)
    secondary_content: Color { r: 0.137, g: 0.145, b: 0.196, a: 1.0, }, // oklch(13.93%...)

    accent: Color { r: 0.655, g: 0.725, b: 0.878, a: 1.0, }, // oklch(77.464%...)
    accent_content: Color { r: 0.155, g: 0.161, b: 0.204, a: 1.0, }, // oklch(15.492%...)

    neutral: Color { r: 0.435, g: 0.459, b: 0.565, a: 1.0, }, // oklch(45.229%...)
    neutral_content: Color { r: 0.902, g: 0.910, b: 0.953, a: 1.0, }, // oklch(89.925%...)

    info: Color { r: 0.769, g: 0.541, b: 0.706, a: 1.0, }, // oklch(69.207%...)
    info_content: Color { r: 0.137, g: 0.145, b: 0.169, a: 1.0, }, // oklch(13.841%...)

    success: Color { r: 0.682, g: 0.808, b: 0.706, a: 1.0, }, // oklch(76.827%...)
    success_content: Color { r: 0.153, g: 0.161, b: 0.180, a: 1.0, }, // oklch(15.365%...)

    warning: Color { r: 0.922, g: 0.796, b: 0.545, a: 1.0, }, // oklch(85.486%...)
    warning_content: Color { r: 0.169, g: 0.176, b: 0.200, a: 1.0, }, // oklch(17.097%...)

    error: Color { r: 0.753, g: 0.333, b: 0.333, a: 1.0, }, // oklch(60.61%...)
    error_content: Color { r: 0.122, g: 0.094, b: 0.094, a: 1.0, }, // oklch(12.122%...)
};

pub const NORD_PALETTE_EXTENSION: PaletteExtension = PaletteExtension {
    is_nightly: false,
    font: OUTFIT_REGULAR,
    font_headers: RALEWAY_BOLD,
    radius_selectors: 1.0,
    radius_fields: 0.25,
    radius_boxes: 0.5,
    size_selectors: 0.25,
    size_fields: 0.25,
    alpha_round_borders: 0.4,
    alpha_round_containers: 0.3,
    buttons_color: NORD_PALETTE.primary,
    containers_color: NORD_PALETTE.base_100,
    red_alert_color: NORD_PALETTE.warning,
};

#[rustfmt::skip]
pub const NIGHT_PALETTE: Palette = Palette {
    base_100: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0, }, // #FFFFFF
    base_200: Color { r: 0.980, g: 0.980, b: 0.980, a: 1.0, }, // #FAFAFA
    base_300: Color { r: 0.949, g: 0.949, b: 0.949, a: 1.0, }, // #F2F2F2
    base_content: Color { r: 0.208, g: 0.208, b: 0.208, a: 1.0, }, // #353535

    primary: Color { r: 0.357, g: 0.239, b: 1.0, a: 1.0, }, // #5B3DFF
    primary_content: Color { r: 0.945, g: 0.933, b: 1.0, a: 1.0, }, // #F1EEFF

    secondary: Color { r: 1.0, g: 0.278, b: 0.439, a: 1.0, }, // #FF4770
    secondary_content: Color { r: 1.0, g: 0.941, b: 0.965, a: 1.0, }, // #FFF0F6

    accent: Color { r: 0.239, g: 0.859, b: 0.722, a: 1.0, }, // #3DDBB8
    accent_content: Color { r: 0.071, g: 0.373, b: 0.322, a: 1.0, }, // #125F52

    neutral: Color { r: 0.133, g: 0.133, b: 0.133, a: 1.0, }, // #222222
    neutral_content: Color { r: 0.922, g: 0.922, b: 0.922, a: 1.0, }, // #EBEBEB

    info: Color { r: 0.247, g: 0.639, b: 1.0, a: 1.0, }, // #3FA3FF
    info_content: Color { r: 0.059, g: 0.216, b: 0.353, a: 1.0, }, // #0F375A

    success: Color { r: 0.239, g: 0.859, b: 0.388, a: 1.0, }, // #3DDB63
    success_content: Color { r: 0.094, g: 0.357, b: 0.165, a: 1.0, }, // #185B2A

    warning: Color { r: 1.0, g: 0.851, b: 0.302, a: 1.0, }, // #FFD94D
    warning_content: Color { r: 0.400, g: 0.278, b: 0.0, a: 1.0, }, // #664700

    error: Color { r: 1.0, g: 0.294, b: 0.294, a: 1.0, }, // #FF4B4B
    error_content: Color { r: 0.361, g: 0.071, b: 0.071, a: 1.0, }, // #5C1212
};

pub const NIGHT_PALETTE_EXTENSION: PaletteExtension = PaletteExtension {
    is_nightly: true,
    font: OUTFIT_REGULAR,
    font_headers: RALEWAY_BOLD,
    radius_selectors: 1.0,
    radius_fields: 0.5,
    radius_boxes: 1.0,
    size_selectors: 0.25,
    size_fields: 0.25,
    alpha_round_borders: 0.4,
    alpha_round_containers: 0.3,
    buttons_color: NIGHT_PALETTE.primary,
    containers_color: NIGHT_PALETTE.base_100,
    red_alert_color: NIGHT_PALETTE.warning,
};

#[rustfmt::skip]
pub const SILK_PALETTE: Palette = Palette {
    base_100: Color { r: 0.970, g: 0.970, b: 0.965, a: 1.0, }, // oklch(97%…)
    base_200: Color { r: 0.950, g: 0.950, b: 0.940, a: 1.0, }, // oklch(95%…)
    base_300: Color { r: 0.900, g: 0.900, b: 0.890, a: 1.0, }, // oklch(90%…)
    base_content: Color { r: 0.400, g: 0.400, b: 0.390, a: 1.0, }, // oklch(40%…)

    primary: Color { r: 0.232, g: 0.220, b: 0.270, a: 1.0, }, // dark violet
    primary_content: Color { r: 0.940, g: 0.960, b: 0.880, a: 1.0, }, // pale green-yellow

    secondary: Color { r: 0.232, g: 0.220, b: 0.270, a: 1.0, }, // same as primary
    secondary_content: Color { r: 0.930, g: 0.820, b: 0.600, a: 1.0, }, // warm gold

    accent: Color { r: 0.232, g: 0.220, b: 0.270, a: 1.0, }, // same as primary
    accent_content: Color { r: 0.820, g: 0.900, b: 0.880, a: 1.0, }, // teal-tinted

    neutral: Color { r: 0.200, g: 0.200, b: 0.200, a: 1.0, }, // dark gray
    neutral_content: Color { r: 0.800, g: 0.800, b: 0.780, a: 1.0, }, // light gray

    info: Color { r: 0.600, g: 0.700, b: 0.950, a: 1.0, }, // light blue
    info_content: Color { r: 0.300, g: 0.350, b: 0.500, a: 1.0, }, // muted blue

    success: Color { r: 0.700, g: 0.850, b: 0.700, a: 1.0, }, // light green
    success_content: Color { r: 0.240, g: 0.400, b: 0.240, a: 1.0, }, // deep green

    warning: Color { r: 0.950, g: 0.850, b: 0.600, a: 1.0, }, // warm yellow-orange
    warning_content: Color { r: 0.600, g: 0.450, b: 0.250, a: 1.0, }, // darker orange-brown

    error: Color { r: 0.850, g: 0.350, b: 0.250, a: 1.0, }, // reddish-orange
    error_content: Color { r: 0.500, g: 0.200, b: 0.150, a: 1.0, }, // deep red-brown
};

pub const SILK_PALETTE_EXTENSION: PaletteExtension = PaletteExtension {
    is_nightly: false,
    font: OUTFIT_REGULAR,
    font_headers: RALEWAY_BOLD,
    radius_selectors: 2.0,
    radius_fields: 0.5,
    radius_boxes: 1.0,
    size_selectors: 0.25,
    size_fields: 0.25,
    alpha_round_borders: 0.4,
    alpha_round_containers: 0.3,
    buttons_color: SILK_PALETTE.primary,
    containers_color: SILK_PALETTE.base_100,
    red_alert_color: SILK_PALETTE.warning,
};

#[rustfmt::skip]
pub const ABYSS_PALETTE: Palette = Palette {
    base_100: Color { r: 0.12, g: 0.18, b: 0.28, a: 1.0, }, // dark blue (oklch(20%…))
    base_200: Color { r: 0.08, g: 0.13, b: 0.22, a: 1.0, }, // deeper navy
    base_300: Color { r: 0.05, g: 0.08, b: 0.15, a: 1.0, }, // near black-blue
    base_content: Color { r: 0.95, g: 0.92, b: 0.80, a: 1.0, }, // pale warm white

    primary: Color { r: 0.85, g: 1.00, b: 0.80, a: 1.0, }, // bright green-yellow
    primary_content: Color { r: 0.40, g: 0.60, b: 0.40, a: 1.0, }, // mid green

    secondary: Color { r: 0.82, g: 0.70, b: 0.90, a: 1.0, }, // light purple
    secondary_content: Color { r: 0.45, g: 0.30, b: 0.55, a: 1.0, }, // deep violet

    accent: Color { r: 0.25, g: 0.25, b: 0.25, a: 1.0, }, // dark gray
    accent_content: Color { r: 0.98, g: 0.98, b: 0.98, a: 1.0, }, // near white

    neutral: Color { r: 0.18, g: 0.25, b: 0.35, a: 1.0, }, // muted navy
    neutral_content: Color { r: 0.95, g: 0.92, b: 0.80, a: 1.0, }, // pale warm white

    info: Color { r: 0.55, g: 0.70, b: 0.95, a: 1.0, }, // sky blue
    info_content: Color { r: 0.25, g: 0.35, b: 0.55, a: 1.0, }, // deep slate blue

    success: Color { r: 0.60, g: 0.85, b: 0.65, a: 1.0, }, // soft green
    success_content: Color { r: 0.25, g: 0.45, b: 0.30, a: 1.0, }, // forest green

    warning: Color { r: 0.95, g: 0.80, b: 0.45, a: 1.0, }, // golden yellow
    warning_content: Color { r: 0.55, g: 0.35, b: 0.10, a: 1.0, }, // brown-orange

    error: Color { r: 0.80, g: 0.35, b: 0.25, a: 1.0, }, // red-orange
    error_content: Color { r: 0.40, g: 0.15, b: 0.10, a: 1.0, }, // deep brown-red
};

pub const ABYSS_PALETTE_EXTENSION: PaletteExtension = PaletteExtension {
    is_nightly: true,
    font: OUTFIT_REGULAR,
    font_headers: RALEWAY_BOLD,
    radius_selectors: 1.0,
    radius_fields: 0.5,
    radius_boxes: 1.0,
    size_selectors: 0.25,
    size_fields: 0.25,
    alpha_round_borders: 0.4,
    alpha_round_containers: 0.3,
    buttons_color: ABYSS_PALETTE.primary,
    containers_color: ABYSS_PALETTE.base_100,
    red_alert_color: ABYSS_PALETTE.warning,
};
