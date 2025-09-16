use iced::{widget::container, window, Element, Task};

use crate::{
    configs::configs::Configs,
    gui::{
        pages::{home::HomeMessage, login::LoginMessage, OpenPage, Pages},
        styles::types::{
            custom_palette::{CustomPalette, ExtraStyles},
            palette::Palette,
            style_type::StyleType,
        },
        types::message::Message,
    },
};

pub const ICON_FONT_FAMILY_NAME: &str = "Icons for Morphiq Lume";
pub const FONT_FAMILY_NAME: &str = "Outfit";
pub const SVG_FULLLOGO_BYTES: &[u8] = include_bytes!("../../assets/logos/icons/full/icon_full.svg");
pub const SVG_EMBLEMLOGO_BYTES: &[u8] = include_bytes!("../../assets/logos/icons/icon_macros.svg");

#[derive(Debug, Clone)]
pub struct Morphiq {
    /// Window ID
    pub id: Option<window::Id>,
    /// Application's configurations: settings and more to come
    pub configs: Configs,
    /// The default page to view
    pub page: Pages,
    /// Change the running page
    pub open_page: OpenPage,
}

impl Morphiq {
    pub fn new(configs: Configs) -> Self {
        Self {
            configs,
            id: None,
            page: Pages::default(),
            open_page: OpenPage::default(),
        }
    }

    pub fn theme(&self) -> StyleType {
        self.configs.settings.style
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::StartApp(id) => {
                self.id = id;

                // Performs some task
            }
            Message::TickDashboard => {}
            Message::Style(style) => {
                self.configs.settings.style = style;
            }
            Message::LoadStyle(path) => {
                self.configs.settings.style_path.clone_from(&path);
                if let Ok(palette) = Palette::from_file(&path) {
                    let style = StyleType::Custom(ExtraStyles::CustomToml(
                        CustomPalette::from_palette(palette),
                    ));

                    self.configs.settings.style = style;
                }
            }
            Message::CloseSettings => {}
            Message::ChangeVolume(_) => {}
            Message::Quit => {
                let _ = self.configs.clone().store();
                return window::close(self.id.unwrap_or_else(window::Id::unique));
            }
            Message::FetchChats => {}
            Message::Login(login_msg) => {
                if let LoginMessage::LoginSubmitted = login_msg {
                    self.open_page = OpenPage::Home;
                } else {
                    self.page.login.update(login_msg);
                }
            }
            Message::Home(home_msg) => {
                if let HomeMessage::Logout = home_msg {
                    self.open_page = OpenPage::Login;
                } else {
                    self.page.home.update(home_msg);
                }
            }
            _ => {}
        }
        Task::none()
    }

    pub fn view<'a>(&'a self) -> Element<'a, Message, StyleType> {
        let content = match self.open_page {
            OpenPage::Login => self.page.login.view(),
            OpenPage::Home => self.page.home.view(self),
        };

        container(content).padding(12.0).into()
    }
}
