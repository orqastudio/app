// Session store for OrqaDev. Manages the list of devtools sessions and the
// "viewing historical" mode that switches the log view from the live ring buffer
// to a SQLite-backed historical session. Uses Svelte 5 runes ($state).
//
// IPC commands bridged here:
//   list_sessions         → Vec<SessionSummary>
//   query_session_events  → SessionEventQueryResponse
//   get_current_session   → SessionInfo
//   rename_session        → ()
//   delete_session        → ()

import { invoke } from "@tauri-apps/api/core";

// Shape of a session summary as returned by the `list_sessions` IPC command.
export interface DevToolsSession {
	readonly id: string;
	readonly started_at: number; // Unix ms
	readonly ended_at: number | null;
	readonly label: string | null;
	readonly event_count: number;
	readonly is_current: boolean;
}

// Query parameters for `query_session_events`.
export interface SessionEventQueryParams {
	readonly session_id: string;
	readonly offset?: number;
	readonly limit?: number;
	readonly source?: string;
	readonly level?: string;
	readonly category?: string;
	readonly search_text?: string;
}

// Response from `query_session_events`.
export interface SessionEventQueryResponse {
	readonly events: unknown[];
	readonly total: number;
}

// All known sessions, newest first. Populated by loadSessions().
export const sessions = $state<DevToolsSession[]>([]);

// When viewingHistorical.value is true, the log view shows events from
// activeSessionId.value instead of the live ring buffer.
export const viewingHistorical = $state<{ value: boolean }>({ value: false });

// The session ID currently being viewed, or null when showing the live feed.
export const activeSessionId = $state<{ value: string | null }>({ value: null });

// Fetch all sessions from the backend and update the reactive list.
/**
 *
 */
export async function loadSessions(): Promise<void> {
	const result = await invoke<DevToolsSession[]>("list_sessions");
	sessions.splice(0, sessions.length, ...result);
}

// Switch the log view to a historical session. The log-store reacts to
// viewingHistorical.value and activeSessionId.value changing.
/**
 *
 * @param sessionId
 */
export async function switchToSession(sessionId: string): Promise<void> {
	activeSessionId.value = sessionId;
	viewingHistorical.value = true;
}

// Return the log view to the live ring buffer stream.
/**
 *
 */
export async function switchToCurrentSession(): Promise<void> {
	viewingHistorical.value = false;
	activeSessionId.value = null;
}

// Rename a session. Reloads the session list to reflect the change.
/**
 *
 * @param sessionId
 * @param label
 */
export async function renameSession(sessionId: string, label: string): Promise<void> {
	await invoke("rename_session", { sessionId, label });
	await loadSessions();
}

// Delete a session and all its events. Reloads the session list.
// If the deleted session was being viewed, switch back to live.
/**
 *
 * @param sessionId
 */
export async function deleteSession(sessionId: string): Promise<void> {
	await invoke("delete_session", { sessionId });
	if (activeSessionId.value === sessionId) {
		await switchToCurrentSession();
	}
	await loadSessions();
}

// Query events for a specific session from the SQLite store.
// Returns the paginated response including total count.
/**
 *
 * @param params
 */
export async function loadSessionEvents(
	params: SessionEventQueryParams,
): Promise<SessionEventQueryResponse> {
	return await invoke<SessionEventQueryResponse>("query_session_events", { params });
}

// Format a session's start timestamp as a human-readable label.
// Used when label is null to show an auto-generated display name.
/**
 *
 * @param session
 */
export function sessionDisplayLabel(session: DevToolsSession): string {
	if (session.label) return session.label;
	const d = new Date(session.started_at);
	const date = d.toLocaleDateString(undefined, {
		year: "numeric",
		month: "2-digit",
		day: "2-digit",
	});
	const time = d.toLocaleTimeString(undefined, { hour: "2-digit", minute: "2-digit" });
	return `Session ${date} ${time}`;
}

// Format the duration between started_at and ended_at as a human-readable string.
// Returns "Active" when ended_at is null (current session).
/**
 *
 * @param session
 */
export function sessionDuration(session: DevToolsSession): string {
	if (session.ended_at === null) return "Active";
	const ms = session.ended_at - session.started_at;
	if (ms < 1000) return "<1s";
	const secs = Math.floor(ms / 1000);
	if (secs < 60) return `${secs}s`;
	const mins = Math.floor(secs / 60);
	const remSecs = secs % 60;
	if (mins < 60) return `${mins}m ${remSecs}s`;
	const hrs = Math.floor(mins / 60);
	const remMins = mins % 60;
	return `${hrs}h ${remMins}m`;
}
