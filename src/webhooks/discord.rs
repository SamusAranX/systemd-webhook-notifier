use crate::args::DiscordArgs;
use crate::systemd_logs::ServiceInfo;
use crate::webhooks::discord_structs::{EmbedField, Webhook, WebhookEmbed};
use crate::webhooks::utils::post_multipart_form;
use anyhow::{Context, Result};
use humansize::DECIMAL;
use reqwest::blocking::multipart::{Form, Part};
use std::time::Duration;

pub fn process_discord_webhook(discord_args: DiscordArgs, service_info: ServiceInfo, last_log: String) -> Result<()> {
	let mut webhook = Webhook::default();
	let mut embed = WebhookEmbed::new(discord_args.thumbnail, discord_args.color);

	embed.title = Some(service_info.name);
	embed.description = Some(service_info.description);
	embed.fields = {
		const NOT_AVAILABLE: &str = "N/A";

		let mut fields: Vec<EmbedField> = vec![];
		fields.push(EmbedField::new("Service File", service_info.fragment_path));
		fields.push(EmbedField::new_inline("Result", service_info.result));
		fields.push(EmbedField::new_inline("Exit Code", format!("{}", service_info.main_status)));

		fields.push(EmbedField::new_inline("Started", service_info.start_timestamp));
		fields.push(EmbedField::new_inline("Exited", service_info.exit_timestamp));

		fields.push(EmbedField::new_inline("User", {
			if service_info.user.is_empty() {
				NOT_AVAILABLE
			} else {
				&service_info.user
			}
		}));

		fields.push(EmbedField::new_inline("Triggered By", {
			if service_info.triggered_by.is_empty() {
				NOT_AVAILABLE
			} else {
				&service_info.triggered_by
			}
		}));

		fields.push(EmbedField::new_inline(
			"CPU Time",
			service_info.cpu_usage_ns
				.map_or(
					NOT_AVAILABLE.to_string(),
					|val| format!("{:.3}s", Duration::from_nanos(val).as_secs_f64()),
				),
		));

		fields.push(EmbedField::new_inline(
			"Memory Peak",
			service_info.memory_peak
				.map_or(
					NOT_AVAILABLE.to_string(),
					|val| humansize::format_size(val, DECIMAL),
				),
		));

		if service_info.environment_vals.is_empty() {
			fields.push(EmbedField::new("Environment Variables", service_info.environment_str));
		} else {
			let environment_var_lines = service_info.environment_vals.iter().map(|(k, v)| format!("* {k}={v}")).collect::<Vec<String>>().join("\n");
			fields.push(EmbedField::new("Environment Variables", environment_var_lines));
		}

		fields.push(EmbedField::new("Invocation ID", service_info.invocation_id));

		Some(fields)
	};

	webhook.embeds.push(embed);

	let webhook_json = serde_json::to_string(&webhook).context("Couldn't serialize webhook").unwrap();

	// prepare multipart payload
	let form = Form::new()
		.text("payload_json", webhook_json)
		.part("files[0]", Part::text(last_log).file_name("last_log.txt"));

	post_multipart_form(form, discord_args.webhook_url)
}
