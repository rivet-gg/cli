use clap::Parser;
use serde::Serialize;
use std::process::ExitCode;
use toolchain::{backend::run_backend_command_passthrough, paths};

use crate::util::global_opts::GlobalOpts;

/// Remove artifacts that Rivet generates
#[derive(Parser, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Opts {
	#[clap(flatten)]
	#[serde(flatten)]
	global: GlobalOpts,
}

impl Opts {
	pub async fn execute(&self) -> ExitCode {
		run_backend_command_passthrough("clean.ts", self, paths::BackendDataType::Dev).await
	}
}
