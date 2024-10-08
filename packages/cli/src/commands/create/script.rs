use clap::Parser;
use serde::Serialize;
use std::process::ExitCode;
use toolchain::{backend::run_backend_command_passthrough, paths};

use crate::util::global_opts::GlobalOpts;

/// Create a new script for a module
#[derive(Parser, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Opts {
	#[clap(flatten)]
	#[serde(flatten)]
	global: GlobalOpts,

	#[clap(index = 1)]
	pub module: String,
	#[clap(index = 2)]
	pub script: String,
}

impl Opts {
	pub async fn execute(&self) -> ExitCode {
		run_backend_command_passthrough("create/script.ts", self, paths::BackendDataType::Dev).await
	}
}
