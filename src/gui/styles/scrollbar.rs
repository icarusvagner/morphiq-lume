use iced::{
    widget::{
        container,
        scrollable::{Catalog, Rail, Scrollbar, Scroller, Status, Style},
    },
    Background, Border, Color,
};

use crate::gui::styles::{
    style_constant::BORDER_ROUNDED_RADIUS,
    types::{palette::mix_colors, style_type::StyleType},
};

#[derive(Default)]
pub enum ScrollbarType {
    #[default]
    Standard,
}

impl ScrollbarType {
    #[allow(clippy::unused_self)]
    fn active(&self, style: &StyleType) -> Style {
        let colors = style.get_palette();
        let ext = style.get_extension();

        let rail = Rail {
            background: Some(Background::Color(Color::TRANSPARENT)),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: BORDER_ROUNDED_RADIUS.into(),
            },
            scroller: Scroller {
                color: colors.base_300,
                border: Border {
                    color: colors.base_100,
                    width: ext.size_fields,
                    radius: ext.radius_boxes.into(),
                },
            },
        };

        Style {
            container: container::Style::default(),
            vertical_rail: rail,
            horizontal_rail: rail,
            gap: None,
        }
    }

    #[allow(clippy::unused_self)]
    fn hovered(&self, style: &StyleType, is_mouse_over_x: bool, is_mouse_over_y: bool) -> Style {
        let colors = style.get_palette();
        let ext = style.get_extension();

        let [horizontal_rail, vertical_rail] =
            [is_mouse_over_x, is_mouse_over_y].map(|is_over| Rail {
                background: Some(Background::Color(Color {
                    a: ext.alpha_round_borders,
                    ..colors.base_200
                })),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: ext.size_fields.into(),
                },
                scroller: Scroller {
                    color: if is_over {
                        colors.primary
                    } else {
                        mix_colors(colors.primary, ext.buttons_color)
                    },
                    border: Border {
                        color: colors.primary,
                        width: 0.0,
                        radius: ext.radius_boxes.into(),
                    },
                },
            });

        Style {
            container: container::Style::default(),
            vertical_rail,
            horizontal_rail,
            gap: None,
        }
    }

    fn properties() -> Scrollbar {
        Scrollbar::new().width(5).scroller_width(5).margin(3)
    }
}

impl Catalog for StyleType {
    type Class<'a> = ScrollbarType;

    fn default<'a>() -> Self::Class<'a> {
        Self::Class::default()
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        match status {
            Status::Active => class.active(self),
            Status::Hovered {
                is_horizontal_scrollbar_hovered,
                is_vertical_scrollbar_hovered,
            } => class.hovered(
                self,
                is_horizontal_scrollbar_hovered,
                is_vertical_scrollbar_hovered,
            ),
            Status::Dragged {
                is_horizontal_scrollbar_dragged,
                is_vertical_scrollbar_dragged,
            } => class.hovered(
                self,
                is_horizontal_scrollbar_dragged,
                is_vertical_scrollbar_dragged,
            ),
        }
    }
}
