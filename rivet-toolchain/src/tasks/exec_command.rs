use global_error::prelude::*;
use serde::{Deserialize, Serialize};

use crate::util::{cmd::shell_cmd, task::TaskCtx};

#[derive(Deserialize)]
pub struct Input {
	pub cwd: String,
	pub cmd: String,
	pub args: Vec<String>,
}

#[derive(Serialize)]
pub struct Output {
	exit_code: i32,
}

pub struct Task;

impl super::Task for Task {
	type Input = Input;
	type Output = Output;

	fn name() -> &'static str {
		"exec_command"
	}

	async fn run(task: TaskCtx, input: Self::Input) -> GlobalResult<Self::Output> {
		let mut cmd = shell_cmd(&input.cmd);
		cmd.args(&input.args).current_dir(input.cwd);
		let exit_code = task.spawn_cmd(cmd).await?;
		Ok(Output {
			exit_code: exit_code.code().unwrap_or(0),
		})
	}
}
