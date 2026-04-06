// Metrics store for OrqaDev. Aggregates perf-level events from the log store
// into per-category running statistics (current, min, max, avg) and maintains
// a window of the last 100 values for sparkline rendering.
// Also tracks error rate over rolling 30-minute windows in one-minute buckets.

// Shape of a single event received from the daemon event bus.
export interface PerfEvent {
	readonly level: "trace" | "debug" | "info" | "warn" | "error" | "perf";
	readonly category: string;
	// Duration in milliseconds for perf events.
	readonly durationMs?: number;
	// Timestamp in milliseconds since epoch (or ISO string — we normalise to ms).
	readonly timestamp?: number | string;
	// Optional source label, e.g. "ipc" or "search".
	readonly source?: string;
	readonly message?: string;
}

// Aggregated stats for one metric category. The identity fields (category, label)
// are readonly; the numeric accumulators are intentionally mutable — recordValue()
// updates them in-place on $state data for Svelte reactivity.
export interface MetricStats {
	// Category key, e.g. "graph_build" or "ipc".
	readonly category: string;
	// Human-readable display label.
	readonly label: string;
	// Most recent recorded value in ms.
	current: number;
	min: number;
	max: number;
	// Rolling average across all observed values.
	avg: number;
	// Up to the last 100 values, newest last.
	history: number[];
	// Sample count.
	count: number;
}

// Error-rate bucket: one entry per minute.
interface ErrorBucket {
	// Minute-level epoch time (floor(Date.now() / 60_000) * 60_000).
	minuteTs: number;
	count: number;
}

// The four required metric categories with their display labels.
export const METRIC_CATEGORIES: Record<string, string> = {
	graph_build: "Graph Build",
	prompt_gen: "Prompt Generation",
	ipc: "IPC Duration",
	search_index: "Search Index",
};

// Maximum number of recent values kept per category for sparklines.
const HISTORY_SIZE = 100;

// Window size for error rate tracking (30 minutes → 30 buckets).
const ERROR_RATE_WINDOW_MINUTES = 30;

// Initialise an empty MetricStats entry for a category.
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

// Resolve the millisecond timestamp from a raw event timestamp field.
function resolveTimestamp(ts: number | string | undefined): number {
	if (ts === undefined) return Date.now();
	if (typeof ts === "number") return ts;
	const parsed = Date.parse(ts);
	return Number.isNaN(parsed) ? Date.now() : parsed;
}

// Derive the category key from a PerfEvent. Uses the event's own category
// field if it matches a known key; falls back to source-based heuristics.
function resolveCategory(event: PerfEvent): string | null {
	if (event.category in METRIC_CATEGORIES) return event.category;
	// Heuristic mapping from common source/message patterns.
	const src = (event.source ?? event.category ?? "").toLowerCase();
	if (src.includes("graph")) return "graph_build";
	if (src.includes("prompt")) return "prompt_gen";
	if (src.includes("ipc")) return "ipc";
	if (src.includes("search") || src.includes("index")) return "search_index";
	return null;
}

// Module-level reactive state, consumed directly by components.
export const metrics = $state<{
	// Per-category stats, keyed by category ID.
	byCategory: Record<string, MetricStats>;
	// Error-rate buckets covering the last 30 minutes.
	errorBuckets: ErrorBucket[];
	// Total event count processed.
	totalEvents: number;
}>({
	byCategory: Object.fromEntries(Object.keys(METRIC_CATEGORIES).map((k) => [k, emptyStats(k)])),
	errorBuckets: [],
	totalEvents: 0,
});

// Append a value to a MetricStats entry, maintaining the rolling history window
// and recomputing running statistics.
function recordValue(stats: MetricStats, value: number): void {
	stats.current = value;
	stats.min = Math.min(stats.min === Infinity ? value : stats.min, value);
	stats.max = Math.max(stats.max === -Infinity ? value : stats.max, value);
	stats.count += 1;
	// Incrementally update average: avg_n = avg_{n-1} + (value - avg_{n-1}) / n
	stats.avg = stats.avg + (value - stats.avg) / stats.count;
	stats.history.push(value);
	if (stats.history.length > HISTORY_SIZE) {
		stats.history.shift();
	}
}

// Record an error event into the rolling 30-minute bucket window.
function recordError(ts: number): void {
	const minuteTs = Math.floor(ts / 60_000) * 60_000;
	const bucket = metrics.errorBuckets.find((b) => b.minuteTs === minuteTs);
	if (bucket) {
		bucket.count += 1;
	} else {
		metrics.errorBuckets.push({ minuteTs, count: 1 });
	}
	// Prune buckets older than 30 minutes.
	const cutoff = Date.now() - ERROR_RATE_WINDOW_MINUTES * 60_000;
	metrics.errorBuckets = metrics.errorBuckets.filter((b) => b.minuteTs >= cutoff);
}

/**
 * Ingest a single event from any source (IPC listener, mock injector, etc.).
 * Components call this directly; the log store will call it when wired up.
 * @param event - the event to ingest
 */
export function ingestEvent(event: PerfEvent): void {
	metrics.totalEvents += 1;

	if (event.level === "error" || event.level === "warn") {
		recordError(resolveTimestamp(event.timestamp));
	}

	if (event.level !== "perf") return;
	if (event.durationMs === undefined || event.durationMs === null) return;

	const cat = resolveCategory(event);
	if (!cat) return;

	if (!(cat in metrics.byCategory)) {
		metrics.byCategory[cat] = emptyStats(cat);
	}
	recordValue(metrics.byCategory[cat], event.durationMs);
}

/**
 * Derive errors-per-minute values for the last 30 minutes, filling zero-gaps
 * so the sparkline always has a continuous series.
 * @returns array of 30 error counts, one per minute, oldest first
 */
export function errorRateHistory(): number[] {
	const now = Date.now();
	const result: number[] = [];
	for (let i = ERROR_RATE_WINDOW_MINUTES - 1; i >= 0; i--) {
		const minuteTs = Math.floor((now - i * 60_000) / 60_000) * 60_000;
		const bucket = metrics.errorBuckets.find((b) => b.minuteTs === minuteTs);
		result.push(bucket?.count ?? 0);
	}
	return result;
}

/**
 * Total errors in the rolling 30-minute window.
 * @returns sum of all error-bucket counts within the last 30 minutes
 */
export function totalErrorsInWindow(): number {
	return metrics.errorBuckets.reduce((sum, b) => sum + b.count, 0);
}
