/**
 * Tests for number formatting utilities: fmt and pct.
 * Covers all branches: integer results, decimal stripping, negative values, zero, precision override.
 */
import { describe, it, expect } from "vitest";
import { fmt, pct } from "../src/utils/format.js";

describe("fmt", () => {
	it("formats an integer — no trailing decimal", () => {
		expect(fmt(3)).toBe("3");
	});

	it("formats an exact decimal", () => {
		expect(fmt(5.79)).toBe("5.79");
	});

	it("strips trailing zeros from a decimal", () => {
		expect(fmt(3.1)).toBe("3.1");
		expect(fmt(100.0)).toBe("100");
	});

	it("rounds to 2 decimal places by default", () => {
		expect(fmt(1.005)).toBe("1"); // floating-point: 1.005 rounds to 1.00 → "1"
		expect(fmt(1.234)).toBe("1.23");
		expect(fmt(1.235)).toBe("1.24");
	});

	it("respects a custom decimals argument", () => {
		expect(fmt(3.14159, 4)).toBe("3.1416");
		expect(fmt(3.14159, 0)).toBe("3");
		expect(fmt(3.14159, 1)).toBe("3.1");
	});

	it("handles negative numbers", () => {
		expect(fmt(-5.5)).toBe("-5.5");
		expect(fmt(-10)).toBe("-10");
	});

	it("handles zero", () => {
		expect(fmt(0)).toBe("0");
		expect(fmt(0.0)).toBe("0");
	});

	it("handles very small numbers near zero", () => {
		expect(fmt(0.001)).toBe("0");
		expect(fmt(0.005)).toBe("0.01");
	});

	it("handles large integers", () => {
		expect(fmt(1000000)).toBe("1000000");
	});
});

describe("pct", () => {
	it("formats 0 as '0'", () => {
		expect(pct(0)).toBe("0");
	});

	it("formats 1 as '100'", () => {
		expect(pct(1)).toBe("100");
	});

	it("formats 0.5 as '50'", () => {
		expect(pct(0.5)).toBe("50");
	});

	it("formats a fractional ratio with decimals stripped", () => {
		expect(pct(0.916)).toBe("91.6");
	});

	it("rounds to 2 decimal places", () => {
		expect(pct(0.9166)).toBe("91.66");
		expect(pct(0.91666)).toBe("91.67");
	});

	it("handles negative ratios", () => {
		expect(pct(-0.1)).toBe("-10");
	});

	it("handles ratios above 1", () => {
		expect(pct(1.5)).toBe("150");
	});
});
