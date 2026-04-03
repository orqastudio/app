// Tests for the cn() utility — merges clsx and tailwind-merge correctly.
import { describe, it, expect } from "vitest";
import { cn } from "../../src/utils/cn.js";

describe("cn", () => {
	it("returns a single class unchanged", () => {
		expect(cn("text-sm")).toBe("text-sm");
	});

	it("joins multiple classes with a space", () => {
		expect(cn("text-sm", "font-bold")).toBe("text-sm font-bold");
	});

	it("ignores falsy values", () => {
		expect(cn("text-sm", false, undefined, null, "font-bold")).toBe("text-sm font-bold");
	});

	it("merges conflicting Tailwind classes (last wins)", () => {
		// tailwind-merge should deduplicate bg-red-500 and bg-blue-500
		const result = cn("bg-red-500", "bg-blue-500");
		expect(result).toBe("bg-blue-500");
	});

	it("handles conditional class objects (clsx syntax)", () => {
		const result = cn({ "text-bold": true, "text-italic": false });
		expect(result).toBe("text-bold");
	});

	it("handles arrays of classes", () => {
		const result = cn(["text-sm", "font-mono"]);
		expect(result).toContain("text-sm");
		expect(result).toContain("font-mono");
	});

	it("returns empty string for no arguments", () => {
		expect(cn()).toBe("");
	});
});
