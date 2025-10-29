use duckdb::Connection;
use iced::{
	Element,
	Task,
	widget::{
		container,
		scrollable,
	},
	window,
};

use crate::{
	configs::config::Configs,
	crates::crate_core::model::store::{
		self,
		Db,
		new_db_pool,
	},
	error::{
		Error,
		Result,
	},
	gui::{
		pages::{
			OpenPage,
			Pages,
			home::HomeMessage,
		},
		styles::types::style_type::StyleType,
		types::{
			message::Message,
			tables::TableMessage,
		},
	},
};

pub const ICON_FONT_FAMILY_NAME: &str = "Icons for Morphiq Lume";
pub const FONT_FAMILY_NAME: &str = "Outfit";
pub const SVG_FULLLOGO_BYTES: &[u8] =
	include_bytes!("../../assets/logos/icons/full/icon_full.svg");
pub const SVG_EMBLEMLOGO_BYTES: &[u8] =
	include_bytes!("../../assets/logos/icons/icon_macros.svg");

#[derive(Debug)]
pub struct Morphiq {
	/// Window ID
	pub id: Option<window::Id>,
	/// Application's configurations: settings and more to come
	pub configs: Configs,
	/// The default page to view
	pub page: Pages,
	/// Change the running page
	pub open_page: OpenPage,
	/// Postgresql database state
	pub pg_pool: Option<store::Db>,
	pub duck_pool: Option<Connection>,
}

impl Morphiq {
	pub fn new(configs: Configs) -> Self {
		Self {
			configs,
			id: None,
			page: Pages::default(),
			open_page: OpenPage::default(),
			pg_pool: None,
			duck_pool: None,
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
				if let Some(callback_msg) = self.page.login.update(login_msg) {
					return self.update(callback_msg);
				}
			}
			Message::ChangePage(open_page) => self.open_page = open_page,
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
			Message::Tables(tbl_msg) => match tbl_msg {
				TableMessage::Dashboard(dashboard_msg) => {
					self.page.home.dashboard.table.update(dashboard_msg);
				}
				TableMessage::Employee(employee_msg) => {
					self.page.home.employee.table.update(employee_msg);
				}
			},
			Message::DashboardTableSyncHeader(offset) => {
				return Task::batch(vec![
					scrollable::scroll_to(
						self.page.home.dashboard.table.table.header.clone(),
						offset,
					),
					scrollable::scroll_to(
						self.page.home.dashboard.table.table.footer.clone(),
						offset,
					),
				]);
			}
			Message::DashboardTableResizing(index, offset) => {
				if let Some(column) =
					self.page.home.dashboard.table.table.columns.get_mut(index)
				{
					column.resize_offset = Some(offset);
				}
			}
			Message::DashboardTableResized => {
				self.page
					.home
					.dashboard
					.table
					.table
					.columns
					.iter_mut()
					.for_each(|column| {
						if let Some(offset) = column.resize_offset.take() {
							column.width += offset;
						}
					});
			}
			Message::EmployeeTableSyncHeader(offset) => {
				return Task::batch(vec![
					scrollable::scroll_to(
						self.page.home.employee.table.table.header.clone(),
						offset,
					),
					scrollable::scroll_to(
						self.page.home.dashboard.table.table.footer.clone(),
						offset,
					),
				]);
			}
			Message::EmployeeTableResizing(index, offset) => {
				if let Some(column) =
					self.page.home.employee.table.table.columns.get_mut(index)
				{
					column.resize_offset = Some(offset);
				}
			}
			Message::EmployeeTableResized => {
				self.page
					.home
					.employee
					.table
					.table
					.columns
					.iter_mut()
					.for_each(|column| {
						if let Some(offset) = column.resize_offset.take() {
							column.width += offset;
						}
					});
			}
			// Message::Chart(_) => {}
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

	async fn connect_db() -> Result<Db> {
		let db_pool = new_db_pool()
			.await
			.map_err(|ex| Error::DbInitFailed(ex.to_string()))?;

		Ok(db_pool)
	}

	async fn connect_duckdb() -> Result<()> {
		todo!()
	}
}
