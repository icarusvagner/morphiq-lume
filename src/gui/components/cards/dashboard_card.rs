use iced::{
    alignment::{Horizontal, Vertical},
    widget::{container, Column, Container, Row, Text},
    Alignment, Length,
};

use crate::{
    gui::{
        styles::{
            container::ContainerType,
            style_constant::fonts::{OUTFIT_BOLD, RALEWAY_BOLD, RALEWAY_REGULAR},
            text::TextType,
            types::style_type::StyleType,
        },
        types::message::Message,
    },
    utils::types::icon::Icon,
};

pub(crate) fn dashboard_card<'a>(
    icon: Icon,
    title: String,
    count: u64,
    bg_color: ContainerType,
) -> Container<'a, Message, StyleType> {
    let col = Column::new()
        .push(
            icon.to_text()
                .size(32)
                .class(TextType::Content)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center),
        )
        .push(
            Text::new(title.clone())
                .size(18)
                .font(RALEWAY_REGULAR)
                .class(TextType::Content),
        )
        .push(
            Text::new(count)
                .size(32)
                .font(OUTFIT_BOLD)
                .class(TextType::Content),
        )
        .align_x(Alignment::Center);

    let row = Row::new().push(col).align_y(Alignment::Center);

    container(row)
        .padding(15.0)
        .height(Length::Fill)
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .class(bg_color)
}

pub(crate) fn sec_card<'a>(
    icon: Icon,
    icon_color: ContainerType,
    title: String,
    count: u64,
) -> Container<'a, Message, StyleType> {
    let col = Column::new()
        .push(Text::new(title.clone()).size(16).font(RALEWAY_REGULAR))
        .push(Text::new(count).size(22).font(RALEWAY_BOLD))
        .spacing(5);

    let row = Row::new()
        .push(
            container(
                icon.to_text()
                    .size(18.0)
                    .height(Length::Fill)
                    .width(Length::Fill)
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center)
                    .class(TextType::Content),
            )
            .class(icon_color)
            .height(50.0)
            .width(50.0),
        )
        .push(col)
        .align_y(Alignment::Start)
        .spacing(15);

    container(row)
        .padding(15.0)
        .height(Length::Fill)
        .width(Length::Fill)
        .align_y(Alignment::Center)
        .align_x(Alignment::Start)
        .class(ContainerType::Base300)
}
