use iced::widget::scrollable;

use crate::gui::{
	pages::home::employee::types::create_msg::CreateEmployeeMsg, types::tables::FilterEmployee
};

#[derive(Debug, Clone)]
pub enum EmployeeMsg {
	Create(CreateEmployeeMsg),
	Table(EmployeeTableMsg),
}

#[derive(Debug, Clone)]
pub enum EmployeeTableMsg {
	Search(String),
	FilteredBy(FilterEmployee),
	TableSyncHeader(scrollable::AbsoluteOffset),
	TableResizing(usize, f32),
	TableResized,
}
