import { assert, assertExists } from "@std/assert";
import { copy, emptyDir, exists } from "@std/fs";
import { dirname, isAbsolute, join, normalize, resolve } from "@std/path";
import { dir } from "@cross/dir";
import * as glob from "glob";
import { readConfig as readProjectConfig } from "../config/project.ts";
import { ProjectConfig } from "../config/project.ts";
import { loadModule, Module } from "./module.ts";
import { loadLocalRegistry, loadRegistry, Registry } from "./registry.ts";
import { ProjectModuleConfig } from "../config/project.ts";
import { validateIdentifier } from "../types/identifiers/mod.ts";
import { Casing } from "../types/identifiers/defs.ts";
import { loadDefaultRegistry } from "./registry.ts";
import { UnreachableError, UserError } from "../error/mod.ts";
import { Runtime } from "../build/mod.ts";
import { PathResolver, QualifiedPathPair } from "../../path_resolver/mod.ts";
import { RouteCollisionError } from "../error/mod.ts";
import { info, verbose } from "../term/status.ts";
import { IndexedModuleConfig } from "../config/module.ts";
import { acquireLock, FsLock, releaseLock } from "../utils/fslock.ts";

export interface Project {
	path: string;
	lock: FsLock;
	configPath: string;
	config: ProjectConfig;
	registries: Map<string, Registry>;
	modules: Map<string, Module>;
}

export interface LoadProjectOpts {
	/** Path to the project root or project config. */
	project: string | null;
}

/**
 * Resolves the input to the backend config path.
 *
 * The project directory path can be resolved from the dirname on this path.
 */
export function loadProjectConfigPath(opts: LoadProjectOpts): string {
	const path = resolve(Deno.cwd(), opts.project ?? ".");
	if (path.endsWith(".json")) {
		return path;
	} else {
		return resolve(path, "rivet.json");
	}
}

