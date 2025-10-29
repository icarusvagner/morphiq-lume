#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone)]
pub enum CreateEmployeeMsg {
	InputFullnameChange(String),
	InputMiddlenameChange(String),
	InputLastnameChange(String),
	InputEmailAddressChange(String),
	SubmitInput,
	UploadPhoto,
}
