use std::hash::{Hash, Hasher};

use iced::{Color, Font};
use serde::{de::Unexpected, Deserialize, Deserializer, Serialize, Serializer};

use crate::gui::styles::{
    style_constant::fonts::{
        OUTFIT_BLACK, OUTFIT_BOLD, OUTFIT_EXTRABOLD, OUTFIT_EXTRALIGHT, OUTFIT_LIGHT,
        OUTFIT_MEDIUM, OUTFIT_REGULAR, OUTFIT_SEMIBOLD, OUTFIT_THIN, RALEWAY_BLACK,
        RALEWAY_BLACK_ITALIC, RALEWAY_BOLD, RALEWAY_BOLD_ITALIC, RALEWAY_EXTRA_BOLD,
        RALEWAY_EXTRA_BOLD_ITALIC, RALEWAY_EXTRA_LIGHT, RALEWAY_EXTRA_LIGHT_ITALIC, RALEWAY_ITALIC,
        RALEWAY_LIGHT, RALEWAY_LIGHT_ITALIC, RALEWAY_MEDIUM, RALEWAY_MEDIUM_ITALIC,
        RALEWAY_REGULAR, RALEWAY_SEMI_BOLD, RALEWAY_SEMI_BOLD_ITALIC, RALEWAY_THIN,
        RALEWAY_THIN_ITALIC,
    },
    types::color_remote::color_hash,
};

use super::color_remote::{deserialize_color, serialize_color};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaletteExtension {
    pub is_nightly: bool,
    #[serde(
        deserialize_with = "deserialize_font",
        serialize_with = "serialize_font"
    )]
    pub font: Font,
    #[serde(
        deserialize_with = "deserialize_font",
        serialize_with = "serialize_font"
    )]
    pub font_headers: Font,
    pub radius_selectors: f32,
    pub radius_fields: f32,
    pub radius_boxes: f32,
    pub size_selectors: f32,
    pub size_fields: f32,
    pub alpha_round_borders: f32,
    pub alpha_round_containers: f32,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub buttons_color: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub containers_color: Color,
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub red_alert_color: Color,
}

impl Hash for PaletteExtension {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let PaletteExtension {
            is_nightly,
            font,
            font_headers,
            radius_selectors,
            radius_fields,
            radius_boxes,
            size_selectors,
            size_fields,
            alpha_round_borders,
            alpha_round_containers,
            buttons_color,
            containers_color,
            red_alert_color,
        } = self;

        is_nightly.hash(state);
        font.hash(state);
        font_headers.hash(state);
        #[allow(clippy::cast_possible_truncation)]
        (997 * (radius_boxes + alpha_round_borders + alpha_round_containers) as i32).hash(state);
        (997 * (radius_selectors + size_selectors) as i32).hash(state);
        (997 * (radius_fields + size_fields) as i32).hash(state);
        color_hash(*buttons_color, state);
        color_hash(*red_alert_color, state);
        color_hash(*containers_color, state);
    }
}

pub(super) fn deserialize_font<'de, D>(deserializer: D) -> Result<Font, D::Error>
where
    D: Deserializer<'de>,
{
    // Name should be SARASA_MONO or SARASA_MONO_BOLD
    let name = String::deserialize(deserializer)?;
    let name_str = name.as_str();

    match name_str {
        // Outfit Fonts
        "OUTFIT_EXTRALIGHT" => Ok(OUTFIT_EXTRALIGHT),
        "OUTFIT_LIGHT" => Ok(OUTFIT_LIGHT),
        "OUTFIT_THIN" => Ok(OUTFIT_THIN),
        "OUTFIT_REGULAR" => Ok(OUTFIT_REGULAR),
        "OUTFIT_MEDIUM" => Ok(OUTFIT_MEDIUM),
        "OUTFIT_SEMIBOLD" => Ok(OUTFIT_SEMIBOLD),
        "OUTFIT_BOLD" => Ok(OUTFIT_BOLD),
        "OUTFIT_EXTRABOLD" => Ok(OUTFIT_EXTRABOLD),
        "OUTFIT_BLACK" => Ok(OUTFIT_BLACK),

        // Raleway Font
        "RALEWAY_REGULAR" => Ok(RALEWAY_REGULAR),
        "RALEWAY_BLACK" => Ok(RALEWAY_BLACK),
        "RALEWAY_BLACK_ITALIC" => Ok(RALEWAY_BLACK_ITALIC),
        "RALEWAY_BOLD" => Ok(RALEWAY_BOLD),
        "RALEWAY_BOLD_ITALIC" => Ok(RALEWAY_BOLD_ITALIC),
        "RALEWAY_EXTRA_BOLD" => Ok(RALEWAY_EXTRA_BOLD),
        "RALEWAY_EXTRA_BOLD_ITALIC" => Ok(RALEWAY_EXTRA_BOLD_ITALIC),
        "RALEWAY_EXTRA_LIGHT" => Ok(RALEWAY_EXTRA_LIGHT),
        "RALEWAY_EXTRA_LIGHT_ITALIC" => Ok(RALEWAY_EXTRA_LIGHT_ITALIC),
        "RALEWAY_LIGHT" => Ok(RALEWAY_LIGHT),
        "RALEWAY_LIGHT_ITALIC" => Ok(RALEWAY_LIGHT_ITALIC),
        "RALEWAY_MEDIUM" => Ok(RALEWAY_MEDIUM),
        "RALEWAY_MEDIUM_ITALIC" => Ok(RALEWAY_MEDIUM_ITALIC),
        "RALEWAY_ITALIC" => Ok(RALEWAY_ITALIC),
        "RALEWAY_SEMI_BOLD" => Ok(RALEWAY_SEMI_BOLD),
        "RALEWAY_SEMI_BOLD_ITALIC" => Ok(RALEWAY_SEMI_BOLD_ITALIC),
        "RALEWAY_THIN" => Ok(RALEWAY_THIN),
        "RALEWAY_THIN_ITALIC" => Ok(RALEWAY_THIN_ITALIC),
        _ => Err(serde::de::Error::invalid_value(
            Unexpected::Str(name_str),
            &"OUTFIT_REGULAR OR RALEWAY_REGULAR",
        )),
    }
}

