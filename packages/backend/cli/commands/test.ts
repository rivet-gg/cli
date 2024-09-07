import { resolve } from "@std/path";
import { Command } from "@cliffy/command";
import * as glob from "glob";
import { GlobalOpts } from "../common.ts";
import { build, DbDriver, Format, Runtime } from "../../toolchain/build/mod.ts";
import { watch } from "../../toolchain/watch/mod.ts";
import { Project } from "../../toolchain/project/mod.ts";
import { UserError } from "../../toolchain/error/mod.ts";
import { info } from "../../toolchain/term/status.ts";
import { convertMigrateMode, migrateMode } from "./../util.ts";
import { ensurePostgresRunning, getDefaultDatabaseUrl } from "../../toolchain/postgres/mod.ts";

// TODO: https://github.com/rivet-gg/opengb-engine/issues/86
export const testCommand = new Command<GlobalOpts>()
	.description("Run tests")
	.type("migrate-mode", migrateMode)
	.arguments("[modules...:string]")
	.option("--no-build", "Don't build source files")
	.option("--no-check", "Don't check source files before running")
	.option("--strict-schemas", "Strictly validate schemas", { default: false })
	.option("--no-migrate", "Disable migrations")
	.option(
		"--migrate-mode <mode:migrate-mode>",
		"Configure how migrations are ran",
		{ default: "dev" },
	)
	.option("-w, --watch", "Automatically rerun tests on changes")
	.option("--filter <name:string>", "Filter tests by name")
	.action(
		async (opts, ...modulesFilter: string[]) => {
			await watch({
				loadProjectOpts: opts,
				disableWatch: !opts.watch,
				fn: async (project: Project, signal: AbortSignal) => {
					await ensurePostgresRunning(project);

					// Build project
					if (opts.build) {
						await build(project, {
							runtime: Runtime.Deno,
							format: Format.Native,
							dbDriver: DbDriver.NodePostgres,
							strictSchemas: opts.strictSchemas,
							// This gets ran on `deno test`
							skipDenoCheck: true,
							migrate: opts.migrate
								? {
									mode: convertMigrateMode(opts.migrateMode),
								}
								: undefined,
							signal,
						});
					}

					// Determine args
					const args = [
						"--allow-env",
						"--allow-net",
						"--allow-read",
					];
					if (opts.check) args.push("--check");
					if (opts.filter) args.push(`--filter=${opts.filter}`);

					// Find test scripts
					const testingModules = [];
					for (const module of project.modules.values()) {
						// Filter modules
						if (modulesFilter.length == 0) {
							// Only test local modules
							if (module.registry.isExternal) continue;
						} else {
							// Only test specified modules. This allows for testing remote modules.
							if (!modulesFilter.includes(module.name)) continue;
						}

						testingModules.push(module.name);

						// Test all modules or filter module tests
						const testPaths = (await glob.glob("*.ts", {
							cwd: resolve(module.path, "tests"),
						}))
							.map((path) => resolve(module.path, "tests", path));
						args.push(...testPaths);
					}

					if (testingModules.length == 0) {
						info("Finished", "No modules to test");
						return;
					}

					// Run tests
					info("Testing", testingModules.join(", "));
					const cmd = await new Deno.Command("deno", {
						args: [
							"test",
							...args,
						],
						stdout: "inherit",
						stderr: "inherit",
						signal,
						env: {
							"DATABASE_URL": await getDefaultDatabaseUrl(project),
							// Force color for test logs
							"OPENGB_TERM_COLOR": Deno.env.get("OPENGB_TERM_COLOR") ?? "always",
						},
					})
						.output();
					if (!cmd.success) throw new UserError("Tests failed.");
				},
			});
		},
	);
