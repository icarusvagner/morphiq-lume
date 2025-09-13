use iced::Color;
mod ext_colors;

pub use ext_colors::*;

// dat theme colors
#[derive(Debug, Clone)]
pub enum Colors {
    Ghost,
    UTOrange,
    SelectiveYellow,
    PrussianBlue,
    BlueGreen,
    SkyBlue,
    AntiFlashWhite,
    Night,
    Silver,
    DarkPastelGreen,
    Xanthous,
    Red,
    Pumpkin,
    MayaBlue,
    Citrine,
}

impl Colors {
    pub fn get(&self) -> Color {
        match self {
            Colors::Citrine => CITRINE,
            Colors::MayaBlue => MAYABLUE,
            Colors::DarkPastelGreen => DARK_PASTEL_GREEN,
            Colors::Xanthous => XANTHOUS,
            Colors::Red => RED,
            Colors::Pumpkin => PUMPKIN,
            Colors::Ghost => GHOST,
            Colors::Silver => SILVER,
            Colors::UTOrange => UT_ORANGE,
            Colors::SelectiveYellow => SELECTIVE_YELLOW,
            Colors::PrussianBlue => PRUSSIAN_BLUE,
            Colors::BlueGreen => BLUE_GREEN,
            Colors::SkyBlue => SKY_BLUE,
            Colors::AntiFlashWhite => ANTI_FLASH_WHITE,
            Colors::Night => NIGHT,
        }
    }
}

#[rustfmt::skip]
const CITRINE: Color = Color { r: 218.0 / 255.0, g: 203.0 / 255.0, b: 38.0 / 255.0, a: 1.0, };

#[rustfmt::skip]
const MAYABLUE: Color = Color { r: 78.0 / 255.0, g: 187.0 / 255.0, b: 255.0, a: 1.0, };

#[rustfmt::skip]
const DARK_PASTEL_GREEN: Color = Color { r: 0.0 / 255.0, g: 201.0 / 255.0, b: 81.0 / 255.0, a: 1.0, };

#[rustfmt::skip]
const RED: Color = Color { r: 231.0 / 255.0, g: 0.0 / 255.0, b: 11.0 / 255.0, a: 1.0, };

#[rustfmt::skip]
const XANTHOUS: Color = Color { r: 250.0 / 255.0, g: 177.0 / 255.0, b: 0.0 / 255.0, a: 1.0, };

#[rustfmt::skip]
const PUMPKIN: Color = Color { r: 255.0, g: 105.0 / 255.0, b: 0.0 / 255.0, a: 1.0, };

#[rustfmt::skip]
const GHOST: Color = Color { r: 121.0 / 255.0, g: 121.0 / 255.0, b: 121.0 / 255.0, a: 10.0 / 100.0, };

#[rustfmt::skip]
const SILVER: Color = Color { r: 201.0 / 255.0, g: 201.0 / 255.0, b: 201.0 / 255.0, a: 1.0, };

#[rustfmt::skip]
const UT_ORANGE: Color = Color { r: 251.0 / 255.0, g: 133.0 / 255.0, b: 0. / 255.00, a: 1.0, };

#[rustfmt::skip]
const SELECTIVE_YELLOW: Color = Color { r: 255.0, g: 183.0 / 255.0, b: 3.0 / 255.0, a: 1.0, };

#[rustfmt::skip]
const PRUSSIAN_BLUE: Color = Color { r: 2.0 / 255.0, g: 48.0 / 255.0, b: 71.0 / 255.0, a: 1.0, };

#[rustfmt::skip]
const BLUE_GREEN: Color = Color { r: 33. / 255.00, g: 158.0 / 255.0, b: 188.0 / 255.0, a: 1.0, };

#[rustfmt::skip]
const SKY_BLUE: Color = Color { r: 142.0 / 255.0, g: 202.0 / 255.0, b: 230.0 / 255.0, a: 1.0, };

#[rustfmt::skip]
const ANTI_FLASH_WHITE: Color = Color { r: 238.0 / 255.0, g: 238.0 / 255.0, b: 238.0 / 255.0, a: 1.0, };

#[rustfmt::skip]
const NIGHT: Color = Color { r: 21.0 / 255.0, g: 21.0 / 255.0, b: 21.0 / 255.0, a: 1.0, };

// red colors for alerts
#[rustfmt::skip]
pub const RED_ALERT_COLOR_NIGHTLY: Color = Color { r: 1.0, g: 0.4, b: 0.4, a: 1.0, };
#[rustfmt::skip]
pub const RED_ALERT_COLOR_DAILY: Color = Color { r: 0.701_960_8, g: 0.0, b: 0.0, a: 1.0, };
