use anyhow::Result;
use clap::Parser;
use console::Term;

pub mod unreal;

#[derive(Parser)]
pub enum SubCommand {
	Unreal {#[clap(subcommand)]command: unreal::SubCommand },
}

impl SubCommand {
	pub async fn execute(&self,term:&Term, ctx: &cli_core::Ctx) -> Result<()> {
		match self {
			SubCommand::Unreal{command} => command.execute(term, ctx).await,
		}
	}
}
