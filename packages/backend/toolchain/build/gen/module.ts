import { genRuntimeActorDriverPath, hasUserConfigSchema, Module, moduleHelperGen, Project } from "../../project/mod.ts";
import { GeneratedCodeBuilder } from "./mod.ts";
import {
	ACTOR_CASE_CONVERSION_MAP_PATH,
	ACTOR_TYPEDEF_PATH,
	DEPENDENCY_CASE_CONVERSION,
	DEPENDENCY_TYPEDEF_PATH,
	DRIZZLE_ORM_REEXPORT,
	genModulePublicExternal,
	PACKAGES_PATH,
	projectDataPath,
	RUNTIME_CONFIG_PATH,
} from "../../project/project.ts";
import { camelify } from "../../../case_conversion/mod.ts";
import { BuildOpts } from "../mod.ts";
import { RUNTIME_MOD_PATH } from "../../project/project.ts";

export async function compileModuleHelper(
	project: Project,
	module: Module,
	opts: BuildOpts,
) {
	const helper = new GeneratedCodeBuilder(moduleHelperGen(project, module), 3);

	const runtimePath = helper.relative(projectDataPath(project, RUNTIME_MOD_PATH));
	const reexportPath = helper.relative(
		projectDataPath(project, PACKAGES_PATH, "runtime", "export_to_module.ts"),
	);
	const dependencyCaseConversionMapPath = helper.relative(projectDataPath(project, DEPENDENCY_CASE_CONVERSION));
	const actorCaseConversionMapPath = helper.relative(projectDataPath(project, ACTOR_CASE_CONVERSION_MAP_PATH));
	const runtimeConfigPath = helper.relative(projectDataPath(project, RUNTIME_CONFIG_PATH));

	// Import & re-export runtime files
	//
	// All runtime imports must be placed before any other generated files in
	// order to prevent `Cannot access 'xxxx' before initialization`.
	helper.chunk.withNewlinesPerChunk(1)
		.append`
			import {
				ModuleContextParams as ModuleContextParamsInner,
				ModuleContext as ModuleContextInner,
				TestContext as TestContextInner,
				ScriptContext as ScriptContextInner,
				ActorContext as ActorContextInner,
				RouteContext as RouteContextInner,
				Runtime,
			} from ${JSON.stringify(runtimePath)};
			import { ActorDriver } from ${JSON.stringify(helper.relative(genRuntimeActorDriverPath(project, opts.runtime)))};
			export * from ${JSON.stringify(reexportPath)};
    `;

	// Import & re-export generated files
	helper.chunk.withNewlinesPerChunk(1)
		.append`
			import config from ${JSON.stringify(runtimeConfigPath)};
			import { dependencyCaseConversionMap } from ${JSON.stringify(dependencyCaseConversionMapPath)};
			import { actorCaseConversionMap } from ${JSON.stringify(actorCaseConversionMapPath)};
		`;

	// Gen blocks
	const { userConfigType } = await genUserConfig(project, module, helper);
	genPublic(project, module, helper);
	genDependencies(project, module, helper);
	genActors(project, module, helper);
	genModule(project, module, helper, userConfigType);
	genTest(project, module, helper);
	genScript(project, module, helper);
	genRoute(project, module, helper);
	genActor(project, module, helper);

	// Write source
	await helper.write();
}

async function genUserConfig(
	_project: Project,
	module: Module,
	helper: GeneratedCodeBuilder,
): Promise<{ userConfigType: string }> {
	let userConfigType = "Record<string, never>";
	if (await hasUserConfigSchema(module)) {
		userConfigType = "UserConfig";
		helper.chunk.append`import { Config as UserConfig } from "./config.ts";`;
	}
	return { userConfigType };
}

function genPublic(project: Project, module: Module, helper: GeneratedCodeBuilder) {
	const publicExternalPath = helper.relative(genModulePublicExternal(project, module));
	helper.append`
		export * as Module from ${JSON.stringify(publicExternalPath)};
	`;
}

