use crate::translations::types::language::Language;

pub const fn notifications_translation(language: Language) -> &'static str {
	match language {
		Language::EN | Language::FR => "Notifications",
		Language::IT => "Notifiche",
		Language::ES => "Notificaciones",
		Language::PL => "Powiadomienia",
		Language::DE => "Benachrichtigungen",
		Language::UK => "Повідомлення",
		Language::ZH | Language::JA | Language::ZH_TW => "通知",
		Language::RO => "Notificări",
		Language::KO => "알림",
		Language::TR => "Bildirimler",
		Language::RU => "Уведомления",
		Language::PT => "Notificações",
		Language::EL => "Ειδοποιήσεις",
		// Language::FA => "اعلان ها",
		Language::SV => "Notifikationer",
		Language::FI => "Ilmoitukset",
		Language::UZ => "Bildirishnomalar",
		Language::VI => "Thông báo",
		Language::ID => "Pemberitahuan",
		Language::NL => "Notificaties",
	}
}

pub const fn style_translation(language: Language) -> &'static str {
	match language {
		Language::EN | Language::FR => "Style",
		Language::IT => "Stile",
		Language::ES | Language::PT => "Estilo",
		Language::PL => "Styl",
		Language::RO | Language::TR | Language::SV => "Stil",
		Language::DE => "Design",
		Language::UK | Language::RU => "Стиль",
		Language::ZH => "主题",
		Language::ZH_TW => "樣式",
		Language::KO => "스타일",
		Language::EL => "Στυλ",
		// Language::FA => "شیوه",
		Language::FI => "Tyyli",
		Language::JA => "スタイル",
		Language::UZ => "Uslub",
		Language::VI => "Chủ đề",
		Language::ID => "Gaya",
		Language::NL => "Stijl",
	}
}

// This is referred to settings (General settings)
pub const fn general_translation(language: Language) -> &'static str {
	#[allow(clippy::match_same_arms)]
	match language {
		Language::EN | Language::RO => "General",
		// Language::FA => "عمومی",
		Language::ES => "Generales",
		Language::IT => "Generali",
		Language::FR => "Général",
		Language::DE => "Allgemein",
		Language::PL => "Ogólne",
		Language::RU => "Общие",
		Language::JA => "一般",
		Language::UZ => "Asosiy",
		Language::SV => "Allmänt",
		Language::VI => "Tổng quan",
		Language::ZH => "通用",
		Language::ZH_TW => "一般",
		Language::KO => "일반",
		Language::TR => "Genel",
		Language::PT => "Geral",
		Language::UK => "Загальні",
		Language::ID => "Umum",
		Language::NL => "Algemeen",
		_ => "General",
	}
}
