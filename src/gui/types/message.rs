use iced::window;

use crate::gui::{
    pages::{home::HomeMessage, login::LoginMessage},
    styles::types::style_type::StyleType,
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
    /// Deserialize a style from a path
    LoadStyle(String),
    /// Hides the current settings view
    CloseSettings,
    /// Set notification volume
    ChangeVolume(u8),
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
}
