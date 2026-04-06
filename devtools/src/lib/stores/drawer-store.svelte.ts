// Drawer store for OrqaDev. Manages state for the EventDrawer panel that
// renders event detail alongside the log stream and issues views. Tracks
// which event is displayed, which tab is active, and which list of events
// is being navigated so next/prev can advance through the visible set.
// Drawer width is persisted to localStorage so the panel retains its size
// across sessions.

import type { LogEvent } from "./log-store.svelte.js";

// localStorage key for the drawer panel width.
const STORAGE_KEY_WIDTH = "orqadev:drawerWidth";

// Default drawer width in pixels when no persisted value is found.
const DEFAULT_WIDTH = 360;

// Whether the drawer is currently visible.
let drawerOpen = $state(false);

// The event currently displayed in the drawer. Null when the drawer is closed.
let drawerEvent = $state<LogEvent | null>(null);

// Active tab in the EventDrawer.
let drawerTab = $state<"stack" | "context" | "related" | "raw">("stack");

// The ordered list of events the drawer is navigating through (e.g. filteredEvents
// or the issue's recent events). Used to compute next/prev indices.
let eventList = $state<LogEvent[]>([]);

// Index of drawerEvent within eventList. -1 when the event is not in the list.
let currentIndex = $state(0);

// Width of the drawer panel in pixels, persisted across sessions.
export const drawerWidth = $state<{ value: number }>({ value: loadWidth() });

/**
 * Load the persisted drawer width from localStorage.
 * Falls back to DEFAULT_WIDTH when nothing is stored or the value is invalid.
 * @returns The stored width in pixels, or DEFAULT_WIDTH.
 */
function loadWidth(): number {
	try {
		const raw = localStorage.getItem(STORAGE_KEY_WIDTH);
		if (raw !== null) {
			const parsed = parseInt(raw, 10);
			if (!isNaN(parsed) && parsed > 0) return parsed;
		}
	} catch {
		// localStorage unavailable — silently fall through.
	}
	return DEFAULT_WIDTH;
}

/**
 * Persist the current drawer width to localStorage.
 * Write failures are silently ignored — persistence is best-effort.
 */
function persistWidth(): void {
	try {
		localStorage.setItem(STORAGE_KEY_WIDTH, String(drawerWidth.value));
	} catch {
		// Ignore write failures.
	}
}

// Watch drawerWidth and persist on change.
$effect.root(() => {
	$effect(() => {
		void drawerWidth.value;
		persistWidth();
	});
});

/**
 * Open the drawer for the given event and navigation list.
 * Resets the active tab to "stack" each time a new event is opened so the
 * most actionable content (stack frames) is shown first.
 * @param event - The log event to display in the drawer.
 * @param list - The ordered array of events to navigate through (next/prev).
 */
export function openDrawer(event: LogEvent, list: LogEvent[]): void {
	drawerEvent = event;
	eventList = list;
	currentIndex = list.findIndex((ev) => ev.id === event.id);
	if (currentIndex === -1) currentIndex = 0;
	drawerTab = "stack";
	drawerOpen = true;
}

/**
 * Close the drawer and clear the displayed event.
 */
export function closeDrawer(): void {
	drawerOpen = false;
	drawerEvent = null;
}

/**
 * Navigate to the next event in the navigation list. Wraps around to the
 * first event when the end of the list is reached.
 */
export function nextEvent(): void {
	if (eventList.length === 0) return;
	currentIndex = (currentIndex + 1) % eventList.length;
	drawerEvent = eventList[currentIndex] ?? null;
}

/**
 * Navigate to the previous event in the navigation list. Wraps around to the
 * last event when the beginning of the list is reached.
 */
export function prevEvent(): void {
	if (eventList.length === 0) return;
	currentIndex = (currentIndex - 1 + eventList.length) % eventList.length;
	drawerEvent = eventList[currentIndex] ?? null;
}

/**
 * Set the active tab. Only "stack", "context", "related", and "raw" are valid;
 * any other value is ignored.
 * @param tab - The tab identifier to activate.
 */
export function setTab(tab: string): void {
	if (tab === "stack" || tab === "context" || tab === "related" || tab === "raw") {
		drawerTab = tab;
	}
}

/**
 * Read-only access to whether the drawer is open.
 * @returns True when the drawer is visible.
 */
export function isDrawerOpen(): boolean {
	return drawerOpen;
}

/**
 * Read-only access to the currently displayed event.
 * @returns The event shown in the drawer, or null when closed.
 */
export function getDrawerEvent(): LogEvent | null {
	return drawerEvent;
}

/**
 * Read-only access to the active tab.
 * @returns The currently active drawer tab identifier.
 */
export function getDrawerTab(): "stack" | "context" | "related" | "raw" {
	return drawerTab;
}
