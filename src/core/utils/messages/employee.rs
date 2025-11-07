use iced::widget::scrollable;
use iced_aw::date_picker::Date;

use crate::{
	core::utils::filters::FilterEmployee, features::employees::employee_type::{EmployeeStatus, EmployeeType}
};

#[derive(Debug, Clone)]
pub enum EmployeeMsg {
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
