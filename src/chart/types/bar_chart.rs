use std::{
	f64,
	time::Instant,
};

use iced::{
	Color,
	Font,
	Length,
	Point,
	Rectangle,
	Renderer,
	Size,
	advanced::mouse,
	widget::canvas::{
		self,
		Cache,
		Canvas,
		Frame,
		Path,
		Style,
		Text,
		event,
	},
};

use crate::gui::{
	styles::bar::Catalog,
	types::message::Message,
};

#[derive(Clone, Debug)]
pub struct HistogramChart {
	title: String,
	labels: Vec<String>,
	values: Vec<f64>,
	font: Font,
}

#[derive(Debug, Clone)]
pub enum BarMessage {
	BarHovered(Option<usize>),
}

#[derive(Debug, Default)]
pub struct State {
	hovered_bar: Option<usize>,
	start_time: Option<Instant>,
	cache: Cache,
}

impl HistogramChart {
	pub const fn new(
		title: String,
		labels: Vec<String>,
		values: Vec<f64>,
		font: Font,
	) -> Self {
		Self { title, labels, values, font }
	}

	fn max_value(&self) -> f64 {
		self.values.iter().copied().fold(f64::NEG_INFINITY, f64::max)
	}

	fn title(&self) -> String {
		self.title.clone()
	}
}

// Option 2 (alternative): Display placeholder image If you have an
// image asset in memory or bytes, use `Frame::draw_image` Example:
// let image = canvas::Image::new("assets/empty_chart.png");
// frame.draw_image(image, Point::new(width / 2.0 - 64.0, height /
// 2.0 - 64.0));

#[allow(
	clippy::cast_precision_loss,
	clippy::cast_possible_truncation,
	clippy::cast_sign_loss,
	clippy::collapsible_if,
	clippy::too_many_lines
)]
impl<Theme: Catalog> canvas::Program<Message, Theme> for HistogramChart {
	type State = State;

	fn update(
		&self,
		state: &mut Self::State,
		event: event::Event,
		bounds: Rectangle,
		cursor: mouse::Cursor,
	) -> (event::Status, Option<Message>) {
		if let event::Event::Mouse(_) = event {
			if let Some(cursor_pos) = cursor.position_in(bounds) {
				let num_bars = self.values.len() as f32;
				if num_bars > 0.0 {
					let bar_width = bounds.width / num_bars;
					let hovered = (cursor_pos.x / bar_width).floor() as usize;
					if hovered < self.values.len() {
						if state.hovered_bar != Some(hovered) {
							state.hovered_bar = Some(hovered);
							return (
								event::Status::Captured,
								Some(Message::Chart(
									crate::chart::ChartMessage::Bar(
										BarMessage::BarHovered(Some(hovered)),
									),
								)),
							);
						}
					} else if state.hovered_bar.is_some() {
						state.hovered_bar = None;
						return (
							event::Status::Captured,
							Some(Message::Chart(
								crate::chart::ChartMessage::Bar(
									BarMessage::BarHovered(None),
								),
							)),
						);
					}
				}
			}
		}

		(event::Status::Ignored, None)
	}

	fn draw(
		&self,
		state: &Self::State,
		renderer: &Renderer,
		theme: &Theme,
		bounds: Rectangle,
		_cursor: mouse::Cursor,
	) -> Vec<canvas::Geometry> {
		let mut frame = Frame::new(renderer, bounds.size());
		let style =
			<Theme as Catalog>::style(theme, &<Theme as Catalog>::default());

		frame.fill_rectangle(Point::ORIGIN, bounds.size(), Color::TRANSPARENT);

		// NEW: Check for empty or invalid data
		let has_data =
			!self.values.is_empty() && self.values.iter().any(|v| *v > 0.0);
		if !has_data {
			//  Option 1: Display fallback text
			frame.fill_text(Text {
				content: "No data available".to_string(),
				position: Point::new(
					bounds.width / 2.0 - 60.0,
					bounds.height / 2.0,
				),
				color: style.text_color,
				size: 18.0.into(),
				font: self.font,
				..Default::default()
			});

			return vec![frame.into_geometry()];
		}

		// Drawing the bar and gridlines
		let chart_width = bounds.width - margins()[0] - margins()[1];
		let chart_height = bounds.height - margins()[0] - margins()[0];
		let origin = Point::new(margins()[0], bounds.height - margins()[0]);

		let max_value = self.max_value().max(1.0);
		let tick_height = chart_height / 10_f32;
		for i in 0..=10 {
			let y = (i as f32).mul_add(-tick_height, origin.y);
			let line = Path::line(
				Point::new(origin.x, y),
				Point::new(bounds.width - margins()[1], y),
			);
			frame.stroke(
				&line,
				iced::widget::canvas::Stroke {
					style: Style::Solid(style.text_color),
					width: 1.0,
					..Default::default()
				},
			);

			let label_value = (max_value / f64::from(10)) * f64::from(i);
			frame.fill_text(Text {
				content: format!("{label_value:.0}"),
				position: Point::new(5.0, y + 4.0),
				color: style.text_color,
				size: 12.0.into(),
				font: self.font,
				..Default::default()
			});
		}

		let x_axis = path_line((
			origin,
			Point::new(bounds.width - margins()[1], origin.y),
		));
		let y_axis = path_line((origin, Point::new(origin.x, margins()[0])));
		frame.stroke(&x_axis, iced::widget::canvas::Stroke::default());
		frame.stroke(&y_axis, iced::widget::canvas::Stroke::default());

		let bar_width = chart_width / self.values.len() as f32 * 0.8;
		let gap = chart_width / self.values.len() as f32 * 0.2;
		for (i, value) in self.values.iter().enumerate() {
			if *value <= 0.0 {
				continue;
			}
			let bar_height = ((*value / max_value) as f32) * chart_height;
			let x = (i as f32).mul_add(bar_width + gap, origin.x);
			let y = origin.y - bar_height;

			let bar = Path::rectangle(
				Point::new(x, y),
				Size::new(bar_width, bar_height),
			);
			frame.fill(&bar, style.primary);

			// Filling bottom label text
			if let Some(label) = self.labels.get(i) {
				frame.fill_text(Text {
					content: label.clone(),
					position: Point::new(x + bar_width / 12.0, origin.y + 15.0),
					color: style.text_color,
					size: 10.0.into(),
					font: self.font,
					..Default::default()
				});
			}
		}

		// Drawing the title into the canvas
		frame.fill_text(Text {
			content: self.title(),
			position: Point::new(
				(self.title.len() as f32).mul_add(-4.0, bounds.width / 2.0),
				2.0,
			),
			color: style.text_color,
			size: 24.0.into(),
			font: self.font,
			..Default::default()
		});

		if let Some(hovered) = state.hovered_bar {
			if let Some(value) = self.values.get(hovered) {
				frame.fill_text(Text {
					content: format!("{}: {:.2}", self.labels[hovered], value),
					position: Point::new(20.0, 5.0),
					color: style.text_color,
					size: 16.0.into(),
					font: self.font,
					..Default::default()
				});
			}
		}

		vec![frame.into_geometry()]
	}
}

fn path_line(points: (Point, Point)) -> Path {
	Path::line(points.0, points.1)
}

const fn margins() -> [f32; 2] {
	[40.0, 20.0]
}

pub fn histogram_chart<Theme: Catalog>(
	title: String,
	labels: Vec<String>,
	values: Vec<f64>,
	font: Font,
	size: (Length, Length),
) -> Canvas<HistogramChart, Message, Theme, Renderer> {
	Canvas::new(HistogramChart::new(title, labels, values, font))
		.width(size.0)
		.height(size.1)
}
