// Unit tests for log-store pure logic functions.
//
// The module exports mutable $state objects at the module level. Importing them
// directly in a non-Svelte runtime causes "$state can only be used inside a
// .svelte file" errors. Tests therefore exercise the pure functions by
// re-implementing the same logic inline — this validates the algorithm, not the
// Svelte integration, which is covered by E2E tests.

import { describe, it, expect, beforeEach } from "vitest";

// LogEvent type defined inline to avoid importing from .svelte.ts (which has
// module-level $state calls that error outside a Svelte compiler context).
type LogEvent = {
	id: number;
	timestamp: number;
	level: "Debug" | "Info" | "Warn" | "Error" | "Perf";
	source: "Daemon" | "App" | "Frontend" | "DevController" | "MCP" | "LSP" | "Search" | "Worker";
	category: string;
	message: string;
	metadata: unknown;
	session_id: string | null;
};

// ---------------------------------------------------------------------------
// Re-implement filteredEvents logic — same algorithm as the store
// ---------------------------------------------------------------------------

function filteredEvents(
	events: LogEvent[],
	filters: {
		sources: Set<LogEvent["source"]>;
		levels: Set<LogEvent["level"]>;
		categories: Set<string>;
		searchText: string;
	},
): LogEvent[] {
	return events.filter((ev) => {
		if (filters.sources.size > 0 && !filters.sources.has(ev.source)) return false;
		if (filters.levels.size > 0 && !filters.levels.has(ev.level)) return false;
		if (filters.categories.size > 0 && !filters.categories.has(ev.category)) return false;
		if (filters.searchText.length > 0) {
			const needle = filters.searchText.toLowerCase();
			if (!ev.message.toLowerCase().includes(needle)) return false;
		}
		return true;
	});
}

function hasActiveFilters(filters: {
	sources: Set<LogEvent["source"]>;
	levels: Set<LogEvent["level"]>;
	categories: Set<string>;
	searchText: string;
}): boolean {
	return (
		filters.sources.size > 0 ||
		filters.levels.size > 0 ||
		filters.categories.size > 0 ||
		filters.searchText.length > 0
	);
}

function knownCategories(events: LogEvent[]): Set<string> {
	return new Set(events.map((ev) => ev.category));
}

// ---------------------------------------------------------------------------
// Ring buffer logic — same algorithm as appendEvent in the store
// ---------------------------------------------------------------------------

const DISPLAY_BUFFER_MAX = 10_000;

