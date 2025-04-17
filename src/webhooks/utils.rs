use reqwest::blocking::multipart::Form;
use std::error::Error;
use std::time::Duration;

pub const NOTIFIER_REPO: &str = "https://github.com/SamusAranX/systemd-webhook-notifier";
pub const NOTIFIER_NAME: &str = "systemd-webhook-notifier";

pub(crate) fn post_multipart_form(form: Form, url: String) -> anyhow::Result<()> {
	let client = reqwest::blocking::Client::new();
	let r = client.post(url).timeout(Duration::from_secs(20)).multipart(form);

	match r.send() {
		Ok(resp) => {
			let status = resp.status();
			let resp_text = resp.text().inspect_err(|e| eprintln!("couldn't get webhook text: {e}"))?;
			println!("send_webhook: {status}");
			if !status.is_success() {
				eprintln!("status: {status}, response: {resp_text}");
				anyhow::bail!(resp_text);
			}
		}
		Err(err) => {
			eprintln!("couldn't send webhook request: {err:?}");
			if err.is_body() {
				eprintln!("error is related to request or response body");
			} else if err.is_builder() {
				eprintln!("error is from a type builder");
			} else if err.is_connect() {
				eprintln!("error is related to connect");
			} else if err.is_decode() {
				eprintln!("error is related to decoding the response body");
			} else if err.is_redirect() {
				eprintln!("error is from a RedirectPolicy");
			} else if err.is_request() {
				eprintln!("error is related to the request");
			} else if err.is_status() {
				eprintln!("error is from Response::error_from_status");
			} else if err.is_timeout() {
				eprintln!("error is related to a timeout");
			}
			if let Some(source) = err.source() {
				eprintln!("error source: {source:?}");
			}
		}
	}

	Ok(())
}
