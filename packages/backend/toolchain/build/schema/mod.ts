import { z as zod } from "zod";
import { AnySchemaElement, is } from "./schema.ts";
import { BACKEND_SCHEMA_TYPESCRIPT_LIB_FILE, DEFAULT_COMPILER_OPTIONS } from "./serializer.ts";
import { Project } from "@ts-morph/ts-morph";

export { schemaElements } from "./schema.ts";
export { createSchemaSerializer } from "./serializer.ts";

export * from "./schema.ts";
export * from "./deserializer.ts";

export { convertZodToSerializedSchema } from "./serializer.ts";

import { z } from "zod";
import { extendZodWithOpenApi } from "@asteasolutions/zod-to-openapi";
extendZodWithOpenApi(z);

export const convertSchemaToZod = (
	schema: AnySchemaElement,
): zod.ZodTypeAny => {
	if (is("unknown", schema)) {
		return zod.unknown();
	}
	if (is("optional", schema)) {
		return zod.optional(convertSchemaToZod(schema.value));
	}
	if (is("date", schema)) {
		return zod.date();
	}
	if (is("string", schema)) {
		return zod.string();
	}
	if (is("number", schema)) {
		return zod.number();
	}
	if (is("boolean", schema)) {
		return zod.boolean();
	}
	if (is("undefined", schema)) {
		return zod.undefined();
	}
	if (is("null", schema)) {
		return zod.null();
	}
	if (is("any", schema)) {
		return zod.any();
	}
	if (is("literal", schema)) {
		return zod.literal(schema.value);
	}
	if (is("tuple", schema)) {
		const [schemaA, schemaB, ...schemas] = schema.items.map(convertSchemaToZod);
		return zod.tuple([schemaA!, schemaB!, ...schemas]);
	}
	if (is("array", schema)) {
		return zod.array(convertSchemaToZod(schema.item));
	}
	if (is("intersection", schema)) {
		return zod.intersection(
			convertSchemaToZod(schema.left),
			convertSchemaToZod(schema.right),
		);
	}
	if (is("union", schema)) {
		const [schemaA, schemaB, ...schemas] = schema.items.map(convertSchemaToZod);
		return zod.union([schemaA!, schemaB!, ...schemas]);
	}
	if (is("nullable", schema)) {
		return zod.nullable(convertSchemaToZod(schema.item));
	}
	if (is("object", schema)) {
		return zod.object(
			Object.fromEntries(
				Object.entries(schema.properties).map((
					[name, type],
				) => [name, convertSchemaToZod(type)]),
			),
		).strict();
	}
	if (is("record", schema)) {
		return zod.record(convertSchemaToZod(schema.elementType));
	}

	return zod.unknown();
};

export const convertSerializedSchemaToZod = (
	serializedSchema: AnySchemaElement,
) => {
	return convertSchemaToZod(serializedSchema);
};

export const getSourceFileDependencies = (path: string, { skipInternal = true }: { skipInternal?: boolean } = {}) => {
	const project = new Project({
		compilerOptions: DEFAULT_COMPILER_OPTIONS,
	});
	project.addSourceFileAtPath(path);
	project.resolveSourceFileDependencies();

	const sourceFiles = project.getSourceFiles().map((sourceFile) => sourceFile.getFilePath());

	if (skipInternal) {
		return sourceFiles.filter((sourceFile) => !sourceFile.includes(BACKEND_SCHEMA_TYPESCRIPT_LIB_FILE));
	}
	return sourceFiles;
};
