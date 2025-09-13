use iced::{Degrees, Gradient};
use serde::{Deserialize, Serialize};

use crate::gui::styles::{style_constant::colors::Colors, types::palette::mix_colors};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GradientType {
    /// A harmonious color gradient
    Mild,
    /// A crazy yet good-looking color gradient
    Wild,
    /// No gradient applied
    #[default]
    None,
}

impl GradientType {
    pub fn set(&self) -> Gradient {
        Gradient::Linear(
            iced::gradient::Linear::new(Degrees(135.0))
                .add_stop(0.0, Colors::UTOrange.get())
                .add_stop(
                    1.0,
                    match self {
                        Self::Mild => {
                            mix_colors(Colors::UTOrange.get(), Colors::SelectiveYellow.get())
                        }
                        Self::Wild => Colors::SelectiveYellow.get(),
                        _ => Colors::SelectiveYellow.get(),
                    },
                ),
        )
    }
}