export async function loadProject(opts: LoadProjectOpts, signal?: AbortSignal): Promise<Project> {
	const projectConfigPath = loadProjectConfigPath(opts);
	const projectRoot = dirname(projectConfigPath);

	// Create data dir if needed
	await Deno.mkdir(backendDataDir(), { recursive: true });

	// Read project config
	const projectConfig = await readProjectConfig(
		projectConfigPath,
	);

	// Lock project
	const lock = await acquireLock({
		path: resolve(backendDataDir(), LOCK_PATH),
		onWaiting: () => {
			info("Waiting", "another process is currently modifying this project");
		},
	});

	// Load registries
	const registries = new Map();

	if (projectConfig.registries) {
		for (const [registryName, registryConfig] of Object.entries(projectConfig.registries)) {
			const registry = await loadRegistry(
				projectRoot,
				registryName,
				registryConfig,
				signal,
			);
			registries.set(registryName, registry);
		}
	}

	if (!registries.has("default")) {
		const defaultRegistry = await loadDefaultRegistry(projectRoot, signal);
		registries.set("default", defaultRegistry);
	}
	if (!registries.has("local")) {
		const localRegistry = await loadLocalRegistry(projectRoot, signal);
		registries.set("local", localRegistry);
	}

	// Validate local registry
	const localRegistry = registries.get("local");
	if (localRegistry) {
		if (!("local" in localRegistry.config)) {
			throw new UserError("Registry named `local` is special and must be configured as a local registry type.", {
				path: projectConfigPath,
			});
		}
		if (localRegistry.isExternal) {
			throw new UserError("Registry named `local` is special and can't be external.", {
				path: projectConfigPath,
			});
		}
	}

	// Load modules
	const modules = new Map<string, Module>();
	for (const [projectModuleName, projectModuleConfig] of Object.entries(projectConfig.modules)) {
		const { path, registry } = await fetchAndResolveModule(
			projectRoot,
			projectConfigPath,
			registries,
			projectModuleName,
			projectModuleConfig,
		);
		const module = await loadModule(
			projectConfigPath,
			path,
			projectModuleName,
			projectModuleConfig,
			registry,
			signal,
		);
		modules.set(projectModuleName, module);
	}

	// Verify uniqueness of storage alias
	for (const moduleA of modules.values()) {
		for (const moduleB of modules.values()) {
			if (moduleA.name != moduleB.name && moduleA.storageAlias == moduleB.storageAlias) {
				throw new UserError("Duplicate module storage alias.", {
					details: `Conflicting modules for alias ${moduleA.storageAlias}: ${moduleA.name}, ${moduleB.name}`,
					path: projectConfigPath,
				});
			}
		}
	}

	// Verify existence of module dependencies
	const missingDepsByModule = new Map<string, string[]>();
	for (const module of modules.values()) {
		const missingDeps = [] as string[];
		for (const dependencyName in module.config.dependencies) {
			const dependencyModule = modules.get(dependencyName);
			if (!dependencyModule) {
				missingDeps.push(dependencyName);
			}
		}

		if (missingDeps.length > 0) {
			missingDepsByModule.set(module.name, missingDeps);
		}
	}

	if (missingDepsByModule.size > 0) {
		let details = "";
		for (const [moduleName, missingDeps] of missingDepsByModule) {
			details += `Cannot resolve dependencies for ${moduleName}: ${missingDeps.join(", ")}\n`;
		}
		throw new UserError("Unresolved module dependencies.", {
			details,
			path: projectConfigPath,
		});
	}

	// Module can't depend on itself
	const selfDepModules = [] as string[];
	for (const module of modules.values()) {
		if (module.config.dependencies?.[module.name]) {
			selfDepModules.push(module.name);
		}
	}

	if (selfDepModules.length > 0) {
		let details = "";
		for (const moduleName of selfDepModules) {
			details += `Module ${moduleName} can't depend on itself\n`;
		}
		throw new UserError("Modules can't depend on themselves.", {
			details,
			path: projectConfigPath,
		});
	}

	// Verify routes don't conflict
	const routes: QualifiedPathPair[] = [];
	for (const [moduleName, module] of modules.entries()) {
		for (const [routeName, route] of module.routes.entries()) {
			const isPrefix = "pathPrefix" in route.config;
			const path = "pathPrefix" in route.config ? route.config.pathPrefix : route.config.path;
			routes.push({
				module: moduleName,
				route: routeName,
				path: { isPrefix, path },
			});
		}
	}

	const collisions = new PathResolver(routes).collisions;
	if (collisions.length > 0) throw new RouteCollisionError(collisions);

	return {
		path: projectRoot,
		lock,
		configPath: projectConfigPath,
		config: projectConfig,
		registries,
		modules,
	};
}

interface FetchAndResolveModuleOutput {
	/**
	 * Path the module was copied to in data dir.
	 */
	path: string;

	/**
	 * Path to the original module source code.
	 */
	sourcePath: string;

	/**
	 * Registry the module was fetched from.
	 */
	registry: Registry;

	/**
	 * A short version of the module config
	 */
	config: IndexedModuleConfig;
}

/**
 * Clones a registry to a local machine and resolves the path to the module.
 */
