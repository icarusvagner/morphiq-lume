use iced::{
    widget::{container, text, Row},
    Element,
};

use crate::gui::{styles::types::style_type::StyleType, types::message::Message};

#[derive(Debug, Clone)]
pub struct NewlyAdded {
    pub title: String,
    pub value: String,
    pub body: String,
}

impl Default for NewlyAdded {
    fn default() -> Self {
        NewlyAdded {
            title: String::from("Newly Added"),
            value: String::from("12"),
            body: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum NewlyAddedMessage {
    FetchCount,
}

impl NewlyAdded {
    pub fn update(&mut self, message: NewlyAddedMessage) {
        match message {
            NewlyAddedMessage::FetchCount => {}
        }
    }

    pub fn view(&self) -> Element<'_, Message, StyleType> {
        let content = Row::new().push(text("Newly added").size(42));
        container(content).into()
    }
}
