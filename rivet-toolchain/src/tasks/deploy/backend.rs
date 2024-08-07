use futures_util::{StreamExt, TryStreamExt};
use global_error::prelude::*;
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
	ctx::Ctx,
	util::task::TaskCtx,
	util::{net::upload, term},
};

pub struct DeployOpts {
	/// Game ID being deployed to.
	pub game_id: String,

	/// The environment to deploy to.
	pub environment_id: String,

	/// The location of the OpenGB project.
	pub project_path: String,

	/// Skip the migration step.
	pub skip_migrate: bool,
}

pub async fn deploy(ctx: &Ctx, task: TaskCtx, opts: DeployOpts) -> GlobalResult<()> {
	task.log_stdout("[Deploying Backend]");

	let project = backend::get_or_create_project(ctx).await?;
	let project_id_str = project.project_id.to_string();
	let environment_id_str = opts.environment_id.to_string();
	let project_path = PathBuf::from(opts.project_path.clone());

	let env = apis::ee_cloud_backend_projects_envs_api::ee_cloud_backend_projects_envs_get(
		&ctx.openapi_config_cloud,
		&project_id_str,
		&environment_id_str,
		None,
	)
	.await?
	.environment;

	// TODO: re-apply both service token and endpoint if endpoint diff than one that's deployed
	task.log_stdout(format!("[Fetching Environment Variables]"));
	// let variables =
	// 	apis::ee_cloud_backend_projects_envs_api::ee_cloud_backend_projects_envs_get_variables(
	// 		&ctx.openapi_config_cloud,
	// 		&project_id_str,
	// 		&environment_id_str,
	// 	)
	// 	.await?
	// 	.variables;
	let mut update_variables = HashMap::<String, _>::new();
	// if !variables.contains_key("OPENGB_PUBLIC_ENDPOINT") {
	// TODO: pull this from the server
	update_variables.insert(
		"OPENGB_PUBLIC_ENDPOINT".to_string(),
		models::EeBackendUpdateVariable {
			text: Some(env.endpoint.clone()),
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
	// if !variables.contains_key("RIVET_SERVICE_TOKEN") {
	let service_token = apis::cloud_games_tokens_api::cloud_games_tokens_create_service_token(
		&ctx.openapi_config_cloud,
		&opts.game_id,
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
	// if !update_variables.is_empty() {
	task.log_stdout(format!(
		"[Updating Environment Variables] {}",
		update_variables
			.keys()
			.cloned()
			.collect::<Vec<_>>()
			.join(", ")
	));
	apis::ee_cloud_backend_projects_envs_api::ee_cloud_backend_projects_envs_update_variables(
		&ctx.openapi_config_cloud,
		&project_id_str,
		&environment_id_str,
		models::EeCloudBackendProjectsEnvsUpdateVariablesRequest {
			variables: update_variables,
		},
	)
	.await?;
	// }

	task.log_stdout(format!("[Building Project] {}", project_path.display()));

	// Build
	let (cmd_env, config_path) = config::settings::try_read(|settings| {
		let mut env = settings.backend.command_environment.clone();
		env.extend(settings.backend.deploy.command_environment.clone());
		Ok((env, settings.backend.deploy.config_path.clone()))
	})
	.await?;
	let cmd = backend::run_opengb_command(
		task.clone(),
		backend::OpenGbCommandOpts {
			config_path: config_path.clone(),
			args: vec![
				"build".into(),
				"--db-driver".into(),
				"neon-serverless".into(),
				"--runtime".into(),
				"cloudflare-workers-platforms".into(),
			],
			env: cmd_env,
			cwd: project_path.clone(),
		},
	)
	.await?;
	ensure!(cmd == 0, "Failed to build OpenGB project");

	backend::database::provision_database(
		task.clone(),
		ctx,
		project.project_id,
		env.environment_id,
	)
	.await?;

	let db_url = config::meta::try_read_project(|config| {
		let project = unwrap!(config.opengb.projects.get(&project.project_id));
		let env = unwrap!(project.environments.get(&env.environment_id));

		Ok(env.url.clone())
	})
	.await?;

	if !opts.skip_migrate {
		task.log_stdout("[Migrating Database]");

		// Migrate
		let mut migrate_env = HashMap::new();
		migrate_env.insert(
			"DATABASE_URL".to_string(),
			unwrap!(db_url, "no db url for env"),
		);

		let migrate_cmd = backend::run_opengb_command(
			task.clone(),
			backend::OpenGbCommandOpts {
				config_path,
				args: vec!["db".into(), "deploy".into()],
				env: migrate_env,
				cwd: project_path.clone(),
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

	task.log_stdout(format!(
		"[Uploading Environment] {name} ({count} files, {size} total)",
		name = &env.display_name,
		count = files.len(),
		size = upload::format_file_size(total_len as u64)?,
	));

	let prepare_res = unwrap!(
		apis::ee_cloud_backend_projects_envs_api::ee_cloud_backend_projects_envs_prepare_deploy(
			&ctx.openapi_config_cloud,
			&project_id_str,
			&environment_id_str,
			models::EeCloudBackendProjectsEnvsPrepareDeployRequest {
				files: files.iter().map(|f| f.prepared.clone()).collect(),
			},
		)
		.await
	);

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
				let file = unwrap!(
					files.iter().find(|f| f.prepared.path == presigned_req.path),
					"missing prepared file"
				);

				upload::upload_file(
					task.clone(),
					&reqwest_client,
					&presigned_req,
					&file.absolute_path,
					file.prepared.content_type.as_ref(),
					pb,
				)
				.await?;

				GlobalResult::<()>::Ok(())
			}
		})
		.await?;

	task.log_stdout(format!("[Deploying Environment] {}", env.display_name));

	let deploy_res =
		apis::ee_cloud_backend_projects_envs_api::ee_cloud_backend_projects_envs_deploy(
			&ctx.openapi_config_cloud,
			&project_id_str,
			&environment_id_str,
			models::EeCloudBackendProjectsEnvsDeployRequest {
				upload_id: prepare_res.upload_id,
			},
		)
		.await?;

	task.log_stdout(format!("[Done] OpenGB API available at {}", deploy_res.url));

	Ok(())
}

#[derive(Deserialize)]
struct GenManifest {
	bundle: String,
	wasm: Option<String>,
}

async fn read_generated_manifest(project_path: &Path) -> GlobalResult<GenManifest> {
	let manifest_str =
		fs::read_to_string(project_path.join(".opengb").join("manifest.json")).await?;
	Ok(serde_json::from_str::<GenManifest>(&manifest_str)?)
}
