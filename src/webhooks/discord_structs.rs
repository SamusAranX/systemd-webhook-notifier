use crate::webhooks::utils::{NOTIFIER_NAME, NOTIFIER_REPO};
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Webhook {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub content: Option<String>,
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub embeds: Vec<WebhookEmbed>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct WebhookEmbed {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub title: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub url: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub timestamp: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub color: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub footer: Option<EmbedFooter>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub image: Option<EmbedImage>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub thumbnail: Option<EmbedImage>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub author: Option<EmbedAuthor>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub fields: Option<Vec<EmbedField>>,
}

#[allow(dead_code)]
impl WebhookEmbed {
	#[allow(clippy::field_reassign_with_default)]
	pub fn new<T: Into<String>>(thumbnail: Option<T>, color: u32) -> Self {
		let mut embed = Self::default();

		embed.author = Some(EmbedAuthor::new_with_url(NOTIFIER_NAME, NOTIFIER_REPO));
		embed.color = Some(color);

		if let Some(thumbnail) = thumbnail {
			embed.thumbnail = Some(EmbedImage::new(thumbnail));
		}

		embed.timestamp = Some(chrono::Utc::now().to_rfc3339());
		embed.footer = Some(EmbedFooter::new(NOTIFIER_NAME));

		embed
	}
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct EmbedImage {
	pub url: String,
}

#[allow(dead_code)]
impl EmbedImage {
	pub fn new<T: Into<String>>(url: T) -> Self {
		Self { url: url.into() }
	}
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct EmbedAuthor {
	pub name: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub url: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub icon_url: Option<String>,
}

#[allow(dead_code)]
impl EmbedAuthor {
	pub fn new<T: Into<String>>(name: T) -> Self {
		Self {
			name: name.into(),
			url: None,
			icon_url: None,
		}
	}

	pub fn new_with_url<T: Into<String>, T2: Into<String>>(name: T, url: T2) -> Self {
		Self {
			name: name.into(),
			url: Some(url.into()),
			icon_url: None,
		}
	}

	pub fn new_with_icon<T: Into<String>, T2: Into<String>>(name: T, icon_url: T2) -> Self {
		Self {
			name: name.into(),
			url: None,
			icon_url: Some(icon_url.into()),
		}
	}

	pub fn new_with_url_and_icon<T: Into<String>, T2: Into<String>, T3: Into<String>>(name: T, url: T2, icon_url: T3) -> Self {
		Self {
			name: name.into(),
			url: Some(url.into()),
			icon_url: Some(icon_url.into()),
		}
	}
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct EmbedFooter {
	pub text: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub icon_url: Option<String>,
}

#[allow(dead_code)]
impl EmbedFooter {
	pub fn new<T: Into<String>>(text: T) -> Self {
		Self {
			text: text.into(),
			icon_url: None,
		}
	}

	pub fn new_with_icon<T: Into<String>, T2: Into<String>>(text: T, icon_url: T2) -> Self {
		Self {
			text: text.into(),
			icon_url: Some(icon_url.into()),
		}
	}
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct EmbedField {
	pub name: String,
	pub value: String,
	pub inline: bool,
}

#[allow(dead_code)]
impl EmbedField {
	pub fn new<T: Into<String>, T2: Into<String>>(name: T, value: T2) -> Self {
		Self {
			name: name.into(),
			value: value.into(),
			inline: false,
		}
	}

	pub fn new_inline<T: Into<String>, T2: Into<String>>(name: T, value: T2) -> Self {
		Self {
			name: name.into(),
			value: value.into(),
			inline: true,
		}
	}
}
