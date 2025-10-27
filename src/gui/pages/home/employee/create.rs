use iced::{
	Alignment,
	Element,
	Length,
	widget::{
		Column,
		Row,
		TextInput,
		button,
		container,
		text,
	},
};

use crate::gui::{
	styles::{
		container::ContainerType,
		style_constant::fonts::{
			RALEWAY_BOLD,
			RALEWAY_SEMI_BOLD,
		},
		text_input::TextInputType,
		types::style_type::StyleType,
	},
	types::message::Message,
};

#[derive(Default, Debug, Clone)]
pub struct CreateEmployee {
	pub first_name: String,
	pub middle_name: String,
	pub last_name: String,
	pub email_add: String,
	pub employee_type: String,
	pub employee_status: String,
	pub date_hired: Option<String>,
	// Additional fields
	pub department: String,
	pub job_title: String,
	pub location: String,
	pub reporting_to: String,
	pub source_of_hire: String,
	pub pay_rate: u64,
}

impl CreateEmployee {
	fn info_fields(&self) -> Element<'_, Message, StyleType> {
		let content = Column::new()
			.push(
				Row::new()
					.push(
						Column::new()
							.push(
								text("First Name")
									.size(16)
									.font(RALEWAY_SEMI_BOLD),
							)
							.push(
								TextInput::new("", &self.first_name)
									.class(TextInputType::Base200)
									.line_height(text::LineHeight::Relative(
										1.7,
									)),
							)
							.spacing(5.0),
					)
					.push(
						Column::new()
							.push(
								text("Middle Name")
									.size(16)
									.font(RALEWAY_SEMI_BOLD),
							)
							.push(
								TextInput::new("", &self.middle_name)
									.class(TextInputType::Base200)
									.line_height(text::LineHeight::Relative(
										1.7,
									)),
							)
							.spacing(5.0),
					)
					.spacing(15.0),
			)
			.push(
				Row::new()
					.push(
						Column::new()
							.push(
								text("Last Name")
									.size(16)
									.font(RALEWAY_SEMI_BOLD),
							)
							.push(
								TextInput::new("", &self.last_name)
									.class(TextInputType::Base200)
									.line_height(text::LineHeight::Relative(
										1.7,
									)),
							)
							.spacing(5.0),
					)
					.push(
						Column::new()
							.push(
								text("Email Address")
									.size(16)
									.font(RALEWAY_SEMI_BOLD),
							)
							.push(
								TextInput::new("", &self.email_add)
									.class(TextInputType::Base200)
									.line_height(text::LineHeight::Relative(
										1.7,
									)),
							)
							.spacing(5.0),
					)
					.spacing(15.0),
			);

		container(content).padding(15.0).class(ContainerType::Bordered).into()
	}

	pub fn view(&self) -> Element<'_, Message, StyleType> {
		let mut content = Column::new()
			.push(
				text("New Employee")
					.size(24)
					.font(RALEWAY_BOLD)
					.width(Length::Fill)
					.align_y(Alignment::Center),
			)
			.spacing(5.0);

		let sub_content = Row::new()
			.push(
				Column::new()
					.push(
						container(text(""))
							.class(ContainerType::Base100)
							.height(200.0)
							.width(Length::Fill),
					)
					.push(
						button(
							text("Upload Photo")
								.width(Length::Fill)
								.align_x(Alignment::Center),
						)
						.width(Length::Fill)
						.height(35.0),
					)
					.width(Length::Fixed(250.0))
					.spacing(15.0),
			)
			.push(self.info_fields());

		content = content.push(sub_content);

		container(content).width(Length::Fill).into()
	}
}