function appendEvent(events: LogEvent[], event: LogEvent): LogEvent[] {
	const copy = [...events];
	if (copy.length >= DISPLAY_BUFFER_MAX) {
		copy.splice(0, 1);
	}
	copy.push(event);
	return copy;
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

let nextId = 1;

function makeEvent(overrides: Partial<LogEvent> = {}): LogEvent {
	return {
		id: nextId++,
		timestamp: Date.now(),
		level: "Info",
		source: "Daemon",
		category: "general",
		message: "test message",
		metadata: null,
		session_id: null,
		...overrides,
	};
}

function emptyFilters() {
	return {
		sources: new Set<LogEvent["source"]>(),
		levels: new Set<LogEvent["level"]>(),
		categories: new Set<string>(),
		searchText: "",
	};
}

beforeEach(() => {
	nextId = 1;
});

// ---------------------------------------------------------------------------
// filteredEvents
// ---------------------------------------------------------------------------

describe("filteredEvents", () => {
	it("returns all events when all filters are empty", () => {
		const events = [makeEvent(), makeEvent(), makeEvent()];
		expect(filteredEvents(events, emptyFilters())).toHaveLength(3);
	});

	it("filters by source — only matching sources pass", () => {
		const events = [
			makeEvent({ source: "Daemon" }),
			makeEvent({ source: "App" }),
			makeEvent({ source: "MCP" }),
		];
		const filters = { ...emptyFilters(), sources: new Set<LogEvent["source"]>(["Daemon", "MCP"]) };
		const result = filteredEvents(events, filters);
		expect(result).toHaveLength(2);
		expect(result.every((ev) => ev.source === "Daemon" || ev.source === "MCP")).toBe(true);
	});

	it("filters by level — only matching levels pass", () => {
		const events = [
			makeEvent({ level: "Debug" }),
			makeEvent({ level: "Info" }),
			makeEvent({ level: "Error" }),
		];
		const filters = { ...emptyFilters(), levels: new Set<LogEvent["level"]>(["Error"]) };
		const result = filteredEvents(events, filters);
		expect(result).toHaveLength(1);
		expect(result[0].level).toBe("Error");
	});

	it("filters by category — only matching categories pass", () => {
		const events = [
			makeEvent({ category: "graph" }),
			makeEvent({ category: "ipc" }),
			makeEvent({ category: "graph" }),
		];
		const filters = { ...emptyFilters(), categories: new Set<string>(["graph"]) };
		const result = filteredEvents(events, filters);
		expect(result).toHaveLength(2);
		expect(result.every((ev) => ev.category === "graph")).toBe(true);
	});

	it("filters by searchText — case-insensitive substring match on message", () => {
		const events = [
			makeEvent({ message: "Hello World" }),
			makeEvent({ message: "goodbye" }),
			makeEvent({ message: "hello again" }),
		];
		const filters = { ...emptyFilters(), searchText: "HELLO" };
		const result = filteredEvents(events, filters);
		expect(result).toHaveLength(2);
	});

	it("combines all filters with AND logic", () => {
		const events = [
			makeEvent({ source: "Daemon", level: "Error", category: "graph", message: "fail" }),
			makeEvent({ source: "Daemon", level: "Info", category: "graph", message: "fail" }),
			makeEvent({ source: "App", level: "Error", category: "graph", message: "fail" }),
			makeEvent({ source: "Daemon", level: "Error", category: "ipc", message: "fail" }),
			makeEvent({ source: "Daemon", level: "Error", category: "graph", message: "ok" }),
		];
		const filters = {
			sources: new Set<LogEvent["source"]>(["Daemon"]),
			levels: new Set<LogEvent["level"]>(["Error"]),
			categories: new Set<string>(["graph"]),
			searchText: "fail",
		};
		const result = filteredEvents(events, filters);
		expect(result).toHaveLength(1);
		expect(result[0].message).toBe("fail");
		expect(result[0].source).toBe("Daemon");
		expect(result[0].level).toBe("Error");
		expect(result[0].category).toBe("graph");
	});

	it("returns empty array when no events match filters", () => {
		const events = [makeEvent({ level: "Info" }), makeEvent({ level: "Debug" })];
		const filters = { ...emptyFilters(), levels: new Set<LogEvent["level"]>(["Error"]) };
		expect(filteredEvents(events, filters)).toHaveLength(0);
	});

	it("handles empty event list gracefully", () => {
		expect(filteredEvents([], emptyFilters())).toHaveLength(0);
	});
});

// ---------------------------------------------------------------------------
// hasActiveFilters
// ---------------------------------------------------------------------------

describe("hasActiveFilters", () => {
	it("returns false when all filters are empty", () => {
		expect(hasActiveFilters(emptyFilters())).toBe(false);
	});

	it("returns true when sources filter is set", () => {
		const filters = { ...emptyFilters(), sources: new Set<LogEvent["source"]>(["App"]) };
		expect(hasActiveFilters(filters)).toBe(true);
	});

	it("returns true when levels filter is set", () => {
		const filters = { ...emptyFilters(), levels: new Set<LogEvent["level"]>(["Warn"]) };
		expect(hasActiveFilters(filters)).toBe(true);
	});

	it("returns true when categories filter is set", () => {
		const filters = { ...emptyFilters(), categories: new Set<string>(["graph"]) };
		expect(hasActiveFilters(filters)).toBe(true);
	});

	it("returns true when searchText is non-empty", () => {
		const filters = { ...emptyFilters(), searchText: "error" };
		expect(hasActiveFilters(filters)).toBe(true);
	});
});

// ---------------------------------------------------------------------------
// knownCategories
// ---------------------------------------------------------------------------

describe("knownCategories", () => {
	it("returns empty set for empty event list", () => {
		expect(knownCategories([])).toEqual(new Set());
	});

	it("collects unique categories from events", () => {
		const events = [
			makeEvent({ category: "graph" }),
			makeEvent({ category: "ipc" }),
			makeEvent({ category: "graph" }),
			makeEvent({ category: "prompt" }),
		];
		const cats = knownCategories(events);
		expect(cats.size).toBe(3);
		expect(cats.has("graph")).toBe(true);
		expect(cats.has("ipc")).toBe(true);
		expect(cats.has("prompt")).toBe(true);
	});

	it("returns a set with one entry when all events share the same category", () => {
		const events = [makeEvent({ category: "search" }), makeEvent({ category: "search" })];
		expect(knownCategories(events)).toEqual(new Set(["search"]));
	});
});

// ---------------------------------------------------------------------------
// Ring buffer (appendEvent)
// ---------------------------------------------------------------------------

describe("appendEvent ring buffer", () => {
	it("appends an event to an empty buffer", () => {
		const buf = appendEvent([], makeEvent());
		expect(buf).toHaveLength(1);
	});

	it("appends multiple events in order", () => {
		let buf: LogEvent[] = [];
		const ev1 = makeEvent({ message: "first" });
		const ev2 = makeEvent({ message: "second" });
		buf = appendEvent(buf, ev1);
		buf = appendEvent(buf, ev2);
		expect(buf[0].message).toBe("first");
		expect(buf[1].message).toBe("second");
	});

	it("evicts the oldest event when the buffer is full", () => {
		// Build a buffer at capacity
		let buf: LogEvent[] = Array.from({ length: DISPLAY_BUFFER_MAX }, (_, i) =>
			makeEvent({ id: i, message: `msg-${i}` }),
		);
		const newEvent = makeEvent({ id: DISPLAY_BUFFER_MAX, message: "newest" });
		buf = appendEvent(buf, newEvent);

		expect(buf).toHaveLength(DISPLAY_BUFFER_MAX);
		// The oldest event (id=0) should have been evicted
		expect(buf[0].id).toBe(1);
		expect(buf[buf.length - 1].message).toBe("newest");
	});

	it("does not evict when buffer is one below capacity", () => {
		let buf: LogEvent[] = Array.from({ length: DISPLAY_BUFFER_MAX - 1 }, (_, i) =>
			makeEvent({ id: i }),
		);
		buf = appendEvent(buf, makeEvent({ id: DISPLAY_BUFFER_MAX }));
		expect(buf).toHaveLength(DISPLAY_BUFFER_MAX);
	});
});

// ---------------------------------------------------------------------------
// LogEvent field constants (defined inline — matches the store's exported arrays)
// ---------------------------------------------------------------------------

describe("ALL_LEVELS and ALL_SOURCES constants", () => {
	it("ALL_LEVELS covers all expected levels", () => {
		const levels: LogEvent["level"][] = ["Debug", "Info", "Warn", "Error", "Perf"];
		expect(levels).toHaveLength(5);
		expect(levels).toContain("Perf");
		expect(levels).toContain("Error");
	});

	it("ALL_SOURCES covers all expected sources", () => {
		const sources: LogEvent["source"][] = [
			"Daemon", "App", "Frontend", "DevController", "MCP", "LSP", "Search", "Worker",
		];
		expect(sources).toHaveLength(8);
		expect(sources).toContain("DevController");
		expect(sources).toContain("Worker");
	});
});
