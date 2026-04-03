/**
 * Tests for pure utility functions in config/frontmatter-config.ts.
 *
 * Both priorityClass and priorityLabel are pure string→string functions;
 * no mocks or DOM interaction needed.
 */

import { describe, it, expect } from "vitest";
import { priorityClass, priorityLabel } from "$lib/config/frontmatter-config";

// ---------------------------------------------------------------------------
// priorityClass
// ---------------------------------------------------------------------------

describe("priorityClass", () => {
	it("returns destructive class for P1", () => {
		const result = priorityClass("P1");
		expect(result).toContain("destructive");
	});

	it("returns warning class for P2", () => {
		const result = priorityClass("P2");
		expect(result).toContain("warning");
	});

	it("returns emerald class for P3", () => {
		const result = priorityClass("P3");
		expect(result).toContain("emerald");
	});

	it("returns empty string for P0 (not a defined priority level)", () => {
		expect(priorityClass("P0")).toBe("");
	});

	it("returns empty string for P4", () => {
		expect(priorityClass("P4")).toBe("");
	});

	it("returns empty string for unknown string", () => {
		expect(priorityClass("critical")).toBe("");
		expect(priorityClass("")).toBe("");
	});

	it("is case-sensitive (lowercase p1 returns empty)", () => {
		expect(priorityClass("p1")).toBe("");
		expect(priorityClass("p2")).toBe("");
	});
});

// ---------------------------------------------------------------------------
// priorityLabel
// ---------------------------------------------------------------------------

describe("priorityLabel", () => {
	it("returns 'P1 — Critical' for P1", () => {
		expect(priorityLabel("P1")).toBe("P1 — Critical");
	});

	it("returns 'P2 — Important' for P2", () => {
		expect(priorityLabel("P2")).toBe("P2 — Important");
	});

	it("returns 'P3 — Nice to Have' for P3", () => {
		expect(priorityLabel("P3")).toBe("P3 — Nice to Have");
	});

	it("returns the original string for unrecognised priorities", () => {
		expect(priorityLabel("P0")).toBe("P0");
		expect(priorityLabel("P4")).toBe("P4");
		expect(priorityLabel("high")).toBe("high");
		expect(priorityLabel("")).toBe("");
	});

	it("is case-sensitive (lowercase p1 falls through to identity)", () => {
		expect(priorityLabel("p1")).toBe("p1");
	});
});
