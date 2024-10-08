import { Hono, type MiddlewareHandler } from "@hono/hono";
import { createFactory } from "@hono/hono/factory";
import { serveStatic } from "@hono/hono/deno";
import { validator } from "@hono/hono/validator";
import { resolve } from "@std/path";
import { decodeBase64 } from "@std/encoding";
import { InternalState } from "./state.ts";
import { info, progress } from "../term/status.ts";
import { ProjectManifest } from "../build/project_manifest.ts";
import { ProjectConfigSchema } from "../config/project.ts";
import { PROJECT_MANIFEST_PATH, projectDataPath } from "../project/mod.ts";
import { BACKEND_ROOT } from "../utils/paths.ts";

interface Env {
	Variables: {
		state: InternalState;
	};
}

export const internalApi = new Hono<Env>()
	.get("/state", (c) => {
		const state = c.get("state").get();
		if (state.value === "idle") {
			return c.json({ error: "No project loaded" }, 400);
		}
		if (state.value === "failure") {
			return c.json({ value: state.value, config: state.project.config, error: state.error });
		}
		return c.json({ value: state.value, config: state.project.config });
	})
	.get("/project.json", (c) => {
		const state = c.get("state").get();
		if (state.value === "idle") {
			return c.json({ error: "No project loaded" }, 400);
		}

		const project = state.project;
		return c.json(project);
	})
	.get("/project_manifest.json", async (c) => {
		const state = c.get("state").get();
		if (state.value === "idle") {
			return c.json({ error: "No project loaded" }, 400);
		}

		const output = await Deno.readTextFile(projectDataPath(state.project, PROJECT_MANIFEST_PATH));
		return c.json<ProjectManifest>(JSON.parse(output), 200);
	})
	.patch(
		"/config",
		validator("json", (value, c) => {
			const result = ProjectConfigSchema.omit({ registries: true, runtime: true }).safeParse(value);
			if (!result.success) {
				return c.json({ error: "Invalid body" }, 400);
			}
			return result.data;
		}),
		async (c) => {
			const state = c.get("state").get();
			if (state.value === "idle") {
				return c.json({ error: "No project loaded" }, 400);
			}

			const existingConfig = await Deno.readTextFile(
				resolve(state.project.path, "rivet.json"),
			);
			await Deno.writeTextFile(
				resolve(state.project.path, "rivet.json"),
				JSON.stringify({ ...JSON.parse(existingConfig), modules: c.req.valid("json").modules }, null, "\t"),
			);

			await new Promise<void>((resolve) => {
				const unsubscribe = c.get("state").on("changed", (state) => {
					if (state.value === "success" || state.value === "failure") {
						unsubscribe();
						resolve();
					}
				});
			});

			return c.body(null, 204);
		},
	);

export type InternalApi = typeof internalApi;

export function createProjectInternalApiRouter(internalState: InternalState) {
	const factory = createFactory<Env>({
		initApp: (app) => {
			app.use(async (c, next) => {
				c.set("state", internalState);
				await next();
			});
		},
	});

	const app = factory.createApp();

	app.route("/__internal", internalApi);
	app.get(
		"/*",
		serveStatic({ root: resolve(BACKEND_ROOT, "artifacts", "editor") }),
	);

	return app;
}

export function createAndStartProjectInternalApiRouter(internalState: InternalState) {
	const app = createProjectInternalApiRouter(internalState);

	const hostname = Deno.env.get("RIVET_EDITOR_HOST") ?? "127.0.0.1";
	const port = parseInt(Deno.env.get("RIVET_EDITOR_PORT") ?? "6421");
	Deno.serve({
		hostname,
		port,
		handler: app.fetch,
		onListen: () => {
			info("Editor", `http://${hostname}:${port}`);
		},
	});

	return app;
}
