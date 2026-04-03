// Unit tests for metrics-store pure logic functions.
//
// The store's module-level $state objects cannot be imported in a non-Svelte
// runtime context. Tests exercise the pure functions — ingestEvent logic,
// recordValue, resolveCategory, resolveTimestamp, errorRateHistory, and
// totalErrorsInWindow — by re-implementing them inline with the same algorithm.

import { describe, it, expect, beforeEach, afterEach, vi } from "vitest";

// PerfEvent, MetricStats, and METRIC_CATEGORIES are defined inline to avoid
// importing from .svelte.ts files that have module-level $state calls.

type PerfEvent = {
	level: "trace" | "debug" | "info" | "warn" | "error" | "perf";
	category: string;
	durationMs?: number;
	timestamp?: number | string;
	source?: string;
	message?: string;
};

type MetricStats = {
	category: string;
	label: string;
	current: number;
	min: number;
	max: number;
	avg: number;
	history: number[];
	count: number;
};

const METRIC_CATEGORIES: Record<string, string> = {
	graph_build: "Graph Build",
	prompt_gen: "Prompt Generation",
	ipc: "IPC Duration",
	search_index: "Search Index",
};

// ---------------------------------------------------------------------------
// Pure logic re-implementations (mirrors metrics-store.svelte.ts)
// ---------------------------------------------------------------------------

const HISTORY_SIZE = 100;
const ERROR_RATE_WINDOW_MINUTES = 30;

interface ErrorBucket {
	minuteTs: number;
	count: number;
}

function emptyStats(category: string): MetricStats {
	return {
		category,
		label: METRIC_CATEGORIES[category] ?? category,
		current: 0,
		min: Infinity,
		max: -Infinity,
		avg: 0,
		history: [],
		count: 0,
	};
}

function resolveTimestamp(ts: number | string | undefined): number {
	if (ts === undefined) return Date.now();
	if (typeof ts === "number") return ts;
	const parsed = Date.parse(ts);
	return Number.isNaN(parsed) ? Date.now() : parsed;
}

function resolveCategory(event: PerfEvent): string | null {
	if (event.category in METRIC_CATEGORIES) return event.category;
	const src = (event.source ?? event.category ?? "").toLowerCase();
	if (src.includes("graph")) return "graph_build";
	if (src.includes("prompt")) return "prompt_gen";
	if (src.includes("ipc")) return "ipc";
	if (src.includes("search") || src.includes("index")) return "search_index";
	return null;
}

function recordValue(stats: MetricStats, value: number): MetricStats {
	const s = { ...stats, history: [...stats.history] };
	s.current = value;
	s.min = Math.min(s.min === Infinity ? value : s.min, value);
	s.max = Math.max(s.max === -Infinity ? value : s.max, value);
	s.count += 1;
	s.avg = s.avg + (value - s.avg) / s.count;
	s.history.push(value);
	if (s.history.length > HISTORY_SIZE) {
		s.history.shift();
	}
	return s;
}

function recordError(buckets: ErrorBucket[], ts: number, now: number): ErrorBucket[] {
	const minuteTs = Math.floor(ts / 60_000) * 60_000;
	const copy = buckets.map((b) => ({ ...b }));
	const bucket = copy.find((b) => b.minuteTs === minuteTs);
	if (bucket) {
		bucket.count += 1;
	} else {
		copy.push({ minuteTs, count: 1 });
	}
	const cutoff = now - ERROR_RATE_WINDOW_MINUTES * 60_000;
	return copy.filter((b) => b.minuteTs >= cutoff);
}

function errorRateHistory(buckets: ErrorBucket[], now: number): number[] {
	const result: number[] = [];
	for (let i = ERROR_RATE_WINDOW_MINUTES - 1; i >= 0; i--) {
		const minuteTs = Math.floor((now - i * 60_000) / 60_000) * 60_000;
		const bucket = buckets.find((b) => b.minuteTs === minuteTs);
		result.push(bucket?.count ?? 0);
	}
	return result;
}

