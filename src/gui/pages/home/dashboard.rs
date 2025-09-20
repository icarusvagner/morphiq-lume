use iced::{
    widget::{container, Column, Row},
    Alignment, Element, Length,
};

use crate::{
    gui::{
        components::cards::dashboard_card::{dashboard_card, sec_card},
        styles::{
            container::ContainerType, style_constant::StandardNames, types::style_type::StyleType,
        },
        types::message::Message,
    },
    utils::types::icon::Icon,
};

#[derive(Debug, Clone, Default)]
pub struct DashboardView;

impl DashboardView {
    pub(crate) fn view<'a>(&'a self) -> Element<'a, Message, StyleType> {
        let first_cards = container(
            Row::new()
                .push(dashboard_card(
                    Icon::Layers,
                    "Total Recruits".to_string(),
                    12,
                    ContainerType::Primary,
                ))
                .push(dashboard_card(
                    Icon::Sync,
                    "Pending Interviews".to_string(),
                    10,
                    ContainerType::Secondary,
                ))
                .push(dashboard_card(
                    Icon::CreditCard,
                    "Monthly Sales".to_string(),
                    123_003,
                    ContainerType::Accent,
                ))
                .spacing(15.0)
                .align_y(Alignment::Center),
        )
        .height(130.0);

        let second_cards = container(
            Row::new()
                .push(sec_card(
                    Icon::ShoppingCart,
                    ContainerType::Icon(StandardNames::Secondary),
                    "Total Order".to_string(),
                    1172,
                ))
                .push(sec_card(
                    Icon::Repeat,
                    ContainerType::Icon(StandardNames::Accent),
                    "Orders Pending".to_string(),
                    27,
                ))
                .push(sec_card(
                    Icon::Package,
                    ContainerType::Icon(StandardNames::Neutral),
                    "Orders Processing".to_string(),
                    4,
                ))
                .push(sec_card(
                    Icon::Check,
                    ContainerType::Icon(StandardNames::Primary),
                    "Orders Delivered".to_string(),
                    40,
                ))
                .spacing(15.0),
        )
        .height(110.0);

        let content = Column::new()
            .push(first_cards)
            .push(second_cards)
            .spacing(15.0);

        container(content).width(Length::Fill).into()
    }
}
