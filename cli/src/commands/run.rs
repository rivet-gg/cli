use crate::util::{cmd, term};
use clap::Parser;
use global_error::prelude::*;

#[derive(Parser)]
pub struct Opts {
	/// Name of the script to run
	#[clap(index = 1)]
	pub script: String,

	/// Namespace to execute script against
	#[clap(short = 'n', long)]
	pub namespace: Option<String>,

	/// Test against your local machine to iterate quickly
	#[clap(short = 'l', alias = "local", long)]
	pub this_machine: bool,

	/// Test against Rivet servers
	#[clap(short = 'r', alias = "remote", long)]
	pub rivet_servers: bool,
}

impl Opts {
	pub async fn execute(&self, ctx: &cli_core::Ctx) -> GlobalResult<()> {
		term::status::warn(
			"EXPERIMENTAL",
			"`rivet run` is experimental and subject to change",
		);

		// Read script
		let config = crate::commands::config::read_config(
			Vec::new(),
			self.namespace.as_ref().map(String::as_str),
		)
		.await?;
		let command = unwrap_with!(
			config.scripts.as_ref().and_then(|x| x.get(&self.script)),
			CLI_SCRIPT_NOT_FOUND
		);

		// Determine token
		let token = match (self.this_machine, self.rivet_servers) {
			(true, true) => {
				bail!("Cannot use both --this-machine and --rivet-servers");
			}
			(_, false) => cmd::RunWithRivetToken::ThisMachine,
			(_, true) => cmd::RunWithRivetToken::RivetServers,
		};

		// Run command
		cmd::run_with_rivet(
			ctx,
			cmd::RunWithRivetOpts {
				command,
				env: Vec::new(),
				namespace: self.namespace.as_deref(),
				token,
			},
		)
		.await?;

		Ok(())
	}
}
