use global_error::prelude::*;
use rivet_api::apis;
use serde::{Deserialize, Serialize};

use crate::{config, ctx, util::task::TaskCtx};

#[derive(Deserialize)]
pub struct Input {
	pub api_endpoint: String,
	pub device_link_token: String,
}

#[derive(Serialize)]
pub struct Output {}

pub struct Task;

impl super::Task for Task {
	type Input = Input;
	type Output = Output;

	fn name() -> &'static str {
		"wait_for_login"
	}

	async fn run(_task: TaskCtx, input: Self::Input) -> GlobalResult<Self::Output> {
		let openapi_config_cloud_unauthed = apis::configuration::Configuration {
			base_path: input.api_endpoint.clone(),
			user_agent: Some(ctx::user_agent()),
			..Default::default()
		};

		let mut watch_index = None;
		let token = loop {
			let prepare_res = apis::cloud_devices_links_api::cloud_devices_links_get(
				&openapi_config_cloud_unauthed,
				&input.device_link_token,
				watch_index.as_ref().map(String::as_str),
			)
			.await?;

			watch_index = Some(prepare_res.watch.index);

			if let Some(token) = prepare_res.cloud_token {
				break token;
			}
		};

		let new_ctx = crate::ctx::init(input.api_endpoint.clone(), token.clone()).await?;

		let inspect_res =
			apis::cloud_auth_api::cloud_auth_inspect(&new_ctx.openapi_config_cloud).await?;

		let game_id = unwrap!(inspect_res.agent.game_cloud, "no game cloud token found").game_id;

		let _game_res = apis::cloud_games_api::cloud_games_get_game_by_id(
			&new_ctx.openapi_config_cloud,
			&game_id.to_string(),
			None,
		)
		.await?;

		config::meta::insert_project(input.api_endpoint, token).await?;

		Ok(Output {})
	}
}
