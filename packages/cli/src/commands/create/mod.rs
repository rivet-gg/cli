pub mod module;
pub mod test;
pub mod script;

use clap::Subcommand;
use std::process::ExitCode;

/// Add functionality to backend
#[derive(Subcommand)]
pub enum SubCommand {
	Module(module::Opts),
	Script(script::Opts),
	Test(test::Opts),
}

impl SubCommand {
	pub async fn execute(&self) -> ExitCode {
		match self {
			SubCommand::Module(opts) => opts.execute().await,
			SubCommand::Script(opts) => opts.execute().await,
			SubCommand::Test(opts) => opts.execute().await,
		}
	}
}
