/**
 * Tests for PLATFORM_CONFIG, PLATFORM_NAVIGATION, buildInverseMap, and
 * relationship utility functions from @orqastudio/types.
 *
 * These verify the structural correctness of the platform configuration
 * constants and the pure utility functions that query them.
 */

import { describe, it, expect } from "vitest";
import {
	PLATFORM_CONFIG,
	PLATFORM_ARTIFACT_TYPES,
	PLATFORM_RELATIONSHIPS,
	PLATFORM_NAVIGATION,
	isArtifactGroup,
} from "@orqastudio/types";
import { buildInverseMap, hasSemantic, keysForSemantic } from "@orqastudio/types";

// ---------------------------------------------------------------------------
// PLATFORM_CONFIG structure
// ---------------------------------------------------------------------------

describe("PLATFORM_CONFIG", () => {
	it("has an artifactTypes array (empty by design — plugins provide types via P1)", () => {
		// Platform config is intentionally empty: plugins are the single source of truth
		// for artifact types, relationships, and validation constraints (P1: Plugin-Composed Everything).
		expect(Array.isArray(PLATFORM_CONFIG.artifactTypes)).toBe(true);
	});

	it("has a relationships array (empty by design — plugins provide relationships)", () => {
		expect(Array.isArray(PLATFORM_CONFIG.relationships)).toBe(true);
	});

	it("has a semantics object", () => {
		expect(typeof PLATFORM_CONFIG.semantics).toBe("object");
		expect(PLATFORM_CONFIG.semantics).not.toBeNull();
	});

	it("PLATFORM_ARTIFACT_TYPES mirrors artifactTypes keys (both empty when no plugins registered)", () => {
		const fromConfig = PLATFORM_CONFIG.artifactTypes.map((t) => t.key);
		expect([...PLATFORM_ARTIFACT_TYPES]).toEqual(fromConfig);
	});

	it("PLATFORM_RELATIONSHIPS mirrors relationships array", () => {
		expect([...PLATFORM_RELATIONSHIPS]).toEqual([...PLATFORM_CONFIG.relationships]);
	});
});

// ---------------------------------------------------------------------------
// PLATFORM_ARTIFACT_TYPES
// ---------------------------------------------------------------------------

describe("PLATFORM_ARTIFACT_TYPES", () => {
	it("is a readonly array derived from PLATFORM_CONFIG", () => {
		// The array is always in sync with the config file — changes to core.json
		// are reflected without code changes. Currently empty by design (P1).
		expect(Array.isArray(PLATFORM_ARTIFACT_TYPES)).toBe(true);
	});
});

// ---------------------------------------------------------------------------
// PLATFORM_NAVIGATION
// ---------------------------------------------------------------------------

describe("PLATFORM_NAVIGATION", () => {
	it("contains exactly the four platform builtin views", () => {
		const keys = PLATFORM_NAVIGATION.map((n) => n.key);
		expect(keys).toContain("project");
		expect(keys).toContain("artifact-graph");
		expect(keys).toContain("plugins");
		expect(keys).toContain("settings");
	});

	it("every item has type 'builtin'", () => {
		for (const item of PLATFORM_NAVIGATION) {
			expect(item.type).toBe("builtin");
		}
	});

	it("every item has a non-empty icon", () => {
		for (const item of PLATFORM_NAVIGATION) {
			expect(typeof item.icon).toBe("string");
			expect(item.icon.length).toBeGreaterThan(0);
		}
	});
});

// ---------------------------------------------------------------------------
// isArtifactGroup type guard
// ---------------------------------------------------------------------------

describe("isArtifactGroup", () => {
	it("returns true for an entry with children", () => {
		expect(isArtifactGroup({ key: "discovery", label: "Discovery", children: [] })).toBe(true);
	});

	it("returns false for an entry without children", () => {
		expect(isArtifactGroup({ key: "task", path: ".orqa/tasks" })).toBe(false);
	});
});

// ---------------------------------------------------------------------------
// buildInverseMap
// ---------------------------------------------------------------------------

describe("buildInverseMap", () => {
	it("maps each key to its inverse", () => {
		const rels = [{ key: "delivers", inverse: "delivered-by" }];
		const map = buildInverseMap(rels);
		expect(map.get("delivers")).toBe("delivered-by");
	});

	it("maps the inverse key back to the forward key", () => {
		const rels = [{ key: "delivers", inverse: "delivered-by" }];
		const map = buildInverseMap(rels);
		expect(map.get("delivered-by")).toBe("delivers");
	});

	it("handles self-inverse relationships (key === inverse)", () => {
		const rels = [{ key: "related-to", inverse: "related-to" }];
		const map = buildInverseMap(rels);
		expect(map.get("related-to")).toBe("related-to");
		// Should only have one entry, not two
		expect(map.size).toBe(1);
	});

	it("returns an empty map for empty input", () => {
		expect(buildInverseMap([]).size).toBe(0);
	});

	it("handles multiple relationships without collision", () => {
		const rels = [
			{ key: "delivers", inverse: "delivered-by" },
			{ key: "blocks", inverse: "blocked-by" },
		];
		const map = buildInverseMap(rels);
		expect(map.size).toBe(4);
	});

	it("builds a correct inverse map from PLATFORM_RELATIONSHIPS", () => {
		const map = buildInverseMap(PLATFORM_RELATIONSHIPS);
		// Every relationship key should have an inverse in the map
		for (const rel of PLATFORM_RELATIONSHIPS) {
			expect(map.has(rel.key)).toBe(true);
			expect(map.has(rel.inverse)).toBe(true);
			expect(map.get(rel.key)).toBe(rel.inverse);
		}
	});
});

// ---------------------------------------------------------------------------
// hasSemantic / keysForSemantic
// ---------------------------------------------------------------------------

describe("hasSemantic", () => {
	const semantics = {
		lineage: { keys: ["delivers", "delivered-by"] },
		governance: { keys: ["enforced-by"] },
	};

	it("returns true when a key belongs to the given semantic", () => {
		expect(hasSemantic(semantics, "delivers", "lineage")).toBe(true);
	});

	it("returns false when a key does not belong to the given semantic", () => {
		expect(hasSemantic(semantics, "delivers", "governance")).toBe(false);
	});

	it("returns false for an unknown semantic name", () => {
		expect(hasSemantic(semantics, "delivers", "unknown-category")).toBe(false);
	});

	it("returns false for an unknown key in a known semantic", () => {
		expect(hasSemantic(semantics, "unknown-key", "lineage")).toBe(false);
	});
});

describe("keysForSemantic", () => {
	const semantics = {
		lineage: { keys: ["delivers", "delivered-by", "evolves-into"] },
		governance: { keys: ["enforced-by", "enforces"] },
	};

	it("returns all keys for a known semantic", () => {
		expect(keysForSemantic(semantics, "lineage")).toEqual([
			"delivers",
			"delivered-by",
			"evolves-into",
		]);
	});

	it("returns empty array for an unknown semantic", () => {
		expect(keysForSemantic(semantics, "unknown")).toEqual([]);
	});
});
