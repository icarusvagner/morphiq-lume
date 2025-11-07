#[derive(Debug, Clone, PartialEq, Eq, Copy, Default)]
pub enum EmployeeType {
	#[default]
	CallCenterAgent,
	CustomerServieRep,
	TechnicalSupportSpecialist,
	SalesAgent,
	Manager,
	ITSpecialist,
	ITSupport,
}

impl EmployeeType {
	pub const ALL: [Self; 7] = [
		Self::CallCenterAgent,
		Self::CustomerServieRep,
		Self::TechnicalSupportSpecialist,
		Self::SalesAgent,
		Self::Manager,
		Self::ITSpecialist,
		Self::ITSupport,
	];
}

impl std::fmt::Display for EmployeeType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::CallCenterAgent => "Call Center Agent",
				Self::CustomerServieRep => "Customer Service Rep",
				Self::TechnicalSupportSpecialist =>
					"Technical Support Specialist",
				Self::SalesAgent => "Sales Agents",
				Self::Manager => "Manager",
				Self::ITSpecialist => "I.T Specialist",
				Self::ITSupport => "I.T Support",
			}
		)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Default)]
pub enum EmployeeStatus {
	#[default]
	Applicant,
	Candidate,
	FullTime,
	PartTime,
	Contract,
}

impl EmployeeStatus {
	pub const ALL: [Self; 5] = [
		Self::Applicant,
		Self::Candidate,
		Self::FullTime,
		Self::PartTime,
		Self::Contract,
	];
}

impl std::fmt::Display for EmployeeStatus {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::Applicant => "Applicant",
				Self::Candidate => "Candidate",
				Self::FullTime => "Full-time",
				Self::PartTime => "Part-time",
				Self::Contract => "Contract",
			}
		)
	}
}
