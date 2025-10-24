use iced::{
	Element,
	Length,
	Padding,
	Renderer,
	widget::{
		Column,
		Row,
		Scrollable,
		button,
		container,
		horizontal_rule,
		scrollable,
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

use crate::{
	gui::{
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
		types::{
			message::Message,
			tables::{
				EmployeeTableMsg,
				FilterEmployee,
				TableMessage,
			},
		},
	},
	utils::types::icon::Icon,
};

#[derive(Debug, Clone)]
pub struct GenTableEmployee {
	title: String,
	header: Vec<String>,
	body: Vec<RowTable>,
	temp: Vec<RowTable>,
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

#[allow(clippy::use_self, clippy::unused_self)]
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
			.row_spacing(5.0)
			.row_height(12.0)
			.column_spacing(5.0)
			.push(self.headers())
			.push(self.separators(separator_type));

		for (i, v) in self.body.iter().enumerate() {
			let mut body_content = GridRow::new();

			body_content = body_content
				.push(text(i))
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

	pub fn new(
		title: String,
		header: Vec<String>,
		body: Vec<RowTable>,
	) -> Self {
		Self { title, header, temp: body.clone(), body, search: String::new() }
	}

	fn menu_bar(&self) -> MenuBar<'_, Message, StyleType, Renderer> {
		let menu_template =
			|items| Menu::new(items).max_width(180.0).offset(6.0);
		let icon_with_name = Row::new()
			.push(Icon::Funnel.to_text().size(18))
			.push(text("Filter").size(18));

		let menu_bar: MenuBar<'_, Message, StyleType, Renderer> = menu_bar!((
			button(icon_with_name).class(ButtonType::Ghost),
			menu_template(menu_items!((button("Department")
				.width(Length::Fill)
				.on_press(Message::Tables(TableMessage::Employee(
					EmployeeTableMsg::FilteredBy(FilterEmployee::Department)
				)))
				.class(ButtonType::Ghost))(
				button("ID Number")
					.width(Length::Fill)
					.on_press(Message::Tables(TableMessage::Employee(
						EmployeeTableMsg::FilteredBy(FilterEmployee::IdNumber)
					)))
					.class(ButtonType::Ghost)
			)(
				button("Status")
					.width(Length::Fill)
					.on_press(Message::Tables(TableMessage::Employee(
						EmployeeTableMsg::FilteredBy(FilterEmployee::Status)
					)))
					.class(ButtonType::Ghost)
			)(
				button("Fullname")
					.width(Length::Fill)
					.on_press(Message::Tables(TableMessage::Employee(
						EmployeeTableMsg::FilteredBy(FilterEmployee::Fullname)
					)))
					.class(ButtonType::Ghost)
			)))
		));

		menu_bar
	}

	pub fn update(&mut self, message: EmployeeTableMsg) {
		match message {
			EmployeeTableMsg::Search(val) => {
				println!("{val}");
				self.search = val;
				if self.search.is_empty() {
					self.body = self.temp.clone();
					return;
				}

				let results: Vec<RowTable> = self
					.temp
					.iter()
					.filter(|v| v.full_name.contains(&self.search))
					.cloned()
					.collect();
				if results.is_empty() {
					self.body = self.temp.clone();
				} else {
					self.body = results;
				}
			}
			EmployeeTableMsg::FilteredBy(filter_by) => match filter_by {
				FilterEmployee::Department => {
					self.body.sort_by(|a, b| a.department.cmp(&b.department));
				}
				FilterEmployee::IdNumber => {
					self.body.sort_by(|a, b| a.id_num.cmp(&b.id_num));
				}
				FilterEmployee::Fullname => {
					self.body.sort_by(|a, b| a.full_name.cmp(&b.full_name));
				}
				FilterEmployee::Status => {
					self.body.sort_by(|a, b| a.status.cmp(&b.status));
				}
			},
			_ => unreachable!(),
		}
	}

	pub fn view(&self, morphiq: &Morphiq) -> Element<'_, Message, StyleType> {
		let style = morphiq.configs.settings.style.get_palette();

		let header_elements = Row::new()
			.push(text(self.title()).size(24.0).font(RALEWAY_BOLD))
			.push(
				container(
					text_input("Search fullname...", &self.search)
						.width(Length::Fill)
						.class(TextInputType::Ghost)
						.on_input(|val| {
							Message::Tables(TableMessage::Employee(
								EmployeeTableMsg::Search(val),
							))
						}),
				)
				.class(ContainerType::Bordered)
				.padding(
					Padding::ZERO.left(5.0).right(5.0).top(2.0).bottom(2.0),
				),
			)
			.push(self.menu_bar())
			.spacing(15.0);

		let scroll_content = Scrollable::new(
			self.bodies(RuleType::PaletteColor(style.base_200, 2)),
		)
		.direction(scrollable::Direction::Vertical(
			scrollable::Scrollbar::new()
				.width(2.0)
				.margin(2.0)
				.scroller_width(4.0)
				.anchor(scrollable::Anchor::Start),
		))
		.width(Length::Fill)
		.height(Length::Fill);

		let content = Column::new()
			.push(header_elements)
			.push(scroll_content)
			.height(Length::Fill)
			.spacing(15.0)
			.padding(5.0);

		container(content)
			.height(550.0)
			.width(Length::Fill)
			.class(ContainerType::Base300)
			.padding(15.0)
			.into()
	}
}
