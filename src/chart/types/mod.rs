use plotters::style::RGBColor;

pub mod bar_chart;
pub mod donut_chart;
pub mod line_chart;

#[derive(Clone, Debug)]
pub struct CategoryData {
	pub label: String,
	pub segments: Vec<(f32, RGBColor)>,
}
