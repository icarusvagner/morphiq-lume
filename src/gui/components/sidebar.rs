use iced::{
	Alignment, Element, Length, alignment::{Horizontal, Vertical}, widget::{
		Column, Row, button, container, horizontal_rule, text, vertical_space
	}
};

use crate::{
	gui::{
		morphiq::Morphiq, pages::home::{ContentView, HomeMessage}, styles::{
			button::ButtonType, container::ContainerType, rule::RuleType, text::TextType, types::style_type::StyleType
		}, types::message::Message
	}, utils::types::icon::Icon
};

#[derive(Debug, Default, Clone, Copy)]
pub struct SidebarMenu;

impl SidebarMenu {
	pub const FIRST_ALL: [(&str, Icon, Message); 5] = [
		(
			"Dashboard",
			Icon::House,
			Message::Home(HomeMessage::Content(ContentView::Dashboard)),
		),
		(
			"Our Staff",
			Icon::UserRound,
			Message::Home(HomeMessage::Content(ContentView::Employee)),
		),
		(
			"Attendance",
			Icon::CalendarDays,
			Message::Home(HomeMessage::Content(ContentView::Attendance)),
		),
		(
			"Payroll",
			Icon::Coins,
			Message::Home(HomeMessage::Content(ContentView::Payroll)),
		),
		(
			"Leaves",
			Icon::CalendarClock,
			Message::Home(HomeMessage::Content(ContentView::Leaves)),
		),
	];
	pub const SECOND_ALL: [(&str, Icon, Message); 3] = [
		(
			"Settings",
			Icon::Settings,
			Message::Home(HomeMessage::Content(ContentView::Settings(
				crate::gui::pages::home::OpenSettings::All,
			))),
		),
		(
			"Report",
			Icon::Flag,
			Message::Home(HomeMessage::Content(ContentView::Dashboard)),
		),
		(
			"Support",
			Icon::Information,
			Message::Home(HomeMessage::Content(ContentView::Dashboard)),
		),
	];

	#[allow(clippy::unused_self)]
	fn get_additional_tooltop_menus(&self) -> Column<'_, Message, StyleType> {
		Column::with_children(Self::SECOND_ALL.map(|v| {
			button(
				Row::new()
					.push(
						v.1.to_text()
							.size(22.0)
							.align_x(Horizontal::Center)
							.align_y(Vertical::Center),
					)
					.push(
						text(v.0)
							.size(22.0)
							.align_x(Horizontal::Center)
							.align_y(Vertical::Center),
					)
					.spacing(15)
					.width(Length::Fill)
					.align_y(Alignment::Center),
			)
			.class(ButtonType::Ghost)
			.on_press(v.2.clone())
			.into()
		}))
	}

	#[allow(clippy::unused_self)]
	fn get_tooltip_menus(&self) -> Column<'_, Message, StyleType> {
		Column::with_children(Self::FIRST_ALL.map(|v| {
			button(
				Row::new()
					.push(
						v.1.to_text()
							.size(22.0)
							.align_x(Horizontal::Center)
							.align_y(Vertical::Center),
					)
					.push(
						text(v.0)
							.size(22.0)
							.align_x(Horizontal::Center)
							.align_y(Vertical::Center),
					)
					.spacing(15)
					.width(Length::Fill)
					.align_y(Alignment::Center),
			)
			.class(ButtonType::Ghost)
			.on_press(v.2.clone())
			.into()
		}))
	}

	pub fn view(&self, morphiq: &Morphiq) -> Element<'_, Message, StyleType> {
		let colors = morphiq.configs.settings.style.get_palette();

		let content = Column::new()
			.push(self.get_tooltip_menus())
			.push(
				horizontal_rule(2.0)
					.class(RuleType::PaletteColor(colors.base_200, 2)),
			)
			.push(self.get_additional_tooltop_menus())
			.push(vertical_space())
			.push(
				horizontal_rule(2.0)
					.class(RuleType::PaletteColor(colors.base_200, 2)),
			)
			.push(
				button(
					container(
						Row::new()
							.push(
								Icon::LogOut
									.to_text()
									.class(TextType::Content)
									.size(20)
									.align_x(Horizontal::Center)
									.align_y(Vertical::Center),
							)
							.push(
								text("Logout")
									.class(TextType::Content)
									.size(20)
									.align_x(Horizontal::Center)
									.align_y(Vertical::Center),
							)
							.spacing(10)
							.align_y(Alignment::Center),
					)
					.width(Length::Fill)
					.class(ContainerType::Ghost)
					.align_x(Alignment::Center)
					.align_y(Alignment::Center),
				)
				.class(ButtonType::Primary)
				.width(Length::Fill)
				.on_press(Message::Home(HomeMessage::Logout)),
			)
			.spacing(15)
			.width(Length::Fill)
			.padding(15)
			.align_x(Horizontal::Center);

		container(content)
			.width(260.0)
			.height(Length::Fill)
			.align_x(Alignment::Center)
			.align_y(Alignment::Start)
			.class(ContainerType::Base100)
			.into()
	}
}
