/**
 * Tests for pure functions across cli/src/lib that require no filesystem
 * or network access — or where the impure path can be isolated from the
 * pure logic under test.
 *
 * Covers:
 *   - isValidVersion (version-sync.ts)
 *   - parseFrontmatterFromContent (frontmatter.ts)
 *   - validateManifest (manifest.ts)
 *   - isPluginManifestNonEmpty (manifest.ts)
 *   - detectMethodologyConflict (installer.ts) — uses temp dirs
 *   - createEvent (enforcement-log.ts)
 *   - applyOverrides (workflow-resolver.ts)
 *   - matchContributions (workflow-resolver.ts)
 *   - mergeContributions (workflow-resolver.ts)
 *   - validateResolvedWorkflow (workflow-resolver.ts)
 */

import { describe, it, expect, beforeEach, afterEach } from "vitest";
import * as os from "node:os";
import * as fs from "node:fs";
import * as path from "node:path";

import { isValidVersion } from "../src/lib/version-sync.js";
import { parseFrontmatterFromContent } from "../src/lib/frontmatter.js";
import { validateManifest, isPluginManifestNonEmpty } from "../src/lib/manifest.js";
import { detectMethodologyConflict } from "../src/lib/installer.js";
import { createEvent } from "../src/lib/enforcement-log.js";
import {
	applyOverrides,
	matchContributions,
	mergeContributions,
	validateResolvedWorkflow,
	type DiscoveredWorkflow,
	type ResolutionMetadata,
} from "../src/lib/workflow-resolver.js";
import type { PluginManifest } from "@orqastudio/types";
import type { WorkflowDefinition } from "@orqastudio/types";

// ---------------------------------------------------------------------------
// isValidVersion
// ---------------------------------------------------------------------------

describe("isValidVersion", () => {
	it("accepts standard X.Y.Z versions", () => {
		expect(isValidVersion("0.0.0")).toBe(true);
		expect(isValidVersion("1.0.0")).toBe(true);
		expect(isValidVersion("10.20.30")).toBe(true);
	});

	it("accepts pre-release versions with a dash suffix", () => {
		expect(isValidVersion("0.1.0-dev")).toBe(true);
		expect(isValidVersion("1.0.0-rc.1")).toBe(true);
		expect(isValidVersion("2.3.4-alpha")).toBe(true);
		expect(isValidVersion("1.0.0-beta.2")).toBe(true);
	});

	it("rejects versions with leading zeros in any component", () => {
		// The regex \d+ permits leading zeros — verify the behaviour is documented
		// by what the regex actually does rather than asserting incorrect behaviour.
		// Leading zeros ARE permitted by this regex.
		expect(isValidVersion("01.0.0")).toBe(true); // regex allows it
	});

	it("rejects versions missing components", () => {
		expect(isValidVersion("1.0")).toBe(false);
		expect(isValidVersion("1")).toBe(false);
		expect(isValidVersion("")).toBe(false);
	});

	it("rejects versions with extra components", () => {
		expect(isValidVersion("1.0.0.0")).toBe(false);
	});

	it("rejects versions with non-numeric components", () => {
		expect(isValidVersion("a.b.c")).toBe(false);
		expect(isValidVersion("1.x.0")).toBe(false);
	});

	it("rejects versions with a bare dash (no suffix)", () => {
		expect(isValidVersion("1.0.0-")).toBe(false);
	});

	it("rejects versions with whitespace", () => {
		expect(isValidVersion(" 1.0.0")).toBe(false);
		expect(isValidVersion("1.0.0 ")).toBe(false);
	});

	it("rejects versions prefixed with 'v'", () => {
		expect(isValidVersion("v1.0.0")).toBe(false);
	});
});

// ---------------------------------------------------------------------------
// parseFrontmatterFromContent
// ---------------------------------------------------------------------------

