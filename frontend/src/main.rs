use std::borrow::Cow;

use iced::{window, Font, Settings, Task};

use crate::{
    configs::configs::CONFIGS,
    gui::{
        morphiq::{Morphiq, FONT_FAMILY_NAME},
        styles::style_constant::fonts::*,
        types::message::Message,
    },
};

mod configs;
mod countries;
mod gui;
#[allow(dead_code)]
mod images;
mod translations;
mod utils;

const MORPHIQ_LOWERCASE: &str = "morphiq_lume";
const MORPHIQ_TITLECASE: &str = "Morphiq Lume";
const WINDOW_ICON: &[u8] = include_bytes!("../../assets/logos/icons/raw/icon.png");

pub fn main() -> iced::Result {
    let configs = CONFIGS.clone();
    let boot_tash_chain = window::get_latest()
        .map(Message::StartApp)
        .chain(Task::done(Message::Start));

    iced::application(MORPHIQ_TITLECASE, Morphiq::update, Morphiq::view)
        .settings(Settings {
            id: Some(String::from(MORPHIQ_LOWERCASE)),
            fonts: [
                Cow::Borrowed(ICONS_BYTES),
                Cow::Borrowed(OUTFIT_EXTRALIGHT_BYTES),
                Cow::Borrowed(OUTFIT_LIGHT_BYTES),
                Cow::Borrowed(OUTFIT_THIN_BYTES),
                Cow::Borrowed(OUTFIT_REGULAR_BYTES),
                Cow::Borrowed(OUTFIT_MEDIUM_BYTES),
                Cow::Borrowed(OUTFIT_SEMIBOLD_BYTES),
                Cow::Borrowed(OUTFIT_BLACK_BYTES),
                Cow::Borrowed(OUTFIT_BOLD_BYTES),
                Cow::Borrowed(OUTFIT_EXTRABOLD_BYTES),
                Cow::Borrowed(RALEWAY_REGULAR_BYTES),
                Cow::Borrowed(RALEWAY_BLACK_BYTES),
                Cow::Borrowed(RALEWAY_BLACK_ITALIC_BYTES),
                Cow::Borrowed(RALEWAY_BOLD_BYTES),
                Cow::Borrowed(RALEWAY_BOLD_ITALIC_BYTES),
                Cow::Borrowed(RALEWAY_EXTRA_BOLD_BYTES),
                Cow::Borrowed(RALEWAY_EXTRA_BOLD_ITALIC_BYTES),
                Cow::Borrowed(RALEWAY_EXTRA_LIGHT_BYTES),
                Cow::Borrowed(RALEWAY_EXTRA_LIGHT_ITALIC_BYTES),
                Cow::Borrowed(RALEWAY_ITALIC_BYTES),
                Cow::Borrowed(RALEWAY_LIGHT_BYTES),
                Cow::Borrowed(RALEWAY_LIGHT_ITALIC_BYTES),
                Cow::Borrowed(RALEWAY_MEDIUM_BYTES),
                Cow::Borrowed(RALEWAY_MEDIUM_ITALIC_BYTES),
                Cow::Borrowed(RALEWAY_SEMI_BOLD_BYTES),
                Cow::Borrowed(RALEWAY_SEMI_BOLD_ITALIC_BYTES),
                Cow::Borrowed(RALEWAY_THIN_BYTES),
                Cow::Borrowed(RALEWAY_THIN_ITALIC_BYTES),
            ]
            .to_vec(),
            default_font: Font::with_name(FONT_FAMILY_NAME),
            default_text_size: iced::Pixels(FONT_SIZE_BODY),
            antialiasing: true,
        })
        .window(window::Settings {
            icon: window::icon::from_file_data(
                WINDOW_ICON,
                Some(iced::advanced::graphics::image::image_rs::ImageFormat::Png),
            )
            .ok(),
            min_size: Some(iced::Size {
                width: 1120.0,
                height: 900.0,
            }),
            max_size: None,
            visible: true,
            resizable: true,
            decorations: true,
            transparent: false,
            ..Default::default()
        })
        .theme(Morphiq::theme)
        .run_with(move || (Morphiq::new(configs), boot_tash_chain))
}
