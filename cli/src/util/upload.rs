use cli_core::rivet_api::models;
use futures_util::stream::StreamExt;
use global_error::prelude::*;
use std::{
	path::{Path, PathBuf},
	time::{Duration, Instant},
};
use tokio::{
	fs::File,
	io::{AsyncReadExt, AsyncSeekExt},
};
use tokio_util::io::ReaderStream;

/// Prepared file that will be uploaded to S3.
#[derive(Clone)]
pub struct UploadFile {
	pub absolute_path: PathBuf,
	pub prepared: models::UploadPrepareFile,
}

pub fn format_file_size(bytes: u64) -> GlobalResult<String> {
	use humansize::FileSize;

	let size = format!(
		"{}",
		unwrap!(bytes.file_size(humansize::file_size_opts::DECIMAL).ok())
	);
	Ok(size)
}

/// Lists all files in a directory and returns the data required to upload them.
pub fn prepare_upload_dir(base_path: &Path) -> GlobalResult<Vec<UploadFile>> {
	use std::path::Component;

	let mut files = Vec::<UploadFile>::new();

	// Walk files while respecting .rivet-cdn-ignore
	let walk = ignore::WalkBuilder::new(base_path)
		.standard_filters(false)
		.add_custom_ignore_filename(".rivet-cdn-ignore")
		.parents(true)
		.build();
	for entry in walk {
		let entry = entry?;
		let file_path = entry.path();
		let file_meta = entry.metadata()?;

		if file_meta.is_file() {
			// Convert path to Unix-style string
			let path_str = entry
				.path()
				.strip_prefix(base_path)?
				.components()
				.filter_map(|c| match c {
					Component::Normal(name) => name.to_str().map(str::to_string),
					_ => None,
				})
				.collect::<Vec<String>>()
				.join("/");

			// Attempt to guess the MIME type
			let content_type = mime_guess::from_path(&file_path)
				.first_raw()
				.map(str::to_string);

			files.push(UploadFile {
				absolute_path: file_path.to_path_buf(),
				prepared: models::UploadPrepareFile {
					path: path_str,
					content_type,
					content_length: file_meta.len() as i64,
				},
			});
		}
	}

	Ok(files)
}

/// Uploads a file to a given URL.
pub async fn upload_file(
	reqwest_client: &reqwest::Client,
	presigned_req: &models::UploadPresignedRequest,
	path: impl AsRef<Path>,
	content_type: Option<impl ToString>,
) -> GlobalResult<()> {
	let content_type = content_type.map(|x| x.to_string());

	// Try the upload multiple times since DigitalOcean spaces is incredibly
	// buggy and spotty internet connections may cause issues. This is
	// especially important since we have files that we need to batch upload, so
	// one failing request is bad.
	let mut attempts = 0;
	let upload_time = 'upload: loop {
		// Read file
		let mut file = File::open(path.as_ref()).await?;
		let file_meta = file.metadata().await?;
		let path = presigned_req.path.clone();

		let file_len = file_meta.len();
		let is_multipart = presigned_req.content_length as u64 != file_len;
		let total_size = if is_multipart {
			format_file_size(presigned_req.content_length as u64)?
		} else {
			format_file_size(file_len)?
		};

		if is_multipart {
			eprintln!(
				"  * {path}: Uploading chunk {} - {} [{mime}]",
				format_file_size(presigned_req.byte_offset as u64)?,
				format_file_size(
					(presigned_req.byte_offset + presigned_req.content_length) as u64
				)?,
				mime = content_type.clone().unwrap_or_default(),
			);
		} else {
			eprintln!(
				"  * {path}: Uploading {total_size} [{mime}]",
				mime = content_type.clone().unwrap_or_default(),
			);
		}

		// Create a reader for the slice of the file we need to read
		file.seek(tokio::io::SeekFrom::Start(presigned_req.byte_offset as u64))
			.await?;
		let mut reader_stream = ReaderStream::new(file.take(presigned_req.content_length as u64));

		let mut uploaded = 0usize;

		let start = Instant::now();
		let mut last_log = Instant::now();
		let log_freq = Duration::from_secs(1);

		// Process the stream with upload progress
		let async_stream = async_stream::stream! {
			while let Some(chunk) = reader_stream.next().await {
				if let Ok(chunk) = &chunk {
					uploaded += chunk.len();

					let last_log_duration = last_log.elapsed();
					if last_log_duration >= log_freq {
						last_log = Instant::now();

						let progress = (uploaded as f64 / file_len as f64) * 100.;

						let duration = start.elapsed();
						let rate = uploaded as f64 / duration.as_secs_f64();

						let time_remaining_total = ((file_len as f64 - uploaded as f64) / rate).ceil() as usize;
						let time_remaining_secs = time_remaining_total % 60;
						let time_remaining_min = (time_remaining_total / 60) % 60;
						let time_remaining_hr = time_remaining_total / 60 / 60;

						let uploaded_size = format_file_size(uploaded as u64).unwrap_or_else(|_| "?".to_string());
						let upload_rate = format_file_size(rate as u64).unwrap_or_else(|_| "?".to_string());

						eprintln!("    {path}: {uploaded_size}/{total_size} [{progress:.1}%] ({upload_rate}/s) [{time_remaining_hr:0>2}:{time_remaining_min:0>2}:{time_remaining_secs:0>2}]");
					}
				}

				yield chunk;
			}
		};

		let body = reqwest::Body::wrap_stream(async_stream);

		// Upload file
		let start = Instant::now();
		let mut req = reqwest_client
			.put(&presigned_req.url)
			.header("content-length", presigned_req.content_length);
		if let Some(content_type) = &content_type {
			req = req.header("content-type", content_type.to_string());
		}
		let res = req.body(body).send().await?;
		if res.status().is_success() {
			let upload_time = start.elapsed();
			break 'upload upload_time;
		} else {
			if attempts > 4 {
				let response_status = res.status();
				let response_text = res.text().await?;
				let text = format!(
					"failed to upload file: {}\n{:?}",
					response_status, response_text
				);
				bail!(&text);
			} else {
				attempts += 1;
				eprintln!(
					"  ! Upload failed with status {status}, will retry (attempt #{attempt}): {body:?}",
					attempt = attempts,
					status = res.status(),
					body = res.text().await,
				);
				tokio::time::sleep(Duration::from_secs(5)).await;
				continue 'upload;
			}
		}
	};

	eprintln!(
		"    {}: Finished in {:.3}s",
		presigned_req.path,
		upload_time.as_secs_f64()
	);

	Ok(())
}
