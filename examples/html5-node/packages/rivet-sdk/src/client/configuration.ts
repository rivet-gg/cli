// This file is auto-generated by the Rivet (https://rivet.gg) build system.
// 
// Do not edit this file directly.

import { Logger } from "./logger.ts";

export interface ConfigurationOpts {
	endpoint?: string;
	gameVersion?: string;
}

export class Configuration {
	endpoint: string;
	gameVersion: string;

	constructor(opts?: ConfigurationOpts) {
		this.endpoint = opts?.endpoint ??
			this._getEnvironmentVariable("RIVET_BACKEND_ENDPOINT") ??
			"http://localhost:6420";
		this.gameVersion = opts?.gameVersion ??
			this._getEnvironmentVariable("GAME_VERSION") ?? "unknown";
		Logger.log(`Endpoint: ${this.endpoint}`);
		Logger.log(`Game version: ${this.gameVersion}`);
	}

	private _getEnvironmentVariable(name: string): string | undefined {
		try {
			return (globalThis as any).process?.env?.[name];
		} catch {
			return undefined;
		}
	}
}
