/**
 * Tests for conflict-resolver pure functions: buildConflictResolutionPrompt and
 * parseConflictResolutionResponse. Covers all conflict types, optional fields,
 * and all parsing branches (JSON block, bare array, malformed, empty).
 */
import { describe, it, expect } from "vitest";
import {
	buildConflictResolutionPrompt,
	parseConflictResolutionResponse,
} from "../src/plugins/conflict-resolver.js";
import type { PluginManifest } from "@orqastudio/types";
import type { RegistrationConflict } from "../src/plugins/plugin-registry.svelte.js";

// ---------------------------------------------------------------------------
// Fixtures
// ---------------------------------------------------------------------------

function makeManifest(overrides: Partial<PluginManifest> = {}): PluginManifest {
	return {
		name: "plugin-a",
		version: "0.1.0",
		categories: ["workflow"],
		provides: {
			schemas: [],
			relationships: [],
			views: [],
			widgets: [],
		},
		...overrides,
	} as PluginManifest;
}

function makeConflict(overrides: Partial<RegistrationConflict> = {}): RegistrationConflict {
	return {
		type: "schema",
		key: "task",
		existingPlugin: "plugin-a",
		newPlugin: "plugin-b",
		detail: 'schema key "task" already owned by "plugin-a"',
		...overrides,
	};
}

// ---------------------------------------------------------------------------
// buildConflictResolutionPrompt
// ---------------------------------------------------------------------------

describe("buildConflictResolutionPrompt", () => {
	it("includes both plugin names in the prompt", () => {
		const existing = makeManifest({ name: "plugin-a", displayName: "Plugin A" });
		const incoming = makeManifest({ name: "plugin-b", displayName: "Plugin B" });
		const prompt = buildConflictResolutionPrompt([makeConflict()], existing, incoming);
		expect(prompt).toContain("plugin-a");
		expect(prompt).toContain("plugin-b");
	});

	it("describes a schema conflict correctly", () => {
		const conflict = makeConflict({ type: "schema", key: "task" });
		const prompt = buildConflictResolutionPrompt(
			[conflict],
			makeManifest(),
			makeManifest({ name: "plugin-b" }),
		);
		expect(prompt).toContain('Schema conflict: both plugins register artifact type "task"');
	});

	it("describes a relationship-key conflict correctly", () => {
		const conflict = makeConflict({ type: "relationship-key", key: "delivers" });
		const prompt = buildConflictResolutionPrompt(
			[conflict],
			makeManifest(),
			makeManifest({ name: "plugin-b" }),
		);
		expect(prompt).toContain('Relationship conflict: both plugins register relationship "delivers"');
	});

	it("describes a relationship-constraint conflict correctly", () => {
		const conflict = makeConflict({ type: "relationship-constraint", key: "delivers" });
		const prompt = buildConflictResolutionPrompt(
			[conflict],
			makeManifest(),
			makeManifest({ name: "plugin-b" }),
		);
		expect(prompt).toContain('Relationship constraint conflict: "delivers" has different from/to types');
	});

	it("numbers multiple conflicts sequentially", () => {
		const conflicts = [
			makeConflict({ type: "schema", key: "task" }),
			makeConflict({ type: "relationship-key", key: "delivers" }),
		];
		const prompt = buildConflictResolutionPrompt(
			conflicts,
			makeManifest(),
			makeManifest({ name: "plugin-b" }),
		);
		expect(prompt).toContain("1.");
		expect(prompt).toContain("2.");
	});

	it("includes project context when provided", () => {
		const prompt = buildConflictResolutionPrompt(
			[makeConflict()],
			makeManifest(),
			makeManifest({ name: "plugin-b" }),
			{ vision: "Build great software", pillars: ["quality", "velocity"] },
		);
		expect(prompt).toContain("Build great software");
		expect(prompt).toContain("quality, velocity");
	});

	it("omits project context section when not provided", () => {
		const prompt = buildConflictResolutionPrompt(
			[makeConflict()],
			makeManifest(),
			makeManifest({ name: "plugin-b" }),
		);
		expect(prompt).not.toContain("Project context:");
	});

	it("shows 'not set' for missing vision and pillars", () => {
		const prompt = buildConflictResolutionPrompt(
			[makeConflict()],
			makeManifest(),
			makeManifest({ name: "plugin-b" }),
			{},
		);
		expect(prompt).toContain("Vision: not set");
		expect(prompt).toContain("Pillars: not set");
	});

	it("includes schema keys from each manifest's provides", () => {
		const existing = makeManifest({
			name: "plugin-a",
			provides: {
				schemas: [
					{
						key: "task",
						label: "Task",
						icon: "check",
						defaultPath: ".orqa/tasks",
						idPrefix: "TASK",
						frontmatter: { type: "object" },
						statusTransitions: {},
					},
				],
				relationships: [],
				views: [],
				widgets: [],
			},
		});
		const prompt = buildConflictResolutionPrompt([makeConflict()], existing, makeManifest({ name: "plugin-b" }));
		expect(prompt).toContain("task (Task)");
	});

	it("shows 'none' when schemas list is empty", () => {
		const prompt = buildConflictResolutionPrompt(
			[makeConflict()],
			makeManifest(),
			makeManifest({ name: "plugin-b" }),
		);
		// Both plugins have empty schemas — both sections say 'none'
		const noneMatches = prompt.match(/Schemas: none/g);
		expect(noneMatches).toHaveLength(2);
	});

	it("falls back to manifest name when displayName is absent", () => {
		const existing = makeManifest({ name: "plugin-no-display" });
		const prompt = buildConflictResolutionPrompt(
			[makeConflict()],
			existing,
			makeManifest({ name: "plugin-b" }),
		);
		expect(prompt).toContain("Display: plugin-no-display");
	});
});

