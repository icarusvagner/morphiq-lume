use iced::window;

use crate::{
	chart::ChartMessage,
	gui::{
		pages::{
			OpenPage,
			home::HomeMessage,
			login::LoginMessage,
		},
		styles::types::style_type::StyleType,
		types::tables::TableMessage,
	},
};

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
	Login(LoginMessage),
	Home(HomeMessage),
	ChangePage(OpenPage),

	/// Messages for Charts
	Chart(ChartMessage),

	/// Tables messages
	Tables(TableMessage),
}