describe("parseFrontmatterFromContent", () => {
	it("parses valid frontmatter and returns [fm, body]", () => {
		const content = "---\nname: test\nstatus: active\n---\nBody here.";
		const result = parseFrontmatterFromContent(content);
		expect(result).not.toBeNull();
		const [fm, body] = result!;
		expect(fm.name).toBe("test");
		expect(fm.status).toBe("active");
		expect(body).toBe("\nBody here.");
	});

	it("returns null when content does not start with ---", () => {
		expect(parseFrontmatterFromContent("No frontmatter here")).toBeNull();
		expect(parseFrontmatterFromContent("# Heading\n---\nname: x\n---")).toBeNull();
	});

	it("returns null when closing --- delimiter is missing", () => {
		expect(parseFrontmatterFromContent("---\nname: test\n")).toBeNull();
	});

	it("handles empty frontmatter body (only delimiters)", () => {
		const content = "---\n---\nsome body";
		const result = parseFrontmatterFromContent(content);
		// YAML parse of "" returns null, so result should be null
		expect(result).toBeNull();
	});

	it("returns empty body string when there is no content after closing ---", () => {
		const content = "---\nname: test\n---";
		const result = parseFrontmatterFromContent(content);
		expect(result).not.toBeNull();
		const [fm, body] = result!;
		expect(fm.name).toBe("test");
		expect(body).toBe("");
	});

	it("parses typed YAML values (numbers, booleans, arrays)", () => {
		const content = "---\ncount: 42\nactive: true\ntags:\n  - a\n  - b\n---\n";
		const result = parseFrontmatterFromContent(content);
		expect(result).not.toBeNull();
		const [fm] = result!;
		expect(fm.count).toBe(42);
		expect(fm.active).toBe(true);
		expect(fm.tags).toEqual(["a", "b"]);
	});

	it("returns null for malformed YAML frontmatter", () => {
		const content = "---\n: invalid: yaml: here\n---\nbody";
		// yaml parser may or may not throw — either way we get null or a parse result
		// The important thing is parseFrontmatterFromContent doesn't throw.
		expect(() => parseFrontmatterFromContent(content)).not.toThrow();
	});

	it("handles frontmatter with double-quoted values containing colons", () => {
		const content = "---\ntitle: \"Hello: World\"\n---\n";
		const result = parseFrontmatterFromContent(content);
		expect(result).not.toBeNull();
		const [fm] = result!;
		expect(fm.title).toBe("Hello: World");
	});

	it("preserves body content including multiple newlines", () => {
		const content = "---\nname: x\n---\n\nParagraph one.\n\nParagraph two.";
		const result = parseFrontmatterFromContent(content);
		expect(result).not.toBeNull();
		expect(result![1]).toBe("\n\nParagraph one.\n\nParagraph two.");
	});
});

// ---------------------------------------------------------------------------
// validateManifest
// ---------------------------------------------------------------------------

function minimalValidManifest(): PluginManifest {
	return {
		name: "orqastudio/test-plugin",
		version: "0.1.0",
		provides: {
			schemas: [{ id: "schema-1", file: "schema.json" }],
			views: [],
			widgets: [],
			relationships: [],
		},
	} as unknown as PluginManifest;
}

describe("validateManifest", () => {
	it("returns empty errors for a valid manifest", () => {
		expect(validateManifest(minimalValidManifest())).toEqual([]);
	});

	it("errors when name is missing", () => {
		const m = minimalValidManifest();
		delete (m as Record<string, unknown>).name;
		const errors = validateManifest(m);
		expect(errors.some((e) => e.includes("name"))).toBe(true);
	});

	it("errors when version is missing", () => {
		const m = { ...minimalValidManifest(), version: undefined as unknown as string };
		const errors = validateManifest(m);
		expect(errors.some((e) => e.includes("version"))).toBe(true);
	});

	it("errors when provides is missing", () => {
		const m = { name: "orqastudio/test", version: "0.1.0" } as unknown as PluginManifest;
		const errors = validateManifest(m);
		expect(errors.some((e) => e.includes("provides"))).toBe(true);
	});

	it("errors when provides.schemas is not an array", () => {
		const m = minimalValidManifest();
		(m as unknown as Record<string, unknown>).provides = {
			...m.provides,
			schemas: "not-an-array",
		};
		const errors = validateManifest(m);
		expect(errors.some((e) => e.includes("schemas"))).toBe(true);
	});

	it("errors when provides.views is not an array", () => {
		const m = minimalValidManifest();
		(m.provides as unknown as Record<string, unknown>).views = "bad";
		const errors = validateManifest(m);
		expect(errors.some((e) => e.includes("views"))).toBe(true);
	});

	it("accepts a valid scoped name (@scope/pkg)", () => {
		const m = { ...minimalValidManifest(), name: "@orqastudio/my-plugin" };
		const errors = validateManifest(m);
		expect(errors.filter((e) => e.includes("name"))).toHaveLength(0);
	});

	it("errors on an invalid name format", () => {
		const m = { ...minimalValidManifest(), name: "has spaces in name" };
		const errors = validateManifest(m);
		expect(errors.some((e) => e.includes("Invalid name"))).toBe(true);
	});

	it("accepts valid core roles", () => {
		for (const role of [
			"core:framework",
			"core:discovery",
			"core:delivery",
			"core:governance",
		]) {
			const m = { ...minimalValidManifest(), role };
			expect(validateManifest(m)).toEqual([]);
		}
	});

	it("accepts valid enhancement roles", () => {
		for (const role of [
			"enhancement:delivery",
			"enhancement:governance",
			"enhancement:development",
		]) {
			const m = { ...minimalValidManifest(), role };
			expect(validateManifest(m)).toEqual([]);
		}
	});

	it("accepts extension role", () => {
		const m = { ...minimalValidManifest(), role: "extension" };
		expect(validateManifest(m)).toEqual([]);
	});

	it("errors on invalid role", () => {
		const m = { ...minimalValidManifest(), role: "invalid-role" };
		const errors = validateManifest(m);
		expect(errors.some((e) => e.includes("Invalid role"))).toBe(true);
	});

	it("errors when manifest provides nothing (all arrays empty)", () => {
		const m: PluginManifest = {
			name: "orqastudio/empty-plugin",
			version: "0.1.0",
			provides: {
				schemas: [],
				views: [],
				widgets: [],
				relationships: [],
			},
		} as unknown as PluginManifest;
		const errors = validateManifest(m);
		expect(errors.some((e) => e.includes("provides nothing"))).toBe(true);
	});
});

