use crate::core::utils::messages::chart::bar::BarMessage;

pub mod bar;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum ChartMessage {
	Bar(BarMessage),
}
