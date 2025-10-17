use std::{
	fs::File,
	hash::{
		Hash,
		Hasher,
	},
	io::{
		BufReader,
		Read,
	},
	path::Path,
};

use iced::Color;
use plotters::style::RGBColor;
use serde::{
	Deserialize,
	Serialize,
};

use super::color_remote::{
	deserialize_color,
	serialize_color,
};
use crate::gui::styles::types::color_remote::color_hash;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Copy)]
pub struct Palette {
	/// Base colors of the GUI for either background, hovered component, active
	/// tab
	#[serde(
		deserialize_with = "deserialize_color",
		serialize_with = "serialize_color"
	)]
	pub base_100: Color,
	#[serde(
		deserialize_with = "deserialize_color",
		serialize_with = "serialize_color"
	)]
	pub base_200: Color,
	#[serde(
		deserialize_with = "deserialize_color",
		serialize_with = "serialize_color"
	)]
	pub base_300: Color,
	#[serde(
		deserialize_with = "deserialize_color",
		serialize_with = "serialize_color"
	)]
	/// Base content color for GUI e.g text color or content inside the
	/// component
	pub base_content: Color,
	#[serde(
		deserialize_with = "deserialize_color",
		serialize_with = "serialize_color"
	)]
	pub primary: Color,
	#[serde(
		deserialize_with = "deserialize_color",
		serialize_with = "serialize_color"
	)]
	pub primary_content: Color,
	#[serde(
		deserialize_with = "deserialize_color",
		serialize_with = "serialize_color"
	)]
	pub secondary: Color,
	#[serde(
		deserialize_with = "deserialize_color",
		serialize_with = "serialize_color"
	)]
	pub secondary_content: Color,
	#[serde(
		deserialize_with = "deserialize_color",
		serialize_with = "serialize_color"
	)]
	pub accent: Color,
	#[serde(
		deserialize_with = "deserialize_color",
		serialize_with = "serialize_color"
	)]
	pub accent_content: Color,
	#[serde(
		deserialize_with = "deserialize_color",
		serialize_with = "serialize_color"
	)]
	pub neutral: Color,
	#[serde(
		deserialize_with = "deserialize_color",
		serialize_with = "serialize_color"
	)]
	pub neutral_content: Color,
	#[serde(
		deserialize_with = "deserialize_color",
		serialize_with = "serialize_color"
	)]
	pub info: Color,
	#[serde(
		deserialize_with = "deserialize_color",
		serialize_with = "serialize_color"
	)]
	pub info_content: Color,
	#[serde(
		deserialize_with = "deserialize_color",
		serialize_with = "serialize_color"
	)]
	pub warning: Color,
	#[serde(
		deserialize_with = "deserialize_color",
		serialize_with = "serialize_color"
	)]
	pub warning_content: Color,
	#[serde(
		deserialize_with = "deserialize_color",
		serialize_with = "serialize_color"
	)]
	pub success: Color,
	#[serde(
		deserialize_with = "deserialize_color",
		serialize_with = "serialize_color"
	)]
	pub success_content: Color,
	#[serde(
		deserialize_with = "deserialize_color",
		serialize_with = "serialize_color"
	)]
	pub error: Color,
	#[serde(
		deserialize_with = "deserialize_color",
		serialize_with = "serialize_color"
	)]
	pub error_content: Color,
}

impl Palette {
	pub fn generate_buttons_color(self) -> Color {
		self.primary
	}

	pub fn generate_containers_color(self) -> Color {
		self.base_100
	}

	/// Deserialize [`Palette`] from `path`
	///
	/// # Arguments
	/// # `path` - Path to a UTF-8 encoded file containing a custom style as TOML.
	pub fn from_file<P>(path: P) -> Result<Self, toml::de::Error>
	where
		P: AsRef<Path>,
	{
		// Try to open the file at `path`
		let mut toml_reader = File::open(path)
			.map_err(serde::de::Error::custom)
			.map(BufReader::new)?;

		// Read the ostensibel TOML
		let mut style_toml = String::new();
		toml_reader
			.read_to_string(&mut style_toml)
			.map_err(serde::de::Error::custom)?;

		toml::de::from_str(&style_toml)
	}
}

impl Hash for Palette {
	fn hash<H: Hasher>(&self, state: &mut H) {
		let Palette {
			base_100,
			base_200,
			base_300,
			base_content,
			primary,
			primary_content,
			secondary,
			secondary_content,
			accent,
			accent_content,
			neutral,
			neutral_content,
			info,
			info_content,
			success,
			success_content,
			warning,
			warning_content,
			error,
			error_content,
		} = self;

		color_hash(*base_100, state);
		color_hash(*base_200, state);
		color_hash(*base_300, state);
		color_hash(*base_content, state);
		color_hash(*primary, state);
		color_hash(*primary_content, state);
		color_hash(*secondary, state);
		color_hash(*secondary_content, state);
		color_hash(*accent, state);
		color_hash(*accent_content, state);
		color_hash(*neutral, state);
		color_hash(*neutral_content, state);
		color_hash(*info, state);
		color_hash(*info_content, state);
		color_hash(*success, state);
		color_hash(*success_content, state);
		color_hash(*warning, state);
		color_hash(*warning_content, state);
		color_hash(*error, state);
		color_hash(*error_content, state);
	}
}

pub fn to_rgb_color(color: Color) -> RGBColor {
	#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
	if color.r <= 1.0
		&& color.r >= 0.0
		&& color.g <= 1.0
		&& color.g >= 0.0
		&& color.b <= 1.0
		&& color.b >= 0.0
	{
		RGBColor(
			(color.r * 255.0) as u8,
			(color.g * 255.0) as u8,
			(color.b * 255.0) as u8,
		)
	} else {
		RGBColor(0, 0, 0) // Black
	}
}

/// Returns the average of two colors; color intensity is fixed to 100%
pub const fn mix_colors(color_1: Color, color_2: Color) -> Color {
	Color {
		r: f32::midpoint(color_1.r, color_2.r),
		g: f32::midpoint(color_1.g, color_2.g),
		b: f32::midpoint(color_1.b, color_2.b),
		a: 1.0,
	}
}