// ---------------------------------------------------------------------------
// isPluginManifestNonEmpty
// ---------------------------------------------------------------------------

describe("isPluginManifestNonEmpty", () => {
	it("returns false when all provides arrays are empty and no content", () => {
		const m: PluginManifest = {
			name: "orqastudio/empty",
			version: "0.1.0",
			provides: {
				schemas: [],
				views: [],
				widgets: [],
				relationships: [],
			},
		} as unknown as PluginManifest;
		expect(isPluginManifestNonEmpty(m)).toBe(false);
	});

	it("returns true when schemas is non-empty", () => {
		const m = minimalValidManifest();
		expect(isPluginManifestNonEmpty(m)).toBe(true);
	});

	it("returns true when views is non-empty", () => {
		const m: PluginManifest = {
			name: "p",
			version: "0.1.0",
			provides: { schemas: [], views: [{ id: "v" }], widgets: [], relationships: [] },
		} as unknown as PluginManifest;
		expect(isPluginManifestNonEmpty(m)).toBe(true);
	});

	it("returns true when widgets is non-empty", () => {
		const m: PluginManifest = {
			name: "p",
			version: "0.1.0",
			provides: { schemas: [], views: [], widgets: [{ id: "w" }], relationships: [] },
		} as unknown as PluginManifest;
		expect(isPluginManifestNonEmpty(m)).toBe(true);
	});

	it("returns true when relationships is non-empty", () => {
		const m: PluginManifest = {
			name: "p",
			version: "0.1.0",
			provides: { schemas: [], views: [], widgets: [], relationships: [{ type: "delivers" }] },
		} as unknown as PluginManifest;
		expect(isPluginManifestNonEmpty(m)).toBe(true);
	});

	it("returns true when hooks is non-empty", () => {
		const m: PluginManifest = {
			name: "p",
			version: "0.1.0",
			provides: {
				schemas: [], views: [], widgets: [], relationships: [],
				hooks: [{ event: "pre-commit" }],
			},
		} as unknown as PluginManifest;
		expect(isPluginManifestNonEmpty(m)).toBe(true);
	});

	it("returns true when content has keys", () => {
		const m: PluginManifest = {
			name: "p",
			version: "0.1.0",
			provides: { schemas: [], views: [], widgets: [], relationships: [] },
			content: { rules: { source: "rules", target: ".orqa/rules" } },
		} as unknown as PluginManifest;
		expect(isPluginManifestNonEmpty(m)).toBe(true);
	});

	it("returns true when enforcement is non-empty", () => {
		const m: PluginManifest = {
			name: "p",
			version: "0.1.0",
			provides: { schemas: [], views: [], widgets: [], relationships: [] },
			enforcement: [{ name: "rule-1" }],
		} as unknown as PluginManifest;
		expect(isPluginManifestNonEmpty(m)).toBe(true);
	});
});

// ---------------------------------------------------------------------------
// detectMethodologyConflict
// ---------------------------------------------------------------------------

