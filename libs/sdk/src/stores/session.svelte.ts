import type { Session, SessionSummary } from "@orqastudio/types";
import { invoke, extractErrorMessage } from "../ipc/invoke.js";
import { logger } from "../logger.js";

const log = logger("session");

/**
 *
 */
export class SessionStore {
	sessions = $state<SessionSummary[]>([]);
	activeSession = $state<Session | null>(null);
	isLoading = $state(false);
	error = $state<string | null>(null);

	/**
	 *
	 */
	get hasActiveSession(): boolean {
		return this.activeSession !== null;
	}

	/**
	 *
	 */
	get activeSessionId(): number | null {
		return this.activeSession?.id ?? null;
	}

	/**
	 *
	 * @param projectId
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
	 *
	 * @param projectId
	 * @param model
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
	 *
	 * @param sessionId
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
	 *
	 * @param sessionId
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
	 * @param sessionId
	 * @param title
	 */
	handleTitleUpdate(sessionId: number, title: string): void {
		if (this.activeSession && this.activeSession.id === sessionId) {
			this.activeSession = { ...this.activeSession, title };
		}
		this.sessions = this.sessions.map((s) => (s.id === sessionId ? { ...s, title } : s));
	}

	/**
	 *
	 * @param sessionId
	 * @param title
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
	 *
	 * @param sessionId
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
	 *
	 * @param sessionId
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
	 *
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
