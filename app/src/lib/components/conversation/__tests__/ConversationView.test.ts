/**
 * Tests for ConversationView.svelte.
 *
 * ConversationView orchestrates the chat panel: it restores sessions on mount,
 * renders messages, and provides send/stop controls. Heavy child components
 * are not mocked so the integration is real, but stores are mocked.
 */

import { describe, it, expect, beforeEach, afterEach, vi } from "vitest";
import { render, screen, waitFor } from "@testing-library/svelte";
import { installMockStores, clearMockStores } from "../../shared/__tests__/mock-stores.js";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
vi.mock("@tauri-apps/api/event", () => ({ listen: vi.fn().mockResolvedValue(() => {}) }));
vi.mock("$lib/utils/tool-display", () => ({
	getActivityPhase: vi.fn().mockReturnValue("Working"),
	getEphemeralLabel: vi.fn().mockReturnValue(null),
}));

import ConversationView from "../ConversationView.svelte";

describe("ConversationView", () => {
	beforeEach(() => {
		installMockStores();
	});

	afterEach(() => {
		clearMockStores();
	});

	it("shows loading spinner before initialization completes", async () => {
		// sessionStore.loadSessions takes time — spinner should show immediately
		installMockStores({
			sessionStore: {
				...({} as any),
				activeSession: null,
				sessions: [],
				hasActiveSession: false,
				isLoading: false,
				error: null,
				loadSessions: vi.fn().mockReturnValue(new Promise(() => {})), // never resolves
				createSession: vi.fn().mockResolvedValue(undefined),
				restoreSession: vi.fn().mockResolvedValue(undefined),
				selectSession: vi.fn(),
				deleteSession: vi.fn(),
				updateTitle: vi.fn(),
				handleTitleUpdate: vi.fn(),
			} as any,
			projectStore: {
				...({} as any),
				hasProject: true,
				activeProject: { id: 1, name: "Proj", path: "/code" },
				projectPath: "/code",
				projectSettings: null,
				settingsLoaded: true,
				hasSettings: true,
				iconDataUrl: null,
				error: null,
				openProject: vi.fn(),
				closeProject: vi.fn(),
				loadActiveProject: vi.fn(),
				loadProjectSettings: vi.fn(),
				checkIsOrqaProject: vi.fn(),
			} as any,
		});
		const { container } = render(ConversationView);
		// Loading spinner should be in the DOM immediately
		expect(container.querySelector("[class*=animate]")).toBeTruthy();
	});

	it("renders empty state when no session is active after init", async () => {
		installMockStores({
			sessionStore: {
				...({} as any),
				activeSession: null,
				sessions: [],
				hasActiveSession: false,
				isLoading: false,
				error: null,
				loadSessions: vi.fn().mockResolvedValue(undefined),
				createSession: vi.fn().mockResolvedValue(undefined),
				restoreSession: vi.fn().mockResolvedValue(undefined),
				selectSession: vi.fn(),
				deleteSession: vi.fn(),
				updateTitle: vi.fn(),
				handleTitleUpdate: vi.fn(),
			} as any,
			projectStore: {
				...({} as any),
				hasProject: false,
				activeProject: null,
				projectPath: null,
				projectSettings: null,
				settingsLoaded: false,
				hasSettings: true,
				iconDataUrl: null,
				error: null,
				openProject: vi.fn(),
				closeProject: vi.fn(),
				loadActiveProject: vi.fn(),
				loadProjectSettings: vi.fn(),
				checkIsOrqaProject: vi.fn(),
			} as any,
			settingsStore: {
				...({} as any),
				sidecarStatus: { state: "connected" },
				daemonHealth: { state: "connected", artifacts: 0, rules: 0 },
				modelDisplayName: "Claude Sonnet",
				sidecarStateLabel: "Connected",
				daemonStateLabel: "Connected",
				activeStartupTask: null,
				lastSessionId: null,
				initialize: vi.fn(),
				destroy: vi.fn(),
				setActiveSection: vi.fn(),
				refreshDaemonHealth: vi.fn(),
			} as any,
		});
		render(ConversationView);
		await waitFor(() => {
			expect(screen.getByText(/No session active/i)).toBeInTheDocument();
		});
	});

	it("renders empty message list state when session has no messages", async () => {
		installMockStores({
			sessionStore: {
				...({} as any),
				activeSession: { id: 1, title: "New Chat" },
				sessions: [{ id: 1, title: "New Chat" }],
				hasActiveSession: true,
				isLoading: false,
				error: null,
				loadSessions: vi.fn().mockResolvedValue(undefined),
				createSession: vi.fn().mockResolvedValue(undefined),
				restoreSession: vi.fn().mockResolvedValue(undefined),
				selectSession: vi.fn(),
				deleteSession: vi.fn(),
				updateTitle: vi.fn(),
				handleTitleUpdate: vi.fn(),
			} as any,
			conversationStore: {
				...({} as any),
				messages: [],
				isStreaming: false,
				isLoading: false,
				error: null,
				streamingContent: "",
				activeToolCalls: new Map(),
				pendingApproval: null,
				processViolations: [],
				contextEntries: [],
				streamingThinking: null,
				lastTitleUpdate: null,
				loadMessages: vi.fn().mockResolvedValue(undefined),
				sendMessage: vi.fn(),
				stopStreaming: vi.fn(),
				clear: vi.fn(),
				respondToApproval: vi.fn(),
			} as any,
			projectStore: {
				...({} as any),
				hasProject: false,
				activeProject: null,
				projectPath: null,
				projectSettings: null,
				settingsLoaded: false,
				hasSettings: true,
				iconDataUrl: null,
				error: null,
				openProject: vi.fn(),
				closeProject: vi.fn(),
				loadActiveProject: vi.fn(),
				loadProjectSettings: vi.fn(),
				checkIsOrqaProject: vi.fn(),
			} as any,
			settingsStore: {
				...({} as any),
				sidecarStatus: { state: "connected" },
				daemonHealth: { state: "connected", artifacts: 0, rules: 0 },
				modelDisplayName: "Claude Sonnet",
				sidecarStateLabel: "Connected",
				daemonStateLabel: "Connected",
				activeStartupTask: null,
				lastSessionId: null,
				initialize: vi.fn(),
				destroy: vi.fn(),
				setActiveSection: vi.fn(),
				refreshDaemonHealth: vi.fn(),
			} as any,
		});
		render(ConversationView);
		await waitFor(() => {
			expect(screen.getByText(/No messages yet/i)).toBeInTheDocument();
		});
	});
});
