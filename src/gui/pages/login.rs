use iced::{
	Alignment,
	Element,
	Font,
	Length,
	alignment::{
		Horizontal,
		Vertical,
	},
	widget::{
		Column,
		Row,
		Svg,
		button,
		container,
		svg::Handle,
		text,
		text_input,
	},
};

use crate::{
	gui::{
		styles::{
			button::ButtonType,
			container::ContainerType,
			style_constant::fonts::RALEWAY_SEMI_BOLD,
			text::TextType,
			text_input::TextInputType,
			types::style_type::StyleType,
		},
		types::message::Message,
	},
	images::IMAGE_03,
	utils::types::icon::Icon,
};

#[allow(clippy::enum_variant_names)]
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum LoginMessage {
	InputFieldChange(String, String),
	LoginSubmitted,
	ShowPassword,
}

#[derive(Debug, Clone)]
pub struct Login {
	pub username: String,
	pub password: String,
	pub is_pwd: bool,
	pub password_required: String,
	pub username_required: String,
}

impl Default for Login {
	#[allow(clippy::use_self)]
	fn default() -> Self {
		Login {
			username: String::new(),
			password: String::new(),
			is_pwd: true,
			password_required: String::new(),
			username_required: String::new(),
		}
	}
}

#[allow(clippy::unused_self)]
impl Login {
	fn _left_image(&self) -> Element<'_, Message, StyleType> {
		container(
			Svg::new(Handle::from_memory(IMAGE_03))
				.content_fit(iced::ContentFit::Contain)
				.width(300.0)
				.height(400.0),
		)
		.padding(1.50)
		.height(Length::Fill)
		.width(Length::Fill)
		.align_x(Alignment::Center)
		.align_y(Alignment::Center)
		.into()
	}

	fn username_input(&self) -> Element<'_, Message, StyleType> {
		let mut content = Column::new()
			.push(
				text("Username")
					.size(18)
					.line_height(text::LineHeight::Relative(1.7)),
			)
			.push(
				container(
					text_input("Username", &self.username)
						.line_height(text::LineHeight::Relative(1.7))
						.on_input(|val| {
							Message::Login(LoginMessage::InputFieldChange(
								val,
								self.password.clone(),
							))
						})
						.class(TextInputType::Ghost),
				)
				.padding(10.0)
				.width(Length::Fill)
				.class(ContainerType::Bordered),
			)
			.spacing(3);

		if !self.username_required.is_empty() {
			content = content.push(
				text("Username is required*").size(14).class(TextType::Error),
			);
		}

		content.into()
	}

	fn password_input(&self) -> Element<'_, Message, StyleType> {
		let mut content = Column::new()
			.push(
				text("Password")
					.size(18)
					.line_height(text::LineHeight::Relative(1.7)),
			)
			.push(
				container(
					Row::new()
						.push(
							text_input("Password", &self.password)
								.line_height(text::LineHeight::Relative(1.7))
								.on_input(|val| {
									Message::Login(
										LoginMessage::InputFieldChange(
											self.username.clone(),
											val,
										),
									)
								})
								.class(TextInputType::Ghost)
								.secure(self.is_pwd),
						)
						.push(
							button(self.toggle_pwd())
								.on_press(Message::Login(
									LoginMessage::ShowPassword,
								))
								.class(ButtonType::Ghost),
						)
						.align_y(Alignment::Center)
						.width(Length::Fill),
				)
				.padding(10.0)
				.width(Length::Fill)
				.class(ContainerType::Base100),
			)
			.spacing(5);

		if !self.password_required.is_empty() {
			content = content.push(
				text("Password is required*").size(14).class(TextType::Error),
			);
		}

		content.into()
	}

	fn toggle_pwd(&self) -> Element<'_, Message, StyleType> {
		if self.is_pwd {
			Icon::Eye
				.to_text()
				.size(20)
				.align_x(Alignment::Center)
				.align_y(Alignment::Center)
				.into()
		} else {
			Icon::EyeOff
				.to_text()
				.size(20)
				.align_x(Alignment::Center)
				.align_y(Alignment::Center)
				.into()
		}
	}

	fn title(&self) -> Element<'_, Message, StyleType> {
		Row::new()
			.push(
				text("Admin Login")
					.size(32)
					.font(RALEWAY_SEMI_BOLD)
					.line_height(text::LineHeight::Relative(1.7)),
			)
			.into()
	}

	pub(crate) fn update(&mut self, message: LoginMessage) -> Option<Message> {
		match message {
			LoginMessage::InputFieldChange(username, password) => {
				self.username = username;
				self.password = password;
				None
			}
			LoginMessage::LoginSubmitted => {
				if self.username.is_empty() && self.password.is_empty() {
					self.password_required = "Password is required".to_string();
					self.username_required = "Username is required".to_string();
					return None;
				}
				Some(Message::ChangePage(super::OpenPage::Home))
			}
			LoginMessage::ShowPassword => {
				self.is_pwd = !self.is_pwd;
				None
			}
		}
	}

	pub(crate) fn view(&self) -> Element<'_, Message, StyleType> {
		let login_col = container(
			Column::new()
				.push(self.title())
				.push(self.username_input())
				.push(self.password_input())
				.push(
					button(
						text("Submit")
							.width(Length::Fill)
							.class(TextType::Content)
							.align_x(Horizontal::Center)
							.align_y(Vertical::Center)
							.size(18)
							.font(Font {
								weight: iced::font::Weight::Medium,
								..Default::default()
							}),
					)
					.class(ButtonType::Primary)
					.padding(10.0)
					.width(Length::Fill)
					.on_press(Message::Login(LoginMessage::LoginSubmitted)),
				)
				.spacing(20.0),
		)
		.class(ContainerType::Ghost)
		.align_y(Alignment::Center)
		.padding(60.0)
		.height(600.0)
		.width(450.0);

		// let with_img = Row::new().push(self.left_image()).push(login_col);

		let content = container(login_col)
			.class(ContainerType::Base100)
			.height(600.0)
			.width(900.0)
			.align_x(Alignment::Center)
			.align_y(Alignment::Center);

		container(content)
			.center_x(Length::Fill)
			.center_y(Length::Fill)
			.align_x(Alignment::Center)
			.align_y(Alignment::Center)
			.into()
	}
}