export async function fetchAndResolveModule(
	projectRoot: string,
	projectConfigPath: string,
	registries: Map<string, Registry>,
	moduleName: string,
	module: ProjectModuleConfig,
): Promise<FetchAndResolveModuleOutput> {
	validateIdentifier(moduleName, Casing.Snake);

	// Lookup registry
	const registryName = registryNameForModule(module);
	const registry = registries.get(registryName);
	if (!registry) {
		throw new UserError(`Registry \`${registryName}\` not found for module \`${module}\`.`, {
			path: projectConfigPath,
		});
	}

	// Resolve module path
	const pathModuleName = moduleNameInRegistry(moduleName, module);
	const moduleConfig = registry.modules[pathModuleName];
	if (!moduleConfig) {
		if (pathModuleName !== moduleName) {
			// Has alias
			throw new UserError(
				`Module \`${pathModuleName}\` (alias of \`${moduleName}\`) not found in registry \`${registryName}\`.`,
				{ paths: [projectConfigPath, registry.path] },
			);
		} else {
			// No alias
			throw new UserError(
				`Module \`${pathModuleName}\` not found in registry \`${registryName}\`.`,
				{ paths: [projectConfigPath, registry.path] },
			);
		}
	}

	const sourcePath = resolve(registry.path, pathModuleName);
	let path: string;
	if (registry.isExternal) {
		// Copy to gen dir
		//
		// We do this so generate files into the gen dir without modifying the
		// original module. For example. if multiple projects are using the same
		// local registry, we don't want conflicting generated files.
		path = resolve(
			backendDataDir(),
			"external_modules",
			moduleName,
		);

		// HACK: Copy gen file to temp dir to avoid overwriting it
		const tempGenDir: string | undefined = await Deno.makeTempDir();

		const moduleGenPath = resolve(path, "module.gen.ts");
		let tempModuleGenPath: string | undefined;
		if (await exists(moduleGenPath, { isFile: true })) {
			tempModuleGenPath = resolve(tempGenDir, "module.gen.ts");
			await copy(moduleGenPath, tempModuleGenPath, { overwrite: true });
		}

		const schemaGenPath = resolve(path, "db", "schema.gen.ts");
		let tempSchemaGenPath: string | undefined;
		if (await exists(schemaGenPath, { isFile: true })) {
			tempSchemaGenPath = resolve(tempGenDir, "schema.gen.ts");
			await copy(schemaGenPath, tempSchemaGenPath, { overwrite: true });
		}

		// TODO: Only copy when needed, this copies every time the CLI is ran
		await emptyDir(path);
		await copy(sourcePath, path, { overwrite: true });

		// HACK: Restore gen file
		// Cannot `move` from a temp dir because /tmp might be in a
		// different file system
		if (tempModuleGenPath) {
			await copy(tempModuleGenPath, moduleGenPath, { overwrite: true });
		}
		if (tempSchemaGenPath) {
			await copy(tempSchemaGenPath, schemaGenPath, { overwrite: true });
		}
		await Deno.remove(tempGenDir, { recursive: true });
	} else {
		// Use original path
		path = sourcePath;
	}

	return { path, sourcePath: sourcePath, registry, config: moduleConfig };
}

function registryNameForModule(module: ProjectModuleConfig): string {
	return module.registry ?? "default";
}

function moduleNameInRegistry(
	moduleName: string,
	module: ProjectModuleConfig,
): string {
	return module.module ?? moduleName;
}

export const BUNDLE_PATH = "output.js";
export const OUTPUT_MANIFEST_PATH = "output_manifest.json";
export const PROJECT_MANIFEST_PATH = "project_manifest.json";
export const OPEN_API_PATH = "openapi.json";
export const CACHE_PATH = "cache.json";
export const SDK_PATH = "sdk";
export const LOCK_PATH = "project_lock";

// Generated project source
export const OUTPUT_SOURCE_PATH = "output_source";

export const ACTOR_CASE_CONVERSION_MAP_PATH = join(OUTPUT_SOURCE_PATH, "actorCaseConversion.ts");
export const ACTOR_TYPEDEF_PATH = join(OUTPUT_SOURCE_PATH, "actors.d.ts");
export const DENO_JSON_PATH = join(OUTPUT_SOURCE_PATH, "deno.jsonc");
export const DENO_LOCK_PATH = join(OUTPUT_SOURCE_PATH, "deno.lock");
export const DEPENDENCY_TYPEDEF_PATH = join(OUTPUT_SOURCE_PATH, "dependencies.d.ts");
export const DEPENDENCY_CASE_CONVERSION = join(OUTPUT_SOURCE_PATH, "dependencyCaseConversion.ts");
export const DRIZZLE_ORM_REEXPORT = join(OUTPUT_SOURCE_PATH, "drizzle_orm_reexport.ts");
export const ENTRYPOINT_PATH = join(OUTPUT_SOURCE_PATH, "entrypoint.ts");
export const RUNTIME_CONFIG_PATH = join(OUTPUT_SOURCE_PATH, "runtime_config.ts");
export const PUBLIC_UTILS_PATH = join(OUTPUT_SOURCE_PATH, "public");
export const DEPENDENCIES_PATH = join(OUTPUT_SOURCE_PATH, "dependencies");

