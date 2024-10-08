import { DbDriver } from "../build/mod.ts";
import { build, Format, Runtime } from "../build/mod.ts";
import { resolve } from "@std/path";
import { printError } from "../error/mod.ts";
import { loadProject, releaseProject } from "../project/mod.ts";
import dedent from "dedent";
import { templateModule } from "./module.ts";
import { templateProject } from "./project.ts";
import { templateScript } from "./script.ts";

Deno.test({
	name: "e2e",

	// TODO: https://github.com/rivet-gg/opengb-engine/issues/35
	sanitizeOps: false,
	sanitizeResources: false,

	async fn() {
		const path = await Deno.makeTempDir();

		await templateProject(path);

		{
			const project = await loadProject({ project: path });
			await templateModule(project, "module_a");
			await releaseProject(project);
		}

		// Append test model to schema
		const schemaPath = resolve(path, "modules", "module_a", "db", "schema.ts");
		let schema = await Deno.readTextFile(schemaPath);
		schema += "\n";
		schema += dedent`
			export const testTable = schema.table("test_table", {
				id: Query.uuid("id").primaryKey().defaultRandom(),
			});
		`;
		await Deno.writeTextFile(schemaPath, schema);

		{
			const project = await loadProject({ project: path });
			await templateScript(project, "module_a", "script_a");
			await releaseProject(project);
		}

		try {
			const project = await loadProject({ project: path });
			await build(project, {
				format: Format.Native,
				runtime: Runtime.Deno,
				dbDriver: DbDriver.NodePostgres,
				strictSchemas: true,
				skipDenoCheck: false,
			});
			await releaseProject(project);
		} catch (err) {
			printError(err);
			throw err;
		}
	},
});
