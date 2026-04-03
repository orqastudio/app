/**
 * Tests for pure utility functions in the SDK and related libraries.
 *
 * These tests cover extractErrorMessage (ipc/invoke), parseFrontmatter,
 * fmt/pct number formatters, and the hash router helpers. All tested
 * functions are pure (no Tauri IPC, no Svelte reactivity), so no mocks
 * are needed beyond the global setup.
 */

import { describe, it, expect } from "vitest";
import { extractErrorMessage, parseFrontmatter, fmt, pct, parseHash } from "@orqastudio/sdk";

// ---------------------------------------------------------------------------
// extractErrorMessage
// ---------------------------------------------------------------------------

describe("extractErrorMessage", () => {
	it("extracts message from an Error instance", () => {
		expect(extractErrorMessage(new Error("boom"))).toBe("boom");
	});

	it("returns a plain string unchanged", () => {
		expect(extractErrorMessage("raw string error")).toBe("raw string error");
	});

	it("extracts message from an OrqaError-shaped object", () => {
		expect(extractErrorMessage({ message: "structured error" })).toBe("structured error");
	});

	it("stringifies an unknown value", () => {
		expect(extractErrorMessage(42)).toBe("42");
	});

	it("handles null", () => {
		expect(extractErrorMessage(null)).toBe("null");
	});

	it("handles undefined", () => {
		expect(extractErrorMessage(undefined)).toBe("undefined");
	});

	it("handles an object without a message field", () => {
		const result = extractErrorMessage({ code: 404 });
		expect(result).toBe("[object Object]");
	});
});

// ---------------------------------------------------------------------------
// parseFrontmatter
// ---------------------------------------------------------------------------

describe("parseFrontmatter", () => {
	it("returns empty metadata and original body when no frontmatter", () => {
		const result = parseFrontmatter("just body content");
		expect(result.metadata).toEqual({});
		expect(result.body).toBe("just body content");
	});

	it("parses simple key-value frontmatter", () => {
		const content = "---\ntitle: Hello World\nstatus: active\n---\nBody here";
		const result = parseFrontmatter(content);
		expect(result.metadata.title).toBe("Hello World");
		expect(result.metadata.status).toBe("active");
		expect(result.body).toBe("Body here");
	});

	it("parses YAML array values (indented list items)", () => {
		const content = "---\ntags:\n  - alpha\n  - beta\n---\nbody";
		const result = parseFrontmatter(content);
		expect(result.metadata.tags).toEqual(["alpha", "beta"]);
	});

	it("parses inline array values", () => {
		const content = "---\ncolors: [red, green, blue]\n---\nbody";
		const result = parseFrontmatter(content);
		expect(result.metadata.colors).toEqual(["red", "green", "blue"]);
	});

	it("strips surrounding quotes from values", () => {
		const content = '---\nname: "quoted"\n---\nbody';
		const result = parseFrontmatter(content);
		expect(result.metadata.name).toBe("quoted");
	});

	it("returns empty metadata when opening delimiter has no closing", () => {
		const result = parseFrontmatter("---\ntitle: test\nno closing delimiter");
		expect(result.metadata).toEqual({});
	});

	it("normalizes CRLF line endings before parsing", () => {
		const content = "---\r\ntitle: CRLF test\r\n---\r\nbody";
		const result = parseFrontmatter(content);
		expect(result.metadata.title).toBe("CRLF test");
		expect(result.body).toBe("body");
	});

	it("handles empty body after frontmatter", () => {
		const content = "---\ntitle: test\n---\n";
		const result = parseFrontmatter(content);
		expect(result.metadata.title).toBe("test");
		expect(result.body).toBe("");
	});

	it("handles multiline block scalar values (| indicator)", () => {
		const content = "---\ndescription: |\n  line one\n  line two\n---\nbody";
		const result = parseFrontmatter(content);
		expect(result.metadata.description).toBe("line one\nline two");
	});
});