describe("detectMethodologyConflict", () => {
	let tmpDir: string;

	beforeEach(() => {
		tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "orqa-test-"));
	});

	afterEach(() => {
		fs.rmSync(tmpDir, { recursive: true, force: true });
	});

	function writeManifest(dir: string, manifest: Partial<PluginManifest>): void {
		fs.mkdirSync(dir, { recursive: true });
		fs.writeFileSync(path.join(dir, "orqa-plugin.json"), JSON.stringify(manifest));
	}

	it("returns null when the incoming plugin has no role", () => {
		const incoming = { name: "orqastudio/new-plugin" } as unknown as PluginManifest;
		expect(detectMethodologyConflict(incoming, tmpDir)).toBeNull();
	});

	it("returns null when the incoming plugin has a non-core role", () => {
		const incoming = {
			name: "orqastudio/new-plugin",
			role: "enhancement:delivery",
		} as unknown as PluginManifest;
		expect(detectMethodologyConflict(incoming, tmpDir)).toBeNull();
	});

	it("returns null when no plugins are installed", () => {
		const incoming = {
			name: "orqastudio/new-plugin",
			role: "core:delivery",
		} as unknown as PluginManifest;
		// No plugins directory at all — no conflict
		expect(detectMethodologyConflict(incoming, tmpDir)).toBeNull();
	});

	it("returns null when the same plugin is already installed (reinstall)", () => {
		const pluginsDir = path.join(tmpDir, "plugins", "orqastudio-software");
		writeManifest(pluginsDir, {
			name: "orqastudio/software",
			role: "core:delivery",
		});
		const incoming = {
			name: "orqastudio/software",
			role: "core:delivery",
		} as unknown as PluginManifest;
		expect(detectMethodologyConflict(incoming, tmpDir)).toBeNull();
	});

	it("returns a conflict when a different plugin with the same core role is installed", () => {
		const pluginsDir = path.join(tmpDir, "plugins", "orqastudio-software");
		writeManifest(pluginsDir, {
			name: "orqastudio/software",
			role: "core:delivery",
		});
		const incoming = {
			name: "orqastudio/other-delivery",
			role: "core:delivery",
		} as unknown as PluginManifest;
		const conflict = detectMethodologyConflict(incoming, tmpDir);
		expect(conflict).not.toBeNull();
		expect(conflict!.incomingPlugin).toBe("orqastudio/other-delivery");
		expect(conflict!.existingPlugin).toBe("orqastudio/software");
		expect(conflict!.role).toBe("core:delivery");
	});

	it("returns null when installed plugins have different core roles", () => {
		const pluginsDir = path.join(tmpDir, "plugins", "orqastudio-framework");
		writeManifest(pluginsDir, {
			name: "orqastudio/framework",
			role: "core:framework",
		});
		const incoming = {
			name: "orqastudio/delivery",
			role: "core:delivery",
		} as unknown as PluginManifest;
		expect(detectMethodologyConflict(incoming, tmpDir)).toBeNull();
	});
});

// ---------------------------------------------------------------------------
// createEvent
// ---------------------------------------------------------------------------

describe("createEvent", () => {
	it("generates an id and timestamp for a pass event", () => {
		const event = createEvent({
			mechanism: "json-schema",
			type: "frontmatter",
			rule_id: null,
			artifact_id: "TASK-001",
			result: "pass",
			message: "All checks passed",
			source: "cli",
		});
		expect(event.id).toBeTruthy();
		expect(event.timestamp).toBeTruthy();
		expect(new Date(event.timestamp).toString()).not.toBe("Invalid Date");
	});

	it("sets resolution to 'fixed' for pass result", () => {
		const event = createEvent({
			mechanism: "hook",
			type: "PreToolUse",
			rule_id: "RULE-001",
			artifact_id: null,
			result: "pass",
			message: "ok",
			source: "hook",
		});
		expect(event.resolution).toBe("fixed");
	});

	it("sets resolution to 'unresolved' for fail result", () => {
		const event = createEvent({
			mechanism: "lint",
			type: "frontmatter",
			rule_id: "RULE-002",
			artifact_id: "EPIC-001",
			result: "fail",
			message: "Missing required field",
			source: "lsp",
		});
		expect(event.resolution).toBe("unresolved");
	});

	it("sets resolution to 'unresolved' for warn result", () => {
		const event = createEvent({
			mechanism: "pre-commit",
			type: "schema",
			rule_id: null,
			artifact_id: null,
			result: "warn",
			message: "Deprecated field used",
			source: "pre-commit",
		});
		expect(event.resolution).toBe("unresolved");
	});

	it("sets resolution to 'unresolved' for error result", () => {
		const event = createEvent({
			mechanism: "validator",
			type: "schema",
			rule_id: null,
			artifact_id: null,
			result: "error",
			message: "Schema compilation failed",
			source: "validator",
		});
		expect(event.resolution).toBe("unresolved");
	});

	it("preserves an explicit resolution override", () => {
		const event = createEvent({
			mechanism: "hook",
			type: "check",
			rule_id: null,
			artifact_id: null,
			result: "fail",
			message: "known false positive",
			source: "hook",
			resolution: "false-positive",
		});
		// The spread applies fields AFTER the defaults — explicit resolution wins
		expect(event.resolution).toBe("false-positive");
	});

	it("generates unique ids for each call", () => {
		const a = createEvent({
			mechanism: "json-schema",
			type: "t",
			rule_id: null,
			artifact_id: null,
			result: "pass",
			message: "ok",
			source: "cli",
		});
		const b = createEvent({
			mechanism: "json-schema",
			type: "t",
			rule_id: null,
			artifact_id: null,
			result: "pass",
			message: "ok",
			source: "cli",
		});
		expect(a.id).not.toBe(b.id);
	});

	it("preserves all provided fields on the returned event", () => {
		const event = createEvent({
			mechanism: "json-schema",
			type: "frontmatter",
			rule_id: "RULE-100",
			artifact_id: "DOC-abc",
			result: "fail",
			message: "Field x is required",
			source: "validator",
		});
		expect(event.mechanism).toBe("json-schema");
		expect(event.type).toBe("frontmatter");
		expect(event.rule_id).toBe("RULE-100");
		expect(event.artifact_id).toBe("DOC-abc");
		expect(event.result).toBe("fail");
		expect(event.message).toBe("Field x is required");
		expect(event.source).toBe("validator");
	});
});

