#[derive(Debug, Clone)]
pub enum TableMessage {
	Dashboard(DashboardTableMsg),
	Employee(EmployeeTableMsg),
}

#[derive(Debug, Clone)]
pub enum DashboardTableMsg {
	Search(String),
	FilteredBy(FilterEmployee),
	Todo1,
	Todo2,
}

#[derive(Debug, Clone)]
pub enum EmployeeTableMsg {
	Search(String),
	FilteredBy(FilterEmployee),
	Todo1,
	Todo2,
}

#[derive(Debug, Clone)]
pub enum FilterEmployee {
	Department,
	IdNumber,
	Fullname,
	Status,
}
