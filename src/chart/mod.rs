use crate::chart::types::bar_chart::BarMessage;

// pub mod manage_chart_data;
pub mod types;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum ChartMessage {
	Bar(BarMessage),
}
