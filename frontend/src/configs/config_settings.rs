use serde::{Deserialize, Serialize};

#[cfg(not(test))]
use crate::utils::error_logger::{ErrorLogger, Location};

#[cfg(not(test))]
use crate::{location, MORPHIQ_LOWERCASE};

use crate::{gui::styles::types::style_type::StyleType, translations::types::language::Language};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct ConfigSettings {
    pub language: Language,
    pub style_path: String,
    // StyleType should be last for deserialize as a table properly
    pub style: StyleType,
}

impl ConfigSettings {
    const FILE_NAME: &'static str = "settings";

    #[cfg(not(test))]
    pub fn load() -> Self {
        if let Ok(settings) = confy::load::<ConfigSettings>(MORPHIQ_LOWERCASE, Self::FILE_NAME) {
            settings
        } else {
            let _ = confy::store(
                MORPHIQ_LOWERCASE,
                Self::FILE_NAME,
                ConfigSettings::default(),
            )
            .log_err(location!());

            ConfigSettings::default()
        }
    }

    #[cfg(not(test))]
    pub fn store(self) -> Result<(), confy::ConfyError> {
        use crate::{location, MORPHIQ_LOWERCASE};

        confy::store(MORPHIQ_LOWERCASE, Self::FILE_NAME, self).log_err(location!())
    }
}

// impl Default for ConfigSettings {
//     fn default() -> Self {
//         ConfigSettings {
//             language: Language::default(),
//             style_path: String::new(),
//             style: StyleType::Dark,
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use crate::configs::config_settings::ConfigSettings;

    impl ConfigSettings {
        pub fn test_path() -> String {
            format!("{}/{}.toml", env!("CARGO_MANIFEST_DIR"), Self::FILE_NAME)
        }

        pub fn load() -> Self {
            confy::load_path::<ConfigSettings>(ConfigSettings::test_path())
                .unwrap_or_else(|_| ConfigSettings::default())
        }

        pub fn store(self) -> Result<(), confy::ConfyError> {
            confy::store_path(ConfigSettings::test_path(), self)
        }
    }
}