// ---------------------------------------------------------------------------
// fmt / pct
// ---------------------------------------------------------------------------

describe("fmt", () => {
	it("formats an integer with no decimals", () => {
		expect(fmt(100)).toBe("100");
	});

	it("strips trailing zeros", () => {
		expect(fmt(3.0)).toBe("3");
	});

	it("keeps significant decimals", () => {
		expect(fmt(5.79)).toBe("5.79");
	});

	it("respects custom decimal places", () => {
		expect(fmt(1.23456, 3)).toBe("1.235");
	});

	it("rounds to 2 places by default", () => {
		expect(fmt(1.999)).toBe("2");
	});

	it("handles zero", () => {
		expect(fmt(0)).toBe("0");
	});

	it("handles negative numbers", () => {
		expect(fmt(-1.5)).toBe("-1.5");
	});
});

describe("pct", () => {
	it("converts ratio 1.0 to 100", () => {
		expect(pct(1.0)).toBe("100");
	});

	it("converts ratio 0 to 0", () => {
		expect(pct(0)).toBe("0");
	});

	it("converts ratio 0.916 to 91.6", () => {
		expect(pct(0.916)).toBe("91.6");
	});

	it("strips trailing zeros from percentage", () => {
		expect(pct(0.5)).toBe("50");
	});
});

// ---------------------------------------------------------------------------
// Router — parseHash / buildHash
// ---------------------------------------------------------------------------

describe("parseHash", () => {
	it("parses empty hash as default route", () => {
		expect(parseHash("")).toEqual({ type: "default" });
	});

	it("parses #/ as default route", () => {
		expect(parseHash("#/")).toEqual({ type: "default" });
	});

	it("parses project route", () => {
		expect(parseHash("#/project")).toEqual({ type: "project" });
	});

	it("parses settings route", () => {
		expect(parseHash("#/settings")).toEqual({ type: "settings" });
	});

	it("parses graph route", () => {
		expect(parseHash("#/graph")).toEqual({ type: "graph" });
	});

	it("parses artifacts activity route", () => {
		expect(parseHash("#/artifacts/tasks")).toEqual({ type: "artifacts", activity: "tasks" });
	});

	it("parses artifact path route", () => {
		const result = parseHash("#/artifacts/tasks/.orqa/implementation/tasks/TASK-001.md");
		expect(result.type).toBe("artifact");
		expect(result.activity).toBe("tasks");
		expect(result.artifactPath).toBe(".orqa/implementation/tasks/TASK-001.md");
	});

	it("parses plugin route with scoped package name", () => {
		const result = parseHash("#/plugin/@orqastudio/plugin-software-kanban/roadmap");
		expect(result.type).toBe("plugin");
		expect(result.pluginName).toBe("@orqastudio/plugin-software-kanban");
		expect(result.viewKey).toBe("roadmap");
	});

	it("parses setup route", () => {
		expect(parseHash("#/setup")).toEqual({ type: "setup" });
	});
});

describe("parseHash — edge cases and format assumptions", () => {
	it("parses a legacy activity segment without /artifacts/ prefix", () => {
		// Backwards-compatible: bare segment treated as activity
		const result = parseHash("#/tasks");
		expect(result.type).toBe("artifacts");
		expect(result.activity).toBe("tasks");
	});

	it("handles an artifacts hash with no trailing activity", () => {
		const result = parseHash("#/artifacts");
		expect(result.type).toBe("artifacts");
		expect(result.activity).toBeUndefined();
	});

	it("treats a plain # as the default route", () => {
		expect(parseHash("#")).toEqual({ type: "default" });
	});

	it("parses plugin route with a simple (non-scoped) name", () => {
		const result = parseHash("#/plugin/my-plugin/view");
		expect(result.type).toBe("plugin");
		expect(result.pluginName).toBe("my-plugin");
		expect(result.viewKey).toBe("view");
	});
});
