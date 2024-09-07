import { move } from "@std/fs";
import { resolve } from "@std/path";
import { CommandError, UnreachableError } from "../error/mod.ts";
import { Project } from "../project/mod.ts";
import { projectGenPath, SDK_PATH } from "../project/project.ts";
import { progress, success } from "../term/status.ts";

import { generateTypescriptAddons } from "./typescript/mod.ts";
import { DEFAULT_PACKAGE_NAME as UNITY_DEFAULT_PACKAGE_NAME, generateUnityAddons } from "./unity/mod.ts";
import { generateGodot } from "./godot/mod.ts";

export enum SdkTarget {
	TypeScript,
	Unity,
	Godot,
}

interface Generator {
	generator: string;
	options: Record<string, string>;
}

const GENERATORS: Record<SdkTarget, Generator> = {
	[SdkTarget.TypeScript]: {
		generator: "typescript-fetch",
		options: {
			generateAliasAsModel: "true",
			npmName: "opengb-sdk",
			disallowAdditionalPropertiesIfNotPresent: "false",
			fileContentDataType: "Blob",
			platform: "browser",
			supportsES6: "true",
		},
	},
	[SdkTarget.Unity]: {
		generator: "csharp",
		options: {
			apiName: "Backend",
			library: "unityWebRequest",
			packageName: UNITY_DEFAULT_PACKAGE_NAME,
			// disallowAdditionalPropertiesIfNotPresent: "false",
			// targetFramework: "netstandard2.1",
		},
	},
	[SdkTarget.Godot]: {
		generator: "manual",
		options: {},
	},
};

export async function generateSdk(
	project: Project,
	target: SdkTarget,
	output: string,
) {
	const targetString = targetToString(target);
	const sdkGenPath = resolve(projectGenPath(project, SDK_PATH), targetString);

	// Clear artifacts
	try {
		await Deno.remove(sdkGenPath, { recursive: true });
	} catch (err) {
		if (!(err instanceof Deno.errors.NotFound)) {
			throw err;
		}
	}

	progress("Building SDK", targetString);

	const config = GENERATORS[target]!;
	let buildOutput;

	if (config.generator != "manual") {
		// Run using deno when in docker
		if (Deno.env.has("RUNNING_IN_DOCKER")) {
			buildOutput = await new Deno.Command("deno", {
				args: [
					"run",
					"-A",
					"npm:@openapitools/openapi-generator-cli@2.13.4",
					"generate",
					"-i",
					`${project.path}/.opengb/openapi.json`,
					"-g",
					config.generator,
					"-o",
					`${project.path}/.opengb/sdk/${targetString}`,
					"--additional-properties=" +
					Object.entries(config.options).map(([key, value]) => `${key}=${value}`).join(","),
				],
			}).output();
		} else {
			buildOutput = await new Deno.Command("docker", {
				args: [
					"run",
					"--rm",
					"-v",
					`${project.path}:/local`,
					"openapitools/openapi-generator-cli:v7.6.0",
					"generate",
					"-i",
					"/local/.opengb/openapi.json",
					"-g",
					config.generator,
					"-o",
					`/local/.opengb/sdk/${targetString}`,
					"--additional-properties=" +
					Object.entries(config.options).map(([key, value]) => `${key}=${value}`).join(","),
				],
			}).output();
		}

		if (!buildOutput.success) {
			throw new CommandError("Failed to generate OpenAPI SDK.", { commandOutput: buildOutput });
		}
	}

	let sdkCopyPath = sdkGenPath;
	if (target == SdkTarget.TypeScript) {
		await generateTypescriptAddons(project, sdkGenPath);
	} else if (target == SdkTarget.Unity) {
		await generateUnityAddons(project, sdkGenPath);
		sdkCopyPath = resolve(sdkGenPath, "src", UNITY_DEFAULT_PACKAGE_NAME);
	} else if (target == SdkTarget.Godot) {
		await generateGodot(project, sdkGenPath);
	}

	await move(sdkCopyPath, output, { overwrite: true });

	success("Success");
}

function targetToString(target: SdkTarget) {
	if (target == SdkTarget.TypeScript) return "typescript";
	if (target == SdkTarget.Unity) return "unity";
	if (target == SdkTarget.Godot) return "godot";
	throw new UnreachableError(target);
}
