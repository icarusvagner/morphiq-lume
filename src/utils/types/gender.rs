#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenderType {
	Male,
	Female,
	Others,
}

impl std::fmt::Display for GenderType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::Male => "Male",
				Self::Female => "Female",
				Self::Others => "Others",
			}
		)
	}
}
