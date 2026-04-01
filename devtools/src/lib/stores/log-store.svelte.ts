// Log event store for OrqaDev. Subscribes to Tauri `orqa://log-event` events
// and maintains a bounded display buffer of log entries for the UI. Exposes
// reactive state for the event list, connection status, and scroll-lock toggle.

import { listen, type UnlistenFn } from "@tauri-apps/api/event";

// Shape of a log event as emitted by the Tauri backend.
export interface LogEvent {
	id: number;
	timestamp: number; // Unix ms
	level: "Debug" | "Info" | "Warn" | "Error" | "Perf";
	source:
		| "Daemon"
		| "App"
		| "Frontend"
		| "DevController"
		| "MCP"
		| "LSP"
		| "Search"
		| "Worker";
	category: string;
	message: string;
	metadata: unknown;
	session_id: string | null;
}

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
