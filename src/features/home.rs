#[allow(clippy::enum_variant_names, clippy::large_enum_variant)]
#[derive(Debug, Clone, Default)]
pub struct Home {
	pub sidebar: SidebarMenu,
	pub header: Header,
	pub content: ContentView,
	pub settings: SettingsView,
	pub dashboard: DashboardView,
	pub employee: EmployeeView,
}

impl Home {
	pub fn update(&mut self, message: HomeMessage) -> Task<HomeMessage> {
		match message {
			HomeMessage::Content(view) => {
				self.content = view;
				Task::none()
			}
			HomeMessage::Employee(employee_msg) => {
				self.employee.update(employee_msg).map(HomeMessage::Employee)
			}
			HomeMessage::Dashboard(db_msg) => {
				self.dashboard.update(db_msg).map(HomeMessage::Dashboard)
			}
			HomeMessage::Logout => {
				// Maybe trigger cleanup or navigation
				Task::none()
			}
			HomeMessage::Header(_) => Task::none(),
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
						.width(2.0)
						.margin(2.0)
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
			ContentView::AddEmployee => self.employee.create.view(),
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
