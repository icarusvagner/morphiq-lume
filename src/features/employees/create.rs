use iced::{
	Alignment, Element, Length, Task, widget::{
		Column, Row, TextInput, button, container, horizontal_space, pick_list, text
	}
};
use iced_aw::{date_picker::Date, helpers::date_picker};

use crate::{
	core::{
		theme::fonts::{RALEWAY_BOLD, RALEWAY_SEMI_BOLD}, utils::{
			icon::Icon, messages::{
				Message, employee::{CreateEmployeeMsg, EmployeeMsg}, home::HomeMessage
			}
		}
	}, features::employees::{
		addition_fields::AdditionalFields, employee_type::{EmployeeStatus, EmployeeType}
	}, styles::{
		button::ButtonType, container::ContainerType, datepicker::DatePickerType, style_type::StyleType, text::TextType, text_input::TextInputType
	}
};

#[derive(Default, Debug, Clone)]
pub struct CreateEmployee {
	pub first_name: String,
	pub middle_name: Option<String>,
	pub last_name: String,
	pub email_add: String,
	pub employee_type: Option<EmployeeType>,
	pub employee_status: Option<EmployeeStatus>,

	/// Datepicker state
	pub date_hired: Date,
	pub end_date: Option<Date>,
	pub show_picker: bool,

	/// Show advance fields
	pub show_advance_fields: bool,
	pub advance_fields: AdditionalFields,
}

impl CreateEmployee {
	fn employee_type(&self) -> Element<'_, Message, StyleType> {
		let pick_list =
			pick_list(&EmployeeType::ALL[..], self.employee_type, |val| {
				Message::Home(HomeMessage::Employee(EmployeeMsg::Create(
					CreateEmployeeMsg::EmployeeTypeSelected(val),
				)))
			})
			.placeholder("Select employee type")
			.width(Length::Fill);

