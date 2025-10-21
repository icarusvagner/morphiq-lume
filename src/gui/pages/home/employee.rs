use iced::{
	Alignment,
	Element,
	Length,
	Pixels,
	alignment::{
		Horizontal,
		Vertical,
	},
	widget::{
		Column,
		Row,
		container,
		horizontal_space,
		text,
		vertical_rule,
		vertical_space,
	},
};
use mockd::{
	job,
	name,
	unique,
};
use rand::{
	Rng,
	rng,
};

use crate::{
	chart::types::{
		bar_chart::histogram_chart,
		donut_chart::donut_chart,
	},
	gui::{
		pages::home::panes::tables::employee_table::{
			EmployeeTable,
			RowTable,
		},
		styles::{
			container::ContainerType,
			rule::RuleType,
			style_constant::fonts::RALEWAY_BOLD,
			types::style_type::StyleType,
		},
		types::message::Message,
	},
};

#[derive(Clone, Debug)]
pub struct EmployeeView {
	pub employee_table: EmployeeTable,
}

impl Default for EmployeeView {
	fn default() -> Self {
		let interaction: [String; 2] =
			[String::from("Clock In"), String::from("Clock Out")];
		let rand_num = rng().random_range(0..=1);
		let rand_hours = rng().random_range(1..=10);

		let statuses: [String; 4] = [
			String::from("Active"),
			String::from("Inactive"),
			String::from("Late"),
			String::from("Onboarding"),
		];
		let rand_num_2 = rng().random_range(0..=3);

		let employee_table = EmployeeTable {
			rows: [
				RowTable {
					id_num: unique::uuid_v4(),
					full_name: name::full(),
					position: job::title(),
					department: job::descriptor(),
					interaction: interaction[rand_num].clone(),
					work_hours: format!("{rand_hours} HRS"),
					status: statuses[rand_num_2].clone(),
				},
				RowTable {
					id_num: unique::uuid_v4(),
					full_name: name::full(),
					position: job::title(),
					department: job::descriptor(),
					interaction: interaction[rand_num].clone(),
					work_hours: format!("{rand_hours} HRS"),
					status: statuses[rand_num_2].clone(),
				},
				RowTable {
					id_num: unique::uuid_v4(),
					full_name: name::full(),
					position: job::title(),
					department: job::descriptor(),
					interaction: interaction[rand_num].clone(),
					work_hours: format!("{rand_hours} HRS"),
					status: statuses[rand_num_2].clone(),
				},
				RowTable {
					id_num: unique::uuid_v4(),
					full_name: name::full(),
					position: job::title(),
					department: job::descriptor(),
					interaction: interaction[rand_num].clone(),
					work_hours: format!("{rand_hours} HRS"),
					status: statuses[rand_num_2].clone(),
				},
				RowTable {
					id_num: unique::uuid_v4(),
					full_name: name::full(),
					position: job::title(),
					department: job::descriptor(),
					interaction: interaction[rand_num].clone(),
					work_hours: format!("{rand_hours} HRS"),
					status: statuses[rand_num_2].clone(),
				},
			]
			.to_vec(),
			..Default::default()
		};

		Self { employee_table }
	}
}

#[allow(clippy::unused_self)]
impl EmployeeView {
	pub fn view(&self) -> Element<'_, Message, StyleType> {
		let content = Column::new()
			.push(
				text("Employee Overview")
					.size(32.0)
					.align_y(Alignment::Center)
					.align_x(Alignment::Start)
					.font(RALEWAY_BOLD),
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
			.push(self.employee_table.view())
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
