use crate::args::{Args, Commands};
use clap::Parser;
use std::process::{Command, ExitCode, Stdio};

mod webhook_structs;
mod constants;
mod args;
mod logs;

#[allow(dead_code)]
const NOTIFIER_AVATAR: &str = "https://cdn.discordapp.com/avatars/1361923984106717295/44195433717f2fb80cc095e9d9b962e5.png?size=256";

// https://discord.com/api/webhooks/1361923984106717295/V9hscUcb3TvRYT7y6WMdvg7JBpB5qUlaW1BHsILk5JwH90omQNsTVw_-6TuP7wLWWgXL

fn main() -> ExitCode {
    // preflight 1: this only runs on linux
    #[cfg(any(target_os = "windows", target_os = "macos"))]
    {
        eprintln!("This utility does not run on macOS or Windows.");
        return ExitCode::FAILURE
    }

    // preflight 2: make sure systemctl and journalctl are available
    #[allow(unreachable_code)]
    for cmd in ["systemctl", "journalctl"] {
        match Command::new(cmd).arg("--version").stdout(Stdio::null()).stderr(Stdio::null()).spawn() {
            Ok(_) => {}
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

    let service_info = match logs::get_service_info(service_name) {
        Ok(service_info) => service_info,
        Err(err) => {
            eprintln!("Couldn't get service info: {err:?}");
            return ExitCode::FAILURE;
        }
    };

    println!("service info: {:?}", &service_info);

    let last_logs = match logs::get_invocation_logs(service_info.invocation_id) {
        Ok(logs) => logs,
        Err(err) => {
            eprintln!("Couldn't get last invocation logs: {err:?}");
            return ExitCode::FAILURE;
        }
    };

    match args.command {
        Commands::Discord(discord_args) => {
            unimplemented!()
        }
        Commands::Print => {
            // eprintln!("last logs:");
            // println!("{last_logs:?}");
        }
    }

    ExitCode::SUCCESS
}
