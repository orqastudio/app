// Trace store for OrqaDev. Holds the currently selected correlation ID and
// derives the matching events from the log-store event buffer. Components use
// this store to power the TraceTimeline view — select a correlation ID from
// the ContextTable, this store filters the log buffer, and TraceView renders it.

import { events, type LogEvent } from "./log-store.svelte.js";

// The correlation ID whose events are currently being visualised.
// null means no trace is selected — TraceView shows EmptyState.
let selectedCorrelationId = $state<string | null>(null);

/**
 * Return the currently selected correlation ID, or null when no trace is active.
 * @returns The active correlation ID string, or null.
 */
export function getSelectedCorrelationId(): string | null {
	return selectedCorrelationId;
}

/**
 * Return all log events whose correlation_id matches the selected correlation ID.
 * Returns an empty array when no correlation ID is selected or no matching events
 * exist in the current buffer.
 * @returns Array of matching LogEvent entries sorted by timestamp ascending.
 */
export function traceEvents(): LogEvent[] {
	if (selectedCorrelationId === null) return [];
	return events
		.filter((ev) => ev.correlation_id === selectedCorrelationId)
		.sort((a, b) => a.timestamp - b.timestamp);
}

/**
 * Select a correlation ID to trace. Filters the log-store event buffer to show
 * only events that share this ID. The selection persists until clearTrace() is
 * called or a new ID is selected.
 * @param correlationId - The correlation ID to trace.
 */
export function selectTrace(correlationId: string): void {
	selectedCorrelationId = correlationId;
}

/**
 * Clear the active trace selection. After calling this, traceEvents() returns
 * an empty array and TraceView shows the EmptyState.
 */
export function clearTrace(): void {
	selectedCorrelationId = null;
}
