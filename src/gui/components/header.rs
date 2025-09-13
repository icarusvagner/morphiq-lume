use iced::{
    alignment::{Horizontal, Vertical},
    widget::{button, container, horizontal_space, text, Row},
    Element, Length,
};

use crate::{
    gui::{
        pages::home::HomeMessage,
        styles::{
            button::ButtonType, container::ContainerType, text::TextType,
            types::style_type::StyleType,
        },
        types::message::Message,
    },
    utils::types::icon::Icon,
};

#[derive(Debug, Default, Clone)]
pub struct Header;

#[allow(clippy::enum_variant_names)]
#[allow(clippy::large_enum_variant)]
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
}

impl Header {
    fn header_btn<'a>(
        &'a self,
        title: Option<String>,
        icon: Icon,
        message: Message,
    ) -> Element<'a, Message, StyleType> {
        if let Some(title) = title {
            let content = Row::new()
                .push(
                    icon.to_text()
                        .size(18.0)
                        .class(TextType::Neutral)
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center),
                )
                .push(
                    text(title)
                        .size(18.0)
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .class(TextType::Neutral)
                        .line_height(text::LineHeight::Relative(1.7)),
                )
                .align_y(Vertical::Center);
            button(content)
                .class(ButtonType::Base)
                .on_press(message)
                .into()
        } else {
            button(
                icon.to_text()
                    .size(22)
                    .class(TextType::Neutral)
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center),
            )
            .class(ButtonType::Base)
            .on_press(message)
            .into()
        }
    }

    fn left_menu<'a>(&'a self) -> Element<'a, Message, StyleType> {
        Row::new()
            .push(self.header_btn(
                None,
                Icon::Morphiq,
                Message::Home(HomeMessage::Header(HeaderMessage::Dashboard)),
            ))
            .push(self.header_btn(
                Some("Search".to_string()),
                Icon::Search,
                Message::Home(HomeMessage::Header(HeaderMessage::Search)),
            ))
            .push(self.header_btn(
                Some("Add Employee".to_string()),
                Icon::UserRoundPlus,
                Message::Home(HomeMessage::Header(HeaderMessage::AddEmployee)),
            ))
            .push(self.header_btn(
                Some("Event Postings".to_string()),
                Icon::CalendarDays,
                Message::Home(HomeMessage::Header(HeaderMessage::EventPosting)),
            ))
            .push(self.header_btn(
                Some("Organization Settings".to_string()),
                Icon::LayoutPanelLeft,
                Message::Home(HomeMessage::Header(HeaderMessage::OrgSettings)),
            ))
            .spacing(5)
            .align_y(Vertical::Center)
            .into()
    }

    fn right_menu<'a>(&'a self) -> Element<'a, Message, StyleType> {
        Row::new()
            .push(self.header_btn(
                None,
                Icon::Globe,
                Message::Home(HomeMessage::Header(HeaderMessage::Language)),
            ))
            .push(self.header_btn(
                None,
                Icon::MessageSquareMore,
                Message::Home(HomeMessage::Header(HeaderMessage::Chat)),
            ))
            .push(self.header_btn(
                None,
                Icon::BellRing,
                Message::Home(HomeMessage::Header(HeaderMessage::Notifications)),
            ))
            .spacing(5)
            .align_y(Vertical::Center)
            .into()
    }

    pub(crate) fn update(&mut self, message: HeaderMessage) {
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
        }
    }

    pub(crate) fn view<'a>(&'a self) -> Element<'a, Message, StyleType> {
        let content = Row::new()
            .push(self.left_menu())
            .push(horizontal_space())
            .push(self.right_menu())
            .align_y(Vertical::Center);

        container(content)
            .width(Length::Fill)
            .class(ContainerType::Ghost)
            .into()
    }
}
