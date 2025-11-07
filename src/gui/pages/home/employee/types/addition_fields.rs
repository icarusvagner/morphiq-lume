use iced_aw::date_picker::Date;

use crate::{
	countries::types::nationalities::Nationality, utils::types::{blood_type::BloodType, marital_status::MaritalStatus}
};

#[derive(Debug, Clone, Default)]
pub struct AdditionalFields {
	pub work: WorkField,
	pub details: PersonalDetailsField,
}

#[derive(Debug, Clone, Default)]
pub struct WorkField {
	pub department: String,
	pub job_title: String,
	pub location: String,
	pub reporting_to: String,
	pub source_of_hired: String,
	pub pay_rate: u32,
}

#[derive(Debug, Clone, Default)]
pub struct PersonalDetailsField {
	pub blood_type: BloodType,
	pub spouse_name: Option<String>,
	pub father_name: String,
	pub mother_name: String,
	pub mobile: String,
	pub other_email: String,
	pub date_of_birth: Date,
	pub nationality: Nationality,
	pub marital_status: MaritalStatus,
	pub driving_license_code: String,
	pub address: AddressFields,
}

#[derive(Debug, Clone, Default)]
pub struct AddressFields {
	pub address_1: String,
	pub address_2: String,
	pub city: String,
	pub province: String,
	pub postal_code: String,
	pub biography: String,
}
