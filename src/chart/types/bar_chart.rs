use std::f64;

use iced::{
	Color,
	Font,
	Renderer,
	alignment::{
		Horizontal,
		Vertical,
	},
	widget::canvas::{
		self,
		Frame,
		Path,
		Text,
	},
};

use crate::gui::{
	styles::{
		bar::Catalog,
		style_constant::{
			CHART_BAR_WIDTH,
			fonts::FONT_SIZE_TITLE,
		},
	},
	types::message::Message,
};

#[derive(Clone, Debug)]
pub struct BarChart {
	title: String,
	labels: Vec<String>,
	values: Vec<f64>,
	font: Font,
}

impl BarChart {
	const fn new(
		title: String,
		labels: Vec<String>,
		values: Vec<f64>,
		font: Font,
	) -> Self {
		Self { title, labels, values, font }
	}

	fn title(&self) -> String {
		self.title.clone()
	}

	fn max_value(&self) -> f64 {
		self.values.iter().copied().fold(f64::NEG_INFINITY, f64::max)
	}
}

impl<Message, Theme: Catalog> canvas::Program<Message, Theme> for BarChart {
	type State = ();

	fn draw(
		&self,
		state: &Self::State,
		renderer: &Renderer,
		theme: &Theme,
		bounds: iced::Rectangle,
		cursor: iced::advanced::mouse::Cursor,
	) -> Vec<canvas::Geometry> {
		let mut frame = Frame::new(renderer, bounds.size());
		let style =
			<Theme as Catalog>::style(theme, &<Theme as Catalog>::default());
		let colors = [style.primary, style.secondary, style.accent];

		let total_width = bounds.width;
		let mut y_offset: f64 = 0.0;
		let left_padding = 13.0;
		let usable_width = bounds.width - left_padding;

		vec![frame.into_geometry()]
	}
}