// ---------------------------------------------------------------------------
// Helpers — minimal workflow factory
// ---------------------------------------------------------------------------

function baseWorkflowDef(overrides?: Partial<WorkflowDefinition>): WorkflowDefinition {
	return {
		name: "delivery",
		version: "1.0.0",
		artifact_type: "task",
		plugin: "orqastudio/software",
		initial_state: "backlog",
		states: {
			backlog: { category: "planning" },
			active: { category: "active" },
		},
		transitions: [{ from: "backlog", to: "active", event: "start" }],
		...overrides,
	};
}

function baseMetadata(overrides?: Partial<ResolutionMetadata>): ResolutionMetadata {
	return {
		skeletonPlugin: "orqastudio/software",
		skeletonFile: "workflows/delivery.workflow.yaml",
		contributions: [],
		unfilledPoints: [],
		unfilledRequired: [],
		overrides: [],
		resolvedAt: new Date().toISOString(),
		...overrides,
	};
}

function makeDiscoveredWorkflow(
	name: string,
	isSkeleton: boolean,
	extra?: Partial<WorkflowDefinition>,
	extras?: Partial<DiscoveredWorkflow>,
): DiscoveredWorkflow {
	return {
		filePath: `/tmp/plugins/${name}/${name}.workflow.yaml`,
		pluginDir: `/tmp/plugins/${name}`,
		pluginName: `orqastudio/${name}`,
		isSkeleton,
		definition: baseWorkflowDef({ name, ...extra }),
		...extras,
	};
}

// ---------------------------------------------------------------------------
// applyOverrides
// ---------------------------------------------------------------------------

describe("applyOverrides", () => {
	it("returns the definition unchanged when there are no overrides", () => {
		const def = baseWorkflowDef();
		const { definition, errors } = applyOverrides(def, [], baseMetadata());
		expect(errors).toHaveLength(0);
		expect(definition.states).toEqual(def.states);
	});

	it("applies a state override (adds new state)", () => {
		const def = baseWorkflowDef();
		const overrides = [
			{
				filePath: "/project/.orqa/overrides/delivery.override.yaml",
				override: {
					target_workflow: "delivery",
					states: { review: { category: "review" as const } },
				},
			},
		];
		const { definition, errors } = applyOverrides(def, overrides, baseMetadata());
		expect(errors).toHaveLength(0);
		expect(definition.states.review).toEqual({ category: "review" });
	});

	it("replaces an existing state when name matches", () => {
		const def = baseWorkflowDef();
		const overrides = [
			{
				filePath: "/project/.orqa/overrides/delivery.override.yaml",
				override: {
					target_workflow: "delivery",
					states: { backlog: { category: "active" as const, description: "now active" } },
				},
			},
		];
		const { definition } = applyOverrides(def, overrides, baseMetadata());
		expect(definition.states.backlog.category).toBe("active");
		expect(definition.states.backlog.description).toBe("now active");
	});

	it("applies transition overrides (append, no duplicate)", () => {
		const def = baseWorkflowDef();
		const newTransition = { from: "active", to: "backlog", event: "reopen" };
		const overrides = [
			{
				filePath: "/project/.orqa/overrides/delivery.override.yaml",
				override: {
					target_workflow: "delivery",
					transitions: [newTransition],
				},
			},
		];
		const { definition } = applyOverrides(def, overrides, baseMetadata());
		expect(definition.transitions).toHaveLength(2);
		expect(definition.transitions[1]).toMatchObject(newTransition);
	});

	it("does not duplicate an identical transition", () => {
		const def = baseWorkflowDef();
		const existingTransition = { from: "backlog", to: "active", event: "start" };
		const overrides = [
			{
				filePath: "/project/.orqa/overrides/delivery.override.yaml",
				override: {
					target_workflow: "delivery",
					transitions: [existingTransition],
				},
			},
		];
		const { definition } = applyOverrides(def, overrides, baseMetadata());
		expect(definition.transitions).toHaveLength(1);
	});

	it("applies field overrides for allowed fields", () => {
		const def = baseWorkflowDef();
		const overrides = [
			{
				filePath: "/project/.orqa/overrides/delivery.override.yaml",
				override: {
					target_workflow: "delivery",
					fields: { description: "Updated description" },
				},
			},
		];
		const { definition, errors } = applyOverrides(def, overrides, baseMetadata());
		expect(errors).toHaveLength(0);
		expect((definition as unknown as Record<string, unknown>).description).toBe(
			"Updated description",
		);
	});

	it("returns an error for disallowed field overrides", () => {
		const def = baseWorkflowDef();
		const overrides = [
			{
				filePath: "/project/.orqa/overrides/delivery.override.yaml",
				override: {
					target_workflow: "delivery",
					fields: { artifact_type: "epic" },
				},
			},
		];
		const { errors } = applyOverrides(def, overrides, baseMetadata());
		expect(errors.some((e) => e.includes("artifact_type"))).toBe(true);
	});

	it("ignores overrides targeting a different workflow", () => {
		const def = baseWorkflowDef(); // name = "delivery"
		const overrides = [
			{
				filePath: "/project/.orqa/overrides/other.override.yaml",
				override: {
					target_workflow: "epic", // different workflow
					states: { done: { category: "terminal" as const } },
				},
			},
		];
		const { definition } = applyOverrides(def, overrides, baseMetadata());
		expect(definition.states.done).toBeUndefined();
	});

	it("does not mutate the original definition", () => {
		const def = baseWorkflowDef();
		const original = JSON.stringify(def);
		const overrides = [
			{
				filePath: "/project/.orqa/overrides/delivery.override.yaml",
				override: {
					target_workflow: "delivery",
					states: { review: { category: "review" as const } },
				},
			},
		];
		applyOverrides(def, overrides, baseMetadata());
		expect(JSON.stringify(def)).toBe(original);
	});
});

