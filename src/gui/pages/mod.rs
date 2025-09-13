use crate::gui::pages::{home::Home, login::Login};

pub mod home;
pub mod login;

#[allow(clippy::enum_variant_names)]
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, Default)]
pub struct Pages {
    pub login: Login,
    pub home: Home,
}

#[allow(clippy::enum_variant_names)]
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, Default)]
pub enum OpenPage {
    #[default]
    Login,
    Home,
}
