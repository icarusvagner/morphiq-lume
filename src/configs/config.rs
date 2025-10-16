use crate::configs::{
	config_settings::ConfigSettings,
	config_window::ConfigWindow,
};

pub static CONFIGS: std::sync::LazyLock<Configs> =
	std::sync::LazyLock::new(Configs::load);

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Configs {
	pub settings: ConfigSettings,
	pub window: ConfigWindow,
}

impl Configs {
	/// This should only be use directly to load fresh configs;
	/// use `CONFIGS` instead to access the initial instance
	pub fn load() -> Self {
		Self { settings: ConfigSettings::load(), window: ConfigWindow::load() }
	}

	pub fn store(self) -> Result<(), confy::ConfyError> {
		self.settings.store()?;

		Ok(())
	}
}
