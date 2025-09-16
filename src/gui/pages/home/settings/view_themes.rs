use iced::{
    widget::{
        container, scrollable::Direction, text, Button, Column, Container, Row, Rule, Scrollable,
        Space,
    },
    Alignment, Element, Length, Padding,
};

use crate::{
    configs::config_settings::ConfigSettings,
    gui::{
        morphiq::Morphiq,
        styles::{
            button::ButtonType,
            container::ContainerType,
            rule::RuleType,
            scrollbar::ScrollbarType,
            style_constant::BORDER_WIDTH,
            types::{custom_palette::ExtraStyles, palette::Palette, style_type::StyleType},
        },
        types::message::Message,
    },
};

#[derive(Default, Debug, Clone)]
pub struct ViewThemes;

impl ViewThemes {
    pub(crate) fn view<'a>(&'a self, morphiq: &Morphiq) -> Element<'a, Message, StyleType> {
        let ConfigSettings { style, .. } = morphiq.configs.settings.clone();

        let mut styles_col = Column::new()
            .align_x(Alignment::Center)
            .width(Length::Fill)
            .push(
                Row::new()
                    .push(self.get_palette_container(style, "Night".to_string(), StyleType::Night))
                    .push(Space::with_width(15))
                    .push(self.get_palette_container(style, "Nord".to_string(), StyleType::Nord)),
            )
            .push(Space::with_height(15))
            .push(
                Row::new()
                    .push(self.get_palette_container(style, "Silk".to_string(), StyleType::Silk))
                    .push(Space::with_width(15))
                    .push(self.get_palette_container(style, "Abyss".to_string(), StyleType::Abyss)),
            )
            .push(Space::with_height(15));

        for children in self.get_extra_palettes(ExtraStyles::all_styles(), style) {
            styles_col = styles_col.push(children);
        }

        let styles_scroll = Scrollable::with_direction(
            styles_col,
            Direction::Vertical(ScrollbarType::properties().margin(10)),
        );

        let content = Column::new().push(styles_scroll);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn get_extra_palettes<'a>(
        &'a self,
        styles: &[ExtraStyles],
        current_style: StyleType,
    ) -> Vec<Element<'a, Message, StyleType>> {
        let mut styles = styles.iter().map(|&style| {
            let name = style.to_string();
            let style = StyleType::Custom(style);
            self.get_palette_container(current_style, name, style)
        });

        let mut children = Vec::with_capacity(styles.len());

        while let (Some(first), second) = (styles.next(), styles.next()) {
            if let Some(second) = second {
                children.extend([
                    Row::new()
                        .push(first)
                        .push(Space::with_width(15))
                        .push(second)
                        .into(),
                    <Space as Into<Element<'a, Message, StyleType>>>::into(Space::with_height(15)),
                ]);
            } else {
                children.extend([
                    Row::new().push(first).into(),
                    <Space as Into<Element<'a, Message, StyleType>>>::into(Space::with_height(15)),
                ]);
            }
        }

        children
    }

    fn get_palette_container<'a>(
        &'a self,
        style: StyleType,
        name: String,
        on_press: StyleType,
    ) -> Button<'a, Message, StyleType> {
        let font = style.get_extension().font;

        let caption = Column::new()
            .push(text(name).size(16).font(font))
            .align_x(Alignment::End);

        let content = Column::new()
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .spacing(5)
            .push(caption)
            .push(self.get_palette_rule(on_press.get_palette()));

        Button::new(content)
            .width(Length::Fill)
            .padding(Padding::from(10))
            .class(ButtonType::GhostHovered)
            .on_press(Message::Style(on_press))
    }

    fn get_palette_rule<'a>(&'a self, palette: Palette) -> Container<'a, Message, StyleType> {
        let height = 65;

        Container::new(
            Row::new()
                .push(Row::new().width(120.0).push(
                    Rule::horizontal(height).class(RuleType::PaletteColor(palette.primary, height)),
                ))
                .push(
                    Row::new().width(120.0).push(
                        Rule::horizontal(height)
                            .class(RuleType::PaletteColor(palette.secondary, height)),
                    ),
                )
                .push(Row::new().width(120.0).push(
                    Rule::horizontal(height).class(RuleType::PaletteColor(palette.accent, height)),
                ))
                .push(Row::new().width(120.0).push(
                    Rule::horizontal(height).class(RuleType::PaletteColor(palette.neutral, height)),
                )),
        )
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .width(300.0 + 2.0 * BORDER_WIDTH)
        .height(f32::from(height) + 1.7 * BORDER_WIDTH)
        .class(ContainerType::Ghost)
    }
}