// ---------------------------------------------------------------------------
// parseConflictResolutionResponse
// ---------------------------------------------------------------------------

describe("parseConflictResolutionResponse", () => {
	it("parses a valid JSON code block", () => {
		const response = `Here are my suggestions:
\`\`\`json
[
  {
    "key": "task",
    "strategy": "rename-new",
    "newAlias": "work-item",
    "rationale": "Preserves plugin-a semantics"
  }
]
\`\`\``;
		const result = parseConflictResolutionResponse(response);
		expect(result).toHaveLength(1);
		expect(result[0]).toMatchObject({
			key: "task",
			strategy: "rename-new",
			newAlias: "work-item",
			rationale: "Preserves plugin-a semantics",
		});
	});

	it("parses a bare JSON array without code block", () => {
		const response = `[{"key":"delivers","strategy":"rename-existing","existingAlias":"ships","rationale":"Clearer"}]`;
		const result = parseConflictResolutionResponse(response);
		expect(result).toHaveLength(1);
		expect(result[0]?.key).toBe("delivers");
	});

	it("returns empty array when no JSON found", () => {
		expect(parseConflictResolutionResponse("No JSON here at all.")).toEqual([]);
	});

	it("returns empty array for empty string", () => {
		expect(parseConflictResolutionResponse("")).toEqual([]);
	});

	it("returns empty array when JSON is not an array", () => {
		const response = '```json\n{"key": "task"}\n```';
		expect(parseConflictResolutionResponse(response)).toEqual([]);
	});

	it("returns empty array when JSON is malformed", () => {
		const response = "```json\n[{broken\n```";
		expect(parseConflictResolutionResponse(response)).toEqual([]);
	});

	it("filters out objects missing required fields", () => {
		const response = `\`\`\`json
[
  {"key": "task", "strategy": "rename-new", "rationale": "Good"},
  {"key": "task", "strategy": "rename-new"},
  {"key": "task", "rationale": "Missing strategy"},
  {"strategy": "rename-new", "rationale": "Missing key"}
]
\`\`\``;
		const result = parseConflictResolutionResponse(response);
		// Only first entry has all required fields
		expect(result).toHaveLength(1);
		expect(result[0]?.key).toBe("task");
	});

	it("handles multiple valid suggestions", () => {
		const response = `\`\`\`json
[
  {"key": "task", "strategy": "rename-new", "newAlias": "issue", "rationale": "Option A"},
  {"key": "task", "strategy": "rename-existing", "existingAlias": "work-item", "rationale": "Option B"},
  {"key": "task", "strategy": "rename-both", "existingAlias": "card", "newAlias": "ticket", "rationale": "Option C"}
]
\`\`\``;
		const result = parseConflictResolutionResponse(response);
		expect(result).toHaveLength(3);
		expect(result[0]?.strategy).toBe("rename-new");
		expect(result[1]?.strategy).toBe("rename-existing");
		expect(result[2]?.strategy).toBe("rename-both");
	});

	it("handles null entries in the parsed array", () => {
		const response = `\`\`\`json
[null, {"key": "task", "strategy": "rename-new", "rationale": "ok"}]
\`\`\``;
		const result = parseConflictResolutionResponse(response);
		expect(result).toHaveLength(1);
	});

	it("handles empty array in response", () => {
		const response = "```json\n[]\n```";
		expect(parseConflictResolutionResponse(response)).toEqual([]);
	});
});
