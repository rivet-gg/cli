import { resolve } from "@std/path";
import { verbose } from "../term/status.ts";
import { CommandError } from "./error.ts";
import { binaryDir, Settings } from "./settings.ts";

export interface Command {
	/**
	 * The program name
	 */
	program: string;

	/**
	 * The arguments for the command
	 */
	args: string[];

	/**
	 * The environment variables for the command
	 */
	envs: Record<string, string>;
}

export function getProgramFile(settings: Settings, program: string): string {
	return resolve(binaryDir(settings), program);
}

export async function execute(settings: Settings, command: Command, timeout?: number): Promise<[string, string]> {
	// Resolve path to program
	const programFile = getProgramFile(settings, command.program);

	verbose(`Executing command`, `${programFile} ${command.args.map((x) => JSON.stringify(x)).join(" ")}`);

	// Add timeout abort controller
	const controller = new AbortController();
	const { signal } = controller;
	let timeoutId: number | undefined;
	if (timeout) {
		timeoutId = setTimeout(() => controller.abort(), timeout);
	}

	try {
		// Windows sometimes hangs with pg_ctl process when getting stdout/stderr
		const usePiped = !(Deno.build.os == "windows" && command.program.endsWith("pg_ctl"));

		const process = await new Deno.Command(programFile, {
			args: command.args,
			env: command.envs,
			stdout: usePiped ? "piped" : "inherit",
			stderr: usePiped ? "piped" : "inherit",
			signal,
		}).spawn();

		let statusCode: number;
		let stdoutStr: string;
		let stderrStr: string;
		if (usePiped) {
			const output = await process.output();
			statusCode = output.code;
			stdoutStr = new TextDecoder().decode(output.stdout);
			stderrStr = new TextDecoder().decode(output.stderr);
		} else {
			const status = await process.status;
			statusCode = status.code;
			stdoutStr = "";
			stderrStr = "";
		}

		verbose("Result", `${statusCode}\nstdout: ${stdoutStr}\nstderr: ${stderrStr}`);

		if (statusCode == 0) {
			return [stdoutStr, stderrStr];
		} else {
			throw new CommandError(`Failed to run ${command.program}`, { stdout: stdoutStr, stderr: stderrStr });
		}
	} finally {
		if (timeoutId !== undefined) {
			clearTimeout(timeoutId);
		}
	}
}
