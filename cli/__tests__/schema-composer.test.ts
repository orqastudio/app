/**
 * Tests for schema-composer.ts — the pure helper functions that can be
 * tested without a real plugin installation.
 *
 * The exported public API is composeSchema() and writeComposedSchema().
 * composeSchema() calls listInstalledPlugins() which requires a real filesystem
 * layout, so we test it with a minimal temporary project.
 *
 * The internal helpers (buildIdPattern, deriveInitialStatus, splitFields) are
 * not exported. We verify their behaviour through the public composeSchema()
 * output.
 */
import { describe, it, expect, beforeEach, afterEach } from "vitest";
import * as fs from "node:fs";
import * as path from "node:path";
import * as os from "node:os";
import { composeSchema, writeComposedSchema } from "../src/lib/schema-composer.js";

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

function makeTmpDir(): string {
	return fs.mkdtempSync(path.join(os.tmpdir(), "orqa-schema-test-"));
}

function rmrf(dir: string): void {
	if (fs.existsSync(dir)) {
		fs.rmSync(dir, { recursive: true, force: true });
	}
}

/**
 * Write a minimal plugin manifest to a tmp project so composeSchema can read it.
 * Returns the project root path.
 *
 * listInstalledPlugins() scans plugins/<taxonomy>/<plugin>/orqa-plugin.json
 * (two levels deep). We place the plugin at plugins/core/<pluginName>/.
 */
function makeProjectWithPlugin(
	pluginName: string,
	schemas: unknown[],
	relationships: unknown[] = [],
): string {
	const root = makeTmpDir();
	// Two-level directory: plugins/<taxonomy>/<name>
	const pluginDir = path.join(root, "plugins", "core", pluginName);
	fs.mkdirSync(pluginDir, { recursive: true });

	const manifest = {
		name: pluginName,
		version: "1.0.0",
		provides: { schemas, relationships },
	};
	// listInstalledPlugins looks for orqa-plugin.json (not plugin.manifest.json)
	fs.writeFileSync(
		path.join(pluginDir, "orqa-plugin.json"),
		JSON.stringify(manifest, null, 2),
	);

	// Create .orqa/ directory.
	fs.mkdirSync(path.join(root, ".orqa"), { recursive: true });

	return root;
}

// ---------------------------------------------------------------------------
// composeSchema — empty project (no plugins installed)
// ---------------------------------------------------------------------------

describe("composeSchema — no plugins installed", () => {
	let root: string;

	beforeEach(() => {
		root = makeTmpDir();
		fs.mkdirSync(path.join(root, ".orqa"), { recursive: true });
		// No plugins.lock.json => no plugins installed.
	});

	afterEach(() => rmrf(root));

	it("returns a valid ComposedSchema structure with no artifact types", () => {
		const schema = composeSchema(root);
		expect(schema.$schema).toBe("https://json-schema.org/draft/2020-12/schema");
		expect(schema.version).toBe("1.0.0");
		expect(schema.generated).toBe(true);
		expect(typeof schema.generatedAt).toBe("string");
		expect(schema.artifactTypes).toEqual({});
		expect(schema.relationshipTypes).toEqual([]);
	});

	it("sets generatedAt to a valid ISO date string", () => {
		const schema = composeSchema(root);
		const parsed = Date.parse(schema.generatedAt);
		expect(isNaN(parsed)).toBe(false);
	});
});

// ---------------------------------------------------------------------------
// composeSchema — single plugin with one artifact type
// ---------------------------------------------------------------------------

