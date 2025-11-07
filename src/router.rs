use crate::features::{auth::login::Login, home::Home};

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
