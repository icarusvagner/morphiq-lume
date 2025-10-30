use iced::{
	Element, Length, alignment::{Horizontal, Vertical}, widget::{Row, button, container, horizontal_space, text}
};

use crate::{
	gui::{
		morphiq::Morphiq, pages::home::{ContentView, HomeMessage}, styles::{
			button::ButtonType, container::ContainerType, types::style_type::StyleType
		}, types::message::Message
	}, utils::types::icon::Icon
};

#[derive(Debug, Default, Clone)]
pub struct Header;

#[allow(clippy::enum_variant_names, clippy::large_enum_variant, dead_code)]
#[derive(Debug, Clone)]
pub enum HeaderMessage {
	Dashboard,
	Search,
	AddEmployee,
	EventPosting,
	OrgSettings,
	Language,
	Chat,
	Notifications,
	Profile,
	Candidates,
}

impl Header {
	#[allow(clippy::unused_self)]
	fn header_btn(
		&self,
		title: Option<String>,
		icon: Icon,
		message: Message,
	) -> Element<'_, Message, StyleType> {
		if let Some(title) = title {
			let content = Row::new()
				.push(
					icon.to_text()
						.size(18.0)
						.align_x(Horizontal::Center)
						.align_y(Vertical::Center),
				)
				.push(
					text(title)
						.size(18.0)
						.align_x(Horizontal::Center)
						.align_y(Vertical::Center)
						.line_height(text::LineHeight::Relative(1.7)),
				)
				.spacing(5.0)
				.align_y(Vertical::Center);
			button(content).on_press(message).class(ButtonType::Base100).into()
		} else {
			button(
				icon.to_text()
					.size(22)
					.align_x(Horizontal::Center)
					.align_y(Vertical::Center),
			)
			.class(ButtonType::Base100)
			.on_press(message)
			.into()
		}
	}

	fn left_menu(&self) -> Element<'_, Message, StyleType> {
		Row::new()
			.push(self.header_btn(
				None,
				Icon::Morphiq,
				Message::Home(HomeMessage::Content(ContentView::Dashboard)),
			))
			.push(self.header_btn(
				Some("Search".to_string()),
				Icon::Search,
				Message::Home(HomeMessage::Content(ContentView::Search)),
			))
			.push(self.header_btn(
				Some("Add Employee".to_string()),
				Icon::UserRoundPlus,
				Message::Home(HomeMessage::Content(ContentView::AddEmployee)),
			))
			.push(self.header_btn(
				Some("Events".to_string()),
				Icon::CalendarDays,
				Message::Home(HomeMessage::Content(ContentView::EventsPosting)),
			))
			.push(self.header_btn(
				Some("Jobs".to_string()),
				Icon::NotebookText,
				Message::Home(HomeMessage::Content(ContentView::Dashboard)),
			))
			.push(self.header_btn(
				Some("Candidates".to_string()),
				Icon::UsersRound,
				Message::Home(HomeMessage::Content(ContentView::Dashboard)),
			))
			.spacing(5)
			.align_y(Vertical::Center)
			.into()
	}

	fn right_menu<'a>(
		&'a self,
		morphiq: &Morphiq,
	) -> Element<'a, Message, StyleType> {
		let toggle_theme = if morphiq.configs.settings.style == StyleType::Dark
		{
			Icon::Sun
		} else {
			Icon::Moon
		};

		Row::new()
			.push(self.header_btn(None, toggle_theme, Message::ChangeTheme))
			.push(self.header_btn(
				None,
				Icon::BellRing,
				Message::Home(HomeMessage::Content(ContentView::Notifications)),
			))
			.push(self.header_btn(
				None,
				Icon::UserRound,
				Message::Home(HomeMessage::Content(ContentView::Profile)),
			))
			.spacing(5)
			.align_y(Vertical::Center)
			.into()
	}

	#[allow(clippy::unused_self, clippy::match_same_arms)]
	pub const fn update(&self, message: HeaderMessage) {
		match message {
			HeaderMessage::Dashboard => {}
			HeaderMessage::Search => {}
			HeaderMessage::AddEmployee => {}
			HeaderMessage::EventPosting => {}
			HeaderMessage::OrgSettings => {}
			HeaderMessage::Language => {}
			HeaderMessage::Chat => {}
			HeaderMessage::Notifications => {}
			HeaderMessage::Profile => {}
			HeaderMessage::Candidates => {}
		}
	}

	pub(crate) fn view<'a>(
		&'a self,
		morphiq: &Morphiq,
	) -> Element<'a, Message, StyleType> {
		let content = Row::new()
			.push(self.left_menu())
			.push(horizontal_space())
			.push(self.right_menu(morphiq))
			.align_y(Vertical::Center);

		container(content)
			.width(Length::Fill)
			.class(ContainerType::Ghost)
			.into()
	}
}
