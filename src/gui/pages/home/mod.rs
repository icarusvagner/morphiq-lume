use iced::{
    widget::{container, Column, Row},
    Element,
};

use crate::gui::{
    components::{
        header::{Header, HeaderMessage},
        sidebar::SidebarMenu,
    },
    pages::home::{
        attendance::AttendanceView, dashboard::DashboardView, documents::DocumentsView,
        edit_profile::EditProfileView, employee::EmployeeView, leaves::LeavesView,
        payroll::PayrollView, settings::SettingsView,
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

#[allow(clippy::enum_variant_names)]
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq, Copy, Default)]
pub enum ContentView {
    #[default]
    Dashboard,
    Employee,
    Attendance,
    Payroll,
    Leaves,
    Documents,
    Settings,
    EditProfile,
}

#[allow(clippy::enum_variant_names)]
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, Default)]
pub struct Home {
    pub sidebar: SidebarMenu,
    pub header: Header,
    pub content: ContentView,
}

#[allow(clippy::enum_variant_names)]
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug)]
pub enum HomeMessage {
    Header(HeaderMessage),
    Content(ContentView),
    Logout,
}

impl Home {
    pub(crate) fn update(&mut self, _message: HomeMessage) {}
    pub(crate) fn view<'a>(&'a self) -> Element<'a, Message, StyleType> {
        let view = Row::new()
            .push(self.sidebar.view())
            .push(self.to_view(self.content))
            .spacing(5);
        let content = Column::new().push(self.header.view()).push(view).spacing(5);

        container(content).into()
    }

    pub(crate) fn to_view<'a>(
        &'a self,
        content_view: ContentView,
    ) -> Element<'a, Message, StyleType> {
        match content_view {
            ContentView::Dashboard => DashboardView::view(),
            ContentView::Employee => EmployeeView::view(),
            ContentView::Attendance => AttendanceView::view(),
            ContentView::Payroll => PayrollView::view(),
            ContentView::Leaves => LeavesView::view(),
            ContentView::Documents => DocumentsView::view(),
            ContentView::Settings => SettingsView::view(),
            ContentView::EditProfile => EditProfileView::view(),
        }
    }
}
