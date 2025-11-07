use iced::widget::scrollable;

use crate::gui::{
	pages::home::employee::types::create_msg::CreateEmployeeMsg, types::tables::FilterEmployee
};

#[derive(Debug, Clone)]
pub enum EmployeeMsg {
	Table(EmployeeTableMsg),

	/// Inputs state
	InputFullnameChange(String),
	InputMiddlenameChange(String),
	InputLastnameChange(String),
	InputEmailAddressChange(String),
	SubmitInput,
	UploadPhoto,

	/// Datepicker state
	ChooseDate,
	SubmitDate(Date),
	CancelDate,

	/// Pick list state
	EmployeeTypeSelected(EmployeeType),
	EmployeeStatusSelected(EmployeeStatus),

	/// Employee Table message
	Search(String),
	FilteredBy(FilterEmployee),
	TableSyncHeader(scrollable::AbsoluteOffset),
	TableResizing(usize, f32),
	TableResized,
}

