use iced::{
    alignment::{Horizontal, Vertical},
    widget::{button, container, horizontal_rule, text, vertical_space, Column, Row},
    Alignment, Element, Length,
};

use crate::{
    gui::{
        pages::home::{ContentView, HomeMessage},
        styles::{
            button::ButtonType, container::ContainerType, rule::RuleType, text::TextType,
            types::style_type::StyleType,
        },
        types::message::Message,
    },
    utils::types::icon::Icon,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct SidebarMenu;

impl SidebarMenu {
    pub const ALL: [(&str, Icon, Message); 6] = [
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
            Icon::PhPeso,
            Message::Home(HomeMessage::Content(ContentView::Payroll)),
        ),
        (
            "Leaves",
            Icon::ArrowBendDoubleUpRight,
            Message::Home(HomeMessage::Content(ContentView::Leaves)),
        ),
        (
            "Documents",
            Icon::Newspaper,
            Message::Home(HomeMessage::Content(ContentView::Documents)),
        ),
    ];

    fn get_tooltip_menus<'a>(&'a self) -> Column<'a, Message, StyleType> {
        Column::with_children(Self::ALL.map(|v| {
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
            .on_press(v.2)
            .into()
        }))
    }

    pub fn view<'a>(&'a self) -> Element<'a, Message, StyleType> {
        let content = Column::new()
            .push(self.get_tooltip_menus())
            .push(vertical_space())
            .push(horizontal_rule(2.0).class(RuleType::Base200))
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
