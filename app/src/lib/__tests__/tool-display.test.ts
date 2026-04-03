/**
 * Tests for pure utility functions in utils/tool-display.ts.
 *
 * stripToolName, getToolDisplay, getCapabilityLabel, groupLabel,
 * getActivityPhase, and getEphemeralLabel are all pure string→string
 * (or string→object) functions.
 *
 * getToolDisplay calls resolveIcon from @orqastudio/svelte-components/pure,
 * which returns a Svelte component. We mock that import so tests don't
 * require compiled Svelte components.
 */

import { describe, it, expect, vi, beforeAll } from "vitest";

// Mock the svelte-components import before importing the module under test.
// resolveIcon is the only symbol we need — return a stable dummy value.
vi.mock("@orqastudio/svelte-components/pure", () => ({
	resolveIcon: (name: string) => `icon:${name}`,
}));

// Mock the sdk logger (used internally by getEphemeralLabel for warnings).
vi.mock("@orqastudio/sdk", () => ({
	logger: () => ({ warn: vi.fn(), info: vi.fn(), error: vi.fn() }),
}));

import {
	stripToolName,
	getToolDisplay,
	getCapabilityLabel,
	groupLabel,
	getActivityPhase,
	getEphemeralLabel,
	CAPABILITY_LABELS,
} from "$lib/utils/tool-display";

// ---------------------------------------------------------------------------
// stripToolName
// ---------------------------------------------------------------------------

describe("stripToolName", () => {
	it("returns the name unchanged when there is no MCP prefix", () => {
		expect(stripToolName("read_file")).toBe("read_file");
		expect(stripToolName("bash")).toBe("bash");
	});

	it("strips the mcp__server__ prefix and returns the last segment", () => {
		expect(stripToolName("mcp__orqa__read_file")).toBe("read_file");
		expect(stripToolName("mcp__filesystem__write_file")).toBe("write_file");
	});

	it("returns the name unchanged when there are only two parts", () => {
		// Only 2 parts after split — not enough for mcp pattern
		expect(stripToolName("mcp__read_file")).toBe("mcp__read_file");
	});

	it("handles empty string", () => {
		expect(stripToolName("")).toBe("");
	});

	it("returns last segment when there are more than 3 parts", () => {
		expect(stripToolName("mcp__server__sub__tool")).toBe("tool");
	});
});

// ---------------------------------------------------------------------------
// getToolDisplay
// ---------------------------------------------------------------------------

describe("getToolDisplay", () => {
	it("returns known label for a known tool name", () => {
		const result = getToolDisplay("read_file");
		expect(result.label).toBe("Read File");
	});

	it("returns the iconName for a known tool", () => {
		const result = getToolDisplay("bash");
		expect(result.iconName).toBe("terminal");
	});

	it("falls back to stripped name as label for unknown tools", () => {
		const result = getToolDisplay("unknown_tool");
		expect(result.label).toBe("unknown_tool");
	});

	it("falls back to 'wrench' icon for unknown tools", () => {
		const result = getToolDisplay("unknown_tool");
		expect(result.iconName).toBe("wrench");
	});

	it("strips MCP prefix before resolving display config", () => {
		const result = getToolDisplay("mcp__orqa__read_file");
		expect(result.label).toBe("Read File");
		expect(result.iconName).toBe("file-text");
	});

	it("returns an icon value from resolveIcon", () => {
		const result = getToolDisplay("edit_file");
		// Our mock returns `icon:<name>` — verify it called resolveIcon with the icon name
		expect(result.icon).toBe("icon:pencil");
	});
});

// ---------------------------------------------------------------------------
// getCapabilityLabel
// ---------------------------------------------------------------------------

describe("getCapabilityLabel", () => {
	it("returns a known label for a known capability", () => {
		expect(getCapabilityLabel("file_read")).toBe("Read Files");
		expect(getCapabilityLabel("shell_execute")).toBe("Run Commands");
		expect(getCapabilityLabel("web_search")).toBe("Web Search");
	});

	it("converts unknown capabilities to title-cased words", () => {
		expect(getCapabilityLabel("some_unknown_capability")).toBe("Some Unknown Capability");
	});

	it("handles single-word capability", () => {
		// No underscores — returned as capitalised single word
		expect(getCapabilityLabel("read")).toBe("Read");
	});

	it("covers all CAPABILITY_LABELS entries", () => {
		for (const key of Object.keys(CAPABILITY_LABELS)) {
			const result = getCapabilityLabel(key);
			expect(result).toBe(CAPABILITY_LABELS[key]);
		}
	});
});

// ---------------------------------------------------------------------------
// groupLabel
// ---------------------------------------------------------------------------

