import { assert } from "@std/assert";
import { exists } from "@std/fs";
import { isAbsolute } from "@std/path";
import { z } from "zod";
import { UserError, ValidationError } from "../error/mod.ts";
import { loadProjectConfigPath } from "../project/mod.ts";
import { SdkTarget } from "../sdk/generate.ts";

const RegistryGitUrlConfigSchema = z.union([z.string(), z.object({ https: z.string(), ssh: z.string() })]).describe(
	"The URL to the git repository. If both HTTPS and SSH URL are provided, they will both be tried and use the one that works",
);

const RegistryGitRefSchema = z.union([
	z.object({ branch: z.string() }),
	z.object({ tag: z.string() }),
	z.object({ rev: z.string() }),
]);

const RegistryLocalConfigSchema = z.object({
	local: z.object({
		directory: z.string(),
		isExternal: z.boolean().optional().describe(
			"If true, this will be treated like an external registry. This is important if multiple projects are using the same registry locally. Modules from this directory will not be tested, formatted, linted, and generate database migrations.",
		),
	}),
});

const RegistryGitConfigSchema = z.object({
	git: z.object({
		url: RegistryGitUrlConfigSchema,
		directory: z.string().optional(),
	}).and(RegistryGitRefSchema),
});

const RegistryGitHubConfigSchema = z.object({
	github: z.object({
		repository: RegistryGitUrlConfigSchema,
		directory: z.string().optional(),
	}).and(RegistryGitRefSchema),
});

const RegistryConfigSchema = z.union([
	RegistryLocalConfigSchema,
	RegistryGitConfigSchema,
	RegistryGitHubConfigSchema,
]);

const CorsConfigSchema = z.object({
	origins: z.array(z.string()).describe("The origins that are allowed to make requests to the server."),
});

const RuntimeConfigSchema = z.object({
	cors: CorsConfigSchema.optional(),
});

const SdkConfigSchema = z.object({
	target: z.enum([SdkTarget.TypeScript, SdkTarget.Unity, SdkTarget.Godot]),
	output: z.string().default("./sdk"),
});

const ProjectRouteConfigSchema = z.object({
	pathPrefix: z.string().optional().describe("The path prefix for all routes in this module."),
});

const ProjectModuleConfigSchema = z.object({
	registry: z.string().optional().describe("The name of the registry to fetch the module from."),
	module: z.string().optional().describe("Overrides the name of the module to fetch inside the registry."),
	config: z.record(z.unknown()).optional().describe(
		"The config that configures how this module is ran at runtime.",
	),
	storageAlias: z.string().optional().describe(
		"Used to store data in a consistent location in case the module name changes. This is used to construct the Postgres database schema and actor identifiers. Changing this will effectively unlink all data stored in this module. Changing it back to the old value will restore the data.",
	),
	routes: ProjectRouteConfigSchema.optional().describe(
		"Config options controlling how the routes are implemented and accessed.",
	),
});

export const ProjectConfigSchema = z.object({
	extends: z.string().optional().describe("Extends a different project config."),
	registries: z.record(RegistryConfigSchema).optional(),
	modules: z.record(ProjectModuleConfigSchema),
	runtime: RuntimeConfigSchema.optional(),
	sdks: z.array(SdkConfigSchema).optional(),
});

export type ProjectConfig = z.infer<typeof ProjectConfigSchema>;
export type RegistryConfig = z.infer<typeof RegistryConfigSchema>;

export type RegistryConfigLocal = z.infer<typeof RegistryLocalConfigSchema>["local"];

export type RegistryConfigGit = z.infer<typeof RegistryGitConfigSchema>["git"];
export type RegistryConfigGitHub = z.infer<typeof RegistryGitHubConfigSchema>["github"];

export type RegistryConfigGitUrl = z.infer<typeof RegistryGitUrlConfigSchema>;
export type RegistryGitRef = z.infer<typeof RegistryGitRefSchema>;

export type ProjectModuleConfig = z.infer<typeof ProjectModuleConfigSchema>;

export type ProjectRouteConfig = z.infer<typeof ProjectRouteConfigSchema>;

export type RuntimeConfig = z.infer<typeof RuntimeConfigSchema>;

export type SdkConfig = z.infer<typeof SdkConfigSchema>;

export type CorsConfig = z.infer<typeof CorsConfigSchema>;

export async function readConfig(projectConfigPath: string, extendedFromPaths: string[] = []): Promise<ProjectConfig> {
	// Resolve absolute path
	assert(isAbsolute(projectConfigPath), "project config path is not absolute");

	// Check for recursive path
	if (extendedFromPaths.includes(projectConfigPath)) {
		throw new UserError("Recursive extend path.", {
			details: `${projectConfigPath} extends from itself.`,
			suggest: "Check your `extends` parameters.",
			path: projectConfigPath,
		});
	}

	// Read config
	if (!await exists(projectConfigPath, { isFile: true })) {
		throw new UserError("Backend project does not exist.", {
			details: `${projectConfigPath} does not exist.`,
			suggest: "Run `rivet init` to create a project.",
			path: projectConfigPath,
		});
	}
	const configRaw = await Deno.readTextFile(projectConfigPath);
	const rawConfig: ProjectConfig = JSON.parse(configRaw);

	// Validate config
	const result = await ProjectConfigSchema.safeParseAsync(rawConfig);
	if (!result.success) {
		throw new ValidationError(`Invalid project configuration.`, {
			validationError: result.error,
			path: projectConfigPath,
		});
	}
	const childConfig = result.data;

	// Merge extended configs
	let config: ProjectConfig;
	if (childConfig.extends) {
		const baseConfig = await readConfig(loadProjectConfigPath({ project: childConfig.extends }), [
			projectConfigPath,
			...extendedFromPaths,
		]);
		config = {
			sdks: childConfig.sdks ?? baseConfig.sdks,
			registries: Object.assign({}, baseConfig.registries ?? {}, childConfig.registries ?? {}),
			modules: Object.assign({}, baseConfig.modules ?? {}, childConfig.modules ?? {}),
			runtime: Object.assign({}, baseConfig.runtime ?? {}, childConfig.runtime ?? {}),
		};
	} else {
		config = childConfig;
	}

	return config;
}
