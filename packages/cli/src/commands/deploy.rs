use clap::Parser;
use std::process::ExitCode;
use toolchain::tasks::{deploy, get_bootstrap_data};

use crate::util::task::{run_task, TaskOutputStyle};

/// Build & upload the game server & backend
#[derive(Parser)]
pub struct Opts {
	environment: String,
	#[clap(long, conflicts_with = "only_modules")]
	only_game_server: bool,
	#[clap(long, conflicts_with = "only_game_server")]
	only_modules: bool,
	#[clap(long)]
	modules_skip_migrate: bool,
}

impl Opts {
	pub async fn execute(&self) -> ExitCode {
		let bootstrap_data = match run_task::<get_bootstrap_data::Task>(
			TaskOutputStyle::None,
			get_bootstrap_data::Input {},
		)
		.await
		{
			Ok(x) => x,
			Err(e) => {
				eprintln!("Error getting bootstrap: {e}");
				return ExitCode::FAILURE;
			}
		};
		let Some(cloud_data) = bootstrap_data.cloud else {
			eprintln!("Not signed in");
			return ExitCode::FAILURE;
		};

		// Find environment
		let environment = match cloud_data
			.envs
			.iter()
			.find(|env| env.slug == self.environment)
		{
			Some(env) => env,
			None => {
				eprintln!(
					"Environment '{}' not found. Available environments:",
					self.environment
				);
				for env in &cloud_data.envs {
					eprintln!("- {}", env.slug);
				}
				return ExitCode::FAILURE;
			}
		};

		match run_task::<deploy::Task>(
			TaskOutputStyle::Plain,
			deploy::Input {
				cwd: std::env::current_dir()
					.unwrap_or_default()
					.to_string_lossy()
					.to_string(),
				environment_id: environment.id,
				game_server: !self.only_modules,
				backend: !self.only_game_server,
				backend_skip_migrate: self.modules_skip_migrate,
			},
		)
		.await
		{
			Ok(_) => ExitCode::SUCCESS,
			Err(e) => {
				eprintln!("Error during deployment: {e}");
				ExitCode::FAILURE
			}
		}
	}
}
