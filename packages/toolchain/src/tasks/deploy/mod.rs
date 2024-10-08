mod backend;
mod game_server;

use anyhow::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::util::task;

#[derive(Deserialize)]
pub struct Input {
	pub cwd: String,
	pub environment_id: Uuid,
	pub game_server: bool,
	pub backend: bool,
	#[serde(default)]
	pub backend_skip_migrate: bool,
}

#[derive(Serialize)]
pub struct Output {
	game_server: Option<game_server::DeployOutput>,
}

pub struct Task;

impl task::Task for Task {
	type Input = Input;
	type Output = Output;

	fn name() -> &'static str {
		"deploy"
	}

	async fn run(task: task::TaskCtx, input: Self::Input) -> Result<Self::Output> {
		// Deploy the backend before the game server in order to ensure that new APIs are exposed
		// before the new game server is deployed.

		let ctx = crate::toolchain_ctx::load().await?;

		let env = crate::game::get_env(&ctx, input.environment_id).await?;

		if input.backend {
			backend::deploy(
				&ctx,
				task.clone(),
				backend::DeployOpts {
					env: env.clone(),
					project_path: input.cwd.clone(),
					skip_migrate: input.backend_skip_migrate,
				},
			)
			.await?;
		}

		let game_server = if input.game_server {
			// TODO: Add support for configuring in project config.
			// Should support multiple dockerfiles and passing from args/env.

			// Game server
			let deploy = game_server::deploy(
				&ctx,
				task.clone(),
				game_server::DeployOpts {
					env: env.clone(),
					build_dir: input.cwd.clone(),
				},
			)
			.await?;

			Some(deploy)
		} else {
			None
		};

		task.log("");
		task.log("[Deploy Finished]");

		Ok(Output { game_server })
	}
}
