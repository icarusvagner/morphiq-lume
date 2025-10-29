use iced::{
	Alignment,
	Element,
	Length,
	Padding,
	Renderer,
	widget::{
		Column,
		Container,
		Row,
		button,
		container,
		responsive,
		text,
		text_input,
	},
};
use iced_aw::{
	menu::{
		Item,
		Menu,
		MenuBar,
	},
	menu_bar,
	menu_items,
};
use iced_table::table;

use crate::{
	gui::{
		morphiq::Morphiq,
		pages::home::panes::tables::dashboard_table::{
			DashboardRow,
			DashboardTable,
		},
		styles::{
			button::ButtonType,
			container::ContainerType,
			style_constant::fonts::RALEWAY_BOLD,
			text_input::TextInputType,
			types::style_type::StyleType,
		},
		types::{
			message::Message,
			tables::{
				DashboardTableMsg,
				FilterEmployee,
				TableMessage,
			},
		},
	},
	utils::types::icon::Icon,
};

#[derive(Debug, Clone, Default)]
pub struct GenTableDashboard {
	pub title: String,
	pub search: String,
	pub table: DashboardTable,
}

#[allow(clippy::use_self, clippy::unused_self)]
impl GenTableDashboard {
	pub fn new(title: String, rows: Vec<DashboardRow>) -> Self {
		let table = DashboardTable::new(rows);

		Self { title, table, ..Default::default() }
	}

	fn title(&self) -> String {
		self.title.clone()
	}

	fn menu_bar(&self) -> MenuBar<'_, Message, StyleType, Renderer> {
		let menu_template =
			|items| Menu::new(items).max_width(180.0).offset(6.0);

		let icon_with_name = Row::new()
			.push(Icon::Funnel.to_text().size(18))
			.push(text("Filter").size(18));

		let menu_bar: MenuBar<'_, Message, StyleType, Renderer> = menu_bar!((
			button(icon_with_name).class(ButtonType::Ghost),
			menu_template(menu_items!((button(
				text("Department")
					.align_x(Alignment::Center)
					.width(Length::Fill)
			)
			.width(Length::Fill)
			.on_press(Message::Tables(TableMessage::Dashboard(
				DashboardTableMsg::FilteredBy(FilterEmployee::Department)
			)))
			.class(ButtonType::Ghost))(
				button(
					text("ID Number")
						.align_x(Alignment::Center)
						.width(Length::Fill)
				)
				.width(Length::Fill)
				.on_press(Message::Tables(TableMessage::Dashboard(
					DashboardTableMsg::FilteredBy(FilterEmployee::IdNumber)
				)))
				.class(ButtonType::Ghost)
			)(
				button(
					text("Status")
						.align_x(Alignment::Center)
						.width(Length::Fill)
				)
				.width(Length::Fill)
				.on_press(Message::Tables(TableMessage::Dashboard(
					DashboardTableMsg::FilteredBy(FilterEmployee::Status)
				)))
				.class(ButtonType::Ghost)
			)(
				button(
					text("Fullname")
						.align_x(Alignment::Center)
						.width(Length::Fill)
				)
				.width(Length::Fill)
				.on_press(Message::Tables(TableMessage::Dashboard(
					DashboardTableMsg::FilteredBy(FilterEmployee::Fullname)
				)))
				.class(ButtonType::Ghost)
			)))
		));

		menu_bar
	}

	fn table(&self) -> Container<'_, Message, StyleType> {
		let table = responsive(move |size| {
			let mut table = table(
				self.table.header.clone(),
				self.table.body.clone(),
				&self.table.columns,
				&self.table.rows,
				Message::DashboardTableSyncHeader,
			);

			table = table.on_column_resize(
				Message::DashboardTableResizing,
				Message::DashboardTableResized,
			);
			table = table.min_width(size.width);
			table.cell_padding(Padding::from(5.0)).divider_width(2.0).into()
		});

		container(table).class(ContainerType::Ghost).width(Length::Fill)
	}

	pub fn update(&mut self, message: DashboardTableMsg) {
		match message {
			DashboardTableMsg::Search(val) => {
				self.search = val;
				// if self.search.is_empty() {
				// 	self.body = self.temp.clone();
				// 	return;
				// }

				// let results: Vec<RowTable> = self
				// 	.temp
				// 	.iter()
				// 	.filter(|v| v.full_name.contains(&self.search))
				// 	.cloned()
				// 	.collect();
				// if results.is_empty() {
				// 	self.body = self.temp.clone();
				// } else {
				// 	self.body = results;
				// }
			}
			DashboardTableMsg::FilteredBy(filter_by) => match filter_by {
				FilterEmployee::Department => {
					self.table
						.rows
						.sort_by(|a, b| a.department.cmp(&b.department));
				}
				FilterEmployee::IdNumber => {
					self.table.rows.sort_by(|a, b| a.id.cmp(&b.id));
				}
				FilterEmployee::Fullname => {
					self.table
						.rows
						.sort_by(|a, b| a.full_name.cmp(&b.full_name));
				}
				FilterEmployee::Status => {
					self.table.rows.sort_by(|a, b| a.status.cmp(&b.status));
				}
				FilterEmployee::Position => {}
			},
			_ => unreachable!(),
		}
	}

	pub fn view(&self, _morphiq: &Morphiq) -> Element<'_, Message, StyleType> {
		let header_elements = Row::new()
			.push(text(self.title()).size(24.0).font(RALEWAY_BOLD))
			.push(
				container(
					text_input("Search fullname...", &self.search)
						.width(Length::Fill)
						.class(TextInputType::Ghost)
						.on_input(|val| {
							Message::Tables(TableMessage::Dashboard(
								DashboardTableMsg::Search(val),
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

		let content = Column::new()
			.push(header_elements)
			.push(self.table())
			.height(Length::Fill)
			.spacing(15.0)
			.padding(5.0);

		container(content)
			.height(550.0)
			.class(ContainerType::Base300)
			.padding(15.0)
			.into()
	}
}
