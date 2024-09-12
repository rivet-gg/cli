use anyhow::*;
use futures_util::{StreamExt, TryStreamExt};
use rivet_api::{apis, models};
use serde::Deserialize;
use std::{
	collections::HashMap,
	path::{Path, PathBuf},
	sync::Arc,
};
use tokio::fs;

use crate::{
	backend, config,
	game::TEMPEnvironment,
	toolchain_ctx::ToolchainCtx,
	util::{net::upload, task, term},
};

pub struct DeployOpts {
	pub env: TEMPEnvironment,

	/// The location of the OpenGB project.
	pub project_path: String,

	/// Skip the migration step.
	pub skip_migrate: bool,
}

pub async fn deploy(ctx: &ToolchainCtx, task: task::TaskCtx, opts: DeployOpts) -> Result<()> {
	task.log("[Deploying Backend]");

	let backend = backend::get_or_create_backend(ctx, opts.env.id).await?;
	let game_id_str = ctx.game_id.to_string();
	let env_id_str = opts.env.id.to_string();
	let project_path = PathBuf::from(opts.project_path.clone());

	// Build
	task.log(format!("[Building Project] {}", project_path.display()));
	let (cmd_env, config_path) = config::settings::try_read(|settings| {
		let mut env = settings.backend.command_environment.clone();
		env.extend(settings.backend.deploy.command_environment.clone());
		Ok((env, settings.backend.deploy.config_path.clone()))
	})
	.await?;
	let cmd = backend::run_opengb_command_from_task(
		task.clone(),
		backend::BackendCommandOpts {
			command: "build",
			opts: serde_json::json!({
				"project": config_path,
				"migrate": false,
				"dbDriver": "neon_serverless",
				"runtime": "cloudflare_workers_platforms",
			}),
			env: cmd_env,
		},
	)
	.await?;
	ensure!(cmd == 0, "Failed to build OpenGB project");

	backend::database::provision_database(task.clone(), ctx, opts.env.id).await?;

	let db_url = config::meta::try_read_project(|config| {
		let env_config = config
			.environments
			.get(&opts.env.id)
			.context("could not find environment")?;
		Ok(env_config.backend.db_url.clone())
	})
	.await?;

	if !opts.skip_migrate {
		task.log("[Migrating Database]");

		// Migrate
		let mut migrate_env = HashMap::new();
		migrate_env.insert(
			"DATABASE_URL".to_string(),
			db_url.context("no db url for env")?,
		);

		let migrate_cmd = backend::run_opengb_command_from_task(
			task.clone(),
			backend::BackendCommandOpts {
				command: "dbMigrateApply",
				opts: serde_json::json!({
					"project": config_path,
				}),
				env: migrate_env,
			},
		)
		.await?;
		ensure!(migrate_cmd == 0, "Failed to migrate OpenGB databases");
	}

	// Read files for upload
	let gen_manifest = read_generated_manifest(&project_path).await?;
	let bundle_path = project_path.join(gen_manifest.bundle);
	let wasm_path = gen_manifest.wasm.map(|x| project_path.join(x));
	let mut files = vec![upload::prepare_upload_file(
		&bundle_path,
		"bundle.js",
		fs::metadata(&bundle_path).await?,
	)?];
	if let Some(wasm) = wasm_path.as_ref() {
		files.push(upload::prepare_upload_file(
			wasm,
			"query-engine.wasm",
			fs::metadata(wasm).await?,
		)?);
	}
	let total_len = files
		.iter()
		.fold(0, |acc, x| acc + x.prepared.content_length);

	task.log(format!(
		"[Uploading Environment] {name} ({count} files, {size} total)",
		name = &opts.env.name,
		count = files.len(),
		size = upload::format_file_size(total_len as u64)?,
	));

	task.log(format!("[Fetching Environment Variables]"));
	// let variables = apis::ee_backend_api::ee_backend_get_variables(
	// 	&ctx.openapi_config_cloud,
	// 	&game_id_str,
	// 	&env_id_str,
	// )
	// .await?
	// .variables;
	let mut update_variables = HashMap::<String, _>::new();
	// if !variables.contains_key("OPENGB_PUBLIC_ENDPOINT") {
	update_variables.insert(
		"OPENGB_PUBLIC_ENDPOINT".to_string(),
		models::EeBackendUpdateVariable {
			text: Some(backend.endpoint.clone()),
			..Default::default()
		},
	);
	// }
	// if !variables.contains_key("RIVET_API_ENDPOINT") {
	update_variables.insert(
		"RIVET_API_ENDPOINT".to_string(),
		models::EeBackendUpdateVariable {
			text: Some(ctx.api_endpoint.clone()),
			..Default::default()
		},
	);
	// }
	// if !variables.contains_key("RIVET_GAME_ID") {
	update_variables.insert(
		"RIVET_GAME_ID".to_string(),
		models::EeBackendUpdateVariable {
			text: Some(game_id_str.clone()),
			..Default::default()
		},
	);
	// }
	// if !variables.contains_key("RIVET_ENVIRONMENT_ID") {
	update_variables.insert(
		"RIVET_ENVIRONMENT_ID".to_string(),
		models::EeBackendUpdateVariable {
			text: Some(env_id_str.clone()),
			..Default::default()
		},
	);
	// }
	// if !variables.contains_key("RIVET_SERVICE_TOKEN") {
	task.log(format!("[Creating Service Token]"));
	let service_token =
		apis::games_environments_tokens_api::games_environments_tokens_create_service_token(
			&ctx.openapi_config_cloud,
			&game_id_str,
			&opts.env.id.to_string(),
		)
		.await?;
	update_variables.insert(
		"RIVET_SERVICE_TOKEN".to_string(),
		models::EeBackendUpdateVariable {
			secret: Some(service_token.token),
			..Default::default()
		},
	);
	// }
	if !update_variables.is_empty() {
		task.log(format!(
			"[Updating Variables] {}",
			update_variables
				.keys()
				.cloned()
				.collect::<Vec<_>>()
				.join(", ")
		));
	}

	let prepare_res = apis::ee_backend_api::ee_backend_prepare_deploy(
		&ctx.openapi_config_cloud,
		&game_id_str,
		&env_id_str,
		models::EeBackendPrepareDeployRequest {
			files: files.iter().map(|f| f.prepared.clone()).collect(),
		},
	)
	.await
	.context("failed to prepare deploy")?;

	// Upload files
	let reqwest_client = Arc::new(reqwest::Client::new());
	let pb = term::EitherProgressBar::Multi(term::multi_progress_bar(task.clone()));

	futures_util::stream::iter(prepare_res.presigned_requests)
		.map(Ok)
		.try_for_each_concurrent(8, |presigned_req| {
			let task = task.clone();
			let pb = pb.clone();
			let files = files.clone();
			let reqwest_client = reqwest_client.clone();

			async move {
				// Find the matching prepared file
				let file = files
					.iter()
					.find(|f| f.prepared.path == presigned_req.path)
					.context("missing prepared file")?;

				upload::upload_file(
					task.clone(),
					&reqwest_client,
					&presigned_req,
					&file.absolute_path,
					file.prepared.content_type.as_ref(),
					pb,
				)
				.await?;

				Result::<()>::Ok(())
			}
		})
		.await?;

	task.log(format!("[Deploying Environment] {}", opts.env.name));

	let deploy_res = apis::ee_backend_api::ee_backend_deploy(
		&ctx.openapi_config_cloud,
		&game_id_str,
		&env_id_str,
		models::EeBackendDeployRequest {
			upload_id: prepare_res.upload_id,
			variables: Some(update_variables),
		},
	)
	.await?;

	task.log(format!(
		"[Done] Backend API available at {}",
		deploy_res.url
	));

	Ok(())
}

#[derive(Deserialize)]
struct GenManifest {
	bundle: String,
	wasm: Option<String>,
}

async fn read_generated_manifest(project_path: &Path) -> Result<GenManifest> {
	let manifest_str =
		fs::read_to_string(project_path.join(".opengb").join("manifest.json")).await?;
	Ok(serde_json::from_str::<GenManifest>(&manifest_str)?)
}