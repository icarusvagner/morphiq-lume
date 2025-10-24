use iced::{
	Element,
	Length,
	Padding,
	widget::{
		Column,
		Row,
		Scrollable,
		container,
		scrollable,
	},
};

use crate::gui::{
	components::{
		header::{
			Header,
			HeaderMessage,
		},
		sidebar::SidebarMenu,
	},
	morphiq::Morphiq,
	pages::home::{
		attendance::AttendanceView,
		dashboard::DashboardView,
		documents::DocumentsView,
		edit_profile::EditProfileView,
		employee::EmployeeView,
		leaves::LeavesView,
		payroll::PayrollView,
	},
	styles::types::style_type::StyleType,
	types::message::Message,
};

mod attendance;
mod dashboard;
mod documents;
mod edit_profile;
mod employee;
mod leaves;
mod panes;
mod payroll;
mod settings;

pub use settings::{
	OpenSettings,
	SettingsView,
};

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
	Settings(OpenSettings),
	Profile,
	EditProfile,
	Search,
	Events,
	EventsPosting,
	Notifications,
}

#[allow(clippy::enum_variant_names, clippy::large_enum_variant)]
#[derive(Debug, Clone, Default)]
pub struct Home {
	pub sidebar: SidebarMenu,
	pub header: Header,
	pub content: ContentView,
	pub settings: SettingsView,
	pub dashboard: DashboardView,
	employee: EmployeeView,
}

#[allow(clippy::enum_variant_names, clippy::large_enum_variant)]
#[derive(Clone, Debug)]
pub enum HomeMessage {
	Header(HeaderMessage),
	Content(ContentView),
	Logout,
}

impl Home {
	pub const fn update(&mut self, message: HomeMessage) {
		match message {
			HomeMessage::Header(header_msg) => self.header.update(header_msg),
			HomeMessage::Content(view) => self.content = view,
			HomeMessage::Logout => {}
		}
	}

	pub fn view<'a>(
		&'a self,
		morphiq: &Morphiq,
	) -> Element<'a, Message, StyleType> {
		let scrollable_content =
			Scrollable::new(self.to_view(self.content, morphiq))
				.direction(scrollable::Direction::Vertical(
					scrollable::Scrollbar::new()
						.width(0.0)
						.margin(0.0)
						.scroller_width(3.0)
						.anchor(scrollable::Anchor::Start),
				))
				.width(Length::Fill)
				.height(Length::Fill);

		let view = Row::new()
			.push(self.sidebar.view(morphiq))
			.push(scrollable_content)
			.spacing(5);

		let content =
			Column::new().push(self.header.view(morphiq)).push(view).spacing(5);

		container(content).into()
	}

	pub fn to_view<'a>(
		&'a self,
		content_view: ContentView,
		morphiq: &Morphiq,
	) -> Element<'a, Message, StyleType> {
		container(match content_view {
			ContentView::Employee => self.employee.view(morphiq),
			ContentView::Attendance => AttendanceView::view(),
			ContentView::Payroll => PayrollView::view(),
			ContentView::Leaves => LeavesView::view(),
			ContentView::Documents => DocumentsView::view(),
			ContentView::Settings(settings_view) => {
				self.settings.view(settings_view, morphiq)
			}
			ContentView::EditProfile => EditProfileView::view(),
			_ => self.dashboard.view(morphiq),
		})
		.padding(Padding::ZERO.left(15).right(15).bottom(5))
		.into()
	}
}
