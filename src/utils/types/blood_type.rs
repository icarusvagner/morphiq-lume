#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum BloodType {
	APositive,
	ANegative,
	BPositive,
	BNegative,
	ABPositive,
	ABNegative,
	#[default]
	OPositive,
	ONegative,
}

impl std::fmt::Display for BloodType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::APositive => "A Positive",
				Self::BPositive => "B Positive",
				Self::ABPositive => "AB Positive",
				Self::OPositive => "O Positive",
				Self::ANegative => "A Negative",
				Self::BNegative => "B Negative",
				Self::ABNegative => "AB Negative",
				Self::ONegative => "O Negative",
			}
		)
	}
}
