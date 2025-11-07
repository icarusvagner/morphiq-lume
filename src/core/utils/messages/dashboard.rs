use iced::widget::scrollable;

use crate::core::utils::filters::FilterEmployee;

#[derive(Debug, Clone)]
pub enum DashboardMsg {
	Search(String),

	/// Dashboard table messages
	FilteredBy(FilterEmployee),
	TableSyncHeader(scrollable::AbsoluteOffset),
	TableResizing(usize, f32),
	TableResized,
}
