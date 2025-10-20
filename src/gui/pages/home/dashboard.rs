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

use crate::{
	chart::types::{
		bar_chart::histogram_chart,
		donut_chart::donut_chart,
	},
	gui::{
		components::cards::dashboard_card::{
			dashboard_card,
			sec_card,
		},
		styles::{
			container::ContainerType,
			rule::RuleType,
			style_constant::{
				StandardNames,
				fonts::RALEWAY_BOLD,
			},
			types::style_type::StyleType,
		},
		types::message::Message,
	},
	utils::types::icon::Icon,
};

#[derive(Debug, Clone, Default)]
pub struct DashboardView;

#[allow(clippy::unused_self)]
impl DashboardView {
	pub fn view(&self) -> Element<'_, Message, StyleType> {
		let first_cards = container(
			Row::new()
				.push(dashboard_card(
					Icon::Layers,
					"Total Recruits".to_string(),
					12,
					ContainerType::Primary,
				))
				.push(dashboard_card(
					Icon::Sync,
					"Pending Interviews".to_string(),
					10,
					ContainerType::Secondary,
				))
				.push(dashboard_card(
					Icon::CreditCard,
					"Monthly Sales".to_string(),
					123_003,
					ContainerType::Accent,
				))
				.spacing(15.0)
				.align_y(Alignment::Center),
		)
		.height(150.0);

		let second_cards = container(
			Row::new()
				.push(sec_card(
					Icon::ShoppingCart,
					ContainerType::Icon(StandardNames::Secondary),
					"Total Order".to_string(),
					1172,
				))
				.push(sec_card(
					Icon::Repeat,
					ContainerType::Icon(StandardNames::Accent),
					"Orders Pending".to_string(),
					27,
				))
				.push(sec_card(
					Icon::Package,
					ContainerType::Icon(StandardNames::Neutral),
					"Orders Processing".to_string(),
					4,
				))
				.push(sec_card(
					Icon::Check,
					ContainerType::Icon(StandardNames::Primary),
					"Orders Delivered".to_string(),
					40,
				))
				.spacing(15.0),
		)
		.height(110.0);

		let content = Column::new()
			.push(
				text("Dashboard Overview")
					.size(32.0)
					.align_y(Alignment::Center)
					.align_x(Alignment::Start)
					.font(RALEWAY_BOLD),
			)
			.push(first_cards)
			.push(second_cards)
			.push(
				Row::new()
					.push(self.chart_01())
					.push(self.chart_02())
					.spacing(15.0),
			)
			.spacing(15.0);

		container(content).width(Length::Fill).into()
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
					"Attendance".to_string(),
					[
						"Christian Perez".to_string(),
						"Bert Casquejo".to_string(),
						"Dian Enovero".to_string(),
						"Lance Phillip Descartin".to_string(),
					]
					.to_vec(),
					[12.0, 10.0, 9.0, 17.0].to_vec(),
					RALEWAY_BOLD,
					(Length::Fill, Length::Fill),
				))
				.height(Length::Fill),
		)
		.padding(15.0)
		.align_y(Alignment::Center)
		.class(ContainerType::Base300)
		.height(500.0)
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
		.padding(30.0)
		.align_y(Alignment::Center)
		.class(ContainerType::Base300)
		.height(500.0)
		.into()
	}
}