describe("groupLabel", () => {
	it("returns 'Read N files' for read_file with count", () => {
		expect(groupLabel("read_file", 3)).toBe("Read 3 files");
	});

	it("returns 'Wrote N files' for write_file", () => {
		expect(groupLabel("write_file", 1)).toBe("Wrote 1 files");
	});

	it("returns 'Edited N files' for edit_file", () => {
		expect(groupLabel("edit_file", 5)).toBe("Edited 5 files");
	});

	it("returns 'Ran N commands' for bash", () => {
		expect(groupLabel("bash", 2)).toBe("Ran 2 commands");
	});

	it("returns 'Found files (N searches)' for glob", () => {
		expect(groupLabel("glob", 4)).toBe("Found files (4 searches)");
	});

	it("returns 'Searched content (N searches)' for grep", () => {
		expect(groupLabel("grep", 7)).toBe("Searched content (7 searches)");
	});

	it("returns 'Regex search (N searches)' for search_regex", () => {
		expect(groupLabel("search_regex", 1)).toBe("Regex search (1 searches)");
	});

	it("returns 'Semantic search (N queries)' for search_semantic", () => {
		expect(groupLabel("search_semantic", 2)).toBe("Semantic search (2 queries)");
	});

	it("returns 'Code research (N queries)' for code_research", () => {
		expect(groupLabel("code_research", 3)).toBe("Code research (3 queries)");
	});

	it("falls back to '<stripped_name> (N calls)' for unknown tool", () => {
		expect(groupLabel("unknown_tool", 6)).toBe("unknown_tool (6 calls)");
	});

	it("strips MCP prefix before resolving label", () => {
		expect(groupLabel("mcp__orqa__read_file", 2)).toBe("Read 2 files");
	});
});

// ---------------------------------------------------------------------------
// getActivityPhase
// ---------------------------------------------------------------------------

describe("getActivityPhase", () => {
	it("returns 'Exploring Code' for read_file", () => {
		expect(getActivityPhase("read_file")).toBe("Exploring Code");
	});

	it("returns 'Exploring Code' for glob", () => {
		expect(getActivityPhase("glob")).toBe("Exploring Code");
	});

	it("returns 'Exploring Code' for grep", () => {
		expect(getActivityPhase("grep")).toBe("Exploring Code");
	});

	it("returns 'Exploring Code' for search_regex", () => {
		expect(getActivityPhase("search_regex")).toBe("Exploring Code");
	});

	it("returns 'Exploring Code' for search_semantic", () => {
		expect(getActivityPhase("search_semantic")).toBe("Exploring Code");
	});

	it("returns 'Researching' for code_research", () => {
		expect(getActivityPhase("code_research")).toBe("Researching");
	});

	it("returns 'Running Commands' for bash", () => {
		expect(getActivityPhase("bash")).toBe("Running Commands");
	});

	it("returns 'Writing Code' for write_file", () => {
		expect(getActivityPhase("write_file")).toBe("Writing Code");
	});

	it("returns 'Writing Code' for edit_file", () => {
		expect(getActivityPhase("edit_file")).toBe("Writing Code");
	});

	it("returns 'Working' for unknown tool", () => {
		expect(getActivityPhase("mystery_tool")).toBe("Working");
	});

	it("strips MCP prefix before resolving phase", () => {
		expect(getActivityPhase("mcp__orqa__bash")).toBe("Running Commands");
	});
});

// ---------------------------------------------------------------------------
// getEphemeralLabel
// ---------------------------------------------------------------------------

describe("getEphemeralLabel", () => {
	it("returns 'Reading <path>' for read_file with file_path in input", () => {
		const label = getEphemeralLabel("read_file", JSON.stringify({ file_path: "/a/b/c.ts" }));
		expect(label).toContain("Reading");
		expect(label).toContain("c.ts");
	});

	it("returns 'Writing <path>' for write_file", () => {
		const label = getEphemeralLabel("write_file", JSON.stringify({ file_path: "/a/b/out.ts" }));
		expect(label).toContain("Writing");
		expect(label).toContain("out.ts");
	});

	it("returns 'Editing <path>' for edit_file", () => {
		const label = getEphemeralLabel("edit_file", JSON.stringify({ file_path: "/a/b/edit.ts" }));
		expect(label).toContain("Editing");
		expect(label).toContain("edit.ts");
	});

	it("returns 'Finding <pattern>' for glob", () => {
		const label = getEphemeralLabel("glob", JSON.stringify({ pattern: "**/*.ts" }));
		expect(label).toBe("Finding **/*.ts");
	});

	it("returns 'Searching for \"<pattern>\"' for grep", () => {
		const label = getEphemeralLabel("grep", JSON.stringify({ pattern: "hello" }));
		expect(label).toBe('Searching for "hello"');
	});

	it("truncates long grep patterns at 40 chars", () => {
		const longPattern = "a".repeat(50);
		const label = getEphemeralLabel("grep", JSON.stringify({ pattern: longPattern }));
		expect(label).toContain("...");
		expect(label.length).toBeLessThan(80);
	});

	it("returns 'Running command' for bash", () => {
		const label = getEphemeralLabel("bash", JSON.stringify({ command: "ls -la" }));
		expect(label).toBe("Running command");
	});

	it("falls back to getToolDisplay label for unknown tools", () => {
		const label = getEphemeralLabel("unknown_tool", JSON.stringify({}));
		// Falls back to getToolDisplay("unknown_tool").label which is "unknown_tool"
		expect(label).toBe("unknown_tool");
	});

	it("falls back gracefully on invalid JSON input", () => {
		// Should not throw — returns fallback label
		expect(() => getEphemeralLabel("read_file", "not-json")).not.toThrow();
	});

	it("strips MCP prefix for ephemeral label resolution", () => {
		const label = getEphemeralLabel("mcp__orqa__bash", JSON.stringify({ command: "x" }));
		expect(label).toBe("Running command");
	});
});
