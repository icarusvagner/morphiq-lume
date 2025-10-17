use iced::{
	Alignment,
	Element,
	Length,
	alignment::{
		Horizontal,
		Vertical,
	},
	widget::{
		Column,
		Container,
		Row,
		container,
		horizontal_space,
		text,
		vertical_space,
	},
};

use crate::{
	chart::types::donut_chart::donut_chart,
	gui::{
		components::cards::dashboard_card::{
			dashboard_card,
			sec_card,
		},
		styles::{
			container::ContainerType,
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

impl DashboardView {
	#[allow(clippy::unused_self)]
	pub(crate) fn view(&self) -> Element<'_, Message, StyleType> {
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
			.push(self.chart_01())
			.spacing(15.0);

		container(content).width(Length::Fill).into()
	}

	#[allow(clippy::unused_self)]
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
					(Length::Fixed(500.0), Length::Fixed(500.0)),
				))
				.push(
					container(
						Column::new()
							.push(vertical_space())
							.push(
								Row::new()
									.push(
										text("Male")
											.size(18)
											.font(RALEWAY_BOLD)
											.line_height(
												text::LineHeight::Relative(1.7),
											)
											.align_x(Horizontal::Left)
											.align_y(Vertical::Center),
									)
									.push(
										text(25)
											.size(20)
											.line_height(
												text::LineHeight::Relative(1.7),
											)
											.align_x(Horizontal::Right)
											.align_y(Vertical::Center),
									)
									.spacing(15.0),
							)
							.push(
								Row::new()
									.push(
										text("Female")
											.size(18)
											.font(RALEWAY_BOLD)
											.line_height(
												text::LineHeight::Relative(1.7),
											)
											.align_x(Horizontal::Left)
											.align_y(Vertical::Center),
									)
									.push(
										text(32)
											.size(20)
											.line_height(
												text::LineHeight::Relative(1.7),
											)
											.align_x(Horizontal::Right)
											.align_y(Vertical::Center),
									)
									.spacing(15.0),
							)
							.push(
								Row::new()
									.push(
										text("Others")
											.size(18)
											.font(RALEWAY_BOLD)
											.line_height(
												text::LineHeight::Relative(1.7),
											)
											.align_x(Horizontal::Left)
											.align_y(Vertical::Center),
									)
									.push(
										text(5)
											.size(20)
											.line_height(
												text::LineHeight::Relative(1.7),
											)
											.align_x(Horizontal::Right)
											.align_y(Vertical::Center),
									)
									.spacing(15.0),
							)
							.push(vertical_space()),
					)
					.class(ContainerType::Ghost)
					.align_y(Alignment::Center),
				)
				.height(Length::Fill),
		)
		.align_y(Alignment::Center)
		.padding(5.0)
		.class(ContainerType::Base300)
		.width(700.0)
		.height(400.0)
		.into()
	}
}
