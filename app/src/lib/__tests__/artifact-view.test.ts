/**
 * Tests for artifact-view utility functions.
 *
 * applyFilters, applySort, applyGrouping, and countFieldValues operate purely
 * on DocNode arrays — no IPC or Svelte reactivity involved.
 */

import { describe, it, expect } from "vitest";
import { applyFilters, applySort, applyGrouping, countFieldValues } from "../utils/artifact-view.js";
import type { DocNode, FilterableField } from "@orqastudio/types";

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

function makeNode(label: string, frontmatter: Record<string, unknown> = {}): DocNode {
	return {
		label,
		path: `.orqa/${label}.md`,
		frontmatter,
		children: null,
	};
}

function makeDir(label: string, children: DocNode[] = []): DocNode {
	return {
		label,
		path: `.orqa/${label}/`,
		children,
	};
}

// ---------------------------------------------------------------------------
// applyFilters
// ---------------------------------------------------------------------------

describe("applyFilters", () => {
	it("returns all nodes when no active filters", () => {
		const nodes = [makeNode("a"), makeNode("b")];
		expect(applyFilters(nodes, {})).toEqual(nodes);
	});

	it("returns all nodes when all filter arrays are empty", () => {
		const nodes = [makeNode("a"), makeNode("b")];
		expect(applyFilters(nodes, { status: [] })).toEqual(nodes);
	});

	it("filters leaf nodes by a single field", () => {
		const nodes = [
			makeNode("active", { status: "active" }),
			makeNode("done", { status: "done" }),
		];
		const result = applyFilters(nodes, { status: ["active"] });
		expect(result).toHaveLength(1);
		expect(result[0].label).toBe("active");
	});

	it("filters by multiple field values (OR within field)", () => {
		const nodes = [
			makeNode("a", { status: "active" }),
			makeNode("b", { status: "done" }),
			makeNode("c", { status: "archived" }),
		];
		const result = applyFilters(nodes, { status: ["active", "done"] });
		expect(result).toHaveLength(2);
	});

	it("applies AND logic across multiple filters", () => {
		const nodes = [
			makeNode("match", { status: "active", type: "epic" }),
			makeNode("wrong-type", { status: "active", type: "task" }),
			makeNode("wrong-status", { status: "done", type: "epic" }),
		];
		const result = applyFilters(nodes, { status: ["active"], type: ["epic"] });
		expect(result).toHaveLength(1);
		expect(result[0].label).toBe("match");
	});

	it("excludes nodes missing a filtered field", () => {
		const nodes = [
			makeNode("has-status", { status: "active" }),
			makeNode("no-status", {}),
		];
		const result = applyFilters(nodes, { status: ["active"] });
		expect(result).toHaveLength(1);
	});

	it("always includes directory nodes (children !== null)", () => {
		const dir = makeDir("mydir");
		const leaf = makeNode("leaf", { status: "done" });
		const result = applyFilters([dir, leaf], { status: ["active"] });
		// Directory passes through; leaf is filtered out
		expect(result).toContain(dir);
		expect(result).not.toContain(leaf);
	});
});

// ---------------------------------------------------------------------------
// applySort
// ---------------------------------------------------------------------------

describe("applySort", () => {
	it("sorts by label ascending (default)", () => {
		const nodes = [makeNode("Banana"), makeNode("Apple"), makeNode("Cherry")];
		const sorted = applySort(nodes, { field: "title", direction: "asc" });
		expect(sorted.map((n) => n.label)).toEqual(["Apple", "Banana", "Cherry"]);
	});

	it("sorts by label descending", () => {
		const nodes = [makeNode("Banana"), makeNode("Apple"), makeNode("Cherry")];
		const sorted = applySort(nodes, { field: "title", direction: "desc" });
		expect(sorted.map((n) => n.label)).toEqual(["Cherry", "Banana", "Apple"]);
	});

	it("sorts by frontmatter string field ascending", () => {
		const nodes = [
			makeNode("c", { priority: "low" }),
			makeNode("a", { priority: "high" }),
			makeNode("b", { priority: "medium" }),
		];
		const sorted = applySort(nodes, { field: "priority", direction: "asc" });
		expect(sorted.map((n) => n.frontmatter?.["priority"])).toEqual(["high", "low", "medium"]);
	});

	it("moves nodes with missing field to the end regardless of direction", () => {
		const nodes = [
			makeNode("no-field"),
			makeNode("has-field", { status: "active" }),
		];
		const asc = applySort(nodes, { field: "status", direction: "asc" });
		const desc = applySort(nodes, { field: "status", direction: "desc" });
		expect(asc[asc.length - 1].label).toBe("no-field");
		expect(desc[desc.length - 1].label).toBe("no-field");
	});

	it("sorts dates chronologically ascending", () => {
		const nodes = [
			makeNode("c", { created: "2024-03-01" }),
			makeNode("a", { created: "2024-01-01" }),
			makeNode("b", { created: "2024-02-01" }),
		];
		const sorted = applySort(nodes, { field: "created", direction: "asc" });
		expect(sorted.map((n) => n.frontmatter?.["created"])).toEqual([
			"2024-01-01",
			"2024-02-01",
			"2024-03-01",
		]);
	});

	it("does not mutate the input array", () => {
		const nodes = [makeNode("b"), makeNode("a")];
		const original = [...nodes];
		applySort(nodes, { field: "title", direction: "asc" });
		expect(nodes).toEqual(original);
	});
});

