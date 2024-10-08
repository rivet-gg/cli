use clap::Parser;
use serde_json::json;
use std::process::ExitCode;
use toolchain::{game_server, rivet_api::apis};

/// Get the current game version
#[derive(Parser)]
pub struct Opts {
	environment: String,
}

impl Opts {
	pub async fn execute(&self) -> ExitCode {
		let ctx = match toolchain::toolchain_ctx::load().await {
			Ok(x) => x,
			Err(_) => {
				eprintln!("failed to get ctx");
				return ExitCode::FAILURE;
			}
		};

		let env = match apis::cloud_games_api::cloud_games_get_game_by_id(
			&ctx.openapi_config_cloud,
			&ctx.game_id.to_string(),
			None,
		)
		.await
		{
			Ok(x) => x,
			Err(err) => {
				eprintln!("failed to get environments: {err}");
				return ExitCode::FAILURE;
			}
		};

		if let Some(env) = env
			.game
			.namespaces
			.iter()
			.find(|x| x.name_id == self.environment)
		{
			// Fetch build with the current tag & select first one
			let tags = serde_json::to_string(&json!({
				game_server::CURRENT_BUILD_TAG: "true"
			}))
			.unwrap_or_default();

			let current_build = match apis::servers_builds_api::servers_builds_list(
				&ctx.openapi_config_cloud,
				&ctx.game_id.to_string(),
				&env.namespace_id.to_string(),
				Some(&tags),
			)
			.await
			{
				Ok(builds) => builds.builds.into_iter().next(),
				Err(err) => {
					eprintln!("failed to get current build: {err}");
					return ExitCode::FAILURE;
				}
			};

			if let Some(build) = current_build {
				if let Some(tag) = build.tags.get(game_server::VERSION_BUILD_TAG) {
					println!("{tag}");
					ExitCode::SUCCESS
				} else {
					eprintln!("No version tag found");
					ExitCode::FAILURE
				}
			} else {
				eprintln!("No current build found");
				ExitCode::FAILURE
			}
		} else {
			eprintln!("environment not found");
			ExitCode::FAILURE
		}
	}
}
