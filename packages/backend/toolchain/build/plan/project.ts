import { BuildState, buildStep, waitForBuildPromises } from "../../build_state/mod.ts";
import * as glob from "glob";
import { relative, resolve } from "@std/path";
import { DENO_JSON_PATH, DENO_LOCK_PATH, Project } from "../../project/mod.ts";
import { BuildOpts, Format, MigrateMode, Runtime } from "../mod.ts";
import { planModuleBuild, planModuleParse } from "./module.ts";
import { compileTypeHelpers } from "../gen/mod.ts";
import { generateDenoConfig } from "../deno_config.ts";
import { generateEntrypoint } from "../entrypoint.ts";
import { UnreachableError, UserError } from "../../error/mod.ts";
import { genProjectManifest } from "../project_manifest.ts";
import {
	BUNDLE_PATH,
	ENTRYPOINT_PATH,
	OPEN_API_PATH,
	OUTPUT_MANIFEST_PATH,
	PACKAGES_PATH,
	projectDataPath,
} from "../../project/project.ts";
import { compileActorTypeHelpers } from "../gen/mod.ts";
import { nodeModulesPolyfillPlugin } from "esbuild-plugins-node-modules-polyfill";
import { planProjectValidate } from "../validate.ts";
import { denoPlugins } from "@rivet-gg/esbuild-deno-loader";
import { migratePush } from "../../migrate/push.ts";
import { migrateApply } from "../../migrate/apply.ts";
import { migrateGenerate } from "../../migrate/generate.ts";
import { generateSdk } from "../../sdk/generate.ts";
import { denoExecutablePath } from "../../utils/deno.ts";
import { copy, exists } from "@std/fs";

import * as esbuild from "esbuild";
import { BACKEND_ROOT } from "../../utils/paths.ts";
import { assert } from "@std/assert";

