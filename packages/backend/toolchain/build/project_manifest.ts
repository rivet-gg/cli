import { camelify, pascalify } from "../../case_conversion/mod.ts";
import type { IndexedModuleConfig, ModuleConfig, ScriptConfig } from "../config/module.ts";
import { ProjectConfig } from "../config/project.ts";
import { RegistryConfig } from "../config/project.ts";
import { hasUserConfigSchema, Project, PROJECT_MANIFEST_PATH } from "../project/mod.ts";
import { projectCachePath } from "../project/project.ts";
import { AnySchemaElement } from "./schema/mod.ts";

export interface ProjectManifest {
	config: ProjectConfig;
	registries: Record<string, RegistryManifest>;
	modules: Record<string, ModuleManifest>;
}

export interface RegistryManifest {
	path: string;
	name: string;
	config: RegistryConfig;
	isExternal: boolean;
	modules: Record<string, IndexedModuleConfig>;
}

export interface ModuleManifest {
	path: string;
	name: string;
	nameCamel: string;
	namePascal: string;
	config: ModuleConfig;
	registryName: string;
	userConfig: unknown;
	userConfigSchema?: AnySchemaElement;
	scripts: Record<string, ScriptManifest>;
	db?: ModuleDatabaseManifest;
	hasUserConfigSchema: boolean;
}

export interface ModuleDatabaseManifest {
	schema: string;
}

export interface ScriptManifest {
	path: string;
	name: string;
	nameCamel: string;
	namePascal: string;
	config: ScriptConfig;
	requestSchema: AnySchemaElement;
	responseSchema: AnySchemaElement;
}

/**
 * Generates manifest file that can be consumed externally to get information about
 * this project. For example, this is used to auto-generate docs from external
 * tools.
 */
export async function genProjectManifest(project: Project) {
	const registries: Record<string, RegistryManifest> = Object.fromEntries(
		Array.from(project.registries.entries()).map(([name, registry]) => [name, {
			path: registry.path,
			name: name,
			config: registry.config,
			modules: registry.modules,
			isExternal: registry.isExternal,
		}]),
	);

	const modules: Record<string, ModuleManifest> = {};
	for (const module of project.modules.values()) {
		modules[module.name] = {
			path: module.path,
			name: module.name,
			nameCamel: camelify(module.name),
			namePascal: pascalify(module.name),
			config: module.config,
			registryName: module.registry.name,
			userConfig: module.userConfig,
			userConfigSchema: module.userConfigSchema,
			scripts: Object.fromEntries(
				Array.from(module.scripts.entries()).map(([name, script]) => [name, {
					path: script.path,
					name: name,
					nameCamel: camelify(name),
					namePascal: pascalify(name),
					config: script.config,
					requestSchema: script.schemas?.request!,
					responseSchema: script.schemas?.response!,
				}]),
			),
			db: module.db,
			hasUserConfigSchema: await hasUserConfigSchema(module),
		};
	}

	const manifest: ProjectManifest = {
		config: project.config,
		registries,
		modules,
	};

	await Deno.writeTextFile(
		projectCachePath(project, PROJECT_MANIFEST_PATH),
		JSON.stringify(manifest, null, 4),
	);
}