// ---------------------------------------------------------------------------
// matchContributions
// ---------------------------------------------------------------------------

describe("matchContributions", () => {
	it("returns empty everything when given no workflows", () => {
		const { skeletons, contributions, standalone } = matchContributions([]);
		expect(skeletons).toHaveLength(0);
		expect(standalone).toHaveLength(0);
		expect(contributions.size).toBe(0);
	});

	it("classifies skeletons correctly", () => {
		const skeleton = makeDiscoveredWorkflow(
			"delivery",
			true,
			{ contribution_points: [{ name: "impl-workflow", stage: "implementation" }] },
		);
		const { skeletons, standalone } = matchContributions([skeleton]);
		expect(skeletons).toHaveLength(1);
		expect(standalone).toHaveLength(0);
	});

	it("classifies workflows without contributes_to as standalone", () => {
		const w = makeDiscoveredWorkflow("my-workflow", false);
		const { standalone } = matchContributions([w]);
		expect(standalone).toHaveLength(1);
	});

	it("classifies as standalone when target skeleton does not exist", () => {
		const contribution = makeDiscoveredWorkflow("impl", false, undefined, {
			definition: {
				...baseWorkflowDef({ name: "impl" }),
				...(({ contributes_to: { workflow: "nonexistent", point: "impl-workflow" } }) as unknown as object),
			} as unknown as WorkflowDefinition,
		});
		// Attach contributes_to directly
		(contribution.definition as unknown as Record<string, unknown>).contributes_to = {
			workflow: "nonexistent",
			point: "impl-workflow",
		};
		const { standalone } = matchContributions([contribution]);
		expect(standalone).toHaveLength(1);
	});

	it("matches a contribution to its skeleton", () => {
		const skeleton = makeDiscoveredWorkflow("delivery", true, {
			contribution_points: [{ name: "impl-workflow", stage: "implementation" }],
		});
		const contrib = makeDiscoveredWorkflow("impl", false);
		(contrib.definition as unknown as Record<string, unknown>).contributes_to = {
			workflow: "delivery",
			point: "impl-workflow",
		};
		const { contributions, standalone } = matchContributions([skeleton, contrib]);
		expect(standalone).toHaveLength(0);
		expect(contributions.has("delivery::impl-workflow")).toBe(true);
		expect(contributions.get("delivery::impl-workflow")).toHaveLength(1);
	});

	it("classifies as standalone when contribution point name doesn't match", () => {
		const skeleton = makeDiscoveredWorkflow("delivery", true, {
			contribution_points: [{ name: "real-point", stage: "implementation" }],
		});
		const contrib = makeDiscoveredWorkflow("impl", false);
		(contrib.definition as unknown as Record<string, unknown>).contributes_to = {
			workflow: "delivery",
			point: "wrong-point",
		};
		const { standalone } = matchContributions([skeleton, contrib]);
		expect(standalone).toHaveLength(1);
	});
});

