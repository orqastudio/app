import type { Session, SessionSummary } from "$lib/types";
import { forgeInvoke } from "$lib/ipc/invoke";

class SessionStore {
	sessions = $state<SessionSummary[]>([]);
	activeSession = $state<Session | null>(null);
	isLoading = $state(false);
	error = $state<string | null>(null);

	get hasActiveSession(): boolean {
		return this.activeSession !== null;
	}

	get activeSessionId(): number | null {
		return this.activeSession?.id ?? null;
	}

	async loadSessions(projectId: number): Promise<void> {
		this.isLoading = true;
		this.error = null;
		try {
			this.sessions = await forgeInvoke<SessionSummary[]>("list_sessions", {
				project_id: projectId,
			});
		} catch (err) {
			this.error = err instanceof Error ? err.message : String(err);
		} finally {
			this.isLoading = false;
		}
	}

	async createSession(projectId: number, model?: string): Promise<Session> {
		this.error = null;
		try {
			const session = await forgeInvoke<Session>("create_session", {
				project_id: projectId,
				model: model ?? "auto",
			});
			this.activeSession = session;
			await this.persistActiveSessionId(session.id);
			await this.loadSessions(projectId);
			return session;
		} catch (err) {
			this.error = err instanceof Error ? err.message : String(err);
			throw err;
		}
	}

	async selectSession(sessionId: number): Promise<void> {
		this.isLoading = true;
		this.error = null;
		try {
			this.activeSession = await forgeInvoke<Session>("get_session", {
				session_id: sessionId,
			});
			await this.persistActiveSessionId(sessionId);
		} catch (err) {
			this.error = err instanceof Error ? err.message : String(err);
		} finally {
			this.isLoading = false;
		}
	}

	async restoreSession(sessionId: number): Promise<boolean> {
		this.isLoading = true;
		this.error = null;
		try {
			this.activeSession = await forgeInvoke<Session>("get_session", {
				session_id: sessionId,
			});
			return true;
		} catch {
			// Session no longer exists — clear persisted ID
			await this.clearPersistedSessionId();
			return false;
		} finally {
			this.isLoading = false;
		}
	}

	async updateTitle(sessionId: number, title: string): Promise<void> {
		this.error = null;
		try {
			await forgeInvoke("update_session_title", {
				session_id: sessionId,
				title,
			});
			if (this.activeSession && this.activeSession.id === sessionId) {
				this.activeSession = { ...this.activeSession, title };
			}
			const summary = this.sessions.find((s) => s.id === sessionId);
			if (summary) {
				summary.title = title;
			}
		} catch (err) {
			this.error = err instanceof Error ? err.message : String(err);
		}
	}

	async endSession(sessionId: number): Promise<void> {
		this.error = null;
		try {
			await forgeInvoke("end_session", { session_id: sessionId });
			if (this.activeSession && this.activeSession.id === sessionId) {
				this.activeSession = { ...this.activeSession, status: "completed" };
			}
			const summary = this.sessions.find((s) => s.id === sessionId);
			if (summary) {
				summary.status = "completed";
			}
		} catch (err) {
			this.error = err instanceof Error ? err.message : String(err);
		}
	}

	async deleteSession(sessionId: number): Promise<void> {
		this.error = null;
		try {
			await forgeInvoke("delete_session", { session_id: sessionId });
			this.sessions = this.sessions.filter((s) => s.id !== sessionId);
			if (this.activeSession && this.activeSession.id === sessionId) {
				this.activeSession = null;
				await this.clearPersistedSessionId();
			}
		} catch (err) {
			this.error = err instanceof Error ? err.message : String(err);
		}
	}

	clear() {
		this.sessions = [];
		this.activeSession = null;
		this.isLoading = false;
		this.error = null;
	}

	private async persistActiveSessionId(sessionId: number): Promise<void> {
		try {
			await forgeInvoke("settings_set", {
				key: "last_session_id",
				value: sessionId,
				scope: "app",
			});
		} catch {
			// Non-critical — best-effort persistence
		}
	}

	private async clearPersistedSessionId(): Promise<void> {
		try {
			await forgeInvoke("settings_set", {
				key: "last_session_id",
				value: null,
				scope: "app",
			});
		} catch {
			// Non-critical — best-effort persistence
		}
	}
}

export const sessionStore = new SessionStore();
