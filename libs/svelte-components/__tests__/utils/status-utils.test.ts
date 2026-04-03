// Tests for status resolution utilities — resolveStatus, statusLabel, etc.
import { describe, it, expect } from "vitest";
import {
	resolveStatus,
	statusLabel,
	statusIconName,
	statusColor,
	statusIsSpinning,
	statusColorClass,
	STATUS_COLOR_CLASSES,
	DEFAULT_STATUSES,
} from "../../src/pure/status/status-utils.js";

describe("resolveStatus", () => {
	it("resolves a known status key from the default list", () => {
		const config = resolveStatus("completed");
		expect(config.key).toBe("completed");
		expect(config.label).toBe("Completed");
	});

	it("is case-insensitive", () => {
		const lower = resolveStatus("active");
		const upper = resolveStatus("ACTIVE");
		expect(upper.key).toBe(lower.key);
	});

	it("returns a fallback for an unknown key", () => {
		const config = resolveStatus("not-a-status");
		expect(config.key).toBe("not-a-status");
		expect(config.label).toBe("not-a-status");
		expect(config.icon).toBe("circle");
	});

	it("searches a custom status list first", () => {
		const custom = [{ key: "custom", label: "Custom Status", icon: "star" }];
		const config = resolveStatus("custom", custom);
		expect(config.label).toBe("Custom Status");
	});

	it("falls back to defaults when custom list does not match", () => {
		const custom = [{ key: "custom", label: "Custom", icon: "star" }];
		const config = resolveStatus("completed", custom);
		expect(config.label).toBe("Completed");
	});
});

describe("statusLabel", () => {
	it("returns the label for a known status", () => {
		expect(statusLabel("blocked")).toBe("Blocked");
	});

	it("returns raw key as label for unknown status", () => {
		expect(statusLabel("mystery")).toBe("mystery");
	});
});

describe("statusIconName", () => {
	it("returns an icon name for a known status", () => {
		const icon = statusIconName("completed");
		expect(typeof icon).toBe("string");
		expect(icon.length).toBeGreaterThan(0);
	});
});

describe("statusColor", () => {
	it("returns a semantic colour for a known status", () => {
		const color = statusColor("blocked");
		expect(color).toBe("destructive");
	});

	it("returns 'muted' for an unknown status", () => {
		expect(statusColor("unknown-status")).toBe("muted");
	});
});

describe("statusIsSpinning", () => {
	it("returns true for the 'active' status (spin=true)", () => {
		expect(statusIsSpinning("active")).toBe(true);
	});

	it("returns false for statuses that do not spin", () => {
		expect(statusIsSpinning("completed")).toBe(false);
	});
});

describe("statusColorClass", () => {
	it("maps 'destructive' to text-destructive", () => {
		expect(statusColorClass("blocked")).toBe("text-destructive");
	});

	it("maps 'muted' to text-muted-foreground", () => {
		expect(statusColorClass("archived")).toBe("text-muted-foreground");
	});

	it("returns text-muted-foreground for unknown colours", () => {
		expect(statusColorClass("totally-unknown")).toBe("text-muted-foreground");
	});
});

describe("STATUS_COLOR_CLASSES", () => {
	it("contains entries for each semantic colour", () => {
		expect(STATUS_COLOR_CLASSES["primary"]).toBe("text-primary");
		expect(STATUS_COLOR_CLASSES["success"]).toBe("text-success");
		expect(STATUS_COLOR_CLASSES["warning"]).toBe("text-warning");
		expect(STATUS_COLOR_CLASSES["destructive"]).toBe("text-destructive");
		expect(STATUS_COLOR_CLASSES["muted"]).toBe("text-muted-foreground");
	});
});

describe("DEFAULT_STATUSES", () => {
	it("contains at least the core lifecycle statuses", () => {
		const keys = DEFAULT_STATUSES.map((s) => s.key);
		expect(keys).toContain("active");
		expect(keys).toContain("completed");
		expect(keys).toContain("blocked");
		expect(keys).toContain("archived");
	});
});