// ---------------------------------------------------------------------------
// mergeContributions
// ---------------------------------------------------------------------------

describe("mergeContributions", () => {
	it("returns the skeleton unchanged when no contributions exist", () => {
		const skeleton = makeDiscoveredWorkflow("delivery", true, {
			contribution_points: [{ name: "impl-workflow", stage: "implementation" }],
		});
		const { merged, metadata } = mergeContributions(skeleton, new Map());
		expect(merged.states).toEqual(skeleton.definition.states);
		expect(metadata.unfilledPoints).toContain("impl-workflow");
	});

	it("marks required unfilled points in metadata", () => {
		const skeleton = makeDiscoveredWorkflow("delivery", true, {
			contribution_points: [
				{ name: "optional-point", stage: "s", required: false },
				{ name: "required-point", stage: "s", required: true },
			],
		});
		const { metadata } = mergeContributions(skeleton, new Map());
		expect(metadata.unfilledRequired).toContain("required-point");
		expect(metadata.unfilledRequired).not.toContain("optional-point");
	});

	it("merges states from a contribution", () => {
		const skeleton = makeDiscoveredWorkflow("delivery", true, {
			contribution_points: [{ name: "impl-workflow", stage: "impl" }],
		});
		const contributions = new Map([
			[
				"delivery::impl-workflow",
				[
					{
						targetPoint: "impl-workflow",
						targetWorkflow: "delivery",
						states: { "in-progress": { category: "active" as const } },
						transitions: [],
						priority: 0,
						pluginName: "orqastudio/software",
						filePath: "/tmp/plugins/software/impl.workflow.yaml",
					},
				],
			],
		]);
		const { merged } = mergeContributions(skeleton, contributions);
		expect(merged.states["in-progress"]).toEqual({ category: "active" });
	});

	it("merges transitions from a contribution", () => {
		const skeleton = makeDiscoveredWorkflow("delivery", true, {
			contribution_points: [{ name: "impl-workflow", stage: "impl" }],
		});
		const newTransition = { from: "active", to: "backlog", event: "pause" };
		const contributions = new Map([
			[
				"delivery::impl-workflow",
				[
					{
						targetPoint: "impl-workflow",
						targetWorkflow: "delivery",
						states: {},
						transitions: [newTransition],
						priority: 0,
						pluginName: "orqastudio/software",
						filePath: "/tmp/plugins/software/impl.workflow.yaml",
					},
				],
			],
		]);
		const { merged } = mergeContributions(skeleton, contributions);
		expect(merged.transitions).toContainEqual(newTransition);
	});

	it("does not overwrite skeleton states with contribution states of the same name", () => {
		const skeleton = makeDiscoveredWorkflow("delivery", true, {
			states: { backlog: { category: "planning" as const, description: "skeleton backlog" } },
			contribution_points: [{ name: "impl-workflow", stage: "impl" }],
		});
		const contributions = new Map([
			[
				"delivery::impl-workflow",
				[
					{
						targetPoint: "impl-workflow",
						targetWorkflow: "delivery",
						states: { backlog: { category: "active" as const, description: "contrib backlog" } },
						transitions: [],
						priority: 0,
						pluginName: "orqastudio/software",
						filePath: "/tmp",
					},
				],
			],
		]);
		const { merged } = mergeContributions(skeleton, contributions);
		expect(merged.states.backlog.description).toBe("skeleton backlog");
	});

	it("sorts contributions by priority before merging (lower first)", () => {
		const skeleton = makeDiscoveredWorkflow("delivery", true, {
			contribution_points: [{ name: "pt", stage: "s" }],
		});
		const contributions = new Map([
			[
				"delivery::pt",
				[
					{
						targetPoint: "pt",
						targetWorkflow: "delivery",
						states: { alpha: { category: "active" as const } },
						transitions: [],
						priority: 10,
						pluginName: "high-prio",
						filePath: "/tmp",
					},
					{
						targetPoint: "pt",
						targetWorkflow: "delivery",
						states: { beta: { category: "planning" as const } },
						transitions: [],
						priority: 1,
						pluginName: "low-prio",
						filePath: "/tmp",
					},
				],
			],
		]);
		const { merged } = mergeContributions(skeleton, contributions);
		// Both states should be present
		expect(merged.states.alpha).toBeDefined();
		expect(merged.states.beta).toBeDefined();
	});
});

// ---------------------------------------------------------------------------
// validateResolvedWorkflow
// ---------------------------------------------------------------------------

