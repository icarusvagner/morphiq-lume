use iced::widget::scrollable;

#[derive(Debug, Clone)]
pub enum TableMessage {
	Dashboard(DashboardTableMsg),
	Employee(EmployeeTableMsg),
}

#[derive(Debug, Clone)]
pub enum DashboardTableMsg {
	SyncHeader(scrollable::AbsoluteOffset),
	Resizing(usize, f32),
	Resized,
}

#[derive(Debug, Clone)]
pub enum EmployeeTableMsg {
	SyncHeader(scrollable::AbsoluteOffset),
	Resizing(usize, f32),
	Resized,
}
