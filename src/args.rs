use crate::constants::CLAP_VERSION;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct DiscordArgs {
	#[arg(short, long, help = "PLACEHOLDER")]
	pub thumbnail: Option<String>,

	#[arg(short, long, default_value_t = 0x5E5CE6, help = "PLACEHOLDER")]
	pub color: u32,

	#[arg(short, long, env = "DISCORD_WEBHOOK", help = "The URL to send the webhook payload to")]
	pub webhook_url: String,
}

#[derive(Parser, Debug)]
pub struct PrintArgs {
	#[arg(short, long, help = "Additionally prints the contents of the ServiceInfo struct")]
	pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
	#[command(about = "Sends the specified service's last invocation's logs and additional information to a Discord webhook")]
	Discord(DiscordArgs),

	#[command(about = "Outputs the specified service's last invocation's logs")]
	Print(PrintArgs),
}

#[derive(Parser, Debug)]
#[command(version = CLAP_VERSION, about = "Enables webhook-based alerting upon service failure")]
pub(crate) struct Args {
	#[command(subcommand)]
	pub command: Commands,

	#[arg(global = true, help = "The target systemd service name")]
	pub service_name: Option<String>,
}
