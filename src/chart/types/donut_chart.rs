use std::f32::consts;

use iced::{
	Font,
	Radians,
	Renderer,
	alignment::{
		Horizontal,
		Vertical,
	},
	widget::canvas::{
		self,
		Frame,
		Text,
		path::Arc,
	},
};

use crate::gui::{
	styles::{
		donut::Catalog,
		style_constant::fonts::FONT_SIZE_SUBTITLE,
	},
	types::message::Message,
};

#[derive(Clone, Debug)]
pub struct DonutChart {
	title: String,
	labels: Vec<String>,
	values: Vec<u32>,
	font: Font,
}

impl DonutChart {
	const fn new(
		title: String,
		font: Font,
		labels: Vec<String>,
		values: Vec<u32>,
	) -> Self {
		Self { title, labels, values, font }
	}

	fn total(&self) -> u32 {
		self.values.iter().sum::<u32>()
	}

	fn title(&self) -> String {
		let total = self.total();
		format!("{total} ({})", self.title)
	}

	fn angles(&self) -> Vec<(Radians, Radians)> {
		#[allow(clippy::cast_precision_loss)]
		let total = self.total() as f32;
		let mut start_angle = Radians(-consts::FRAC_2_PI);
		self.values
			.iter()
			.map(|&value| {
				#[allow(clippy::cast_precision_loss)]
				let proportion = value as f32 / total;
				let sweep = Radians(consts::TAU * proportion);
				let start = start_angle;
				let end = start + sweep;
				start_angle = end;
				(start, end)
			})
			.collect()
	}
}

impl<Message, Theme: Catalog> canvas::Program<Message, Theme> for DonutChart {
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
		let center = frame.center();
		let radius = (frame.width().min(frame.height()) / 2.0) * 0.9;

		let style =
			<Theme as Catalog>::style(theme, &<Theme as Catalog>::default());
		let colors = [style.primary, style.secondary, style.accent];

		for ((start_angle, end_angle), color) in
			self.angles().into_iter().zip(colors)
		{
			let path = canvas::Path::new(|builder| {
				builder.arc(Arc { center, radius, start_angle, end_angle });
				builder.line_to(center);
				builder.close();
			});
			frame.fill(&path, color);
		}

		let inner_circle = canvas::Path::circle(center, radius - 6.0);
		frame.fill(&inner_circle, style.background);
		frame.fill_text(Text {
			content: self.title().clone(),
			position: center,
			vertical_alignment: Vertical::Center,
			horizontal_alignment: Horizontal::Center,
			color: style.text_color,
			size: FONT_SIZE_SUBTITLE,
			font: self.font,
			..Default::default()
		});

		vec![frame.into_geometry()]
	}
}
