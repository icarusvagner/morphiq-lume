#[allow(clippy::enum_variant_names, clippy::large_enum_variant, dead_code)]
#[derive(Debug, Clone)]
pub enum HeaderMessage {
	Dashboard,
	Search,
	AddEmployee,
	EventPosting,
	OrgSettings,
	Language,
	Chat,
	Notifications,
	Profile,
	Candidates,
}