export async function planProjectBuild(
	buildState: BuildState,
	project: Project,
	opts: BuildOpts,
) {
	const signal = buildState.signal;

	for (const module of project.modules.values()) {
		await planModuleParse(buildState, project, module, opts);
	}

	// Wait for modules parse
	await waitForBuildPromises(buildState);

	planProjectValidate(buildState, project);

	buildStep(buildState, {
		id: `project.generate.manifest`,
		name: "Generate",
		description: "project_manifest.json",
		dontLogProgress: true,
		async build() {
			await genProjectManifest(project);
		},
	});

	// Wait for rivet.json validation
	await waitForBuildPromises(buildState);

	// TODO: Add way to compare runtime artifacts (or let this be handled by the cache version and never rerun?)
	buildStep(buildState, {
		id: `project.generate.copy_runtime`,
		name: "Generate",
		description: "packages/",
		dontLogProgress: true,
		async build({ signal }) {
			const destDir = projectDataPath(project, PACKAGES_PATH);

			// Clear dest dir
			if (await exists(destDir, { isDirectory: true })) {
				await Deno.remove(destDir, { recursive: true });
			}

			// Copy runtime source files to the dest path
			const sourceDir = resolve(BACKEND_ROOT);
			const directories = ["runtime", "case_conversion", "path_resolver"];
			for (const dir of directories) {
				const sourcePath = resolve(sourceDir, dir);
				const destPath = resolve(destDir, dir);
				await copy(sourcePath, destPath);
			}
		},
	});

	await waitForBuildPromises(buildState);

	buildStep(buildState, {
		id: `project.generate.dependencies`,
		name: "Generate",
		description: "dependencies.d.ts",
		dontLogProgress: true,
		condition: {
			files: [...project.modules.values()].map((m) => resolve(m.path, "module.json")),
		},
		async build() {
			await compileTypeHelpers(project);
		},
	});

	buildStep(buildState, {
		id: `project.generate.actors`,
		name: "Generate",
		description: "actors.d.ts",
		dontLogProgress: true,
		condition: {
			files: [...project.modules.values()].map((m) => resolve(m.path, "module.json")),
		},
		async build() {
			await compileActorTypeHelpers(project);
		},
	});

	buildStep(buildState, {
		id: `project.generate.deno_config`,
		name: "Generate",
		description: "deno.json",
		dontLogProgress: true,
		async build() {
			await generateDenoConfig(project);
		},
	});

	for (const module of project.modules.values()) {
		await planModuleBuild(buildState, project, module, opts);
	}

	// Wait for module schemas requestSchema/responseSchema
	await waitForBuildPromises(buildState);

	buildStep(buildState, {
		id: `project.generate.entrypoint`,
		name: "Generate",
		description: "entrypoint.ts",
		dontLogProgress: true,
		async build() {
			await generateEntrypoint(project, opts);
		},
	});

	if (opts.sdk && project.config.sdks) {
		for (const sdk of project.config.sdks) {
			buildStep(buildState, {
				id: `project.generate.sdk`,
				name: "Generate SDK",
				description: sdk.output,
				async build() {
					await generateSdk(project, sdk);
				},
			});
		}
	}

	if (opts.format == Format.Bundled) {
		buildStep(buildState, {
			id: `project.bundle`,
			name: "Bundle",
			description: "bundle.js",
			async build({ signal }) {
				const bundledFile = projectDataPath(project, BUNDLE_PATH);

				// See Cloudflare Wrangler implementation:
				//
				// https://github.com/cloudflare/workers-sdk/blob/e8997b879605fb2eabc3e241086feb7aa219ef03/packages/wrangler/src/deployment-bundle/bundle.ts#L276
				const analyzeResult = Deno.env.get("_BACKEND_ESBUILD_META") == "1";
				const noMinify = Deno.env.get("_BACKEND_ESBUILD_NO_MINIFY") == "1";
				const result = await esbuild.build({
					entryPoints: [projectDataPath(project, ENTRYPOINT_PATH)],
					outfile: bundledFile,
					format: "esm",
					sourcemap: true,
					plugins: [
						// Bundle Deno dependencies
						...denoPlugins(),

						// HACK: esbuild-deno-loader does not play nice with
						// Windows paths, so we manually resolve any paths that
						// start with a Windows path separator (\) and resolve
						// them to the full path.
						{
							name: "fix-windows-paths",
							setup(build: esbuild.PluginBuild) {
								build.onResolve({ filter: /^\\.*/ }, (args) => {
									const resolvedPath = resolve(args.resolveDir, args.path);
									if (!exists(resolvedPath, { isFile: true })) {
										return {
											errors: [{ text: `File could not be resolved: ${resolvedPath}` }],
										};
									}

									return {
										path: resolve(args.resolveDir, args.path),
									};
								});
							},
						} satisfies esbuild.Plugin,

						// Remove unused Node imports
						nodeModulesPolyfillPlugin({
							globals: {
								Buffer: true,
								process: true,
							},
							modules: {
								// Not used:
								// https://github.com/brianc/node-postgres/blob/50c06f9bc6ff2ca1e8d7b7268b9af54ce49d72c1/packages/pg/lib/crypto/utils.js#L3
								crypto: "empty",
								dns: "empty",
								events: true,
								fs: "empty",
								net: "empty",
								path: "empty",
								string_decoder: true,
								tls: "empty",
								buffer: true,
							},
						}),
					],
					define: {
						// HACK: Disable `process.domain` in order to correctly handle this edge case:
						// https://github.com/brianc/node-postgres/blob/50c06f9bc6ff2ca1e8d7b7268b9af54ce49d72c1/packages/pg/lib/native/query.js#L126
						"process.domain": "undefined",
					},
					external: [
						// Check supported compat by Cloudflare Workers:
						// https://developers.cloudflare.com/workers/runtime-apis/nodejs/
						"node:process",
						"node:stream",
						"node:util",

						// TODO: Why is node:crypto not working? Are any of these external imports working?
						// https://community.cloudflare.com/t/not-being-able-to-import-node-crypto/613973
						// "node:crypto",

						// pg-native is overridden with pg-cloudflare at runtime
						"pg-native",

						// Wasm must be loaded as a separate file manually, cannot be bundled
						"*.wasm",
						"*.wasm?module",

						// This import only exists when running on cloudflare
						"cloudflare:*",
					],
					bundle: true,
					minify: !noMinify,

					logLevel: analyzeResult ? "debug" : "error",
					metafile: analyzeResult,
				});

				if (result.metafile) {
					console.log(await esbuild.analyzeMetafile(result.metafile));
				}

				signal.throwIfAborted();

				if (opts.runtime == Runtime.CloudflareWorkersPlatforms) {
					const bundleStr = await Deno.readTextFile(bundledFile);

					// TODO: Add ability for injecting WASM modules
					// // Find any `query-engine.wasm`
					// let wasmPath;
					// for (const module of project.modules.values()) {
					// 	const moduleWasmPath = resolve(
					// 		genPrismaOutputFolder(project, module),
					// 		"query_engine_bg.wasm",
					// 	);
					//
					// 	if (await exists(moduleWasmPath)) {
					// 		wasmPath = moduleWasmPath;
					// 		break;
					// 	}
					// }
					//
					// // Check if wasm is actually required
					// if (wasmPath) {
					// 	// Make wasm import relative
					// 	bundleStr = bundleStr.replaceAll(
					// 		/file:[\w\\/\.\-]+query_engine_bg\.wasm/g,
					// 		"query-engine.wasm",
					// 	);
					// } else if (/file:[\w\\/\.\-]+query_engine_bg\.wasm/.test(bundleStr)) {
					// 	throw new InternalError("Failed to find required query_engine_bg.wasm", { path: bundledFile });
					// }

					signal.throwIfAborted();

					await Deno.writeTextFile(bundledFile, bundleStr);

					// Write manifest of file paths for easier upload from Rivet CLI
					//
					// These modules are relative to the project root in case this was
					// generated from a Docker container.
					const manifest = {
						bundle: relative(project.path, bundledFile),
						wasm: undefined,
						// wasm: wasmPath ? relative(project.path, wasmPath) : undefined,
					};

					signal.throwIfAborted();

					await Deno.writeTextFile(
						projectDataPath(project, OUTPUT_MANIFEST_PATH),
						JSON.stringify(manifest),
					);
				}
			},
		});
	}

	await waitForBuildPromises(buildState);

	// TODO: This is disabled when building for cf because there is an unresolved import
	if (opts.runtime != Runtime.CloudflareWorkersPlatforms) {
		buildStep(buildState, {
			id: `project.check.entrypoint`,
			name: "Check",
			description: "entrypoint.ts",
			async build() {
				const checkOutput = await new Deno.Command(denoExecutablePath(), {
					args: [
						"check",
						"--config",
						projectDataPath(project, DENO_JSON_PATH),
						"--lock",
						projectDataPath(project, DENO_LOCK_PATH),
						"--quiet",
						projectDataPath(project, ENTRYPOINT_PATH),
					],
					signal,
				}).output();
				if (!checkOutput.success) {
					throw new UserError("Check failed.", {
						details: new TextDecoder().decode(checkOutput.stderr).trim(),
					});
				}
			},
		});
	}

	await waitForBuildPromises(buildState);

	// Run migrations
	if (opts.migrate) {
		// Run local module migrations.
		//
		// We push the schema changes in order to allow developers to iterate
		// quickly. This has a risk of data loss, but we assume that this is fine
		// in development.

		// Split modules between how to apply the migration
		//
		// External modules are applied since they are not being modified.
		//
		// Local/dev modules are pushed since the schema will frequently change.
		// This has a risk of data loss, but this is required if iterating quickly
		// on a schema.
		const generateMigrations = [];
		const applyMigrations = [];
		const pushMigrations = [];
		for (const module of project.modules.values()) {
			if (module.db) {
				if (opts.migrate.mode == MigrateMode.Dev) {
					if (module.registry.isExternal) {
						applyMigrations.push(module);
					} else {
						pushMigrations.push(module);
					}
				} else if (opts.migrate.mode == MigrateMode.Generate) {
					if (!module.registry.isExternal) {
						generateMigrations.push(module);
					}
				} else if (opts.migrate.mode == MigrateMode.GenerateAndApply) {
					if (!module.registry.isExternal) {
						generateMigrations.push(module);
					}
					applyMigrations.push(module);
				} else if (opts.migrate.mode == MigrateMode.Apply) {
					applyMigrations.push(module);
				} else {
					throw new UnreachableError(opts.migrate.mode);
				}
			}
		}

		// Run generate commands one-by-one since they may have an interactive promp
		for (const module of generateMigrations) {
			buildStep(buildState, {
				id: `module.${module.name}.migrate.generate`,
				name: "Generate Migrations",
				module,
				description: "generate",
				condition: {
					files: [resolve(module.path, "db", "schema.ts")],
				},
				async build({ signal }) {
					await migrateGenerate(project, [module], signal);
				},
			});

			await waitForBuildPromises(buildState);
		}

		// Run push commands one-by-one since they may have an interactive promp
		for (const module of pushMigrations) {
			buildStep(buildState, {
				id: `module.${module.name}.migrate.push`,
				name: "Migrate Database",
				module,
				description: "push",
				condition: {
					files: [resolve(module.path, "db", "schema.ts")],
				},
				async build({ signal }) {
					await migratePush(project, [module], signal);
				},
			});

			await waitForBuildPromises(buildState);
		}

		// Run apply in parallel since it's non-interactive
		for (const module of applyMigrations) {
			// List all migrations
			const migrationsCwd = resolve(module.path, "db", "migrations");
			const migrationsRelative = await glob.glob("*.sql", { cwd: migrationsCwd });
			assert(migrationsRelative.length > 0, `must have more than 1 migration for module ${module.name}`);
			const migrations = migrationsRelative.map((x) => resolve(migrationsCwd, x));

			buildStep(buildState, {
				id: `module.${module.name}.migrate.apply`,
				name: "Migrate Database",
				module,
				description: "apply",
				condition: {
					files: migrations,
				},
				async build({ signal }) {
					await migrateApply(project, [module], signal);
				},
			});
		}

		await waitForBuildPromises(buildState);
	}
}
