use crate::args::{Args, Commands};
use crate::webhooks::discord::process_discord_webhook;
use clap::Parser;
use std::process::{Command, ExitCode, Stdio};

mod args;
mod constants;
mod systemd_logs;
mod webhooks;

fn main() -> ExitCode {
	// preflight 1: this only runs on linux
	#[cfg(not(target_os="linux"))]
	{
		eprintln!("This utility does not run on macOS or Windows.");
		return ExitCode::FAILURE;
	}

	// preflight 2: make sure systemctl and journalctl are available
	#[allow(unreachable_code)]
	for cmd in ["systemctl", "journalctl"] {
		match Command::new(cmd).arg("--version").stdout(Stdio::null()).stderr(Stdio::null()).spawn() {
			Ok(_) => (),
			Err(e) => {
				eprintln!("[preflight] error running {cmd}: {e}");
				return ExitCode::FAILURE;
			}
		}
	}

	let args = Args::parse();

	let service_name = args.service_name.unwrap_or(String::new());

	// preflight 3: we actually need a service name
	if service_name.trim().is_empty() {
		eprintln!("You must specify a service name.");
		return ExitCode::FAILURE;
	}

	match systemd_logs::does_unit_exist(&service_name) {
		Ok(exists) => {
			if !exists {
				eprintln!("The specified unit \"{service_name}\" does not exist.");
				return ExitCode::FAILURE;
			}
		}
		Err(err) => {
			eprintln!("Couldn't check whether unit exists:");
			eprintln!("{err}");
			return ExitCode::FAILURE;
		}
	}

	let service_info = match systemd_logs::get_service_info(&service_name) {
		Ok(service_info) => service_info,
		Err(err) => {
			eprintln!("Couldn't get service info: {err:?}");
			return ExitCode::FAILURE;
		}
	};

	let last_log = match systemd_logs::get_invocation_logs(&service_info.invocation_id) {
		Ok(logs) => logs,
		Err(err) => {
			eprintln!("Couldn't get last invocation logs: {err:?}");
			return ExitCode::FAILURE;
		}
	};

	match args.command {
		Commands::Discord(discord_args) => {
			match process_discord_webhook(discord_args, service_info, last_log) {
				Ok(_) => (),
				Err(err) => {
					eprintln!("Error invoking Discord webhook:");
					eprintln!("{err}");
					return ExitCode::FAILURE;
				}
			}
		},
		Commands::Print(print_args) => {
			if print_args.verbose {
				println!("service info: {:?}", &service_info);
			}

			println!("{last_log}");
		}
	}

	ExitCode::SUCCESS
}
