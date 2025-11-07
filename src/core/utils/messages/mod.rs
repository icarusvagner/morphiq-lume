use iced::window;

use crate::styles::style_type::StyleType;

pub mod dashboard;
pub mod employee;
pub mod header;
pub mod home;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum Message {
	/// Run task to initialize app
	StartApp(Option<window::Id>),
	/// Sent by the backend to get the data to display
	TickDashboard,
	/// Change application Style
	Style(StyleType),
	/// Hides the current settings view
	CloseSettings,
	/// Set notification volume
	ChangeVolume(u8),
	/// Change the appearance of the system
	ChangeTheme,
	/// Save the configuration of the app and quit
	Quit,
	/// Start with some dashboard data
	Start,
	/// Fetch mock chat data
	FetchChats,

	// Messages for views
	/// Login view message
	Login(AuthMessage),
	Home(HomeMessage),
	ChangePage(OpenPage),

	/// Messages for Charts
	Chart(ChartMessage),
}
