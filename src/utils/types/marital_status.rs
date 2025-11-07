#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum MaritalStatus {
	#[default]
	Single,
	Married,
	Divorced,
	Widowed,
	Separated,
	RegisteredPartnership,
	LivingWithPartner,
	NotLivingWithPartner,
}

impl std::fmt::Display for MaritalStatus {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::Single => "Single",
				Self::Married => "Married",
				Self::Divorced => "Divorced",
				Self::Widowed => "Widowed",
				Self::Separated => "Separated",
				Self::RegisteredPartnership => "Registered partnership",
				Self::LivingWithPartner => "Living with partner",
				Self::NotLivingWithPartner => "Not living with partner",
			}
		)
	}
}
