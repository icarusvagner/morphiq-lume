use std::{
	cell::RefCell,
	rc::{
		Rc,
		Weak,
	},
};

use iced::{
	Element,
	Length,
	Renderer,
	widget::{
		button,
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
	types::{
		message::Message,
		tables::{
			DashboardState,
			DashboardTableMsg,
		},
	},
};

pub struct DashboardTable {
	columns: Vec<ColumnTable>,
	rows: Vec<RowTable>,
	header: scrollable::Id,
	body: scrollable::Id,
	footer: scrollable::Id,
	state: Rc<RefCell<DashboardState>>,
}

impl DashboardTable {
	pub fn new(state: Rc<RefCell<DashboardState>>) -> Self {
		Self {
			columns: vec![
				ColumnTable::new(ColumnKind::EmployeeId),
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
			state,
		}
	}
}

impl DashboardTable {
	pub fn update(&mut self, message: DashboardTableMsg) {
		match message {
			DashboardTableMsg::Resizing(index, offset) => {
				if let Some(column) = self.columns.get_mut(index) {
					column.resize_offset = Some(offset);
				}
			}
			DashboardTableMsg::Resized => {
				self.columns.iter_mut().for_each(|column| {
					if let Some(offset) = column.resize_offset.take() {
						column.width += offset;
					}
				});
			}
			DashboardTableMsg::SelectEmployee(id) => {
				let mut state = self.state.borrow_mut();
				state.select_employee(id);
			}
			_ => {}
		}
	}

	pub fn view(&self) -> Element<'_, Message> {
		let table = responsive(|size| {
			let mut table = table(
				self.header.clone(),
				self.body.clone(),
				&self.columns,
				&self.rows,
				DashboardTableMsg::SyncHeader,
			);
			table.on_column_resize(
				DashboardTableMsg::Resizing,
				DashboardTableMsg::Resized,
			);
			table.into()
		});

		container(table).class(ContainerType::Base200).into()
	}
}

pub enum ColumnKind {
	EmployeeId,
	Fullname,
	Department,
	Interaction,
	WorkHours,
	Status,
}

pub struct ColumnTable {
	kind: ColumnKind,
	width: f32,
	resize_offset: Option<f32>,
}

impl ColumnTable {
	const fn new(kind: ColumnKind) -> Self {
		let width = match kind {
			ColumnKind::Fullname => 125.0,
			ColumnKind::Department => 100.0,
			ColumnKind::Interaction => 120.0,
			ColumnKind::WorkHours => 40.0,
			_ => 60.0,
		};

		Self { kind, width, resize_offset: None }
	}
}

#[derive(Default)]
pub struct RowTable {
	id_num: String,
	full_name: String,
	department: String,
	interaction: String,
	work_hours: String,
	status: String,
	state: Option<Weak<RefCell<DashboardState>>>,
}

impl RowTable {
	pub fn with_state(
		id_num: &str,
		full_name: &str,
		department: &str,
		state: &Rc<RefCell<DashboardState>>,
	) -> Self {
		Self {
			id_num: id_num.to_string(),
			full_name: full_name.to_string(),
			department: department.to_string(),
			interaction: "Active".into(),
			work_hours: "5 HRS".into(),
			status: "Present".into(),
			state: Some(Rc::downgrade(state)),
		}
	}

	pub fn select(&self) {
		if let Some(state_rc) = self.state.as_ref().and_then(|c| c.upgrade()) {
			let mut state = state_rc.borrow_mut();
			state.select_employee(self.id_num.clone());
		}
	}
}

impl<'a> table::Column<'a, Message, StyleType, Renderer> for ColumnTable {
	type Row = RowTable;

	fn header(&'a self, _: usize) -> Element<'a, Message, StyleType> {
		let content = match self.kind {
			ColumnKind::EmployeeId => "ID Number",
			ColumnKind::Fullname => "Fullname",
			ColumnKind::Department => "Department",
			ColumnKind::Interaction => "Interaction",
			ColumnKind::WorkHours => "Working Hours",
			ColumnKind::Status => "Status",
		};

		container(text(content)).center_y(24).into()
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
			ColumnKind::Department => row.department.clone(),
			ColumnKind::Interaction => row.interaction.clone(),
			ColumnKind::WorkHours => row.work_hours.clone(),
			ColumnKind::Status => row.status.clone(),
		};

		let button = button(text(content)).on_press(Message::Tables(
			crate::gui::types::tables::TableMessage::Dashboard(
				DashboardTableMsg::SelectEmployee(row.id_num.clone()),
			),
		));

		container(button).width(Length::Fill).center_y(32).into()
	}

	fn width(&self) -> f32 {
		self.width
	}

	fn resize_offset(&self) -> Option<f32> {
		self.resize_offset
	}
}
