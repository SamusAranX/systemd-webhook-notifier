use anyhow::{bail, Context, Result};
use std::collections::HashMap;
use std::process::Command;

#[derive(Default, Debug)]
pub struct ServiceInfo {
	pub invocation_id: String,
	pub start_timestamp: String,
	pub exit_timestamp: String,
	pub memory_peak: u64,
	pub cpu_usage_ns: u64,
	pub environment_str: String,
	pub environment_map: HashMap<String, String>,
	pub user: String,
	pub status_errno: u64,
	pub result: String,
}

const JOURNAL_PROPERTIES: [&str; 9] = [
	"InvocationID",
	"ExecMainStartTimestamp",
	"ExecMainExitTimestamp",
	"MemoryPeak",
	"CPUUsageNSec",
	"Environment",
	"User",
	"StatusErrno",
	"Result",
];

pub fn get_service_info(service_name: String) -> Result<ServiceInfo> {
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
		Err(_) => { bail!("stdout contained corrupted data") }
	};

	let mut s_info = ServiceInfo::default();
	for line in stdout.lines() {
		match line.split_once("=") {
			Some(("InvocationID", val)) => { s_info.invocation_id = val.to_string() }
			Some(("ExecMainStartTimestamp", val)) => { s_info.start_timestamp = val.to_string() }
			Some(("ExecMainExitTimestamp", val)) => { s_info.exit_timestamp = val.to_string() }
			Some(("MemoryPeak", val)) => { s_info.memory_peak = val.parse()? }
			Some(("CPUUsageNSec", val)) => { s_info.cpu_usage_ns = val.parse()? }
			Some(("Environment", val)) => {
				s_info.environment_str = val.to_string();

				let split = shell_words::split(val).unwrap_or_default();
				s_info.environment_map = HashMap::from_iter(
					split
						.iter().map(|kv| kv.split_once("="))
						.flatten()
						.map(|(k, v)| (k.to_string(), v.to_string()))
				);
			}
			Some(("User", val)) => { s_info.user = val.to_string() }
			Some(("StatusErrno", val)) => { s_info.status_errno = val.parse()? }
			Some(("Result", val)) => { s_info.result = val.to_string() }
			_ => (),
		}
	}

	Ok(s_info)
}

pub fn get_invocation_logs(invocation_id: String) -> Result<String> {
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
		Err(_) => { bail!("stdout contained corrupted data") }
	}
}