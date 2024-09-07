import { UserError } from "../error/mod.ts";
import { ModuleConfig } from "../config/module.ts";
import { resolve } from "@std/path";
import { getLocalRegistry, Project } from "../project/mod.ts";
import dedent from "dedent";

export async function templateModule(project: Project, moduleName: string) {
	const localRegistry = getLocalRegistry(project);
	if (!localRegistry) throw new UserError("No \`local\` registry found in backend.json.");
	const localModulesPath = localRegistry.path;

	if (project.modules.has(moduleName)) {
		throw new UserError("Module already exists.");
	}

	// Create directories
	const modulePath = resolve(localModulesPath, moduleName);
	await Deno.mkdir(modulePath);
	await Deno.mkdir(resolve(modulePath, "scripts"));
	await Deno.mkdir(resolve(modulePath, "tests"));
	await Deno.mkdir(resolve(modulePath, "db"));

	// Write default config
	const defaultModule: ModuleConfig = {
		scripts: {},
		errors: {},
	};
	await Deno.writeTextFile(
		resolve(modulePath, "module.json"),
		JSON.stringify(defaultModule, null, "\t"),
	);

	// Write default migration
	const defaultSchema = dedent`
		import { schema, Query } from "./schema.gen.ts";

		export const myTable = schema.table("my_table", {
			id: Query.uuid("id").primaryKey().defaultRandom(),
			myColumn: Query.text("my_column").notNull(),
			createdAt: Query.timestamp("created_at").notNull().defaultNow(),
			updatedAt: Query.timestamp('updated_at').notNull().$onUpdate(() => new Date()),
		});
	`;
	await Deno.writeTextFile(
		resolve(modulePath, "db", "schema.ts"),
		defaultSchema,
	);

	// Add to backend.json
	const newConfig = structuredClone(project.config);
	newConfig.modules[moduleName] = {
		registry: "local",
	};
	await Deno.writeTextFile(
		resolve(project.path, "backend.json"),
		JSON.stringify(newConfig, null, "\t"),
	);
}