		container(pick_list)
			.padding(5.0)
			.class(ContainerType::Bordered)
			.width(Length::Fill)
			.into()
	}

	fn employee_status(&self) -> Element<'_, Message, StyleType> {
		let pick_list =
			pick_list(&EmployeeStatus::ALL[..], self.employee_status, |val| {
				Message::Home(HomeMessage::Employee(EmployeeMsg::Create(
					CreateEmployeeMsg::EmployeeStatusSelected(val),
				)))
			})
			.placeholder("Select employee status (applicant)")
			.width(Length::Fill);

		container(pick_list)
			.padding(5.0)
			.class(ContainerType::Bordered)
			.width(Length::Fill)
			.into()
	}

	fn date_pick(&self) -> Element<'_, Message, StyleType> {
		let date_btn = button(Icon::Calendar.to_text().size(18))
			.class(ButtonType::Ghost)
			.on_press(Message::Home(HomeMessage::Employee(
				EmployeeMsg::Create(CreateEmployeeMsg::ChooseDate),
			)));

		let datepicker = date_picker(
			self.show_picker,
			self.date_hired,
			date_btn,
			Message::Home(HomeMessage::Employee(EmployeeMsg::Create(
				CreateEmployeeMsg::CancelDate,
			))),
			|val| {
				Message::Home(HomeMessage::Employee(EmployeeMsg::Create(
					CreateEmployeeMsg::SubmitDate(val),
				)))
			},
		)
		.class(DatePickerType::Base300);

		let content = Row::new()
			.push(text(self.date_hired.to_string()).width(Length::Fill))
			.push(datepicker)
			.align_y(Alignment::Center);

		container(content).class(ContainerType::Bordered).padding(5.0).into()
	}

	fn first_row(&self) -> Row<'_, Message, StyleType> {
		Row::new()
			.push(
				Column::new()
					.push(text("First Name").size(16).font(RALEWAY_SEMI_BOLD))
					.push(
						TextInput::new("", &self.first_name)
							.class(TextInputType::Base200)
							.line_height(text::LineHeight::Relative(1.7)),
					)
					.spacing(5.0),
			)
			.push(
				Column::new()
					.push(text("Middle Name").size(16).font(RALEWAY_SEMI_BOLD))
					.push(
						TextInput::new(
							"",
							&self.middle_name.clone().unwrap_or_default(),
						)
						.class(TextInputType::Base200)
						.line_height(text::LineHeight::Relative(1.7)),
					)
					.spacing(5.0),
			)
			.spacing(15.0)
	}

	fn second_row(&self) -> Row<'_, Message, StyleType> {
		Row::new()
			.push(
				Column::new()
					.push(text("Last Name").size(16).font(RALEWAY_SEMI_BOLD))
					.push(
						TextInput::new("", &self.last_name)
							.class(TextInputType::Base200)
							.line_height(text::LineHeight::Relative(1.7)),
					)
					.spacing(5.0),
			)
			.push(
				Column::new()
					.push(
						text("Email Address").size(16).font(RALEWAY_SEMI_BOLD),
					)
					.push(
						TextInput::new("", &self.email_add)
							.class(TextInputType::Base200)
							.line_height(text::LineHeight::Relative(1.7)),
					)
					.spacing(5.0),
			)
			.spacing(15.0)
	}

	fn info_fields(&self) -> Element<'_, Message, StyleType> {
		let content = Column::new()
			.push(self.first_row())
			.push(self.second_row())
			.push(
				Row::new()
					.push(
						Column::new()
							.push(
								text("Employee Type")
									.size(16)
									.font(RALEWAY_SEMI_BOLD),
							)
							.push(self.employee_type())
							.spacing(5.0),
					)
					.push(
						Column::new()
							.push(
								text("Employee Status")
									.size(16)
									.font(RALEWAY_SEMI_BOLD),
							)
							.push(self.employee_status())
							.spacing(5.0),
					)
					.spacing(15.0),
			)
			.push(
				Row::new()
					.push(
						Column::new()
							.push(
								text("Date of hire")
									.size(16)
									.font(RALEWAY_SEMI_BOLD),
							)
							.push(self.date_pick())
							.spacing(5.0),
					)
					.push(horizontal_space())
					.spacing(15.0),
			)
			.spacing(15.0);

		container(content).padding(15.0).class(ContainerType::Bordered).into()
	}

	fn upload_photo(&self) -> Row<'_, Message, StyleType> {
		Row::new()
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
							Row::new()
								.push(horizontal_space())
								.push(
									Icon::CloudUpload
										.to_text()
										.size(24.0)
										.align_y(Alignment::Center)
										.align_x(Alignment::Center)
										.class(TextType::Content),
								)
								.push(
									text("Upload Photo")
										.size(18.0)
										.align_x(Alignment::Center)
										.class(TextType::Content),
								)
								.push(horizontal_space())
								.width(Length::Fill)
								.align_y(Alignment::Center)
								.spacing(2.0),
						)
						.width(Length::Fill)
						.padding(5.0)
						.height(35.0)
						.on_press(Message::Home(HomeMessage::Employee(
							EmployeeMsg::Create(CreateEmployeeMsg::UploadPhoto),
						))),
					)
					.width(Length::Fixed(250.0))
					.spacing(15.0),
			)
			.push(self.info_fields())
			.spacing(35.0)
	}

	pub fn update(
		&mut self,
		message: CreateEmployeeMsg,
	) -> Task<CreateEmployeeMsg> {
		match message {
			CreateEmployeeMsg::InputFullnameChange(fname) => {
				self.first_name = fname;
			}
			CreateEmployeeMsg::InputMiddlenameChange(mname) => {
				self.middle_name = Some(mname);
			}
			CreateEmployeeMsg::InputLastnameChange(lname) => {
				self.last_name = lname;
			}
			CreateEmployeeMsg::InputEmailAddressChange(email) => {
				self.email_add = email;
			}
			CreateEmployeeMsg::SubmitInput => {
				tracing::info!("Submit input clicked");

				// Example: simulate async saving or a success toast
				// return Task::perform(async { /* perform network request */ },
				// |_| CreateEmployeeMsg::SubmitDone);
			}
			CreateEmployeeMsg::ChooseDate => {
				self.show_picker = true;
			}
			CreateEmployeeMsg::SubmitDate(date) => {
				self.date_hired = date;
				self.show_picker = false;
			}
			CreateEmployeeMsg::CancelDate => {
				self.show_picker = false;
			}
			CreateEmployeeMsg::UploadPhoto => {
				tracing::info!("Uploading photo");
				// Example: return Task::perform(file_picker::open(),
				// CreateEmployeeMsg::PhotoUploaded);
			}
			CreateEmployeeMsg::EmployeeTypeSelected(val) => {
				self.employee_type = Some(val);
			}
			CreateEmployeeMsg::EmployeeStatusSelected(val) => {
				self.employee_status = Some(val);
			}
		}

		Task::none()
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

		let btn_submit = container(
			Row::new()
				.push(
					button(
						text("Submit")
							.size(18)
							.width(Length::Fill)
							.align_x(Alignment::Center)
							.class(TextType::Content),
					)
					.class(ButtonType::Primary),
				)
				.width(300.0),
		)
		.width(Length::Fill)
		.align_x(Alignment::End);

		content = content.push(self.upload_photo());

		content = content.push(btn_submit);

		container(content).width(Length::Fill).into()
	}
}
