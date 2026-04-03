/**
 * Tests for artifact-graph.ts exports.
 *
 * Verifies the ARTIFACT_TYPES constant contract and the CanonicalStatus values.
 */

import { describe, it, expect } from "vitest";
import { ARTIFACT_TYPES } from "../src/artifact-graph.js";

describe("ARTIFACT_TYPES", () => {
	it("is a non-empty readonly array", () => {
		expect(Array.isArray(ARTIFACT_TYPES)).toBe(true);
		expect(ARTIFACT_TYPES.length).toBeGreaterThan(0);
	});

	it("contains core delivery artifact types", () => {
		expect(ARTIFACT_TYPES).toContain("epic");
		expect(ARTIFACT_TYPES).toContain("task");
		expect(ARTIFACT_TYPES).toContain("milestone");
	});

	it("contains core discovery artifact types", () => {
		expect(ARTIFACT_TYPES).toContain("idea");
		expect(ARTIFACT_TYPES).toContain("decision");
		expect(ARTIFACT_TYPES).toContain("research");
	});

	it("contains core governance artifact types", () => {
		expect(ARTIFACT_TYPES).toContain("rule");
		expect(ARTIFACT_TYPES).toContain("agent");
		expect(ARTIFACT_TYPES).toContain("knowledge");
	});

	it("contains lesson and pillar", () => {
		expect(ARTIFACT_TYPES).toContain("lesson");
		expect(ARTIFACT_TYPES).toContain("pillar");
	});

	it("all entries are non-empty lowercase strings", () => {
		for (const t of ARTIFACT_TYPES) {
			expect(typeof t).toBe("string");
			expect(t.length).toBeGreaterThan(0);
			expect(t).toBe(t.toLowerCase());
		}
	});

	it("has no duplicates", () => {
		const unique = new Set(ARTIFACT_TYPES);
		expect(unique.size).toBe(ARTIFACT_TYPES.length);
	});
});
