use iced::{
	Element,
	Task,
	widget::container,
	window,
};

use crate::{
	configs::config::Configs,
	gui::{
		pages::{
			OpenPage,
			Pages,
			home::HomeMessage,
			login::LoginMessage,
		},
		styles::types::style_type::StyleType,
		types::{
			message::Message,
			tables::DashboardTableMsg,
		},
	},
};

pub const ICON_FONT_FAMILY_NAME: &str = "Icons for Morphiq Lume";
pub const FONT_FAMILY_NAME: &str = "Outfit";
pub const SVG_FULLLOGO_BYTES: &[u8] =
	include_bytes!("../../assets/logos/icons/full/icon_full.svg");
pub const SVG_EMBLEMLOGO_BYTES: &[u8] =
	include_bytes!("../../assets/logos/icons/icon_macros.svg");

#[derive(Debug, Clone)]
pub struct Morphiq {
	/// Window ID
	pub id: Option<window::Id>,
	/// Application's configurations: settings and more to come
	pub configs: Configs,
	/// The default page to view
	pub page: Pages,
	/// Change the running page
	pub open_page: OpenPage,
}

impl Morphiq {
	pub fn new(configs: Configs) -> Self {
		Self {
			configs,
			id: None,
			page: Pages::default(),
			open_page: OpenPage::default(),
		}
	}

	pub const fn theme(&self) -> StyleType {
		self.configs.settings.style
	}

	pub fn update(&mut self, message: Message) -> Task<Message> {
		match message {
			Message::StartApp(id) => {
				// self.configs.settings.style = StyleType::Dark;
				self.id = id;

				// Performs some task
			}
			Message::Style(style) => {
				self.configs.settings.style = style;
			}
			// Message::TickDashboard => {}
			// Message::CloseSettings => {}
			// Message::ChangeVolume(_) => {}
			// Message::FetchChats => {}
			Message::Quit => {
				let _ = self.configs.clone().store();
				return window::close(
					self.id.unwrap_or_else(window::Id::unique),
				);
			}
			Message::Login(login_msg) => {
				if matches!(login_msg, LoginMessage::LoginSubmitted) {
					self.open_page = OpenPage::Home;
				} else {
					self.page.login.update(login_msg);
				}
			}
			Message::Home(home_msg) => {
				if matches!(home_msg, HomeMessage::Logout) {
					self.open_page = OpenPage::Login;
				} else {
					self.page.home.update(home_msg);
				}
			}
			Message::ChangeTheme => {
				if self.configs.settings.style == StyleType::Light {
					self.configs.settings.style = StyleType::Dark;
				} else {
					self.configs.settings.style = StyleType::Light;
				}
			}
			Message::Tables(tbl_msg) => {
				if matches!(
					tbl_msg,
					crate::gui::types::tables::TableMessage::Dashboard(
						DashboardTableMsg::SyncHeader(_)
					)
				) {
					self.page.home.dashboard.view();
				}
			}
			Message::Chart(_) => {}
			_ => {}
		}
		Task::none()
	}

	pub fn view(&self) -> Element<'_, Message, StyleType> {
		let content = match self.open_page {
			OpenPage::Login => self.page.login.view(),
			OpenPage::Home => self.page.home.view(self),
		};

		container(content).padding(12.0).into()
	}

	pub const fn scale_factor(&self) -> f64 {
		self.configs.settings.scale_factor
	}
}
