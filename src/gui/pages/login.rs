use iced::{
    alignment::{Horizontal, Vertical},
    widget::{button, container, image::Handle, text, text_input, Column, Image, Row},
    Alignment, Element, Font, Length,
};

use crate::{
    gui::{
        styles::{
            button::ButtonType, container::ContainerType, style_constant::fonts::RALEWAY_SEMI_BOLD,
            text::TextType, text_input::TextInputType, types::style_type::StyleType,
        },
        types::message::Message,
    },
    images::IMAGE_01,
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
}

impl Default for Login {
    fn default() -> Self {
        Login {
            username: String::new(),
            password: String::new(),
            is_pwd: true,
        }
    }
}

impl Login {
    fn left_image<'a>(&'a self) -> Element<'a, Message, StyleType> {
        container(
            Image::new(Handle::from_bytes(IMAGE_01))
                .content_fit(iced::ContentFit::Cover)
                .height(Length::Fill)
                .width(Length::Fill),
        )
        .width(450.0)
        .height(600.0)
        .padding(1.50)
        .into()
    }

    fn username_input<'a>(&'a self) -> Element<'a, Message, StyleType> {
        Column::new()
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
            .spacing(3)
            .into()
    }

    fn password_input<'a>(&'a self) -> Element<'a, Message, StyleType> {
        Column::new()
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
                                    Message::Login(LoginMessage::InputFieldChange(
                                        self.username.clone(),
                                        val,
                                    ))
                                })
                                .class(TextInputType::Ghost)
                                .secure(self.is_pwd),
                        )
                        .push(
                            button(self.toggle_pwd())
                                .on_press(Message::Login(LoginMessage::ShowPassword))
                                .class(ButtonType::Ghost),
                        )
                        .align_y(Alignment::Center)
                        .width(Length::Fill),
                )
                .padding(10.0)
                .width(Length::Fill)
                .class(ContainerType::Base100),
            )
            .spacing(5)
            .into()
    }

    fn toggle_pwd<'a>(&'a self) -> Element<'a, Message, StyleType> {
        if !self.is_pwd {
            Icon::EyeOff
                .to_text()
                .size(20)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
                .into()
        } else {
            Icon::Eye
                .to_text()
                .size(20)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
                .into()
        }
    }

    fn title<'a>(&'a self) -> Element<'a, Message, StyleType> {
        Row::new()
            .push(
                text("Admin Login")
                    .size(32)
                    .font(RALEWAY_SEMI_BOLD)
                    .line_height(text::LineHeight::Relative(1.7)),
            )
            .into()
    }

    pub(crate) fn update(&mut self, message: LoginMessage) {
        match message {
            LoginMessage::InputFieldChange(username, password) => {
                self.username = username;
                self.password = password;
            }
            LoginMessage::LoginSubmitted => {}
            LoginMessage::ShowPassword => self.is_pwd = !self.is_pwd,
        }
    }

    pub(crate) fn view<'a>(&'a self) -> Element<'a, Message, StyleType> {
        let login_col = container(
            Column::new()
                .push(self.title())
                .push(self.username_input())
                .push(self.password_input())
                .push(
                    button(
                        text("Submit")
                            .width(Length::Fill)
                            .align_x(Horizontal::Center)
                            .align_y(Vertical::Center)
                            .size(18)
                            .class(TextType::BaseContent)
                            .font(Font {
                                weight: iced::font::Weight::Medium,
                                ..Default::default()
                            }),
                    )
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

        let with_img = Row::new().push(self.left_image()).push(login_col);

        let content = container(with_img)
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
