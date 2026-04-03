/**
 * Tests for project.ts exports.
 *
 * Covers the isArtifactGroup type guard
 * and PLATFORM_CONFIG structure loaded from core.json.
 */

import { describe, it, expect } from "vitest";
import {
	isArtifactGroup,
	PLATFORM_CONFIG,
	PLATFORM_ARTIFACT_TYPES,
	PLATFORM_RELATIONSHIPS,
	PLATFORM_SEMANTICS,
	PLATFORM_NAVIGATION,
} from "../src/project.js";
import type { ArtifactTypeConfig, ArtifactGroupConfig } from "../src/project.js";

// ---------------------------------------------------------------------------
// isArtifactGroup
// ---------------------------------------------------------------------------

describe("isArtifactGroup()", () => {
	it("returns true for entries that have a children array", () => {
		const group: ArtifactGroupConfig = {
			key: "governance",
			children: [
				{ key: "rule", path: ".orqa/rules" },
				{ key: "agent", path: ".orqa/agents" },
			],
		};
		expect(isArtifactGroup(group)).toBe(true);
	});

	it("returns false for flat artifact type entries (no children)", () => {
		const type: ArtifactTypeConfig = { key: "epic", path: ".orqa/epics" };
		expect(isArtifactGroup(type)).toBe(false);
	});

	it("correctly narrows: children array is accessible after the guard returns true", () => {
		const entry: ArtifactTypeConfig | ArtifactGroupConfig = {
			key: "delivery",
			children: [{ key: "task", path: ".orqa/tasks" }],
		};
		if (isArtifactGroup(entry)) {
			expect(entry.children).toHaveLength(1);
			expect(entry.children[0].key).toBe("task");
		} else {
			throw new Error("guard should have returned true");
		}
	});
});

// ---------------------------------------------------------------------------
// PLATFORM_CONFIG — loaded from core.json (currently empty — plugins provide content)
// ---------------------------------------------------------------------------

describe("PLATFORM_CONFIG", () => {
	it("has an artifactTypes array", () => {
		expect(Array.isArray(PLATFORM_CONFIG.artifactTypes)).toBe(true);
	});

	it("has a relationships array", () => {
		expect(Array.isArray(PLATFORM_CONFIG.relationships)).toBe(true);
	});

	it("has a semantics object", () => {
		expect(typeof PLATFORM_CONFIG.semantics).toBe("object");
		expect(PLATFORM_CONFIG.semantics).not.toBeNull();
	});
});

// ---------------------------------------------------------------------------
// Derived constants from PLATFORM_CONFIG
// ---------------------------------------------------------------------------

describe("PLATFORM_ARTIFACT_TYPES", () => {
	it("is a readonly array derived from PLATFORM_CONFIG.artifactTypes", () => {
		expect(Array.isArray(PLATFORM_ARTIFACT_TYPES)).toBe(true);
		expect(PLATFORM_ARTIFACT_TYPES.length).toBe(PLATFORM_CONFIG.artifactTypes.length);
	});
});

describe("PLATFORM_RELATIONSHIPS", () => {
	it("matches PLATFORM_CONFIG.relationships", () => {
		expect(PLATFORM_RELATIONSHIPS).toBe(PLATFORM_CONFIG.relationships);
	});
});

describe("PLATFORM_SEMANTICS", () => {
	it("matches PLATFORM_CONFIG.semantics", () => {
		expect(PLATFORM_SEMANTICS).toBe(PLATFORM_CONFIG.semantics);
	});
});

// ---------------------------------------------------------------------------
// PLATFORM_NAVIGATION — fixed engine-level builtins
// ---------------------------------------------------------------------------

describe("PLATFORM_NAVIGATION", () => {
	it("contains exactly the four engine builtins", () => {
		const keys = PLATFORM_NAVIGATION.map((n) => n.key);
		expect(keys).toContain("project");
		expect(keys).toContain("artifact-graph");
		expect(keys).toContain("plugins");
		expect(keys).toContain("settings");
	});

	it("has exactly 4 entries", () => {
		expect(PLATFORM_NAVIGATION).toHaveLength(4);
	});

	it("all entries have type builtin", () => {
		for (const item of PLATFORM_NAVIGATION) {
			expect(item.type).toBe("builtin");
		}
	});

	it("all entries have an icon", () => {
		for (const item of PLATFORM_NAVIGATION) {
			expect(typeof item.icon).toBe("string");
			expect(item.icon.length).toBeGreaterThan(0);
		}
	});
});
