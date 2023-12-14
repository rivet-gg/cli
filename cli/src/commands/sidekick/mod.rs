use clap::Parser;
use console::Term;
use global_error::prelude::*;
use serde::Serialize;
use serde_json::{json, Value};
use std::process::Command;

use crate::util::{
	global_config,
	struct_fmt::{self, Format},
};

pub mod check_login_state;
pub mod deploy;
pub mod get_bootstrap_data;
pub mod get_cli_version;
pub mod get_link;
pub mod get_version;
pub mod wait_for_login;

pub trait SideKickHandler: Serialize {
	fn print(&self) {
		println!("{}", serde_json::to_string_pretty(self).unwrap());
	}
}

#[derive(Parser)]
pub enum SubCommand {
	/// Get the link for the user to sign in
	GetLink(get_link::Opts),
	/// Long poll the server to check if the user has signed in
	WaitForLogin(wait_for_login::Opts),
	/// Check if the CLI is logged in already
	CheckLoginState(check_login_state::Opts),
	/// Get the token from the CLI
	GetBootstrapData(get_bootstrap_data::Opts),
	/// Get the version of the CLI
	GetVersion(get_version::Opts),
	/// Deploy a version
	Deploy(deploy::Opts),
	/// Get the CLI version
	GetCliVersion(get_cli_version::Opts),
}

/// Any response that can come from the sidekick. There should only be a single
/// response from any sidekick call, though it might include multiple messages.
/// This is so a single schema can be parsed by whatever is consuming the
/// sidekick output.
#[derive(Serialize)]
pub struct SideKickResponse(pub Value);

fn serialize_output(v: GlobalResult<impl Serialize>) -> GlobalResult<Value> {
	Ok(unwrap!(serde_json::to_value(&unwrap!(v))))
}

pub enum PreExecuteHandled {
	Yes,
	No,
}

struct Terminal {
	/// The name of the terminal emulator command
	name: &'static str,
	/// The flag to pass the command to the terminal emulator
	prompt_str: &'static str,
	/// Whether the command should be passed as a single argument (quoted) or as
	/// multiple arguments
	quote: bool,
}

/// Terminals that don't work (note, more work might make them work):
///
/// - guake (runs the whole window, doesn't handle closing)
/// - upterm (doesn't have an arg to pass a command it)
/// - x-terminal-emulator
const TERMINALS: [Terminal; 10] = [
	Terminal {
		name: "gnome-terminal",
		prompt_str: "--",
		quote: false,
	},
	Terminal {
		name: "kitty",
		prompt_str: "-e",
		quote: false,
	},
	Terminal {
		name: "konsole",
		prompt_str: "-e",
		quote: false,
	},
	Terminal {
		name: "st",
		prompt_str: "-e",
		quote: false,
	},
	Terminal {
		name: "terminator",
		prompt_str: "-e",
		quote: true,
	},
	Terminal {
		name: "tilda",
		prompt_str: "-c",
		quote: true,
	},
	Terminal {
		name: "tilix",
		prompt_str: "-e",
		quote: false,
	},
	Terminal {
		name: "urxvt",
		prompt_str: "-e",
		quote: false,
	},
	Terminal {
		name: "xfce4-terminal",
		prompt_str: "-e",
		quote: true,
	},
	Terminal {
		name: "xterm",
		prompt_str: "-e",
		quote: false,
	},
];

impl SubCommand {
	/// These commands run before a token is required, so they don't have access
	/// to ctx
	pub async fn pre_execute(&self, token: &Option<String>) -> GlobalResult<PreExecuteHandled> {
		let mut handled = PreExecuteHandled::Yes;
		let response = match self {
			SubCommand::GetLink(opts) => serialize_output(opts.execute().await),
			SubCommand::WaitForLogin(opts) => serialize_output(opts.execute().await),
			SubCommand::CheckLoginState(_opts) => serialize_output(self.validate_token(&token)),
			SubCommand::GetCliVersion(opts) => serialize_output(opts.execute().await),
			_ => {
				// If the command is anything else, we need to check if a token
				// has already been provided. If not, we need to print an error
				// and return early since that's what the plugins will expect.
				if let Err(_) = self.validate_token(&token) {
					// The message has already been printed out so we can just
					// return Ok here.
					serialize_output(Ok(SideKickResponse(json!({
						"output": "Token not found. Please run `rivet sidekick get-link` to sign in."
					}))))
				} else {
					handled = PreExecuteHandled::No;

					serialize_output(Ok(String::new()))
				}
			}
		};

		if let PreExecuteHandled::Yes = handled {
			// Print the response
			SubCommand::print(response)?;
		}

		Ok(handled)
	}

