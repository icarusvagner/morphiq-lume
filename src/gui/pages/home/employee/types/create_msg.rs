use iced_aw::date_picker::Date;

use crate::gui::pages::home::employee::types::employee_type::{
	EmployeeStatus, EmployeeType
};

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone)]
pub enum CreateEmployeeMsg {
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
}