// ---------------------------------------------------------------------------
// applyGrouping
// ---------------------------------------------------------------------------

describe("applyGrouping", () => {
	it("groups nodes by a frontmatter field", () => {
		const nodes = [
			makeNode("task-a", { status: "active" }),
			makeNode("task-b", { status: "done" }),
			makeNode("task-c", { status: "active" }),
		];
		const groups = applyGrouping(nodes, "status", undefined, []);
		const active = groups.find((g) => g.label === "Active");
		const done = groups.find((g) => g.label === "Done");
		expect(active?.nodes).toHaveLength(2);
		expect(done?.nodes).toHaveLength(1);
	});

	it("puts nodes without the group field into Other", () => {
		const nodes = [
			makeNode("has-status", { status: "active" }),
			makeNode("no-status"),
		];
		const groups = applyGrouping(nodes, "status", undefined, []);
		const other = groups.find((g) => g.label === "Other");
		expect(other?.nodes).toHaveLength(1);
		expect(other?.nodes[0].label).toBe("no-status");
	});

	it("respects explicit groupOrder from navigation config", () => {
		const nodes = [
			makeNode("b", { status: "done" }),
			makeNode("a", { status: "active" }),
			makeNode("c", { status: "pending" }),
		];
		const groups = applyGrouping(nodes, "status", ["active", "pending", "done"], []);
		expect(groups.map((g) => g.label)).toEqual(["Active", "Pending", "Done"]);
	});

	it("respects schema enum order from FilterableField when no groupOrder", () => {
		const nodes = [
			makeNode("b", { status: "done" }),
			makeNode("a", { status: "active" }),
		];
		const fields: FilterableField[] = [{ name: "status", values: ["active", "done"] }];
		const groups = applyGrouping(nodes, "status", undefined, fields);
		expect(groups[0].label).toBe("Active");
		expect(groups[1].label).toBe("Done");
	});

	it("puts directory nodes into Other group", () => {
		const dir = makeDir("subdir");
		const leaf = makeNode("leaf", { status: "active" });
		const groups = applyGrouping([dir, leaf], "status", undefined, []);
		const other = groups.find((g) => g.label === "Other");
		expect(other?.nodes).toContain(dir);
	});

	it("omits empty groups from output", () => {
		const nodes = [makeNode("a", { status: "active" })];
		const fields: FilterableField[] = [{ name: "status", values: ["active", "done", "archived"] }];
		const groups = applyGrouping(nodes, "status", undefined, fields);
		// Only "active" has nodes — done and archived should not appear
		expect(groups).toHaveLength(1);
		expect(groups[0].label).toBe("Active");
	});

	it("humanizes group labels (hyphens → spaces, title case)", () => {
		const nodes = [makeNode("a", { status: "in-progress" })];
		const groups = applyGrouping(nodes, "status", undefined, []);
		expect(groups[0].label).toBe("In Progress");
	});
});

// ---------------------------------------------------------------------------
// countFieldValues
// ---------------------------------------------------------------------------

describe("countFieldValues", () => {
	it("counts occurrences of each unique field value", () => {
		const nodes = [
			makeNode("a", { status: "active" }),
			makeNode("b", { status: "active" }),
			makeNode("c", { status: "done" }),
		];
		const counts = countFieldValues(nodes, "status");
		expect(counts["active"]).toBe(2);
		expect(counts["done"]).toBe(1);
	});

	it("skips directory nodes", () => {
		const dir = makeDir("dir");
		const leaf = makeNode("leaf", { status: "active" });
		const counts = countFieldValues([dir, leaf], "status");
		expect(counts["active"]).toBe(1);
	});

	it("skips nodes without the field", () => {
		const nodes = [makeNode("a", { status: "active" }), makeNode("b")];
		const counts = countFieldValues(nodes, "status");
		expect(counts["active"]).toBe(1);
		expect(Object.keys(counts)).toHaveLength(1);
	});

	it("returns empty object when no nodes have the field", () => {
		const nodes = [makeNode("a"), makeNode("b")];
		expect(countFieldValues(nodes, "status")).toEqual({});
	});
});
