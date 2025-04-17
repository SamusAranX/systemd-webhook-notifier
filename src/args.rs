use crate::constants::CLAP_VERSION;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct DiscordArgs {
	// #[arg(short, long, help = "PLACEHOLDER")]
	// content: Option<String>,
	//
	// #[arg(short, long, help = "PLACEHOLDER")]
	// title: Option<String>,
	//
	// #[arg(short, long, help = "PLACEHOLDER")]
	// description: Option<String>,
	//
	// #[arg(short, long, help = "PLACEHOLDER")]
	// url: Option<String>,
	//
	// #[arg(short, help = "PLACEHOLDER", action = ArgAction::Append)]
	// files: Vec<PathBuf>,

	#[arg(short, long, help = "PLACEHOLDER")]
	thumbnail: Option<String>,

	#[arg(short, long, help = "PLACEHOLDER")]
	color: Option<u32>,

	#[arg(short, long, env = "NOTIFIER_WEBHOOK", help = "The URL to send the webhook payload to")]
	webhook_url: String,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
	#[command(about = "Sends the specified service's last invocation's logs and additional information to a Discord webhook")]
	Discord(DiscordArgs),

	#[command(about = "Outputs the specified service's last invocation's logs")]
	Print,
}

#[derive(Parser, Debug)]
#[command(version = CLAP_VERSION, about = "Enables webhook-based alerting upon service failure")]
pub(crate) struct Args {
	#[command(subcommand)]
	pub command: Commands,

	#[arg(global = true, help = "The target systemd service name")]
	pub service_name: Option<String>,
}