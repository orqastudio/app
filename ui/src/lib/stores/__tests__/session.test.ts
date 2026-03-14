import { describe, it, expect, beforeEach } from "vitest";
import { mockInvoke } from "./setup";

import { sessionStore } from "../session.svelte";
import type { Session, SessionSummary } from "$lib/types";

const fakeSession: Session = {
	id: 1,
	project_id: 10,
	title: "Test session",
	model: "auto",
	system_prompt: null,
	status: "active",
	summary: null,
	handoff_notes: null,
	total_input_tokens: 0,
	total_output_tokens: 0,
	total_cost_usd: 0,
	created_at: "2026-01-01T00:00:00Z",
	updated_at: "2026-01-01T00:00:00Z",
};

const fakeSummary: SessionSummary = {
	id: 1,
	title: "Test session",
	status: "active",
	message_count: 5,
	preview: null,
	created_at: "2026-01-01T00:00:00Z",
	updated_at: "2026-01-01T00:00:00Z",
};

beforeEach(() => {
	mockInvoke.mockReset();
	sessionStore.clear();
});

describe("SessionStore", () => {
	describe("initial state", () => {
		it("starts with no sessions and no active session", () => {
			expect(sessionStore.sessions).toEqual([]);
			expect(sessionStore.activeSession).toBeNull();
			expect(sessionStore.isLoading).toBe(false);
			expect(sessionStore.error).toBeNull();
		});

		it("hasActiveSession is false initially", () => {
			expect(sessionStore.hasActiveSession).toBe(false);
		});

		it("activeSessionId is null initially", () => {
			expect(sessionStore.activeSessionId).toBeNull();
		});
	});

	describe("loadSessions", () => {
		it("loads sessions from backend", async () => {
			const summaries = [fakeSummary];
			mockInvoke.mockResolvedValueOnce(summaries);

			await sessionStore.loadSessions(10);

			expect(mockInvoke).toHaveBeenCalledWith("session_list", { projectId: 10 });
			expect(sessionStore.sessions).toEqual(summaries);
			expect(sessionStore.isLoading).toBe(false);
		});

		it("sets error on failure", async () => {
			mockInvoke.mockRejectedValueOnce(new Error("DB error"));

			await sessionStore.loadSessions(10);

			expect(sessionStore.error).toBe("DB error");
			expect(sessionStore.isLoading).toBe(false);
		});
	});

	describe("createSession", () => {
		it("creates session and sets it active", async () => {
			mockInvoke
				.mockResolvedValueOnce(fakeSession) // session_create
				.mockResolvedValueOnce(undefined) // settings_set (persist ID)
				.mockResolvedValueOnce([fakeSummary]); // session_list (reload)

			const session = await sessionStore.createSession(10, "auto");

			expect(session).toEqual(fakeSession);
			expect(sessionStore.activeSession).toEqual(fakeSession);
			expect(mockInvoke).toHaveBeenCalledWith("session_create", { projectId: 10, model: "auto" });
		});

		it("defaults model to 'auto' when not specified", async () => {
			mockInvoke
				.mockResolvedValueOnce(fakeSession)
				.mockResolvedValueOnce(undefined)
				.mockResolvedValueOnce([]);

			await sessionStore.createSession(10);

			expect(mockInvoke).toHaveBeenCalledWith("session_create", { projectId: 10, model: "auto" });
		});

		it("throws and sets error on failure", async () => {
			mockInvoke.mockRejectedValueOnce(new Error("Create failed"));

			await expect(sessionStore.createSession(10)).rejects.toThrow("Create failed");
			expect(sessionStore.error).toBe("Create failed");
		});
	});

	describe("selectSession", () => {
		it("fetches and activates the session", async () => {
			mockInvoke
				.mockResolvedValueOnce(fakeSession) // session_get
				.mockResolvedValueOnce(undefined); // settings_set (persist ID)

			await sessionStore.selectSession(1);

			expect(sessionStore.activeSession).toEqual(fakeSession);
			expect(sessionStore.isLoading).toBe(false);
		});

		it("sets error on failure", async () => {
			mockInvoke.mockRejectedValueOnce(new Error("Not found"));

			await sessionStore.selectSession(999);

			expect(sessionStore.error).toBe("Not found");
		});
	});

	describe("restoreSession", () => {
		it("returns true on success", async () => {
			mockInvoke.mockResolvedValueOnce(fakeSession);

			const result = await sessionStore.restoreSession(1);

			expect(result).toBe(true);
			expect(sessionStore.activeSession).toEqual(fakeSession);
		});

		it("returns false and clears persisted ID on failure", async () => {
			mockInvoke
				.mockRejectedValueOnce(new Error("Gone")) // session_get
				.mockResolvedValueOnce(undefined); // settings_set (clear)

			const result = await sessionStore.restoreSession(1);

			expect(result).toBe(false);
			expect(sessionStore.activeSession).toBeNull();
		});
	});

	describe("handleTitleUpdate", () => {
		it("updates active session title", () => {
			sessionStore.activeSession = { ...fakeSession };
			sessionStore.sessions = [{ ...fakeSummary }];

			sessionStore.handleTitleUpdate(1, "New Title");

			expect(sessionStore.activeSession?.title).toBe("New Title");
			expect(sessionStore.sessions[0].title).toBe("New Title");
		});

		it("does nothing for non-matching session", () => {
			sessionStore.activeSession = { ...fakeSession };

			sessionStore.handleTitleUpdate(999, "Other");

			expect(sessionStore.activeSession.title).toBe("Test session");
		});
	});

	describe("updateTitle", () => {
		it("calls backend and updates local state", async () => {
			sessionStore.activeSession = { ...fakeSession };
			sessionStore.sessions = [{ ...fakeSummary }];
			mockInvoke.mockResolvedValueOnce(undefined);

			await sessionStore.updateTitle(1, "Updated");

			expect(mockInvoke).toHaveBeenCalledWith("session_update_title", { sessionId: 1, title: "Updated" });
			expect(sessionStore.activeSession?.title).toBe("Updated");
		});
	});

	describe("endSession", () => {
		it("marks session as completed", async () => {
			sessionStore.activeSession = { ...fakeSession };
			sessionStore.sessions = [{ ...fakeSummary }];
			mockInvoke.mockResolvedValueOnce(undefined);

			await sessionStore.endSession(1);

			expect(mockInvoke).toHaveBeenCalledWith("session_end", { sessionId: 1 });
			expect(sessionStore.activeSession?.status).toBe("completed");
			expect(sessionStore.sessions[0].status).toBe("completed");
		});
	});

	describe("deleteSession", () => {
		it("optimistically removes from list and clears active", async () => {
			sessionStore.activeSession = { ...fakeSession };
			sessionStore.sessions = [{ ...fakeSummary }];
			mockInvoke
				.mockResolvedValueOnce(undefined) // session_delete
				.mockResolvedValueOnce(undefined); // settings_set (clear)

			await sessionStore.deleteSession(1);

			expect(sessionStore.sessions).toEqual([]);
			expect(sessionStore.activeSession).toBeNull();
		});
	});

	describe("clear", () => {
		it("resets all state", () => {
			sessionStore.activeSession = fakeSession;
			sessionStore.sessions = [fakeSummary];
			sessionStore.isLoading = true;
			sessionStore.error = "some error";

			sessionStore.clear();

			expect(sessionStore.sessions).toEqual([]);
			expect(sessionStore.activeSession).toBeNull();
			expect(sessionStore.isLoading).toBe(false);
			expect(sessionStore.error).toBeNull();
		});
	});
});
