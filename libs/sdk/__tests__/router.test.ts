/**
 * Tests for the hash-based router: parseHash and buildHash.
 * Covers all route types, edge cases, and round-trip fidelity.
 */
import { describe, it, expect } from "vitest";
import { parseHash, buildHash } from "../src/router.js";
import type { ParsedRoute } from "../src/router.js";

describe("parseHash", () => {
	it("returns default route for empty string", () => {
		expect(parseHash("")).toEqual({ type: "default" });
	});

	it("returns default route for bare '#'", () => {
		expect(parseHash("#")).toEqual({ type: "default" });
	});

	it("returns default route for '#/'", () => {
		expect(parseHash("#/")).toEqual({ type: "default" });
	});

	it("parses #/project", () => {
		expect(parseHash("#/project")).toEqual({ type: "project" });
	});

	it("parses #/settings", () => {
		expect(parseHash("#/settings")).toEqual({ type: "settings" });
	});

	it("parses #/graph", () => {
		expect(parseHash("#/graph")).toEqual({ type: "graph" });
	});

	it("parses #/setup", () => {
		expect(parseHash("#/setup")).toEqual({ type: "setup" });
	});

	it("parses #/artifacts without activity", () => {
		expect(parseHash("#/artifacts")).toEqual({ type: "artifacts" });
	});

	it("parses #/artifacts/:activity", () => {
		expect(parseHash("#/artifacts/roadmap")).toEqual({ type: "artifacts", activity: "roadmap" });
		expect(parseHash("#/artifacts/lessons")).toEqual({ type: "artifacts", activity: "lessons" });
	});

	it("parses #/artifacts/:activity/:path — artifact viewer route", () => {
		const result = parseHash("#/artifacts/explorer/.orqa/implementation/tasks/TASK-001.md");
		expect(result).toEqual({
			type: "artifact",
			activity: "explorer",
			artifactPath: ".orqa/implementation/tasks/TASK-001.md",
		});
	});

	it("parses a plugin route with simple plugin name", () => {
		const result = parseHash("#/plugin/my-plugin/dashboard");
		expect(result).toEqual({ type: "plugin", pluginName: "my-plugin", viewKey: "dashboard" });
	});

	it("parses a scoped plugin route (@ prefix with org)", () => {
		const result = parseHash("#/plugin/@orqastudio/plugin-software-kanban/roadmap");
		expect(result).toEqual({
			type: "plugin",
			pluginName: "@orqastudio/plugin-software-kanban",
			viewKey: "roadmap",
		});
	});

	it("treats unknown top-level segment as activity (backwards compat)", () => {
		const result = parseHash("#/ideas");
		expect(result).toEqual({ type: "artifacts", activity: "ideas" });
	});

	it("handles artifact path with backslash separator", () => {
		const result = parseHash("#/artifacts/explorer/.orqa\\tasks\\TASK-002.md");
		expect(result).toEqual({
			type: "artifact",
			activity: "explorer",
			artifactPath: ".orqa\\tasks\\TASK-002.md",
		});
	});

	it("handles hash without leading # character", () => {
		// Input without # still parses correctly (hash.replace strips only ^#\/?)
		expect(parseHash("project")).toEqual({ type: "project" });
	});
});

describe("buildHash", () => {
	it("builds #/ for default route", () => {
		expect(buildHash({ type: "default" })).toBe("#/");
	});

	it("builds #/project", () => {
		expect(buildHash({ type: "project" })).toBe("#/project");
	});

	it("builds #/settings", () => {
		expect(buildHash({ type: "settings" })).toBe("#/settings");
	});

	it("builds #/graph", () => {
		expect(buildHash({ type: "graph" })).toBe("#/graph");
	});

	it("builds #/setup", () => {
		expect(buildHash({ type: "setup" })).toBe("#/setup");
	});

	it("builds #/artifacts without activity", () => {
		expect(buildHash({ type: "artifacts" })).toBe("#/artifacts");
	});

	it("builds #/artifacts/:activity", () => {
		expect(buildHash({ type: "artifacts", activity: "roadmap" })).toBe("#/artifacts/roadmap");
	});

	it("builds artifact viewer route", () => {
		expect(
			buildHash({
				type: "artifact",
				activity: "explorer",
				artifactPath: ".orqa/implementation/tasks/TASK-001.md",
			}),
		).toBe("#/artifacts/explorer/.orqa/implementation/tasks/TASK-001.md");
	});

	it("builds plugin route with simple name", () => {
		expect(buildHash({ type: "plugin", pluginName: "my-plugin", viewKey: "dashboard" })).toBe(
			"#/plugin/my-plugin/dashboard",
		);
	});

	it("builds plugin route with scoped package name", () => {
		expect(
			buildHash({
				type: "plugin",
				pluginName: "@orqastudio/plugin-software-kanban",
				viewKey: "roadmap",
			}),
		).toBe("#/plugin/@orqastudio/plugin-software-kanban/roadmap");
	});
});

describe("parseHash / buildHash round-trip", () => {
	const roundTripCases: ParsedRoute[] = [
		{ type: "default" },
		{ type: "project" },
		{ type: "settings" },
		{ type: "graph" },
		{ type: "setup" },
		{ type: "artifacts" },
		{ type: "artifacts", activity: "roadmap" },
		{ type: "artifact", activity: "explorer", artifactPath: ".orqa/tasks/TASK-001.md" },
		{ type: "plugin", pluginName: "@orqastudio/plugin-kanban", viewKey: "board" },
	];

	for (const route of roundTripCases) {
		it(`round-trips ${route.type}${route.activity ? `/${route.activity}` : ""}`, () => {
			const hash = buildHash(route);
			const reparsed = parseHash(hash);
			expect(reparsed).toEqual(route);
		});
	}
});
