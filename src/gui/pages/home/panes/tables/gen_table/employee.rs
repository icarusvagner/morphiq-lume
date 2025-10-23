use iced::{
	Element,
	Length,
	Padding,
	Renderer,
	widget::{
		Column,
		Row,
		button,
		container,
		horizontal_rule,
		text,
		text_input,
	},
};
use iced_aw::{
	Grid,
	GridRow,
	menu::{
		Item,
		Menu,
		MenuBar,
	},
	menu_bar,
	menu_items,
};

use crate::gui::{
	morphiq::Morphiq,
	styles::{
		button::ButtonType,
		container::ContainerType,
		rule::RuleType,
		style_constant::fonts::{
			OUTFIT_BOLD,
			RALEWAY_BOLD,
		},
		text_input::TextInputType,
		types::style_type::StyleType,
	},
	types::message::Message,
};

#[derive(Debug, Clone)]
pub struct GenTableEmployee {
	title: String,
	header: Vec<String>,
	body: Vec<RowTable>,
	search: String,
}

#[derive(Default, Debug, Clone)]
pub struct RowTable {
	pub id_num: String,
	pub full_name: String,
	pub position: String,
	pub department: String,
	pub interaction: String,
	pub work_hours: String,
	pub status: String,
}

impl RowTable {
	pub fn new(
		id_num: String,
		full_name: String,
		position: String,
		department: String,
		interaction: String,
		work_hours: String,
		status: String,
	) -> Self {
		Self {
			id_num,
			full_name,
			position,
			department,
			interaction,
			work_hours,
			status: status.to_uppercase(),
		}
	}
}

impl GenTableEmployee {
	fn headers(&self) -> GridRow<'_, Message, StyleType> {
		let mut grid_row = GridRow::new();
		for i in 0..self.header.len() {
			grid_row = grid_row.push(
				text(self.header[i].clone()).size(18.0).font(OUTFIT_BOLD),
			);
		}

		grid_row
	}

	fn separators(
		&self,
		rule_type: RuleType,
	) -> GridRow<'_, Message, StyleType> {
		let max_len = self.header.len();
		let elements = (0..=max_len)
			.map(|_| horizontal_rule(2.0).class(rule_type.clone()))
			.collect();

		GridRow::with_elements(elements)
	}

	fn bodies(&self, separator_type: RuleType) -> Grid<'_, Message, StyleType> {
		let mut content = Grid::new()
			.width(Length::Fill)
			.row_spacing(5.0)
			.row_height(12.0)
			.column_width(Length::Fill)
			.column_spacing(5.0)
			.push(self.headers())
			.push(self.separators(separator_type));

		for v in &self.body {
			let mut body_content = GridRow::new();

			body_content = body_content
				.push(text(v.id_num.clone()))
				.push(text(v.full_name.clone()))
				.push(text(v.position.clone()))
				.push(text(v.department.clone()))
				.push(text(v.interaction.clone()))
				.push(text(v.work_hours.clone()))
				.push(text(v.status.clone()));
			content = content.push(body_content);
		}

		content
	}

	fn title(&self) -> String {
		self.title.clone()
	}

	pub const fn new(
		title: String,
		header: Vec<String>,
		body: Vec<RowTable>,
	) -> Self {
		Self { title, header, body, search: String::new() }
	}

	pub fn view(&self, morphiq: &Morphiq) -> Element<'_, Message, StyleType> {
		let style = morphiq.configs.settings.style.get_palette();
		let menu_template =
			|items| Menu::new(items).max_width(180.0).offset(6.0);

		let menu_bar: MenuBar<'_, Message, StyleType, Renderer> = menu_bar!((
			button("Filter"),
			menu_template(menu_items!((button("Department")
				.class(ButtonType::Ghost))(
				button("ID Number").class(ButtonType::Ghost)
			)(
				button("Status").class(ButtonType::Ghost)
			)))
		));

		let header_elements = Row::new()
			.push(text(self.title()).size(24.0).font(RALEWAY_BOLD))
			.push(
				container(
					text_input("Search...", &self.search)
						.width(Length::Fill)
						.class(TextInputType::Ghost),
				)
				.class(ContainerType::Bordered)
				.padding(
					Padding::ZERO.left(5.0).right(5.0).top(2.0).bottom(2.0),
				),
			)
			.push(menu_bar)
			.spacing(15.0);

		let content = Column::new()
			.push(header_elements)
			.push(self.bodies(RuleType::PaletteColor(style.base_200, 2)))
			.spacing(15.0)
			.padding(5.0);

		container(content).class(ContainerType::Base300).padding(15.0).into()
	}
}
