use serde::{Deserialize, Serialize};

use crate::styles::style_type::StyleType;
#[cfg(not(test))]
use crate::{
	MORPHIQ_LOWERCASE, core::utils::error_logger::{ErrorLogger, Location}, location
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ConfigSettings {
	pub style_path: String,
	// StyleType should be last for deserialize as a table properly
	pub style: StyleType,
	pub scale_factor: f64,
}

impl ConfigSettings {
	const FILE_NAME: &'static str = "settings";

	#[cfg(not(test))]
	pub fn load() -> Self {
		confy::load::<Self>(MORPHIQ_LOWERCASE, Self::FILE_NAME).unwrap_or_else(
			|_| {
				let _ = confy::store(
					MORPHIQ_LOWERCASE,
					Self::FILE_NAME,
					Self::default(),
				)
				.log_err(location!());
				Self::default()
			},
		)
	}

	#[cfg(not(test))]
	pub fn store(self) -> Result<(), confy::ConfyError> {
		use crate::{MORPHIQ_LOWERCASE, location};

		confy::store(MORPHIQ_LOWERCASE, Self::FILE_NAME, self)
			.log_err(location!())
	}
}

impl Default for ConfigSettings {
	fn default() -> Self {
		Self {
			style_path: String::new(),
			style: StyleType::default(),
			scale_factor: 1.0,
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::core::utils::configs::config_settings::ConfigSettings;

	impl ConfigSettings {
		pub fn test_path() -> String {
			format!("{}/{}.toml", env!("CARGO_MANIFEST_DIR"), Self::FILE_NAME)
		}

		pub fn load() -> Self {
			confy::load_path::<Self>(Self::test_path())
				.unwrap_or_else(|_| Self::default())
		}

		pub fn store(self) -> Result<(), confy::ConfyError> {
			confy::store_path(Self::test_path(), self)
		}
	}
}
