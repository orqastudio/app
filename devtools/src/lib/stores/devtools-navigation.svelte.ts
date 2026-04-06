// Navigation state store for OrqaDev. Tracks which tab is active using
// Svelte 5 $state so components re-render reactively on tab changes.
// Active tab is persisted to localStorage so it survives app restarts.
//
// Also tracks connection state to the daemon SSE stream, updated via the
// orqa://connection-state Tauri event. The status bar reads this state to show
// "Connected", "Reconnecting (attempt N)", or "Waiting for daemon...".
//
// Tabs: issues, stream, processes, storybook, metrics, trace.

import { listen } from "@tauri-apps/api/event";
import { assertNever } from "@orqastudio/types";

export type DevToolsTab = "issues" | "stream" | "processes" | "storybook" | "metrics" | "trace";

// All six tabs with their display labels, ordered for the tab bar.
// "issues" is first and is the default active tab.
export const TABS: { value: DevToolsTab; label: string }[] = [
	{ value: "issues", label: "Issues" },
	{ value: "stream", label: "Stream" },
	{ value: "processes", label: "Processes" },
	{ value: "storybook", label: "Storybook" },
	{ value: "metrics", label: "Metrics" },
	{ value: "trace", label: "Trace" },
];

// The three connection states emitted by the Rust backend.
export type ConnectionState =
	| { state: "connected" }
	| { state: "reconnecting"; attempt: number }
	| { state: "waiting-for-daemon" };

// localStorage key for the active tab.
const STORAGE_KEY_TAB = "orqadev:activeTab";

// Valid tab values — used to reject corrupted or stale persisted values.
const VALID_TABS = new Set<DevToolsTab>(TABS.map((t) => t.value));

// Migrate legacy tab names from localStorage to their current equivalents.
// "logs" was renamed to "stream" — map it so existing installations don't lose
// their persisted tab selection.
function migrateLegacyTab(stored: string): string {
	if (stored === "logs") return "stream";
	return stored;
}

// Restore the previously selected tab from localStorage, falling back to "issues"
// if nothing is stored or the stored value is not a valid tab name.
// Applies legacy migration before validation so renamed tabs are handled correctly.
function loadPersistedTab(): DevToolsTab {
	try {
		const raw = localStorage.getItem(STORAGE_KEY_TAB);
		if (raw) {
			const stored = migrateLegacyTab(raw);
			if (VALID_TABS.has(stored as DevToolsTab)) {
				return stored as DevToolsTab;
			}
		}
	} catch {
		// localStorage unavailable (e.g., sandboxed context) — silently fall through.
	}
	return "issues";
}

// Persist the active tab to localStorage.
function persistTab(tab: DevToolsTab): void {
	try {
		localStorage.setItem(STORAGE_KEY_TAB, tab);
	} catch {
		// Ignore write failures — persistence is best-effort.
	}
}

/**
 * Return a human-readable status bar label for the given connection state.
 * @param conn - The current connection state discriminated union value.
 * @returns Display string suitable for the status bar.
 */
export function connectionLabel(conn: ConnectionState): string {
	switch (conn.state) {
		case "connected":
			return "Connected";
		case "reconnecting":
			return `Reconnecting (attempt ${conn.attempt})`;
		case "waiting-for-daemon":
			return "Waiting for daemon...";
		default:
			return assertNever(conn);
	}
}

// Module-level reactive state. Exported as a plain object so any component
// can read `navigation.activeTab` / `navigation.connection` and write to them.
// Tab is restored from localStorage; connection starts in waiting-for-daemon
// state until the Rust backend emits its first orqa://connection-state event.
export const navigation = $state<{
	activeTab: DevToolsTab;
	connection: ConnectionState;
}>({
	activeTab: loadPersistedTab(),
	connection: { state: "waiting-for-daemon" },
});

// $effect runs after state initialises. Persists the active tab to localStorage
// and subscribes to orqa://connection-state Tauri events so the status bar
// reflects the live connection state from the Rust SSE consumer.
$effect.root(() => {
	$effect(() => {
		persistTab(navigation.activeTab);
	});

	// Listen for connection state changes emitted by events.rs and update the
	// reactive navigation state. The listener is set up once and runs for the
	// lifetime of the app — no cleanup needed.
	listen<ConnectionState>("orqa://connection-state", (event) => {
		navigation.connection = event.payload;
	});
});
