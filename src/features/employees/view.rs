use iced::{
	Alignment, Element, Length, Pixels, Task, alignment::{Horizontal, Vertical}, widget::{
		Column, Row, container, horizontal_space, text, vertical_rule, vertical_space
	}
};

use crate::{
	core::{theme::fonts::RALEWAY_BOLD, utils::message::Message}, features::{
		employees::{create::CreateEmployee, employee_msg::EmployeeMsg}, tables::employee::{employee::GenTableEmployee, table::EmployeeRow}
	}, morphiq::Morphiq, styles::{container::ContainerType, style_type::StyleType}
};

#[derive(Clone, Debug)]
pub struct EmployeeView {
	pub table: GenTableEmployee,
	pub create: CreateEmployee,
}

impl Default for EmployeeView {
	fn default() -> Self {
		let table = GenTableEmployee::new(
			"Employee List".to_string(),
			(0..40).map(|_| EmployeeRow::generate_sample()).collect(),
		);

		Self { table, create: CreateEmployee::default() }
	}
}

#[allow(clippy::unused_self)]
impl EmployeeView {
	pub fn update(&mut self, message: EmployeeMsg) -> Task<EmployeeMsg> {
		match message {
			EmployeeMsg::Create(create_msg) => {
				self.create.update(create_msg).map(EmployeeMsg::Create)
			}
			EmployeeMsg::Table(tbl_msg) => {
				self.table.update(tbl_msg).map(EmployeeMsg::Table)
			}
		}
	}

	pub fn view(&self, morphiq: &Morphiq) -> Element<'_, Message, StyleType> {
		let content = Column::new()
			.push(
				Row::new()
					.push(
						text("Employee Overview")
							.size(32.0)
							.align_y(Alignment::Center)
							.align_x(Alignment::Start)
							.font(RALEWAY_BOLD),
					)
					.align_y(Alignment::Center),
			)
			.push(
				Row::new()
					.push(
						container(
							Column::new()
								.push(
									text("Total Genders")
										.size(18.0)
										.align_y(Alignment::Center)
										.align_x(Alignment::Center)
										.font(RALEWAY_BOLD)
										.width(Length::Fill),
								)
								.push(self.chart_01()),
						)
						.padding(15.0)
						.class(ContainerType::Base300),
					)
					.push(self.chart_02())
					.height(500.0)
					.spacing(15.0),
			)
			.push(self.table.view(morphiq))
			.spacing(15.0);

		container(content).into()
	}

	fn labels(
		&self,
		rule_type: RuleType,
		title: String,
		count: u32,
	) -> Element<'_, Message, StyleType> {
		Row::new()
			.push(vertical_rule(Pixels(5.0)).class(rule_type))
			.push(
				Row::new()
					.push(
						text(title)
							.size(18)
							.font(RALEWAY_BOLD)
							.line_height(text::LineHeight::Relative(1.7))
							.align_x(Horizontal::Left)
							.align_y(Vertical::Center),
					)
					.push(horizontal_space())
					.push(
						text(count)
							.size(20)
							.line_height(text::LineHeight::Relative(1.7))
							.align_x(Horizontal::Right)
							.align_y(Vertical::Center),
					),
			)
			.spacing(15.0)
			.padding(5)
			.align_y(Alignment::Center)
			.into()
	}

	fn chart_02(&self) -> Element<'_, Message, StyleType> {
		container(
			Row::new()
				.push(histogram_chart(
					"Top 4 Performer".to_string(),
					[
						"Christian Perez".to_string(),
						"Bert Casquejo".to_string(),
						"Dian Enovero".to_string(),
						"Lance Phillip Descartin".to_string(),
						"Darven Jay Tibon".to_string(),
					]
					.to_vec(),
					[12.0, 10.0, 9.0, 17.0, 15.0].to_vec(),
					RALEWAY_BOLD,
					(Length::Fill, Length::Fill),
				))
				.height(Length::Fill),
		)
		.padding(15.0)
		.align_y(Alignment::Center)
		.class(ContainerType::Base300)
		.height(Length::Fill)
		.into()
	}

	fn chart_01(&self) -> Element<'_, Message, StyleType> {
		container(
			Row::new()
				.push(donut_chart(
					"Genders".to_string(),
					RALEWAY_BOLD,
					[
						"Male".to_string(),
						"Female".to_string(),
						"Other".to_string(),
					]
					.to_vec(),
					[25, 32, 5].to_vec(),
					(Length::Fixed(380.0), Length::Fill),
				))
				.push(
					container(
						Column::new()
							.push(vertical_space())
							.push(self.labels(
								RuleType::Primary,
								"Male".to_string(),
								25,
							))
							.push(self.labels(
								RuleType::Secondary,
								"Female".to_string(),
								32,
							))
							.push(self.labels(
								RuleType::Accent,
								"Others".to_string(),
								5,
							))
							.spacing(10.0)
							.push(vertical_space()),
					)
					.class(ContainerType::Ghost)
					.height(Length::Fill)
					.width(Length::FillPortion(2)),
				)
				.spacing(20.0)
				.height(Length::Fill),
		)
		.align_y(Alignment::Center)
		.height(Length::Fill)
		.class(ContainerType::Ghost)
		.into()
	}
}
