use iced::{
	Element, Length, alignment::Horizontal, widget::{container, mouse_area, opaque, stack}
};

use crate::{
	core::utils::messages::Message, styles::{container::ContainerType, style_type::StyleType}
};

pub fn modal<'a>(
	base: Element<'a, Message, StyleType>,
	content: Element<'a, Message, StyleType>,
	on_blur: Message,
) -> Element<'a, Message, StyleType> {
	stack![
		base,
		opaque(
			mouse_area(
				container(content)
					.align_x(Horizontal::Right)
					.width(Length::Fill)
					.height(Length::Fill)
					.class(ContainerType::Overlay)
			)
			.on_press(on_blur)
		)
	]
	.into()
}
