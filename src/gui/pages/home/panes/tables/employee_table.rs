use iced::{
	Element,
	Length,
	Renderer,
	alignment::{
		Horizontal,
		Vertical,
	},
	widget::{
		container,
		responsive,
		scrollable,
		text,
	},
};
use iced_table::table;

use crate::gui::{
	styles::{
		container::ContainerType,
		types::style_type::StyleType,
	},
	types::message::Message,
};

#[derive(Debug, Clone)]
pub struct EmployeeTable {
	pub columns: Vec<ColumnTable>,
	pub rows: Vec<RowTable>,
	pub header: scrollable::Id,
	pub body: scrollable::Id,
	pub footer: scrollable::Id,
}

impl Default for EmployeeTable {
	fn default() -> Self {
		Self {
			columns: vec![
				ColumnTable::new(ColumnKind::default()),
				ColumnTable::new(ColumnKind::Fullname),
				ColumnTable::new(ColumnKind::Department),
				ColumnTable::new(ColumnKind::Interaction),
				ColumnTable::new(ColumnKind::WorkHours),
				ColumnTable::new(ColumnKind::Status),
			],
			rows: Vec::default(),
			header: scrollable::Id::unique(),
			body: scrollable::Id::unique(),
			footer: scrollable::Id::unique(),
		}
	}
}

impl EmployeeTable {
	pub fn view(&self) -> Element<'_, Message, StyleType> {
		let table = responsive(|_| {
			let mut table = table(
				self.header.clone(),
				self.body.clone(),
				&self.columns,
				&self.rows,
				Message::SyncHeader,
			);
			table = table.on_column_resize(Message::Resizing, Message::Resized);
			table.into()
		});

		container(
			container(table)
				.padding(15.0)
				.width(Length::Fill)
				.height(Length::Fill)
				.class(ContainerType::Base300),
		)
		.width(Length::Fill)
		.height(Length::Fixed(600.0))
		.center_x(Length::Fill)
		.into()
	}
}

#[derive(Debug, Clone, Default)]
pub enum ColumnKind {
	#[default]
	EmployeeId,
	Fullname,
	Position,
	Department,
	Interaction,
	WorkHours,
	Status,
}

#[derive(Debug, Clone, Default)]
pub struct ColumnTable {
	pub kind: ColumnKind,
	pub width: f32,
	pub resize_offset: Option<f32>,
}

impl ColumnTable {
	const fn new(kind: ColumnKind) -> Self {
		let width = match kind {
			ColumnKind::EmployeeId => 100.0,
			ColumnKind::Fullname => 400.0,
			ColumnKind::Department | ColumnKind::Position => 205.0,
			ColumnKind::Interaction => 420.0,
			ColumnKind::WorkHours => 135.0,
			ColumnKind::Status => 90.0,
		};

		Self { kind, width, resize_offset: None }
	}
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

impl<'a> table::Column<'a, Message, StyleType, Renderer> for ColumnTable {
	type Row = RowTable;

	fn header(&'a self, _: usize) -> Element<'a, Message, StyleType> {
		let content = match self.kind {
			ColumnKind::EmployeeId => "ID Number",
			ColumnKind::Fullname => "Fullname",
			ColumnKind::Position => "Position",
			ColumnKind::Department => "Department",
			ColumnKind::Interaction => "Interaction",
			ColumnKind::WorkHours => "Working Hours",
			ColumnKind::Status => "Status",
		};

		container(
			text(content).align_x(Horizontal::Center).align_y(Vertical::Center),
		)
		.center_y(24)
		.into()
	}

	fn cell(
		&'a self,
		_: usize,
		_: usize,
		row: &'a RowTable,
	) -> Element<'a, Message, StyleType> {
		let content = match self.kind {
			ColumnKind::EmployeeId => row.id_num.clone(),
			ColumnKind::Fullname => row.full_name.clone(),
			ColumnKind::Position => row.position.clone(),
			ColumnKind::Department => row.department.clone(),
			ColumnKind::Interaction => row.interaction.clone(),
			ColumnKind::WorkHours => row.work_hours.clone(),
			ColumnKind::Status => row.status.clone(),
		};

		container(text(content)).width(Length::Fill).center_y(32).into()
	}

	fn width(&self) -> f32 {
		self.width
	}

	fn resize_offset(&self) -> Option<f32> {
		self.resize_offset
	}
}
