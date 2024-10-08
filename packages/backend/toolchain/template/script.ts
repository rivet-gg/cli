import { UserError } from "../error/mod.ts";
import { resolve } from "@std/path";
import dedent from "dedent";
import { getLocalRegistry, Project } from "../project/mod.ts";

export async function templateScript(
	project: Project,
	moduleName: string,
	scriptName: string,
) {
	if (!getLocalRegistry(project)) throw new UserError("No \`local\` registry found in rivet.json.");

	const mod = project.modules.get(moduleName);
	if (!mod) throw new UserError(`Module \`${moduleName}\` does not exist.`);
	if (!("local" in mod.registry.config)) {
		throw new UserError(`Module \`${moduleName}\` does not belong to a local registry.`);
	}
	if (mod.registry.isExternal) throw new UserError(`Module \`${moduleName}\` must not be in an external registry.`);

	// Create scripts
	const scriptPath = resolve(
		mod.path,
		"scripts",
		scriptName + ".ts",
	);
	try {
		await Deno.stat(scriptPath);
		throw new UserError(`Script \`${scriptName}\` already exists in module \`${moduleName}\`.`);
	} catch (error) {
		if (!(error instanceof Deno.errors.NotFound)) {
			throw error;
		}
	}

	// Add script to config
	const newConfig = structuredClone(mod.config);
	newConfig.scripts = { ...newConfig.scripts };
	newConfig.scripts[scriptName] = { public: false };
	const newConfigRaw = JSON.stringify(newConfig, null, "\t");
	await Deno.writeTextFile(
		resolve(mod.path, "module.json"),
		newConfigRaw,
	);

	// Write default config
	const scriptTs = dedent`
		import { ScriptContext, Query, Database } from "../module.gen.ts";

		export interface Request {
			foo: string;
		}

		export interface Response {
			bar: string;
		}

		export async function run(ctx: ScriptContext, req: Request): Promise<Response> {
			throw new Error("Unimplemented: ${moduleName}.${scriptName}");
		}
	`;
	await Deno.writeTextFile(scriptPath, scriptTs);
}
