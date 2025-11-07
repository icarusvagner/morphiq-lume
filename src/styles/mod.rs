pub mod bar;
pub mod button;
pub(super) mod color_remote;
pub mod container;
pub mod datepicker;
pub mod donut;
pub mod menu;
pub mod palette;
pub mod pick_list;
pub mod rule;
pub mod scrollable;
pub mod style_type;
pub mod svg;
pub mod table;
pub mod text;
pub mod text_input;

// border styles

#[allow(clippy::enum_variant_names, clippy::large_enum_variant)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StandardNames {
	Base100,
	Base200,
	Base300,
	Primary,
	Secondary,
	Accent,
	Neutral,
	Info,
	Success,
	Warning,
	Error,
}
