use global_error::prelude::*;
use serde::{Deserialize, Serialize};

use crate::util::task::TaskCtx;

#[derive(Deserialize)]
pub struct Input {
	path: String,
}

#[derive(Serialize)]
pub struct Output {}

pub struct Task;

impl super::Task for Task {
	type Input = Input;
	type Output = Output;

	fn name() -> &'static str {
		"open"
	}

	async fn run(_task: TaskCtx, input: Self::Input) -> GlobalResult<Self::Output> {
		open::that_detached(input.path)?;
		Ok(Output {})
	}
}
