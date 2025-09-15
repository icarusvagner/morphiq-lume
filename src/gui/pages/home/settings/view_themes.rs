use iced::{
    widget::{container, text, Button, Column, Container, Row},
    Alignment, Element, Length,
};

use crate::gui::{
    styles::{
        container::ContainerType,
        text::TextType,
        types::{palette::Palette, palette_extension::PaletteExtension, style_type::StyleType},
    },
    types::message::Message,
};

#[derive(Default, Debug, Clone)]
pub struct ViewThemes;

impl ViewThemes {
    fn get_palette_container<'a>(
        &'a self,
        name: String,
        style: StyleType,
        on_press: StyleType,
    ) -> Button<'a, Message, StyleType> {
        let font = style.get_extension().font;
        let PaletteExtension { is_nightly, .. } = on_press.get_extension();

        let content = Column::new()
            .push(text(name).font(font))
            .width(Length::Fill)
            .align_x(Alignment::End);

        Button::new(content)
    }

    fn themes_card<'a>(
        &'a self,
        title: String,
        theme: std::sync::LazyLock<Palette>,
    ) -> Element<'a, Message, StyleType> {
        let content_1 = Column::new()
            .push(
                container(text(""))
                    .class(ContainerType::PrimaryNoBorder)
                    .height(100.0)
                    .width(Length::Fill),
            )
            .push(
                container(text(""))
                    .class(ContainerType::SecondaryNoBorder)
                    .height(100.0)
                    .width(Length::Fill),
            )
            .push(
                container(text(""))
                    .class(ContainerType::AccentNoBorder)
                    .height(100.0)
                    .width(Length::Fill),
            )
            .push(
                container(text(""))
                    .class(ContainerType::NeutralNoBorder)
                    .height(100.0)
                    .width(Length::Fill),
            );

        let content_2 = Row::new()
            .push(
                container(text(""))
                    .class(ContainerType::Base100NoBorder)
                    .width(Length::Fill)
                    .height(250.0),
            )
            .push(
                container(text(""))
                    .class(ContainerType::Base200NoBorder)
                    .width(Length::Fill)
                    .height(250.0),
            )
            .push(
                container(text(""))
                    .class(ContainerType::Base300NoBorder)
                    .width(Length::Fill)
                    .height(250.0),
            );

        let content_3 = Row::new().push(
            container(text(""))
                .class(ContainerType::InfoNoBorder)
                .height(100.0)
                .width(Length::Fill),
        );

        let content = Column::new()
            .push(content_1)
            .push(content_2)
            .push(content_3)
            .spacing(15.0);

        container(content)
            .padding(15.0)
            .class(ContainerType::Bordered)
            .into()
    }
    pub(crate) fn view<'a>(&'a self) -> Element<'a, Message, StyleType> {
        container(
            text("Themes Settings view")
                .size(42)
                .class(TextType::Neutral),
        )
        .into()
    }
}