function genDependencies(
	project: Project,
	module: Module,
	helper: GeneratedCodeBuilder,
) {
	const typedefPath = projectDataPath(project, DEPENDENCY_TYPEDEF_PATH);

	helper.append`
		import type {
			DependenciesSnake as DependenciesSnakeFull,
			DependenciesCamel as DependenciesCamelFull,
		} from ${JSON.stringify(helper.relative(typedefPath))};
	`;

	const dependencyTypedefSnake = helper.chunk.withNewlinesPerChunk(1);
	const dependencyTypedefCamel = helper.chunk.withNewlinesPerChunk(1);

	const moduleNameSnake = module.name;
	const moduleNameCamel = camelify(module.name);

	for (const dependencyName of Object.keys(module.config.dependencies || {})) {
		const dependencyNameSnake = dependencyName;
		const dependencyNameCamel = camelify(dependencyName);

		dependencyTypedefSnake.append`
			${dependencyNameSnake}: DependenciesSnakeFull["${dependencyNameSnake}"];
		`;
		dependencyTypedefCamel.append`
			${dependencyNameCamel}: DependenciesCamelFull["${dependencyNameCamel}"];
		`;
	}

	dependencyTypedefSnake.prepend`${moduleNameSnake}: DependenciesSnakeFull["${moduleNameSnake}"];`;
	dependencyTypedefCamel.prepend`${moduleNameCamel}: DependenciesCamelFull["${moduleNameCamel}"];`;

	GeneratedCodeBuilder.wrap(
		"interface DependenciesSnake {",
		dependencyTypedefSnake,
		"}",
	);
	GeneratedCodeBuilder.wrap(
		"interface DependenciesCamel {",
		dependencyTypedefCamel,
		"}",
	);
}

function genActors(
	project: Project,
	module: Module,
	helper: GeneratedCodeBuilder,
) {
	const typedefPath = projectDataPath(project, ACTOR_TYPEDEF_PATH);

	helper.append`
		import type {
			ActorsSnake as ActorsSnakeFull,
			ActorsCamel as ActorsCamelFull,
		} from ${JSON.stringify(helper.relative(typedefPath))};
	`;

	const moduleNameSnake = module.name;
	const moduleNameCamel = camelify(module.name);

	helper.append`
    type ActorsSnake = ActorsSnakeFull["${moduleNameSnake}"];
    type ActorsCamel = ActorsCamelFull["${moduleNameCamel}"];
  `;
}

function genModule(
	project: Project,
	module: Module,
	helper: GeneratedCodeBuilder,
	userConfigType: string,
) {
	// Database
	if (module.db) {
		helper.append`
    export * as Query from ${JSON.stringify(helper.relative(projectDataPath(project, DRIZZLE_ORM_REEXPORT)))};
    export * as Database from "./db/schema.ts";
    `;
		helper.append`
    import type * as databaseSchema from "./db/schema.ts"
    type ModuleDatabaseSchema = typeof databaseSchema;
    `;
	}

	// Export block
	helper.chunk.withNewlinesPerChunk(2)
		.append`
      interface ModuleContextParams extends ModuleContextParamsInner {
        dependenciesSnake: DependenciesSnake;
        dependenciesCamel: DependenciesCamel;
        actorsSnake: ActorsSnake;
        actorsCamel: ActorsCamel;
        userConfig: ${userConfigType};
        databaseSchema: ${module.db ? "ModuleDatabaseSchema" : "undefined"};
      }
    `
		.append`
			export type ModuleContext = ModuleContextInner<ModuleContextParams>;
		`;
}

function genTest(
	_project: Project,
	module: Module,
	helper: GeneratedCodeBuilder,
) {
	// Export block
	helper.chunk.withNewlinesPerChunk(1)
		.newline()
		.append`
			export type TestContext = TestContextInner<ModuleContextParams>;
		`;

	// Test Block
	helper.chunk.withNewlinesPerChunk(2)
		.append`export type TestFn = (ctx: TestContext) => Promise<void>;`
		.append`
			export function test(name: string, fn: TestFn) {
				Runtime.test(
					config,
					new ActorDriver(Deno.env, config, dependencyCaseConversionMap, actorCaseConversionMap),
					"${module.name}",
					name,
					fn,
					dependencyCaseConversionMap,
					actorCaseConversionMap,
				);
			}
		`;
}

function genScript(
	_project: Project,
	_module: Module,
	helper: GeneratedCodeBuilder,
) {
	// Export block
	helper.chunk.withNewlinesPerChunk(1)
		.newline()
		.append`
			export type ScriptContext = ScriptContextInner<ModuleContextParams>;
		`;
}

function genActor(
	_project: Project,
	_module: Module,
	helper: GeneratedCodeBuilder,
) {
	// Export block
	helper.chunk.withNewlinesPerChunk(1)
		.newline()
		.append`
			export type ActorContext = ActorContextInner<ModuleContextParams>;
		`;
}

function genRoute(
	_project: Project,
	_module: Module,
	helper: GeneratedCodeBuilder,
) {
	// Export block
	helper.chunk.withNewlinesPerChunk(1)
		.newline()
		.append`
			export type RouteContext = RouteContextInner<ModuleContextParams>;

			export type RouteRequest = Request;
			export const RouteRequest = Request;

			export type RouteResponse = Response;
			export const RouteResponse = Response;
		`;
}
