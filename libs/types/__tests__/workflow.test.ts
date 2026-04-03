/**
 * Tests for workflow.ts STATE_CATEGORIES export.
 *
 * Verifies the runtime constant matches the expected state category values
 * used across the plugin-owned state machines.
 */

import { describe, it, expect } from "vitest";
import { STATE_CATEGORIES } from "../src/workflow.js";

describe("STATE_CATEGORIES", () => {
	it("is a non-empty readonly array", () => {
		expect(Array.isArray(STATE_CATEGORIES)).toBe(true);
		expect(STATE_CATEGORIES.length).toBeGreaterThan(0);
	});

	it("contains all five canonical state categories", () => {
		expect(STATE_CATEGORIES).toContain("planning");
		expect(STATE_CATEGORIES).toContain("active");
		expect(STATE_CATEGORIES).toContain("review");
		expect(STATE_CATEGORIES).toContain("completed");
		expect(STATE_CATEGORIES).toContain("terminal");
	});

	it("has exactly 5 entries", () => {
		expect(STATE_CATEGORIES).toHaveLength(5);
	});

	it("has no duplicates", () => {
		const unique = new Set(STATE_CATEGORIES);
		expect(unique.size).toBe(STATE_CATEGORIES.length);
	});
});
