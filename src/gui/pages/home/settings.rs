mod view_all;
mod view_languages;
mod view_org;

use view_all::ViewAll;
use view_languages::ViewLanguages;
use view_org::ViewOrganization;

use iced::{widget::container, Element};

use crate::gui::{morphiq::Morphiq, styles::types::style_type::StyleType, types::message::Message};

#[derive(Default, Clone, Debug)]
pub struct SettingsView {
    pub view_all: ViewAll,
    pub view_languages: ViewLanguages,
    pub view_org: ViewOrganization,
}

#[allow(clippy::enum_variant_names)]
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq, Copy, Default)]
pub enum OpenSettings {
    #[default]
    All,
    Languages,
    OrgSettings,
}

impl SettingsView {
    pub(crate) fn view<'a>(
        &'a self,
        view: OpenSettings,
        morphiq: &Morphiq,
    ) -> Element<'a, Message, StyleType> {
        let content = self.to_view(view, morphiq);

        container(content).into()
    }

    fn to_view<'a>(
        &'a self,
        view: OpenSettings,
        _morphiq: &Morphiq,
    ) -> Element<'a, Message, StyleType> {
        match view {
            OpenSettings::All => self.view_all.view(),
            OpenSettings::Languages => self.view_languages.view(),
            OpenSettings::OrgSettings => self.view_org.view(),
        }
    }
}
