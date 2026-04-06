import type { Session, SessionSummary } from "@orqastudio/types";
import { invoke, extractErrorMessage } from "../ipc/invoke.js";
import { logger } from "../logger.js";

const log = logger("session");

/**
 * Reactive store managing conversation sessions, including loading, creation, selection, and deletion.
 */
export class SessionStore {
	sessions = $state<SessionSummary[]>([]);
	activeSession = $state<Session | null>(null);
	isLoading = $state(false);
	error = $state<string | null>(null);

	/**
	 * Returns true when a session is currently active.
	 * @returns Whether there is a currently active session.
	 */
	get hasActiveSession(): boolean {
		return this.activeSession !== null;
	}

	/**
	 * Returns the numeric ID of the active session, or null if no session is active.
	 * @returns The active session ID, or null.
	 */
	get activeSessionId(): number | null {
		return this.activeSession?.id ?? null;
	}

	/**
	 * Loads all session summaries for the given project from the backend.
	 * @param projectId - The numeric ID of the project whose sessions to load.
	 */
	async loadSessions(projectId: number): Promise<void> {
		this.isLoading = true;
		this.error = null;
		try {
			this.sessions = await invoke<SessionSummary[]>("session_list", {
				projectId,
			});
		} catch (err) {
			this.error = extractErrorMessage(err);
		} finally {
			this.isLoading = false;
		}
	}

	/**
	 * Creates a new session for the project, sets it as active, and refreshes the sessions list.
	 * @param projectId - The numeric ID of the project to create the session under.
	 * @param model - Optional model identifier; defaults to "auto" when omitted.
	 * @returns The newly created session object.
	 */
	async createSession(projectId: number, model?: string): Promise<Session> {
		this.error = null;
		try {
			const session = await invoke<Session>("session_create", {
				projectId,
				model: model ?? "auto",
			});
			this.activeSession = session;
			log.info(`createSession: created session_id=${session.id} for project_id=${projectId}`);
			await this.persistActiveSessionId(session.id);
			await this.loadSessions(projectId);
			return session;
		} catch (err) {
			this.error = extractErrorMessage(err);
			throw err;
		}
	}

	/**
	 * Fetches the full session by ID and sets it as the active session.
	 * @param sessionId - The numeric ID of the session to select.
	 */
	async selectSession(sessionId: number): Promise<void> {
		this.isLoading = true;
		this.error = null;
		try {
			this.activeSession = await invoke<Session>("session_get", {
				sessionId,
			});
			await this.persistActiveSessionId(sessionId);
		} catch (err) {
			this.error = extractErrorMessage(err);
		} finally {
			this.isLoading = false;
		}
	}

	/**
	 * Attempts to restore a previously active session by ID; clears the persisted ID if the session no longer exists.
	 * @param sessionId - The numeric ID of the session to restore.
	 * @returns True if the session was restored, false if it no longer exists.
	 */
	async restoreSession(sessionId: number): Promise<boolean> {
		this.isLoading = true;
		this.error = null;
		try {
			this.activeSession = await invoke<Session>("session_get", {
				sessionId,
			});
			log.info(`restoreSession: restored session_id=${sessionId}`);
			return true;
		} catch {
			// Session no longer exists — clear persisted ID
			log.info(`restoreSession: session_id=${sessionId} no longer exists, clearing`);
			await this.clearPersistedSessionId();
			return false;
		} finally {
			this.isLoading = false;
		}
	}

	/**
	 * Handle an auto-generated title update from the backend.
	 * @param sessionId - The numeric ID of the session whose title was updated.
	 * @param title - The new title string to apply to the session.
	 */
	handleTitleUpdate(sessionId: number, title: string): void {
		if (this.activeSession && this.activeSession.id === sessionId) {
			this.activeSession = { ...this.activeSession, title };
		}
		this.sessions = this.sessions.map((s) => (s.id === sessionId ? { ...s, title } : s));
	}

	/**
	 * Persists a manually set title to the backend and updates local reactive state.
	 * @param sessionId - The numeric ID of the session to rename.
	 * @param title - The new title to assign.
	 */
	async updateTitle(sessionId: number, title: string): Promise<void> {
		this.error = null;
		try {
			await invoke("session_update_title", {
				sessionId,
				title,
			});
			if (this.activeSession && this.activeSession.id === sessionId) {
				this.activeSession = { ...this.activeSession, title };
			}
			this.sessions = this.sessions.map((s) => (s.id === sessionId ? { ...s, title } : s));
		} catch (err) {
			this.error = extractErrorMessage(err);
		}
	}

	/**
	 * Marks a session as completed in the backend and updates the local session list.
	 * @param sessionId - The numeric ID of the session to end.
	 */
	async endSession(sessionId: number): Promise<void> {
		this.error = null;
		try {
			await invoke("session_end", { sessionId });
			if (this.activeSession && this.activeSession.id === sessionId) {
				this.activeSession = { ...this.activeSession, status: "completed" };
			}
			this.sessions = this.sessions.map((s) =>
				s.id === sessionId ? { ...s, status: "completed" as const } : s,
			);
		} catch (err) {
			this.error = extractErrorMessage(err);
		}
	}

	/**
	 * Deletes a session from the backend, optimistically removing it from the local list first.
	 * @param sessionId - The numeric ID of the session to delete.
	 */
	async deleteSession(sessionId: number): Promise<void> {
		this.error = null;
		// Optimistically remove from list for immediate UI update
		this.sessions = this.sessions.filter((s) => s.id !== sessionId);
		if (this.activeSession && this.activeSession.id === sessionId) {
			this.activeSession = null;
		}
		try {
			await invoke("session_delete", { sessionId });
			await this.clearPersistedSessionId();
		} catch (err) {
			this.error = extractErrorMessage(err);
		}
	}

	/**
	 * Resets all session state to its initial empty values.
	 */
	clear() {
		this.sessions = [];
		this.activeSession = null;
		this.isLoading = false;
		this.error = null;
	}

	private async persistActiveSessionId(sessionId: number): Promise<void> {
		try {
			await invoke("settings_set", {
				key: "last_session_id",
				value: sessionId,
				scope: "app",
			});
		} catch (err: unknown) {
			log.warn("failed to persist active session id", err);
		}
	}

	private async clearPersistedSessionId(): Promise<void> {
		try {
			await invoke("settings_set", {
				key: "last_session_id",
				value: null,
				scope: "app",
			});
		} catch (err: unknown) {
			log.warn("failed to clear persisted session id", err);
		}
	}
}
