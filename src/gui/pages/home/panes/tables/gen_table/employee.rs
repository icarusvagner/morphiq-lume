use iced::{
	Element,
	Length,
	widget::{
		Column,
		container,
		text,
	},
};
use iced_aw::{
	Grid,
	GridRow,
};

use crate::gui::{
	styles::{
		container::ContainerType,
		types::style_type::StyleType,
	},
	types::message::Message,
};

#[derive(Debug, Clone)]
pub struct GenTable {
	title: String,
	header: Vec<String>,
	body: Vec<RowTable>,
}

#[derive(Default, Debug, Clone)]
pub struct RowTable {
	pub id_num: String,
	pub full_name: String,
	pub position: String,
	pub department: String,
	pub interaction: String,
	pub work_hours: String,
	pub status: String,
}

impl RowTable {
	pub fn new(
		id_num: String,
		full_name: String,
		position: String,
		department: String,
		interaction: String,
		work_hours: String,
		status: String,
	) -> Self {
		Self {
			id_num,
			full_name,
			position,
			department,
			interaction,
			work_hours,
			status: status.to_uppercase(),
		}
	}
}

impl GenTable {
	fn headers(&self) -> GridRow<'_, Message> {
		let mut grid_row = GridRow::new();
		for i in 0..self.header.len() {
			grid_row = grid_row.push(text(self.header[i].clone()));
		}

		grid_row
	}

	fn bodies(&self) -> Grid<'_, Message> {
		let mut content = Grid::new()
			.width(Length::Fill)
			.row_spacing(5.0)
			.row_height(12.0)
			.column_width(Length::Fill);
		for v in &self.body {
			let mut body_content = GridRow::new();
			body_content = body_content
				.push(text(v.id_num.clone()))
				.push(text(v.full_name.clone()))
				.push(text(v.position.clone()))
				.push(text(v.department.clone()))
				.push(text(v.interaction.clone()))
				.push(text(v.work_hours.clone()))
				.push(text(v.status.clone()));
			content = content.push(body_content);
		}

		content
	}

	fn title(&self) -> String {
		self.title.clone()
	}

	pub const fn new(
		title: String,
		header: Vec<String>,
		body: Vec<RowTable>,
	) -> Self {
		Self { title, header, body }
	}

	pub fn view(&self) -> Element<'_, Message, StyleType> {
		let content = Column::new()
			.push(text(self.title()))
			.push(self.bodies())
			.spacing(15.0)
			.padding(5.0);

		container(content).class(ContainerType::Base300).into()
	}
}
