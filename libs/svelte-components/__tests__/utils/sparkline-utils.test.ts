// Tests for sparkline path generation and trend calculation utilities.
import { describe, it, expect } from "vitest";
import {
	sparklinePath,
	trendPercent,
	formatTrend,
	trendArrow,
	trendColorClass,
} from "../../src/pure/sparkline/sparkline-utils.js";

describe("sparklinePath", () => {
	it("returns empty string for fewer than 2 values", () => {
		expect(sparklinePath([], 100, 40)).toBe("");
		expect(sparklinePath([5], 100, 40)).toBe("");
	});

	it("generates an SVG M...L path for 2+ values", () => {
		const path = sparklinePath([0, 10], 100, 40);
		expect(path).toMatch(/^M/);
		expect(path).toContain("L");
	});

	it("generates a path starting at 0,x for the first point when min=0 and value=0", () => {
		const path = sparklinePath([0, 10], 100, 40);
		expect(path.startsWith("M0,")).toBe(true);
	});

	it("respects fixedMax option to cap values", () => {
		const pathDefault = sparklinePath([0, 5, 10], 100, 40);
		const pathCapped = sparklinePath([0, 5, 10], 100, 40, { fixedMax: 20 });
		// With a higher fixed max the y values will differ
		expect(pathDefault).not.toBe(pathCapped);
	});

	it("respects padding option", () => {
		const pathNoPad = sparklinePath([0, 10], 100, 40, { padding: 0 });
		const pathPad = sparklinePath([0, 10], 100, 40, { padding: 8 });
		expect(pathNoPad).not.toBe(pathPad);
	});
});

describe("trendPercent", () => {
	it("calculates percentage increase correctly", () => {
		expect(trendPercent(110, 100)).toBe(10);
	});

	it("calculates percentage decrease correctly", () => {
		expect(trendPercent(90, 100)).toBe(-10);
	});

	it("returns 0 when both values are 0", () => {
		expect(trendPercent(0, 0)).toBe(0);
	});

	it("returns 100 when previous is 0 and current is positive", () => {
		expect(trendPercent(5, 0)).toBe(100);
	});

	it("rounds to integer", () => {
		// 110/3 ≈ 36.67%
		expect(trendPercent(100, 73)).toBe(37);
	});
});

describe("formatTrend", () => {
	it("returns empty string for null", () => {
		expect(formatTrend(null)).toBe("");
	});

	it("returns '0%' for zero", () => {
		expect(formatTrend(0)).toBe("0%");
	});

	it("prefixes positive values with +", () => {
		expect(formatTrend(15)).toBe("+15%");
	});

	it("uses - sign for negative values", () => {
		expect(formatTrend(-8)).toBe("-8%");
	});
});

describe("trendArrow", () => {
	it("returns empty string for null", () => {
		expect(trendArrow(null)).toBe("");
	});

	it("returns empty string for zero", () => {
		expect(trendArrow(0)).toBe("");
	});

	it("returns up arrow for positive trend", () => {
		expect(trendArrow(10)).toBe("\u2191");
	});

	it("returns down arrow for negative trend", () => {
		expect(trendArrow(-5)).toBe("\u2193");
	});
});

describe("trendColorClass", () => {
	it("returns muted class for null", () => {
		expect(trendColorClass(null)).toBe("text-muted-foreground");
	});

	it("returns muted class for zero", () => {
		expect(trendColorClass(0)).toBe("text-muted-foreground");
	});

	it("returns success for negative trend when lowerIsBetter=true (default)", () => {
		expect(trendColorClass(-10)).toBe("text-success");
	});

	it("returns destructive for positive trend when lowerIsBetter=true (default)", () => {
		expect(trendColorClass(10)).toBe("text-destructive");
	});

	it("returns success for positive trend when lowerIsBetter=false", () => {
		expect(trendColorClass(10, false)).toBe("text-success");
	});

	it("returns destructive for negative trend when lowerIsBetter=false", () => {
		expect(trendColorClass(-10, false)).toBe("text-destructive");
	});
});
