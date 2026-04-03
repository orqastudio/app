/**
 * Tests for relationship utility functions in libs/types/src/constants.ts.
 *
 * Verifies that buildInverseMap, hasSemantic, and keysForSemantic correctly
 * derive relationship inverses and answer semantic queries without hardcoded keys.
 */

import { describe, it, expect } from "vitest";
import { buildInverseMap, hasSemantic, keysForSemantic } from "../src/constants.js";

// ---------------------------------------------------------------------------
// buildInverseMap
// ---------------------------------------------------------------------------

describe("buildInverseMap()", () => {
	it("maps each key to its inverse", () => {
		const map = buildInverseMap([
			{ key: "delivers", inverse: "delivered-by" },
			{ key: "blocks", inverse: "blocked-by" },
		]);

		expect(map.get("delivers")).toBe("delivered-by");
		expect(map.get("blocks")).toBe("blocked-by");
	});

	it("maps each inverse back to its key (bidirectional)", () => {
		const map = buildInverseMap([
			{ key: "delivers", inverse: "delivered-by" },
		]);

		expect(map.get("delivered-by")).toBe("delivers");
	});

	it("handles self-referential relationships (key === inverse) without double entries", () => {
		const map = buildInverseMap([
			{ key: "related-to", inverse: "related-to" },
		]);

		// Only one entry: related-to → related-to
		expect(map.get("related-to")).toBe("related-to");
		// Size should be 1, not 2
		expect(map.size).toBe(1);
	});

	it("returns a read-only Map (is a Map instance)", () => {
		const map = buildInverseMap([{ key: "a", inverse: "b" }]);
		expect(map instanceof Map).toBe(true);
	});

	it("returns an empty map for an empty input array", () => {
		const map = buildInverseMap([]);
		expect(map.size).toBe(0);
	});

	it("handles multiple relationships in one call", () => {
		const rels = [
			{ key: "delivers", inverse: "delivered-by" },
			{ key: "blocks", inverse: "blocked-by" },
			{ key: "grounded-by", inverse: "grounds" },
			{ key: "related-to", inverse: "related-to" },
		];
		const map = buildInverseMap(rels);

		// 3 asymmetric pairs × 2 entries each + 1 self-referential = 7
		expect(map.size).toBe(7);
		expect(map.get("grounded-by")).toBe("grounds");
		expect(map.get("grounds")).toBe("grounded-by");
	});

	it("later entries overwrite earlier ones on key collision", () => {
		// Degenerate case: two relationships share the same key
		const map = buildInverseMap([
			{ key: "depends-on", inverse: "depended-on-by" },
			{ key: "depends-on", inverse: "required-by" },
		]);

		// The second entry wins
		expect(map.get("depends-on")).toBe("required-by");
	});
});

// ---------------------------------------------------------------------------
// hasSemantic
// ---------------------------------------------------------------------------

describe("hasSemantic()", () => {
	const semantics: Record<string, { keys: string[] }> = {
		lineage: { keys: ["evolves-into", "evolves-from", "merged-into"] },
		delivery: { keys: ["delivers", "delivered-by"] },
	};

	it("returns true when the key is in the named semantic category", () => {
		expect(hasSemantic(semantics, "evolves-into", "lineage")).toBe(true);
	});

	it("returns false when the key is not in the named semantic category", () => {
		expect(hasSemantic(semantics, "delivers", "lineage")).toBe(false);
	});

	it("returns false for an unknown semantic name", () => {
		expect(hasSemantic(semantics, "delivers", "unknown-category")).toBe(false);
	});

	it("returns false for an unknown relationship key in a known category", () => {
		expect(hasSemantic(semantics, "no-such-key", "lineage")).toBe(false);
	});

	it("returns false for an empty semantics map", () => {
		expect(hasSemantic({}, "delivers", "delivery")).toBe(false);
	});
});

// ---------------------------------------------------------------------------
// keysForSemantic
// ---------------------------------------------------------------------------

describe("keysForSemantic()", () => {
	const semantics: Record<string, { keys: string[] }> = {
		lineage: { keys: ["evolves-into", "evolves-from", "merged-into", "merged-from"] },
		delivery: { keys: ["delivers", "delivered-by"] },
	};

	it("returns all keys for the named semantic category", () => {
		const keys = keysForSemantic(semantics, "lineage");
		expect(keys).toEqual(["evolves-into", "evolves-from", "merged-into", "merged-from"]);
	});

	it("returns an empty array for an unknown semantic name", () => {
		const keys = keysForSemantic(semantics, "nonexistent");
		expect(keys).toEqual([]);
	});

	it("returns an empty array for an empty semantics map", () => {
		const keys = keysForSemantic({}, "lineage");
		expect(keys).toEqual([]);
	});

	it("returns the delivery keys correctly", () => {
		const keys = keysForSemantic(semantics, "delivery");
		expect(keys).toContain("delivers");
		expect(keys).toContain("delivered-by");
	});
});
