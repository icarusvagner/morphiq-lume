use crate::{
	core::utils::messages::{
		dashboard::DashboardMsg, employee::EmployeeMsg, header::HeaderMessage
	}, features::contentview::ContentView
};

#[allow(clippy::enum_variant_names, clippy::large_enum_variant)]
#[derive(Clone, Debug)]
pub enum HomeMessage {
	Header(HeaderMessage),
	Content(ContentView),
	Logout,
	Employee(EmployeeMsg),
	Dashboard(DashboardMsg),
}
