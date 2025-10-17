pub mod colors;
pub mod fonts;

// border styles
pub const BORDER_WIDTH: f32 = 2.0;
pub const CHARTS_LINE_BORDER: u32 = 1;
pub const BORDER_ROUNDED_RADIUS: f32 = 8.0;
pub const BORDER_RADIUS: f32 = 8.0;
pub const BORDER_BUTTON_RADIUS: f32 = 180.0;
pub const CHART_BAR_WIDTH: f64 = 24.0;

#[allow(clippy::enum_variant_names, clippy::large_enum_variant)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StandardNames {
	Base100,
	Base200,
	Base300,
	Primary,
	Secondary,
	Accent,
	Neutral,
	Info,
	Success,
	Warning,
	Error,
}
