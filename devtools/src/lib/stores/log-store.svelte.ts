// Log event store for OrqaDev. Subscribes to Tauri `orqa://log-event` events
// and maintains a bounded display buffer of log entries for the UI. Exposes
// reactive state for the event list, connection status, scroll-lock toggle,
// and filter state for source/level/category/text filtering. Filter state is
// persisted to localStorage so selections survive app restarts.
//
// Historical mode: when viewingHistorical.value (from session-store) is true,
// events are loaded from SQLite via query_session_events instead of the live
// ring buffer. Switching modes clears the buffer and re-populates it from the
// appropriate source. The Tauri event listener is paused while viewing history.

import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import {
	viewingHistorical,
	activeSessionId,
	loadSessionEvents,
	type DevToolsSession,
} from "./session-store.svelte.js";

// Shape of a log event as emitted by the Tauri backend.
export interface LogEvent {
	readonly id: number;
	readonly timestamp: number; // Unix ms
	readonly level: "Debug" | "Info" | "Warn" | "Error" | "Perf";
	readonly source:
		| "Daemon"
		| "App"
		| "Frontend"
		| "DevController"
		| "MCP"
		| "LSP"
		| "Search"
		| "Worker";
	readonly category: string;
	readonly message: string;
	readonly metadata: unknown;
	readonly session_id: string | null;
}

// All valid log levels in display order.
export const ALL_LEVELS: LogEvent["level"][] = ["Debug", "Info", "Warn", "Error", "Perf"];

// All valid log sources in display order.
export const ALL_SOURCES: LogEvent["source"][] = [
	"Daemon",
	"App",
	"Frontend",
	"DevController",
	"MCP",
	"LSP",
	"Search",
	"Worker",
];

// Maximum number of events held in the display buffer. Events beyond this
// limit are evicted from the front (oldest-first) to keep memory bounded.
const DISPLAY_BUFFER_MAX = 10_000;

// The Tauri event name emitted by the backend for each new log entry.
const TAURI_LOG_EVENT = "orqa://log-event";

// Reactive event buffer. Components read this array; the store appends to it.
export const events = $state<LogEvent[]>([]);

// Connection status to the daemon event stream. The store sets this to
// "connected" after the first event arrives and "disconnected" on unlisten.
export const connectionStatus = $state<{
	value: "connecting" | "connected" | "disconnected";
}>({ value: "connecting" });

// Total number of events received this session (monotonically increasing,
// not reset when the buffer is trimmed).
export const totalReceived = $state<{ value: number }>({ value: 0 });

// Whether auto-scroll to the bottom is active. The table toggles this off
// when the user scrolls up and back on when they reach the bottom again.
export const scrollLock = $state<{ enabled: boolean }>({ enabled: true });

// localStorage key for persisted filter state.
const STORAGE_KEY_FILTERS = "orqadev:logFilters";

// Shape of the serialisable filter snapshot written to localStorage.
// Sets cannot be JSON-serialised directly, so we use arrays.
interface PersistedFilters {
	sources: LogEvent["source"][];
	levels: LogEvent["level"][];
	categories: string[];
	searchText: string;
}

// Restore filter state from localStorage. Returns defaults when nothing is
// stored or the stored value cannot be parsed.
function loadPersistedFilters(): {
	sources: Set<LogEvent["source"]>;
	levels: Set<LogEvent["level"]>;
	categories: Set<string>;
	searchText: string;
} {
	try {
		const raw = localStorage.getItem(STORAGE_KEY_FILTERS);
		if (raw) {
			const parsed: PersistedFilters = JSON.parse(raw);
			return {
				sources: new Set(parsed.sources ?? []),
				levels: new Set(parsed.levels ?? []),
				categories: new Set(parsed.categories ?? []),
				searchText: typeof parsed.searchText === "string" ? parsed.searchText : "",
			};
		}
	} catch {
		// Corrupted or unavailable — silently fall through to defaults.
	}
	return { sources: new Set(), levels: new Set(), categories: new Set(), searchText: "" };
}

// Write current filter state to localStorage.
function persistFilters(): void {
	try {
		const snapshot: PersistedFilters = {
			sources: [...filters.sources],
			levels: [...filters.levels],
			categories: [...filters.categories],
			searchText: filters.searchText,
		};
		localStorage.setItem(STORAGE_KEY_FILTERS, JSON.stringify(snapshot));
	} catch {
		// Ignore write failures — persistence is best-effort.
	}
}

// Active filter state. Empty sets mean "show all" for that dimension.
// searchText is applied as a case-insensitive substring match on message.
// Initialised from localStorage so the last-used filters are restored on startup.
export const filters = $state<{
	sources: Set<LogEvent["source"]>;
	levels: Set<LogEvent["level"]>;
	categories: Set<string>;
	searchText: string;
}>(loadPersistedFilters());

// $effect watches filter state and writes to localStorage on every change.
$effect.root(() => {
	$effect(() => {
		// Access all filter fields to establish reactive dependencies.
		filters.sources;
		filters.levels;
		filters.categories;
		filters.searchText;
		persistFilters();
	});
});

