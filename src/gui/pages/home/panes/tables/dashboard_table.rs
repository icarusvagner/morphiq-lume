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
pub struct DashboardTable {
	pub columns: Vec<DashboardColumn>,
	pub rows: Vec<DashboardRow>,
	pub header: scrollable::Id,
	pub body: scrollable::Id,
	pub footer: scrollable::Id,
}

impl Default for DashboardTable {
	fn default() -> Self {
		Self {
			columns: vec![
				DashboardColumn::new(DashboardColumnKind::Id),
				DashboardColumn::new(DashboardColumnKind::FullName),
				DashboardColumn::new(DashboardColumnKind::Department),
				DashboardColumn::new(DashboardColumnKind::Interaction),
				DashboardColumn::new(DashboardColumnKind::WorkHours),
				DashboardColumn::new(DashboardColumnKind::Status),
			],
			rows: (0..50).map(|_| DashboardRow::generate_sample()).collect(),
			header: scrollable::Id::unique(),
			body: scrollable::Id::unique(),
			footer: scrollable::Id::unique(),
		}
	}
}

impl DashboardTable {
	pub fn new(rows: Vec<DashboardRow>) -> Self {
		Self { rows, ..Default::default() }
	}
}

#[derive(Debug, Clone)]
pub struct DashboardColumn {
	pub kind: DashboardColumnKind,
	pub width: f32,
	pub resize_offset: Option<f32>,
}

impl DashboardColumn {
	pub const fn new(kind: DashboardColumnKind) -> Self {
		let width = match kind {
			DashboardColumnKind::Id => 480.0,
			DashboardColumnKind::FullName => 250.0,
			DashboardColumnKind::Department => 300.0,
			DashboardColumnKind::Interaction => 200.0,
			DashboardColumnKind::WorkHours => 180.0,
			DashboardColumnKind::Status => 150.0,
		};

		Self { kind, width, resize_offset: None }
	}
}

#[derive(Debug, Clone)]
pub enum DashboardColumnKind {
	Id,
	FullName,
	Department,
	Interaction,
	WorkHours,
	Status,
}

#[derive(Debug, Clone)]
pub struct DashboardRow {
	pub id: String,
	pub full_name: String,
	pub department: String,
	pub interaction: String,
	pub work_hours: String,
	pub status: String,
}

impl DashboardRow {
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
			department: job::descriptor(),
			interaction: interaction[rand_num].clone(),
			work_hours: format!("{rand_hours} HRS"),
			status: statuses[rand_num_2].clone(),
		}
	}
}

impl<'a> table::Column<'a, Message, StyleType, Renderer> for DashboardColumn {
	type Row = DashboardRow;

	fn header(&'a self, _: usize) -> Element<'a, Message, StyleType, Renderer> {
		let content = match self.kind {
			DashboardColumnKind::Id => "Employee ID",
			DashboardColumnKind::FullName => "Fullname",
			DashboardColumnKind::Department => "Department",
			DashboardColumnKind::Interaction => "Clocks",
			DashboardColumnKind::WorkHours => "Work Hours",
			DashboardColumnKind::Status => "Status",
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
		row: &'a DashboardRow,
	) -> Element<'a, Message, StyleType, Renderer> {
		let content: Element<'_, Message, StyleType> = match self.kind {
			DashboardColumnKind::Id => text(row.id.clone()).into(),
			DashboardColumnKind::FullName => text(row.full_name.clone()).into(),
			DashboardColumnKind::Department => {
				text(row.department.clone()).into()
			}
			DashboardColumnKind::Interaction => {
				text(row.interaction.clone()).into()
			}
			DashboardColumnKind::WorkHours => {
				text(format!("{} HR(S)", row.work_hours)).into()
			}
			DashboardColumnKind::Status => {
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
