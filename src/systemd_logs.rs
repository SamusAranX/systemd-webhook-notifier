use anyhow::{bail, Context, Result};
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
	pub memory_peak: u64,
	pub cpu_usage_ns: u64,
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

pub fn get_service_info<S: Into<String>>(service_name: S) -> Result<ServiceInfo> {
	let service_name = service_name.into();

	let output = Command::new("systemctl")
		.args(["show", "--no-pager"])
		.args(JOURNAL_PROPERTIES.iter().flat_map(|prop| vec!["-p", prop]))
		.arg(service_name)
		.output()
		.context("failed to run journalctl")?;

	if !output.status.success() {
		bail!("wh")
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
			Some(("MemoryPeak", val)) => s_info.memory_peak = val.parse()?,
			Some(("CPUUsageNSec", val)) => s_info.cpu_usage_ns = val.parse()?,
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

	Ok(s_info)
}

pub fn get_invocation_logs<S: Into<String>>(invocation_id: S) -> Result<String> {
	let invocation_id = invocation_id.into();

	let output = Command::new("journalctl")
		.args(["--no-pager", "-o", "cat"])
		.arg(format!("_SYSTEMD_INVOCATION_ID={invocation_id}"))
		.output()
		.context("failed to run journalctl")?;

	if !output.status.success() {
		bail!("wh")
	}

	match String::from_utf8(output.stdout) {
		Ok(str) => Ok(str),
		Err(_) => {
			bail!("stdout contained corrupted data")
		}
	}
}