export const PACKAGES_PATH = join(OUTPUT_SOURCE_PATH, "packages");
export const RUNTIME_ACTOR_PATH = join(PACKAGES_PATH, "runtime", "actor", "actor.ts");
export const RUNTIME_POSTGRES_PATH = join(PACKAGES_PATH, "runtime", "postgres.ts");
export const RUNTIME_MOD_PATH = join(PACKAGES_PATH, "runtime", "mod.ts");

/**
 * Path at which all backend data is stored, including caches, build outputs,
 * etc.
 */
export function backendDataDir(): string {
	let dataDir = Deno.env.get("BACKEND_DATA_DIR");
	assertExists(dataDir, "missing BACKEND_DATA_DIR");
	return dataDir;
}

export function projectDataPath(_project: Project, ...pathSegments: string[]): string {
	return resolve(backendDataDir(), ...pathSegments);
}

export function genRuntimeActorDriverPath(project: Project, runtime: Runtime): string {
	let actorDriverName: string;
	if (runtime == Runtime.Deno) {
		actorDriverName = "memory";
	} else if (runtime == Runtime.CloudflareWorkersPlatforms) {
		actorDriverName = "cloudflare_durable_objects";
	} else {
		throw new UnreachableError(runtime);
	}

	return projectDataPath(
		project,
		PACKAGES_PATH,
		"runtime",
		"actor",
		"drivers",
		actorDriverName,
		"driver.ts",
	);
}

/**
 * Inner file used to nest any imports related to this module.
 *
 * This will be re-imported in other `genModulePublicExternal`.
 */
export function genModulePublicInternal(project: Project, module: Module): string {
	return projectDataPath(project, PUBLIC_UTILS_PATH, `internal_${module.name}.ts`);
}

/**
 * File that gets imported as `Module` in the module.gen.ts.
 *
 * This exports the dependencies (the `genModulePublicInternal` files) with
 * their given module names.
 */
export function genModulePublicExternal(project: Project, module: Module): string {
	return projectDataPath(project, PUBLIC_UTILS_PATH, `external_${module.name}.ts`);
}

export function genModuleDependenciesPath(project: Project, module: Module): string {
	return projectDataPath(project, DEPENDENCIES_PATH, `dependencies_${module.name}.ts`);
}

export interface ListSourceFileOpts {
	/**
	 * Only include local files.
	 *
	 * Useful for commands that are only relevant to modules written by the user.
	 */
	localOnly?: boolean;
}

/**
 * List all paths to TypeScript files in the project.
 */
export async function listSourceFiles(
	project: Project,
	opts: ListSourceFileOpts = {},
): Promise<string[]> {
	const files: string[] = [];
	for (const module of project.modules.values()) {
		// Skip non-local files
		if (opts.localOnly && module.registry.isExternal) continue;

		const moduleFiles = (await glob.glob("**/*.ts", { cwd: module.path }))
			.map((x) => resolve(module.path, x));
		files.push(...moduleFiles);
	}
	return files;
}

export async function cleanProject(project: Project) {
	// Delete project files
	verbose("Removing project gen path");
	await Deno.remove(projectDataPath(project), { recursive: true });
}

export function getDefaultRegistry(project: Project): Registry | undefined {
	return project.registries.get("default");
}

export function getLocalRegistry(
	project: Project,
): Registry | undefined {
	const localRegistry = project.registries.get("local");
	if (localRegistry) {
		assert("local" in localRegistry.config);
		assert(!localRegistry.isExternal);
		return localRegistry;
	} else {
		return undefined;
	}
}

export async function releaseProject(project: Project) {
	// Release lock
	await releaseLock(project.lock);
}

export function ensureLocked(project: Project) {
	assert(!project.lock.released, "project lock must not be released");
}