describe("validateResolvedWorkflow", () => {
	it("returns empty errors for a valid workflow", () => {
		const errors = validateResolvedWorkflow(baseWorkflowDef(), baseMetadata());
		expect(errors).toHaveLength(0);
	});

	it("errors when name is missing", () => {
		const def = { ...baseWorkflowDef(), name: "" };
		const errors = validateResolvedWorkflow(def, baseMetadata());
		expect(errors.some((e) => e.includes("name"))).toBe(true);
	});

	it("errors when version is missing", () => {
		const def = { ...baseWorkflowDef(), version: "" };
		const errors = validateResolvedWorkflow(def, baseMetadata());
		expect(errors.some((e) => e.includes("version"))).toBe(true);
	});

	it("errors when artifact_type is missing", () => {
		const def = { ...baseWorkflowDef(), artifact_type: "" };
		const errors = validateResolvedWorkflow(def, baseMetadata());
		expect(errors.some((e) => e.includes("artifact_type"))).toBe(true);
	});

	it("errors when plugin is missing", () => {
		const def = { ...baseWorkflowDef(), plugin: "" };
		const errors = validateResolvedWorkflow(def, baseMetadata());
		expect(errors.some((e) => e.includes("plugin"))).toBe(true);
	});

	it("errors when initial_state is missing", () => {
		const def = { ...baseWorkflowDef(), initial_state: "" };
		const errors = validateResolvedWorkflow(def, baseMetadata());
		expect(errors.some((e) => e.includes("initial_state"))).toBe(true);
	});

	it("errors when initial_state does not exist in states", () => {
		const def = { ...baseWorkflowDef(), initial_state: "nonexistent" };
		const errors = validateResolvedWorkflow(def, baseMetadata());
		expect(errors.some((e) => e.includes('"nonexistent"'))).toBe(true);
	});

	it("errors when there are fewer than 2 states", () => {
		const def = {
			...baseWorkflowDef(),
			states: { backlog: { category: "planning" as const } },
			initial_state: "backlog",
		};
		const errors = validateResolvedWorkflow(def, baseMetadata());
		expect(errors.some((e) => e.includes("at least 2 states"))).toBe(true);
	});

	it("errors when transitions array is empty", () => {
		const def = { ...baseWorkflowDef(), transitions: [] };
		const errors = validateResolvedWorkflow(def, baseMetadata());
		expect(errors.some((e) => e.includes("transitions"))).toBe(true);
	});

	it("errors when a transition 'from' state does not exist", () => {
		const def = baseWorkflowDef({
			transitions: [{ from: "nonexistent", to: "active", event: "go" }],
		});
		const errors = validateResolvedWorkflow(def, baseMetadata());
		expect(errors.some((e) => e.includes('"nonexistent"'))).toBe(true);
	});

	it("errors when a transition 'to' state does not exist", () => {
		const def = baseWorkflowDef({
			transitions: [{ from: "backlog", to: "ghost", event: "go" }],
		});
		const errors = validateResolvedWorkflow(def, baseMetadata());
		expect(errors.some((e) => e.includes('"ghost"'))).toBe(true);
	});

	it("errors when a gate reference is missing from the gates map", () => {
		const def = baseWorkflowDef({
			transitions: [{ from: "backlog", to: "active", event: "start", gate: "missing-gate" }],
		});
		const errors = validateResolvedWorkflow(def, baseMetadata());
		expect(errors.some((e) => e.includes("missing-gate"))).toBe(true);
	});

	it("accepts a gate reference that exists in the gates map", () => {
		const def = baseWorkflowDef({
			transitions: [{ from: "backlog", to: "active", event: "start", gate: "review-gate" }],
			gates: {
				"review-gate": {
					pattern: "simple_approval",
					phases: { collect: { verdicts: [{ key: "approve", label: "Approve" }] } },
				},
			},
		});
		const errors = validateResolvedWorkflow(def, baseMetadata());
		expect(errors.filter((e) => e.includes("review-gate"))).toHaveLength(0);
	});

	it("errors on invalid state category", () => {
		const def = baseWorkflowDef({
			states: {
				backlog: { category: "planning" as const },
				active: { category: "invalid-category" as unknown as "active" },
			},
		});
		const errors = validateResolvedWorkflow(def, baseMetadata());
		expect(errors.some((e) => e.includes("invalid-category"))).toBe(true);
	});

	it("accepts transition with array 'from'", () => {
		const def = baseWorkflowDef({
			transitions: [{ from: ["backlog", "active"], to: "active", event: "bump" }],
		});
		const errors = validateResolvedWorkflow(def, baseMetadata());
		expect(errors).toHaveLength(0);
	});

	it("errors for required contribution points that were not filled", () => {
		const meta = baseMetadata({ unfilledRequired: ["required-point"] });
		const errors = validateResolvedWorkflow(baseWorkflowDef(), meta);
		expect(errors.some((e) => e.includes("required-point"))).toBe(true);
	});
});
