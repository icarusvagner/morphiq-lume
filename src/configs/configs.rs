use crate::configs::config_settings::ConfigSettings;

pub static CONFIGS: std::sync::LazyLock<Configs> = std::sync::LazyLock::new(Configs::load);

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Configs {
    pub settings: ConfigSettings,
}

impl Configs {
    /// This should only be use directly to load fresh configs;
    /// use `CONFIGS` instead to access the initial instance
    pub fn load() -> Self {
        Configs {
            settings: ConfigSettings::load(),
        }
    }

    pub fn store(self) -> Result<(), confy::ConfyError> {
        self.settings.store()?;

        Ok(())
    }
}