// Derive the set of categories that have appeared in the event buffer so the
// category filter dropdown only shows categories that exist in current data.
// Exported as a function because Svelte 5 cannot export $derived from modules.
/**
 *
 */
export function knownCategories(): Set<string> {
	return new Set(events.map((ev) => ev.category));
}

// Filtered view of the event buffer. Applies all active filters in order:
// source → level → category → text search. An empty filter set passes all events.
/**
 *
 */
export function filteredEvents(): LogEvent[] {
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

// Returns true when any filter is active (used to show the Clear button).
/**
 *
 */
export function hasActiveFilters(): boolean {
	return (
		filters.sources.size > 0 ||
		filters.levels.size > 0 ||
		filters.categories.size > 0 ||
		filters.searchText.length > 0
	);
}

// Reset all filters to their default (show-all) state.
/**
 *
 */
export function clearFilters(): void {
	filters.sources = new Set();
	filters.levels = new Set();
	filters.categories = new Set();
	filters.searchText = "";
}

// Holds the Tauri unlisten function so we can clean up on destroy.
let unlisten: UnlistenFn | null = null;

// Append a new event to the display buffer, evicting from the front when the
// buffer is full so memory stays bounded.
function appendEvent(event: LogEvent): void {
	if (events.length >= DISPLAY_BUFFER_MAX) {
		events.splice(0, 1);
	}
	events.push(event);
	totalReceived.value += 1;
	// Mark connected on first event — proves the stream is live.
	if (connectionStatus.value === "connecting") {
		connectionStatus.value = "connected";
	}
}

/**
 * Start listening for Tauri log events. Safe to call multiple times; only one
 * listener is registered. Returns a cleanup function for use in onDestroy.
 */
export async function startLogStream(): Promise<() => void> {
	if (unlisten !== null) {
		// Already started — return a no-op cleanup.
		return () => {};
	}

	unlisten = await listen<LogEvent>(TAURI_LOG_EVENT, (tauri_event) => {
		appendEvent(tauri_event.payload);
	});

	connectionStatus.value = "connecting";

	return () => {
		if (unlisten !== null) {
			unlisten();
			unlisten = null;
			connectionStatus.value = "disconnected";
		}
	};
}

/** Clear all buffered events and reset counters. Does not affect the listener. */
export function clearEvents(): void {
	events.splice(0, events.length);
	totalReceived.value = 0;
}

// Whether a history load is currently in flight. Used to disable the button
// while the request is pending so the user cannot double-submit.
export const historyLoading = $state<{ value: boolean }>({ value: false });

// Whether all history has been loaded (no events older than the oldest visible).
// Set to true when loadHistory returns fewer events than the page size.
export const historyExhausted = $state<{ value: boolean }>({ value: false });

// Whether the store is currently in historical session browsing mode.
// Components read this to hide the Follow button and show the session banner.
export const historicalMode = $state<{ value: boolean }>({ value: false });

// Total event count for the historical session being browsed.
export const historicalTotal = $state<{ value: number }>({ value: 0 });

// Whether more historical events can be loaded (pagination).
export const historicalExhausted = $state<{ value: boolean }>({ value: false });

// Offset for the next historical page load.
let historicalOffset = 0;

// Page size for historical session queries.
const HISTORICAL_PAGE_SIZE = 1000;

// Page size for history queries — matches the IPC command cap.
const HISTORY_PAGE_SIZE = 1000;

/**
 * Load historical events from the daemon's SQLite store and prepend them to
 * the in-memory buffer. Deduplication is by event `id` so calling this
 * multiple times is safe. Queries events before the oldest event currently
 * in the buffer; no-ops when the buffer is empty or a load is in progress.
 *
 * Sets `historyExhausted` to true when the result set is smaller than the
 * page size, indicating there is no more history to load.
 */
export async function loadHistory(): Promise<void> {
	if (historyLoading.value) return;

	// Determine the timestamp cutoff: the oldest event currently in the buffer.
	const oldestTimestamp = events.length > 0 ? events[0].timestamp : undefined;

	historyLoading.value = true;
	try {
		const params: {
			before?: number;
			source?: string;
			level?: string;
			limit: number;
		} = {
			limit: HISTORY_PAGE_SIZE,
		};
		if (oldestTimestamp !== undefined) {
			params.before = oldestTimestamp;
		}
		// Source/level filters from the active filter state are intentionally NOT
		// applied here — history loads the full unfiltered stream so the filter
		// derived view remains consistent with the live buffer.

		const raw = await invoke<unknown[]>("devtools_query_history", { params });

		// Map raw JSON objects from the daemon to LogEvent shape. The daemon
		// serialises level/source as Pascal-case strings matching the frontend type.
		const historical: LogEvent[] = raw.map((item) => {
			const obj = item as Record<string, unknown>;
			return {
				id: Number(obj.id),
				timestamp: Number(obj.timestamp),
				level: obj.level as LogEvent["level"],
				source: obj.source as LogEvent["source"],
				category: String(obj.category ?? ""),
				message: String(obj.message ?? ""),
				metadata: obj.metadata ?? null,
				session_id: (obj.session_id as string | null) ?? null,
			};
		});

		if (historical.length < HISTORY_PAGE_SIZE) {
			historyExhausted.value = true;
		}

		if (historical.length === 0) return;

		// Deduplicate against events already in the buffer by id.
		const existingIds = new Set(events.map((ev) => ev.id));
		const newEvents = historical.filter((ev) => !existingIds.has(ev.id));

		if (newEvents.length === 0) return;

		// Sort ascending by timestamp so prepend order is consistent.
		newEvents.sort((a, b) => a.timestamp - b.timestamp);

		// Prepend to the buffer. Respect DISPLAY_BUFFER_MAX by evicting from the
		// tail (newest) if needed — history loading trades off live tail for past.
		const available = DISPLAY_BUFFER_MAX - events.length;
		const toInsert = newEvents.slice(0, Math.max(0, available + newEvents.length));
		// If the combined size exceeds DISPLAY_BUFFER_MAX, trim the tail.
		const combined = [...toInsert, ...events];
		if (combined.length > DISPLAY_BUFFER_MAX) {
			combined.splice(DISPLAY_BUFFER_MAX, combined.length - DISPLAY_BUFFER_MAX);
		}
		events.splice(0, events.length, ...combined);
	} catch (err) {
		console.error("[log-store] history load failed:", err);
	} finally {
		historyLoading.value = false;
	}
}

/**
 * Map a raw JSON object from query_session_events into a LogEvent. The SQLite
 * store serialises level/source via Rust's Debug/Display formatting which
 * matches the TypeScript union types used in the frontend.
 * @param obj
 */
function rawToLogEvent(obj: Record<string, unknown>): LogEvent {
	return {
		id: Number(obj.id ?? obj.rowid),
		timestamp: Number(obj.timestamp),
		level: obj.level as LogEvent["level"],
		source: obj.source as LogEvent["source"],
		category: String(obj.category ?? ""),
		message: String(obj.message ?? ""),
		metadata: obj.metadata ?? null,
		session_id: (obj.session_id as string | null) ?? null,
	};
}

/**
 * Switch the log view to a historical session. Stops the Tauri live listener,
 * clears the buffer, loads the first page of events from SQLite, and sets
 * historicalMode so components update their UI accordingly.
 * @param session
 */
export async function enterHistoricalMode(session: DevToolsSession): Promise<void> {
	// Pause live streaming while browsing history.
	if (unlisten !== null) {
		unlisten();
		unlisten = null;
	}

	// Clear the buffer and reset pagination state.
	events.splice(0, events.length);
	totalReceived.value = 0;
	historicalOffset = 0;
	historicalExhausted.value = false;
	historicalTotal.value = 0;

	// Disable auto-scroll; the user is exploring static data.
	scrollLock.enabled = false;

	historicalMode.value = true;

	await loadMoreHistoricalEvents(session.id);
}

/**
 * Load the next page of events for the current historical session and append
 * them to the buffer. Updates historicalOffset and historicalExhausted.
 * @param sessionId
 */
export async function loadMoreHistoricalEvents(sessionId: string): Promise<void> {
	if (historyLoading.value || historicalExhausted.value) return;

	historyLoading.value = true;
	try {
		const response = await loadSessionEvents({
			session_id: sessionId,
			offset: historicalOffset,
			limit: HISTORICAL_PAGE_SIZE,
		});

		historicalTotal.value = response.total;

		const page: LogEvent[] = (response.events as Record<string, unknown>[]).map(rawToLogEvent);

		if (page.length < HISTORICAL_PAGE_SIZE) {
			historicalExhausted.value = true;
		}

		historicalOffset += page.length;
		events.push(...page);
		totalReceived.value = events.length;
	} catch (err) {
		console.error("[log-store] historical load failed:", err);
	} finally {
		historyLoading.value = false;
	}
}

/**
 * Return the log view to the live ring buffer stream. Re-subscribes to the
 * Tauri log event, repopulates the buffer from the in-memory ring buffer via
 * the get_events IPC command, and re-enables auto-scroll.
 */
export async function exitHistoricalMode(): Promise<void> {
	historicalMode.value = false;
	historicalOffset = 0;
	historicalExhausted.value = false;
	historicalTotal.value = 0;

	// Clear the buffer before repopulating from the ring buffer.
	events.splice(0, events.length);
	totalReceived.value = 0;

	// Re-enable auto-scroll for the live feed.
	scrollLock.enabled = true;

	// Repopulate from the backend ring buffer so events already in memory are shown.
	try {
		const response = await invoke<{
			events: Record<string, unknown>[];
			total: number;
			dropped: number;
		}>("get_events", { params: { limit: DISPLAY_BUFFER_MAX } });
		const buffered: LogEvent[] = response.events.map(rawToLogEvent);
		events.push(...buffered);
		totalReceived.value = response.total;
	} catch (err) {
		console.error("[log-store] ring buffer repopulation failed:", err);
	}

	// Re-subscribe to live Tauri events.
	await startLogStream();
}