	pub async fn execute(
		&self,
		ctx: &cli_core::Ctx,
		_term: &Term,
		show_terminal: bool,
	) -> GlobalResult<()> {
		if show_terminal {
			SubCommand::show_terminal(ctx).await?;
			return Ok(());
		}

		let (_api_endpoint, _token) = global_config::read_project(|x| {
			(x.cluster.api_endpoint.clone(), x.tokens.cloud.clone())
		})
		.await?;

		let response = match self {
			SubCommand::GetLink(_)
			| SubCommand::CheckLoginState(_)
			| SubCommand::WaitForLogin(_)
			| SubCommand::GetCliVersion(_) => {
				unreachable!("This command should be handled before this")
			}
			SubCommand::GetBootstrapData(opts) => serialize_output(opts.execute(ctx).await),
			SubCommand::GetVersion(opts) => serialize_output(opts.execute(ctx).await),
			SubCommand::Deploy(opts) => serialize_output(opts.execute(ctx).await),
		};

		// Print the response
		SubCommand::print(response)?;

		Ok(())
	}

	pub fn validate_token(&self, token: &Option<String>) -> GlobalResult<SideKickResponse> {
		if token.is_none() {
			bail!("No Rivet token found, please do the sign in process");
		}

		Ok(SideKickResponse(json!({
			"output": "Token Valid",
		})))
	}

	pub fn print(response: GlobalResult<Value>) -> GlobalResult<()> {
		match response {
			Ok(sidekick_response) => {
				struct_fmt::print(&Format::Json, &json!({ "Ok": sidekick_response }))?;
			}
			Err(global_error) => {
				struct_fmt::print(
					&Format::Json,
					&json!({
						"Err": global_error.to_string()
					}),
				)?;
			}
		}

		Ok(())
	}

	pub async fn show_terminal(ctx: &cli_core::Ctx) -> GlobalResult<()> {
		// TODO(forest): The code doesn't handle the case where the binary
		// path or the arguments contain special characters that might need
		// to be escaped or quoted.
		let binary_path = std::env::current_exe()?;

		// TODO(forest): The command is constructed by joining the arguments
		// with spaces. This might not work correctly if any of the
		// arguments contain spaces themselves. You might need to escape or
		// quote these arguments.
		let args = std::env::args().collect::<Vec<_>>();

		// We can get rid of the first since it's the relative path to the
		// binary. Then, we need to remove any argument that starts with
		// `--show-shell`
		let mut args = args
			.into_iter()
			.skip(1)
			.filter(|x| !x.starts_with("--show-terminal"))
			.collect::<Vec<_>>();

		// Add the binary path back as the first argument
		args.insert(0, binary_path.to_str().unwrap().to_string());

		let command_to_run = format!("{} {}", binary_path.to_str().unwrap(), args.join(" "));

		#[cfg(target_os = "windows")]
		Command::new("cmd.exe")
			.arg("/C")
			.args(args)
			.spawn()
			.expect("cmd.exe failed to start");

		#[cfg(target_os = "macos")]
		{
			let command_to_run = args.join(" ");
			let apple_script = format!(
				"tell application \"Terminal\"
						activate
						do script \"{}\"
					end tell",
				command_to_run
			);

			Command::new("osascript")
				.arg("-e")
				.arg(apple_script)
				.spawn()
				.expect("Terminal failed to start");
		}

		#[cfg(target_os = "linux")]
		{
			// TODO(forest): For Linux, the code is trying to find an
			// available terminal emulator from a predefined list and
			// then run the command in it. However, the way to run a
			// command in a terminal emulator can vary between different
			// emulators. The -e flag used here might not work for all
			// of them.
			let mut command = None;

			for terminal in &TERMINALS {
				if which::which(terminal.name).is_ok() {
					command = Some(terminal);
					break;
				}
			}

			match command {
				Some(terminal) => {
					if terminal.quote {
						args = vec![args.join(" ")];
					}

					Command::new(terminal.name)
						// This is the flag to run a command in the
						// terminal. Most will use -e, but some might use
						// something different.
						.arg(terminal.prompt_str)
						.args(args)
						.spawn()
						.expect("Terminal emulator failed to start");
				}
				None => {
					panic!("No terminal emulator found");
				}
			}
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use std::fs;
	use std::path::Path;
	use std::process::Command;

	use super::TERMINALS;

	#[test]
	#[ignore]
	/// This test makes sure that the configuration to run a command in each
	/// terminal works. It shouldn't run in CI, since it would be difficult to
	/// configure. It can be run locally if each terminal in the const is
	/// installed.
	fn test_terminals() {
		for terminal in TERMINALS {
			let file_name = format!("{}.txt", terminal.name);

			let mut args = Vec::new();
			// If we want to pass the command as a single string, we can just do
			// an entire string
			if terminal.quote {
				args.push(format!("touch {}", file_name));
			} else {
				args.push("touch".to_string());
				args.push(file_name.clone());
			}

			let output = Command::new(terminal.name)
				.arg(terminal.prompt_str)
				.args(&args)
				.output()
				.expect("Failed to execute command");

			assert!(output.status.success(), "Command failed: {}", terminal.name);

			let file_path = Path::new(&file_name);
			assert!(file_path.exists(), "File does not exist: {}", file_name);

			// Clean up the file
			fs::remove_file(file_path).expect("Failed to remove file");
		}
	}
}