#[allow(clippy::enum_variant_names, clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq, Copy, Default)]
pub enum ContentView {
	#[default]
	Dashboard,
	Employee,
	AddEmployee,
	Attendance,
	Payroll,
	Leaves,
	Documents,
	Settings,
	Profile,
	EditProfile,
	Search,
	Events,
	EventsPosting,
	Notifications,
}