describe("composeSchema — single plugin schema", () => {
	let root: string;

	const TASK_SCHEMA = {
		key: "task",
		idPrefix: "TASK",
		label: "Task",
		plural: "Tasks",
		defaultPath: ".orqa/delivery/tasks",
		icon: "check-square",
		statusTransitions: {
			captured: ["active"],
			active: ["completed", "blocked"],
			completed: [],
			blocked: ["active"],
		},
		frontmatter: {
			required: ["title"],
			properties: {
				title: { type: "string" },
				priority: { type: "string" },
				labels: { type: "array", items: { type: "string" } },
			},
			additionalProperties: false,
		},
	};

	beforeEach(() => {
		root = makeProjectWithPlugin("software", [TASK_SCHEMA]);
	});

	afterEach(() => rmrf(root));

	it("includes the artifact type from the plugin", () => {
		const schema = composeSchema(root);
		expect("task" in schema.artifactTypes).toBe(true);
	});

	it("sets id_prefix correctly", () => {
		const schema = composeSchema(root);
		expect(schema.artifactTypes["task"]!.id_prefix).toBe("TASK");
	});

	it("sets label and plural correctly", () => {
		const schema = composeSchema(root);
		const type = schema.artifactTypes["task"]!;
		expect(type.label).toBe("Task");
		expect(type.plural).toBe("Tasks");
	});

	it("builds a correct id_pattern regex", () => {
		const schema = composeSchema(root);
		const pattern = schema.artifactTypes["task"]!.id_pattern;
		// Pattern should match TASK-abcdef12
		expect(pattern).toMatch(/TASK/);
		expect(new RegExp(pattern).test("TASK-abcdef12")).toBe(true);
		expect(new RegExp(pattern).test("EPIC-abcdef12")).toBe(false);
	});

	it("appends trailing slash to default_path", () => {
		const schema = composeSchema(root);
		const defaultPath = schema.artifactTypes["task"]!.default_path;
		expect(defaultPath.endsWith("/")).toBe(true);
	});

	it("records the source plugin name", () => {
		const schema = composeSchema(root);
		expect(schema.artifactTypes["task"]!.source).toBe("software");
	});

	it("sets additionalProperties from schema", () => {
		const schema = composeSchema(root);
		expect(schema.artifactTypes["task"]!.additionalProperties).toBe(false);
	});

	it("extracts statuses from statusTransitions keys", () => {
		const schema = composeSchema(root);
		const statuses = schema.artifactTypes["task"]!.statuses;
		expect(statuses).toContain("captured");
		expect(statuses).toContain("active");
		expect(statuses).toContain("completed");
		expect(statuses).toContain("blocked");
	});

	it("derives initialStatus as first key of statusTransitions", () => {
		const schema = composeSchema(root);
		// First key of TASK_SCHEMA.statusTransitions is "captured"
		expect(schema.artifactTypes["task"]!.initialStatus).toBe("captured");
	});

	it("splits required vs optional frontmatter fields", () => {
		const schema = composeSchema(root);
		const fields = schema.artifactTypes["task"]!.fields;
		expect("title" in fields.required).toBe(true);
		expect("priority" in fields.optional).toBe(true);
		expect("labels" in fields.optional).toBe(true);
		expect("priority" in fields.required).toBe(false);
	});
});

// ---------------------------------------------------------------------------
// composeSchema — relationship types
// ---------------------------------------------------------------------------

describe("composeSchema — relationship types", () => {
	let root: string;

	const DELIVERS_REL = {
		key: "delivers",
		inverse: "delivered-by",
		label: "Delivers",
		inverseLabel: "Delivered by",
		from: ["task"],
		to: ["epic"],
		description: "A task that delivers towards an epic.",
		semantic: "forward",
	};

	beforeEach(() => {
		root = makeProjectWithPlugin("software", [], [DELIVERS_REL]);
	});

	afterEach(() => rmrf(root));

	it("includes the relationship type from the plugin", () => {
		const schema = composeSchema(root);
		expect(schema.relationshipTypes).toHaveLength(1);
		expect(schema.relationshipTypes[0]!.key).toBe("delivers");
	});

	it("sets inverse, label, and inverseLabel", () => {
		const schema = composeSchema(root);
		const rel = schema.relationshipTypes[0]!;
		expect(rel.inverse).toBe("delivered-by");
		expect(rel.label).toBe("Delivers");
		expect(rel.inverseLabel).toBe("Delivered by");
	});

	it("sets from and to arrays", () => {
		const schema = composeSchema(root);
		const rel = schema.relationshipTypes[0]!;
		expect(rel.from).toEqual(["task"]);
		expect(rel.to).toEqual(["epic"]);
	});

	it("includes semantic field when present", () => {
		const schema = composeSchema(root);
		expect(schema.relationshipTypes[0]!.semantic).toBe("forward");
	});

	it("deduplicates relationship types across plugins", () => {
		// If the same key is provided twice, only one should appear.
		const root2 = makeProjectWithPlugin("software2", [], [DELIVERS_REL, DELIVERS_REL]);
		try {
			const schema = composeSchema(root2);
			const deliverRels = schema.relationshipTypes.filter((r) => r.key === "delivers");
			expect(deliverRels).toHaveLength(1);
		} finally {
			rmrf(root2);
		}
	});
});

// ---------------------------------------------------------------------------
// writeComposedSchema
// ---------------------------------------------------------------------------

describe("writeComposedSchema", () => {
	let root: string;

	beforeEach(() => {
		root = makeTmpDir();
		fs.mkdirSync(path.join(root, ".orqa"), { recursive: true });
	});

	afterEach(() => rmrf(root));

	it("writes schema.composed.json to .orqa/", () => {
		const outputPath = writeComposedSchema(root);
		expect(fs.existsSync(outputPath)).toBe(true);
		expect(outputPath).toContain("schema.composed.json");
	});

	it("writes valid JSON", () => {
		const outputPath = writeComposedSchema(root);
		const content = fs.readFileSync(outputPath, "utf-8");
		expect(() => JSON.parse(content)).not.toThrow();
	});

	it("returns the absolute path to the written file", () => {
		const outputPath = writeComposedSchema(root);
		expect(path.isAbsolute(outputPath)).toBe(true);
	});

	it("creates .orqa/ if it does not exist", () => {
		const root2 = makeTmpDir();
		// Do not create .orqa/ — writeComposedSchema should create it.
		try {
			const outputPath = writeComposedSchema(root2);
			expect(fs.existsSync(outputPath)).toBe(true);
		} finally {
			rmrf(root2);
		}
	});
});
