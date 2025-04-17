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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct EmbedImage {
	pub url: String,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct EmbedFooter {
	pub text: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub icon_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct EmbedField {
	pub name: String,
	pub value: String,
	pub inline: bool,
}