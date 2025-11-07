#[allow(clippy::enum_variant_names)]
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum AuthMessage {
	InputFieldChange(String, String),
	LoginSubmitted,
	ShowPassword,
}