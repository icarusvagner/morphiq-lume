use iced::Font;

#[derive(Clone, Debug)]
pub struct BarChart {
	pub title: String,
	pub labels: Vec<String>,
	pub values: Vec<f64>,
	pub font: Font,
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
}
