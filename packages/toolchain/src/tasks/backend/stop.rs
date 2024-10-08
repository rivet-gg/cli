use anyhow::*;
use serde::{Deserialize, Serialize};

use crate::{backend, util::task};

#[derive(Deserialize)]
pub struct Input {}

#[derive(Serialize)]
pub struct Output {}

pub struct Task;

impl task::Task for Task {
	type Input = Input;
	type Output = Output;

	fn name() -> &'static str {
		"backend.stop"
	}

	async fn run(_task: task::TaskCtx, _input: Self::Input) -> Result<Self::Output> {
		backend::PROCESS_MANAGER_DEV.stop().await?;
		Ok(Output {})
	}
}
