// Based on https://github.com/rivet-gg/rivet-term/blob/f70f76f63eba12e535e53db1b436d5297edf979a/src/status.rs

import * as colors from "@std/fmt/colors";

// Override colors to respect OpenGB env.
if (Deno.env.get("OPENGB_TERM_COLOR") === "never") colors.setColorEnabled(false);
else if (Deno.env.get("OPENGB_TERM_COLOR") === "always") colors.setColorEnabled(true);

export function verbose(msg: string, data = "") {
	if (Deno.env.get("VERBOSE")) console.error(`${colors.bold(colors.gray(msg))} ${data}`);
}

export function info(msg: string, data = "") {
	console.error(`${colors.bold(colors.blue(msg))} ${data}`);
}

export function progress(msg: string, data = "") {
	console.error(`${colors.bold(colors.green(msg))} ${data}`);
}

export function success(msg: string, data = "") {
	console.error(`${colors.bold(colors.green(msg))} ${data}`);
}

export function warn(msg: string, data = "") {
	console.error(`${colors.bold(colors.yellow(msg))} ${data}`);
}

export function error(msg: string, data = "") {
	console.error(`${colors.bold(colors.red(msg))} ${data}`);
}
