use iced::widget::scrollable;

#[derive(Debug, Clone)]
pub enum TableMessage {
	Dashboard(DashboardTableMsg),
}

#[derive(Debug, Clone)]
pub enum DashboardTableMsg {
	SyncHeader(scrollable::AbsoluteOffset),
	Resizing(usize, f32),
	SelectEmployee(String),
	Resized,
}

#[derive(Default, Debug)]
pub struct DashboardState {
	pub selected_employee: Option<String>,
	pub search_query: String,
	pub total_row: usize,
}

impl DashboardState {
	pub fn select_employee(&mut self, id: String) {
		self.selected_employee = Some(id);
	}

	pub fn clear_selection(&mut self) {
		self.selected_employee = None;
	}
}