pub(super) fn serialize_font<S>(font: &Font, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match *font {
        // Outfit Fonts
        OUTFIT_BLACK => serializer.serialize_str("OUTFIT_BLACK"),
        OUTFIT_BOLD => serializer.serialize_str("OUTFIT_BOLD"),
        OUTFIT_EXTRABOLD => serializer.serialize_str("OUTFIT_EXTRABOLD"),
        OUTFIT_EXTRALIGHT => serializer.serialize_str("OUTFIT_EXTRALIGHT"),
        OUTFIT_LIGHT => serializer.serialize_str("OUTFIT_LIGHT"),
        OUTFIT_MEDIUM => serializer.serialize_str("OUTFIT_MEDIUM"),
        OUTFIT_REGULAR => serializer.serialize_str("OUTFIT_REGULAR"),
        OUTFIT_SEMIBOLD => serializer.serialize_str("OUTFIT_SEMIBOLD"),
        OUTFIT_THIN => serializer.serialize_str("OUTFIT_THIN"),

        // Raleway Fonts
        RALEWAY_BLACK => serializer.serialize_str("RALEWAY_BLACK"),
        RALEWAY_BLACK_ITALIC => serializer.serialize_str("RALEWAY_BLACK_ITALIC"),
        RALEWAY_BOLD => serializer.serialize_str("RALEWAY_BOLD"),
        RALEWAY_BOLD_ITALIC => serializer.serialize_str("RALEWAY_BOLD_ITALIC"),
        RALEWAY_EXTRA_BOLD => serializer.serialize_str("RALEWAY_EXTRA_BOLD"),
        RALEWAY_EXTRA_BOLD_ITALIC => serializer.serialize_str("RALEWAY_EXTRA_BOLD_ITALIC"),
        RALEWAY_EXTRA_LIGHT => serializer.serialize_str("RALEWAY_EXTRA_LIGHT"),
        RALEWAY_EXTRA_LIGHT_ITALIC => serializer.serialize_str("RALEWAY_EXTRA_LIGHT_ITALIC"),
        RALEWAY_ITALIC => serializer.serialize_str("RALEWAY_ITALIC"),
        RALEWAY_LIGHT => serializer.serialize_str("RALEWAY_LIGHT"),
        RALEWAY_LIGHT_ITALIC => serializer.serialize_str("RALEWAY_LIGHT_ITALIC"),
        RALEWAY_MEDIUM => serializer.serialize_str("RALEWAY_MEDIUM"),
        RALEWAY_MEDIUM_ITALIC => serializer.serialize_str("RALEWAY_MEDIUM_ITALIC"),
        RALEWAY_REGULAR => serializer.serialize_str("RALEWAY_REGULAR"),
        RALEWAY_SEMI_BOLD => serializer.serialize_str("RALEWAY_SEMI_BOLD"),
        RALEWAY_SEMI_BOLD_ITALIC => serializer.serialize_str("RALEWAY_SEMI_BOLD_ITALIC"),
        RALEWAY_THIN => serializer.serialize_str("RALEWAY_THIN"),
        RALEWAY_THIN_ITALIC => serializer.serialize_str("RALEWAY_THIN_ITALIC"),
        _ => Err(serde::ser::Error::custom("invalid font")),
    }
}
