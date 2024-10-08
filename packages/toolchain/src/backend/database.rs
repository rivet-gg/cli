use anyhow::*;
use rivet_api::apis;
use uuid::Uuid;

use crate::{config, paths, toolchain_ctx::ToolchainCtx, util::task};

pub async fn provision_database(
	task: task::TaskCtx,
	ctx: &ToolchainCtx,
	env_id: Uuid,
) -> Result<()> {
	task.log("[Provisioning Database]");

	apis::ee_backend_api::ee_backend_provision_database(
		&ctx.openapi_config_cloud,
		&ctx.game_id.to_string(),
		&env_id.to_string(),
	)
	.await?;

	// Fetch remote DB URL
	let mut env_config = config::meta::try_mutate_project(&paths::data_dir()?, |config| {
		Ok(config
			.cloud
			.as_mut()
			.context("config.cloud")?
			.environments
			.entry(env_id)
			.or_default()
			.clone())
	})
	.await?;

	if env_config.backend.db_url.is_none() {
		task.log("[Fetching Connection]");

		let db_url_res = apis::ee_backend_api::ee_backend_get_db_url(
			&ctx.openapi_config_cloud,
			&ctx.game_id.to_string(),
			&env_id.to_string(),
		)
		.await?;

		// Add missing db url
		env_config.backend.db_url = db_url_res.url;

		// Update cache
		config::meta::try_mutate_project(&paths::data_dir()?, |config| {
			config
				.cloud
				.as_mut()
				.context("config.cloud")?
				.environments
				.insert(env_id, env_config.clone());
			Ok(())
		})
		.await?;
	}

	Ok(())
}
