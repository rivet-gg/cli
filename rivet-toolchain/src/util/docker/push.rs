use futures_util::stream::{StreamExt, TryStreamExt};
use global_error::prelude::*;
use rivet_api::{apis, models};
use serde::Serialize;
use std::{path::PathBuf, sync::Arc};
use tokio::fs;
use uuid::Uuid;

use crate::{
	config,
	ctx::Ctx,
	util::{net::upload, task::TaskCtx, term},
};

use super::{BuildCompression, BuildKind};

pub struct PushOpts {
	/// Path to already created tar.
	pub path: PathBuf,

	/// Docker inside the image.
	pub tag: String,

	/// Name of the image
	pub name: Option<String>,

	pub kind: BuildKind,

	pub compression: BuildCompression,
}

#[derive(Serialize)]
pub struct PushOutput {
	pub image_id: Uuid,
}

pub async fn push_tar(ctx: &Ctx, task: TaskCtx, push_opts: &PushOpts) -> GlobalResult<PushOutput> {
	let multipart_enabled =
		config::settings::try_read(|x| Ok(!x.net.disable_upload_multipart)).await?;

	let reqwest_client = Arc::new(reqwest::Client::new());

	// Inspect the image
	let image_file_meta = fs::metadata(&push_opts.path).await?;
	ensure!(image_file_meta.len() > 0, "docker image archive is empty");

	// Create image
	let display_name = push_opts
		.name
		.clone()
		.unwrap_or_else(|| push_opts.tag.clone());
	let content_type = "binary/octet-stream";

	task.log_stdout(format!(
		"[Uploading Image] {name} ({size})",
		name = display_name,
		size = upload::format_file_size(image_file_meta.len())?
	));

	let build_res = apis::cloud_games_builds_api::cloud_games_builds_create_game_build(
		&ctx.openapi_config_cloud,
		&ctx.game_id,
		models::CloudGamesCreateGameBuildRequest {
			display_name: display_name.clone(),
			image_tag: push_opts.tag.clone(),
			image_file: Box::new(models::UploadPrepareFile {
				path: "image.tar".into(),
				content_type: Some(content_type.into()),
				content_length: image_file_meta.len() as i64,
			}),
			kind: Some(match push_opts.kind {
				BuildKind::DockerImage => models::CloudGamesBuildKind::DockerImage,
				BuildKind::OciBundle => models::CloudGamesBuildKind::OciBundle,
			}),
			compression: Some(match push_opts.compression {
				BuildCompression::None => models::CloudGamesBuildCompression::None,
				BuildCompression::Lz4 => models::CloudGamesBuildCompression::Lz4,
			}),
			multipart_upload: Some(multipart_enabled),
		},
	)
	.await;
	if let Err(err) = build_res.as_ref() {
		task.log_stderr(format!("{err:?}"))
	}
	let build_res = unwrap!(build_res,);
	let image_id = build_res.build_id;
	let pb = term::EitherProgressBar::Multi(term::multi_progress_bar(task.clone()));

	if multipart_enabled {
		// Upload chunks in parallel
		futures_util::stream::iter(build_res.image_presigned_requests.unwrap())
			.map(|presigned_request| {
				let task = task.clone();
				let reqwest_client = reqwest_client.clone();
				let pb = pb.clone();

				async move {
					upload::upload_file(
						task.clone(),
						&reqwest_client,
						&presigned_request,
						&push_opts.path,
						Some(content_type),
						pb,
					)
					.await
				}
			})
			.buffer_unordered(8)
			.try_collect::<Vec<_>>()
			.await?;
	} else {
		// Upload file
		upload::upload_file(
			task.clone(),
			&reqwest_client,
			&build_res.image_presigned_request.unwrap(),
			&push_opts.path,
			Some(content_type),
			pb,
		)
		.await?;
	}

	let complete_res = apis::cloud_uploads_api::cloud_uploads_complete_upload(
		&ctx.openapi_config_cloud,
		&build_res.upload_id.to_string(),
	)
	.await;
	if let Err(err) = complete_res.as_ref() {
		task.log_stderr(format!("{err:?}"));
	}
	unwrap!(complete_res);
	task.log_stdout(format!("[Image Upload Complete] {image_id}"));

	Ok(PushOutput {
		image_id: image_id.to_owned(),
	})
}
