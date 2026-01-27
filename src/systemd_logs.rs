use anyhow::{bail, Context, Result};
use nix::unistd::{Uid, User};
use once_cell::sync::Lazy;
use regex::Regex;
use std::process::Command;

#[derive(Default, Debug)]
pub struct ServiceInfo {
	pub name: String,
	pub description: String,
	pub triggered_by: String,
	pub fragment_path: String,
	pub invocation_id: String,
	pub start_timestamp: String,
	pub exit_timestamp: String,
	pub memory_peak: Option<u64>,
	pub cpu_usage_ns: Option<u64>,
	pub environment_str: String,
	pub environment_vals: Vec<(String, String)>,
	pub user: String,
	pub main_status: i64,
	pub result: String,
}

const JOURNAL_PROPERTIES: [&str; 13] = [
	"Names",
	"Description",
	"TriggeredBy",
	"FragmentPath",
	"InvocationID",
	"ExecMainStartTimestamp",
	"ExecMainExitTimestamp",
	"MemoryPeak",
	"CPUUsageNSec",
	"Environment",
	"User",
	"ExecMainStatus",
	"Result",
];

/// Returns \["--user"\] only if the executing user is **not** root.
/// Used when calling `systemctl` or `journalctl`.
fn user_args() -> Vec<String> {
	if Uid::current().is_root() {
		return Vec::new();
	}

	vec!["--user".to_owned()]
}

fn get_user_name() -> Result<String> {
	let user = User::from_uid(Uid::current()).context("Couldn't find user")?;
	match user {
		None => bail!("No user found"),
		Some(user) => Ok(user.name),
	}
}

pub fn does_unit_exist<S: Into<String>>(unit_name: S) -> Result<bool> {
	// suffixes listed here https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#Description
	static UNIT_SUFFIXES: Lazy<Regex> = Lazy::new(|| Regex::new(r"\.(service|socket|device|mount|automount|swap|target|path|timer|slice|scope)$").unwrap());

	let mut unit_name = unit_name.into();
	if !UNIT_SUFFIXES.is_match(&unit_name) {
		// systemctl list-unit-files needs the .service suffix for some reason
		// so we add it here if it's missing
		unit_name += ".service";
	}

	let output = Command::new("systemctl")
		.args(user_args())
		.args(["list-unit-files", "-q"])
		.arg(unit_name)
		.output()
		.context("Failed to run systemctl")?;

	Ok(output.status.success())
}

pub fn get_service_info<S: Into<String>>(service_name: S) -> Result<ServiceInfo> {
	let service_name = service_name.into();

	let output = Command::new("systemctl")
		.args(user_args())
		.args(["show", "--no-pager"])
		.args(JOURNAL_PROPERTIES.iter().flat_map(|prop| vec!["-p", prop]))
		.arg(service_name)
		.output()
		.context("Failed to run systemctl")?;

	if !output.status.success() {
		let code = output.status.code().map_or("N/A".to_string(), |code| format!("{code}"));
		bail!("systemctl exited with status code {code}")
	}

	let stdout = match String::from_utf8(output.stdout) {
		Ok(str) => str,
		Err(_) => {
			bail!("stdout contained corrupted data")
		}
	};

	let mut s_info = ServiceInfo::default();
	for line in stdout.lines() {
		match line.split_once("=") {
			Some(("Names", val)) => s_info.name = val.to_string(),
			Some(("Description", val)) => s_info.description = val.to_string(),
			Some(("TriggeredBy", val)) => s_info.triggered_by = val.to_string(),
			Some(("FragmentPath", val)) => s_info.fragment_path = val.to_string(),
			Some(("InvocationID", val)) => s_info.invocation_id = val.to_string(),
			Some(("ExecMainStartTimestamp", val)) => s_info.start_timestamp = val.to_string(),
			Some(("ExecMainExitTimestamp", val)) => s_info.exit_timestamp = val.to_string(),
			Some(("MemoryPeak", val)) => s_info.memory_peak = val.parse().ok(),
			Some(("CPUUsageNSec", val)) => s_info.cpu_usage_ns = val.parse().ok(),
			Some(("Environment", val)) => {
				s_info.environment_str = val.to_string();

				let split = shell_words::split(val).unwrap_or_default();
				s_info.environment_vals = split.iter().filter_map(|kv| kv.split_once("=")).map(|(k, v)| (k.to_string(), v.to_string())).collect();
			}
			Some(("User", val)) => s_info.user = val.to_string(),
			Some(("ExecMainStatus", val)) => s_info.main_status = val.parse()?,
			Some(("Result", val)) => s_info.result = val.to_string(),
			_ => (),
		}
	}

	// the User= field doesn't get set for user units so we'll backfill it
	if s_info.user.is_empty() {
		if let Ok(user_name) = get_user_name() {
			// append an asterisk so it's recognizable as a backfill
			s_info.user = format!("*{user_name}");
		} else {
			eprintln!("get_user_name(): failure");
		}
	}

	Ok(s_info)
}

pub fn get_invocation_logs<S: Into<String>>(invocation_id: S) -> Result<String> {
	let invocation_id = invocation_id.into();

	let output = Command::new("journalctl")
		.args(user_args())
		.args(["--no-pager", "-o", "cat"])
		.arg(format!("_SYSTEMD_INVOCATION_ID={invocation_id}"))
		.output()
		.context("Failed to run journalctl")?;

	if !output.status.success() {
		let code = output.status.code().map_or("N/A".to_string(), |code| format!("{code}"));
		bail!("journalctl exited with status code {code}")
	}

	match String::from_utf8(output.stdout) {
		Ok(str) => Ok(str),
		Err(_) => {
			bail!("stdout contained corrupted data")
		}
	}
}