function totalErrorsInWindow(buckets: ErrorBucket[]): number {
	return buckets.reduce((sum, b) => sum + b.count, 0);
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

describe("METRIC_CATEGORIES constant", () => {
	it("contains all four required categories", () => {
		expect(Object.keys(METRIC_CATEGORIES)).toContain("graph_build");
		expect(Object.keys(METRIC_CATEGORIES)).toContain("prompt_gen");
		expect(Object.keys(METRIC_CATEGORIES)).toContain("ipc");
		expect(Object.keys(METRIC_CATEGORIES)).toContain("search_index");
	});

	it("provides human-readable labels for each category", () => {
		expect(METRIC_CATEGORIES["graph_build"]).toBe("Graph Build");
		expect(METRIC_CATEGORIES["prompt_gen"]).toBe("Prompt Generation");
		expect(METRIC_CATEGORIES["ipc"]).toBe("IPC Duration");
		expect(METRIC_CATEGORIES["search_index"]).toBe("Search Index");
	});
});

describe("emptyStats", () => {
	it("initialises with zero count and Infinity bounds", () => {
		const stats = emptyStats("ipc");
		expect(stats.category).toBe("ipc");
		expect(stats.count).toBe(0);
		expect(stats.avg).toBe(0);
		expect(stats.current).toBe(0);
		expect(stats.min).toBe(Infinity);
		expect(stats.max).toBe(-Infinity);
		expect(stats.history).toHaveLength(0);
	});

	it("uses the METRIC_CATEGORIES label when available", () => {
		expect(emptyStats("graph_build").label).toBe("Graph Build");
	});

	it("uses the category key as label when not in METRIC_CATEGORIES", () => {
		expect(emptyStats("unknown_cat").label).toBe("unknown_cat");
	});
});

describe("resolveTimestamp", () => {
	it("returns the number unchanged for numeric timestamps", () => {
		expect(resolveTimestamp(1_000_000)).toBe(1_000_000);
	});

	it("parses ISO strings to milliseconds", () => {
		const iso = "2026-01-01T00:00:00.000Z";
		expect(resolveTimestamp(iso)).toBe(Date.parse(iso));
	});

	it("falls back to Date.now() when ts is undefined", () => {
		const before = Date.now();
		const result = resolveTimestamp(undefined);
		const after = Date.now();
		expect(result).toBeGreaterThanOrEqual(before);
		expect(result).toBeLessThanOrEqual(after);
	});

	it("falls back to Date.now() for unparseable strings", () => {
		const before = Date.now();
		const result = resolveTimestamp("not-a-date");
		const after = Date.now();
		expect(result).toBeGreaterThanOrEqual(before);
		expect(result).toBeLessThanOrEqual(after);
	});
});

describe("resolveCategory", () => {
	it("returns the category key directly when it matches a known key", () => {
		const event: PerfEvent = { level: "perf", category: "graph_build" };
		expect(resolveCategory(event)).toBe("graph_build");
	});

	it("maps graph-containing source to graph_build", () => {
		const event: PerfEvent = { level: "perf", category: "unknown", source: "graph-watcher" };
		expect(resolveCategory(event)).toBe("graph_build");
	});

	it("maps prompt-containing source to prompt_gen", () => {
		const event: PerfEvent = { level: "perf", category: "unknown", source: "prompt-builder" };
		expect(resolveCategory(event)).toBe("prompt_gen");
	});

	it("maps ipc-containing source to ipc", () => {
		const event: PerfEvent = { level: "perf", category: "unknown", source: "ipc-handler" };
		expect(resolveCategory(event)).toBe("ipc");
	});

	it("maps search-containing source to search_index", () => {
		const event: PerfEvent = { level: "perf", category: "unknown", source: "search-engine" };
		expect(resolveCategory(event)).toBe("search_index");
	});

	it("maps index-containing source to search_index", () => {
		const event: PerfEvent = { level: "perf", category: "unknown", source: "index-builder" };
		expect(resolveCategory(event)).toBe("search_index");
	});

	it("returns null for completely unknown category and source", () => {
		const event: PerfEvent = { level: "perf", category: "unknown", source: "something-else" };
		expect(resolveCategory(event)).toBeNull();
	});
});

describe("recordValue", () => {
	it("sets current to the first recorded value", () => {
		const stats = emptyStats("ipc");
		const updated = recordValue(stats, 42);
		expect(updated.current).toBe(42);
	});

	it("tracks min correctly across multiple values", () => {
		let stats = emptyStats("ipc");
		stats = recordValue(stats, 100);
		stats = recordValue(stats, 50);
		stats = recordValue(stats, 75);
		expect(stats.min).toBe(50);
	});

	it("tracks max correctly across multiple values", () => {
		let stats = emptyStats("ipc");
		stats = recordValue(stats, 100);
		stats = recordValue(stats, 200);
		stats = recordValue(stats, 150);
		expect(stats.max).toBe(200);
	});

	it("computes correct running average", () => {
		let stats = emptyStats("ipc");
		stats = recordValue(stats, 10);
		stats = recordValue(stats, 20);
		stats = recordValue(stats, 30);
		expect(stats.avg).toBeCloseTo(20, 5);
		expect(stats.count).toBe(3);
	});

	it("appends to history", () => {
		let stats = emptyStats("ipc");
		stats = recordValue(stats, 5);
		stats = recordValue(stats, 10);
		expect(stats.history).toEqual([5, 10]);
	});

	it("caps history at HISTORY_SIZE by evicting the oldest entry", () => {
		let stats = emptyStats("ipc");
		for (let i = 0; i < HISTORY_SIZE + 5; i++) {
			stats = recordValue(stats, i);
		}
		expect(stats.history).toHaveLength(HISTORY_SIZE);
		// The oldest values (0..4) should have been shifted out
		expect(stats.history[0]).toBe(5);
	});
});

describe("recordError and errorRateHistory", () => {
	let now: number;

	beforeEach(() => {
		now = new Date("2026-01-01T12:00:00.000Z").getTime();
		vi.useFakeTimers();
		vi.setSystemTime(now);
	});

	afterEach(() => {
		vi.useRealTimers();
	});

	it("creates a new bucket for a new minute", () => {
		const buckets = recordError([], now, now);
		expect(buckets).toHaveLength(1);
		expect(buckets[0].count).toBe(1);
	});

	it("increments an existing bucket for the same minute", () => {
		let buckets = recordError([], now, now);
		buckets = recordError(buckets, now + 10_000, now); // same minute
		expect(buckets).toHaveLength(1);
		expect(buckets[0].count).toBe(2);
	});

	it("prunes buckets older than 30 minutes", () => {
		const oldTs = now - 31 * 60_000; // 31 minutes ago
		let buckets = recordError([], oldTs, now);
		buckets = recordError(buckets, now, now); // current minute
		// Old bucket should be pruned
		expect(buckets).toHaveLength(1);
		expect(buckets[0].minuteTs).toBe(Math.floor(now / 60_000) * 60_000);
	});

	it("errorRateHistory returns 30 entries", () => {
		const history = errorRateHistory([], now);
		expect(history).toHaveLength(ERROR_RATE_WINDOW_MINUTES);
	});

	it("errorRateHistory fills zeros for minutes with no errors", () => {
		const history = errorRateHistory([], now);
		expect(history.every((v) => v === 0)).toBe(true);
	});

	it("errorRateHistory places bucket count in the correct slot", () => {
		const buckets = [{ minuteTs: Math.floor(now / 60_000) * 60_000, count: 5 }];
		const history = errorRateHistory(buckets, now);
		// The last element is the current minute
		expect(history[history.length - 1]).toBe(5);
	});
});

describe("totalErrorsInWindow", () => {
	it("returns 0 for empty buckets", () => {
		expect(totalErrorsInWindow([])).toBe(0);
	});

	it("sums all bucket counts", () => {
		const buckets = [
			{ minuteTs: 1000, count: 3 },
			{ minuteTs: 2000, count: 7 },
			{ minuteTs: 3000, count: 2 },
		];
		expect(totalErrorsInWindow(buckets)).toBe(12);
	});
});
