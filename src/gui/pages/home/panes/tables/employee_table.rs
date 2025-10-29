use iced::{
	Element,
	Length,
	Renderer,
	widget::{
		container,
		scrollable,
		text,
	},
};
use iced_table::table;
use mockd::{
	job,
	name,
	unique,
};
use rand::{
	Rng,
	rng,
};

use crate::{
	crates::crate_utils::b32::b32hexu_encode,
	gui::{
		styles::{
			container::ContainerType,
			style_constant::fonts::RALEWAY_BOLD,
			types::style_type::StyleType,
		},
		types::message::Message,
	},
};

#[derive(Debug, Clone)]
pub struct EmployeeTable {
	pub columns: Vec<EmployeeColumn>,
	pub rows: Vec<EmployeeRow>,
	pub header: scrollable::Id,
	pub body: scrollable::Id,
	pub footer: scrollable::Id,
}

impl Default for EmployeeTable {
	fn default() -> Self {
		Self {
			columns: vec![
				EmployeeColumn::new(EmployeeColumnKind::Id),
				EmployeeColumn::new(EmployeeColumnKind::FullName),
				EmployeeColumn::new(EmployeeColumnKind::Position),
				EmployeeColumn::new(EmployeeColumnKind::Department),
				EmployeeColumn::new(EmployeeColumnKind::Interaction),
				EmployeeColumn::new(EmployeeColumnKind::WorkHours),
				EmployeeColumn::new(EmployeeColumnKind::Status),
			],
			rows: (0..50).map(|_| EmployeeRow::generate_sample()).collect(),
			header: scrollable::Id::unique(),
			body: scrollable::Id::unique(),
			footer: scrollable::Id::unique(),
		}
	}
}

impl EmployeeTable {
	pub fn new(rows: Vec<EmployeeRow>) -> Self {
		Self { rows, ..Default::default() }
	}
}

#[derive(Debug, Clone)]
pub struct EmployeeColumn {
	pub kind: EmployeeColumnKind,
	pub width: f32,
	pub resize_offset: Option<f32>,
}

impl EmployeeColumn {
	pub const fn new(kind: EmployeeColumnKind) -> Self {
		let width = match kind {
			EmployeeColumnKind::Id => 340.0,
			EmployeeColumnKind::FullName => 250.0,
			EmployeeColumnKind::Position => 140.0,
			EmployeeColumnKind::Department => 300.0,
			EmployeeColumnKind::Interaction => 200.0,
			EmployeeColumnKind::WorkHours => 180.0,
			EmployeeColumnKind::Status => 150.0,
		};

		Self { kind, width, resize_offset: None }
	}
}

#[derive(Debug, Clone)]
pub enum EmployeeColumnKind {
	Id,
	FullName,
	Position,
	Department,
	Interaction,
	WorkHours,
	Status,
}

#[derive(Debug, Clone)]
pub struct EmployeeRow {
	pub id: String,
	pub full_name: String,
	pub position: String,
	pub department: String,
	pub interaction: String,
	pub work_hours: String,
	pub status: String,
}

impl EmployeeRow {
	pub fn generate_sample() -> Self {
		let interaction: [String; 2] =
			[String::from("Clock In"), String::from("Clock Out")];
		let rand_num = rng().random_range(0..=1);
		let rand_hours = rng().random_range(1..=10);

		let statuses: [String; 4] = [
			String::from("Active"),
			String::from("Inactive"),
			String::from("Late"),
			String::from("Onboarding"),
		];
		let rand_num_2 = rng().random_range(0..=3);

		Self {
			id: b32hexu_encode(unique::uuid_v4().as_str()),
			full_name: name::full(),
			position: job::title(),
			department: job::descriptor(),
			interaction: interaction[rand_num].clone(),
			work_hours: format!("{rand_hours} HRS"),
			status: statuses[rand_num_2].clone(),
		}
	}
}

impl<'a> table::Column<'a, Message, StyleType, Renderer> for EmployeeColumn {
	type Row = EmployeeRow;

	fn header(&'a self, _: usize) -> Element<'a, Message, StyleType, Renderer> {
		let content = match self.kind {
			EmployeeColumnKind::Id => "Employee ID",
			EmployeeColumnKind::FullName => "Fullname",
			EmployeeColumnKind::Position => "Position",
			EmployeeColumnKind::Department => "Department",
			EmployeeColumnKind::Interaction => "Clocks",
			EmployeeColumnKind::WorkHours => "Work Hours",
			EmployeeColumnKind::Status => "Status",
		};

		container(text(content).font(RALEWAY_BOLD))
			.class(ContainerType::Ghost)
			.center_y(34)
			.into()
	}

	fn cell(
		&'a self,
		_col_index: usize,
		_row_index: usize,
		row: &'a EmployeeRow,
	) -> Element<'a, Message, StyleType, Renderer> {
		let content: Element<'_, Message, StyleType> = match self.kind {
			EmployeeColumnKind::Id => text(row.id.clone()).into(),
			EmployeeColumnKind::FullName => text(row.full_name.clone()).into(),
			EmployeeColumnKind::Position => text(row.position.clone()).into(),
			EmployeeColumnKind::Department => {
				text(row.department.clone()).into()
			}
			EmployeeColumnKind::Interaction => {
				text(row.interaction.clone()).into()
			}
			EmployeeColumnKind::WorkHours => {
				text(format!("{} HR(S)", row.work_hours)).into()
			}
			EmployeeColumnKind::Status => {
				text(row.status.to_uppercase()).into()
			}
		};

		container(content)
			.width(Length::Fill)
			.center_y(32)
			.class(ContainerType::Ghost)
			.into()
	}

	fn footer(
		&'a self,
		_col_index: usize,
		_rows: &'a [Self::Row],
	) -> Option<Element<'a, Message, StyleType, Renderer>> {
		None
	}

	fn width(&self) -> f32 {
		self.width
	}

	fn resize_offset(&self) -> Option<f32> {
		self.resize_offset
	}
}
