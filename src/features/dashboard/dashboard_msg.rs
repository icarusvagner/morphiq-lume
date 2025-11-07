use iced::widget::scrollable;

use crate::gui::types::tables::FilterEmployee;

#[derive(Debug, Clone)]
pub enum DashboardMsg {
	Search(String),
	Table(DashboardTableMsg),
}

#[derive(Debug, Clone)]
pub enum DashboardTableMsg {
	Search(String),
	FilteredBy(FilterEmployee),
	TableSyncHeader(scrollable::AbsoluteOffset),
	TableResizing(usize, f32),
	TableResized,
}
