use crate::args::{Args, Commands};
use crate::webhooks::discord::process_discord_webhook;
use anyhow::{bail, Result};
use clap::Parser;
use std::process::{Command, Stdio};

mod args;
mod constants;
mod systemd_logs;
mod webhooks;

fn main() -> Result<()> {
	// preflight 1: this only runs on linux
	#[cfg(any(target_os = "windows", target_os = "macos"))]
	{
		bail!("This utility does not run on macOS or Windows.");
	}

	// preflight 2: make sure systemctl and journalctl are available
	#[allow(unreachable_code)]
	for cmd in ["systemctl", "journalctl"] {
		match Command::new(cmd).arg("--version").stdout(Stdio::null()).stderr(Stdio::null()).spawn() {
			Ok(_) => (),
			Err(e) => {
				bail!("[preflight] error running {cmd}: {e}");
			}
		}
	}

	let args = Args::parse();

	let service_name = args.service_name.unwrap_or(String::new());

	// preflight 3: we actually need a service name
	if service_name.trim().is_empty() {
		bail!("You must specify a service name.");
	}

	if !systemd_logs::does_unit_exist(&service_name)? {
		bail!("The specified unit \"{service_name}\" does not exist.");
	}

	let service_info = match systemd_logs::get_service_info(&service_name) {
		Ok(service_info) => service_info,
		Err(err) => {
			bail!("Couldn't get service info: {err:?}");
		}
	};

	let last_log = match systemd_logs::get_invocation_logs(&service_info.invocation_id) {
		Ok(logs) => logs,
		Err(err) => {
			bail!("Couldn't get last invocation logs: {err:?}");
		}
	};

	match args.command {
		Commands::Discord(discord_args) => process_discord_webhook(discord_args, service_info, last_log)?,
		Commands::Print(print_args) => {
			if print_args.verbose {
				println!("service info: {:?}", &service_info);
			}

			println!("{last_log}");
		}
	}

	Ok(())
}
