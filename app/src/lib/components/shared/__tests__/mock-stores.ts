/**
 * Mock store factory for Svelte component tests.
 *
 * Populates `globalThis.__orqa_stores` with lightweight stub objects so
 * that any component calling `getStores()` from @orqastudio/sdk gets
 * predictable, controllable state without hitting Tauri IPC.
 *
 * Call `installMockStores(overrides?)` in a beforeEach to wire up stubs,
 * and `clearMockStores()` in an afterEach to reset global state.
 */

import { vi } from "vitest";
import type { OrqaStores } from "@orqastudio/sdk";

// ---------------------------------------------------------------------------
// Default stub shapes
// ---------------------------------------------------------------------------

function makeMockSettingsStore() {
	return {
		sidecarStatus: { state: "connected" as const },
		daemonHealth: { state: "connected" as const, artifacts: 42, rules: 10 },
		modelDisplayName: "Claude Sonnet",
		sidecarStateLabel: "Connected",
		daemonStateLabel: "Connected",
		activeStartupTask: null,
		lastSessionId: null,
		initialize: vi.fn(),
		destroy: vi.fn(),
		setActiveSection: vi.fn(),
		refreshDaemonHealth: vi.fn(),
	};
}

function makeMockSessionStore() {
	return {
		activeSession: null,
		sessions: [],
		hasActiveSession: false,
		isLoading: false,
		error: null,
		loadSessions: vi.fn().mockResolvedValue(undefined),
		createSession: vi.fn().mockResolvedValue(undefined),
		restoreSession: vi.fn().mockResolvedValue(undefined),
		selectSession: vi.fn().mockResolvedValue(undefined),
		deleteSession: vi.fn().mockResolvedValue(undefined),
		updateTitle: vi.fn(),
		handleTitleUpdate: vi.fn(),
	};
}

function makeMockNavigationStore() {
	return {
		activeActivity: "project" as string,
		activeGroup: null as string | null,
		selectedArtifactPath: null as string | null,
		breadcrumbs: [] as string[],
		topLevelNavItems: [],
		showNavPanel: false,
		groupSubCategories: {} as Record<string, string[]>,
		isArtifactActivity: false,
		searchOpen: false,
		setActivity: vi.fn(),
		setGroup: vi.fn(),
		toggleSearch: vi.fn(),
		navigateToArtifact: vi.fn(),
		openArtifact: vi.fn(),
		getNavType: vi.fn().mockReturnValue(null),
		getLabelForKey: vi.fn().mockReturnValue(""),
	};
}

function makeMockProjectStore() {
	return {
		hasProject: false,
		activeProject: null as { id: number; name: string; path: string } | null,
		projectPath: null as string | null,
		projectSettings: null,
		settingsLoaded: false,
		hasSettings: true,
		iconDataUrl: null as string | null,
		error: null as string | null,
		openProject: vi.fn().mockResolvedValue(undefined),
		closeProject: vi.fn(),
		loadActiveProject: vi.fn(),
		loadProjectSettings: vi.fn().mockResolvedValue(undefined),
		checkIsOrqaProject: vi.fn().mockResolvedValue(false),
	};
}

function makeMockArtifactStore() {
	return {
		navTree: null,
		activeContent: null as string | null,
		activeContentLoading: false,
		activeContentError: null as string | null,
		loadNavTree: vi.fn(),
		loadContent: vi.fn(),
		invalidateNavTree: vi.fn(),
	};
}

function makeMockConversationStore() {
	return {
		messages: [],
		isStreaming: false,
		isLoading: false,
		error: null as string | null,
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
	};
}

function makeMockArtifactGraphSDK() {
	return {
		graph: { size: 0 },
		loading: false,
		error: null as string | null,
		refresh: vi.fn().mockResolvedValue(undefined),
		resolveByPath: vi.fn().mockReturnValue(undefined),
		resolve: vi.fn().mockReturnValue(undefined),
		referencesTo: vi.fn().mockReturnValue([]),
		referencesFrom: vi.fn().mockReturnValue([]),
		byType: vi.fn().mockReturnValue([]),
		pathIndex: new Map<string, string>(),
		getTraceability: vi.fn().mockResolvedValue(null),
		runIntegrityScan: vi.fn().mockResolvedValue([]),
		storeHealthSnapshot: vi.fn().mockResolvedValue(undefined),
		getHealthSnapshots: vi.fn().mockResolvedValue([]),
		getGraphHealth: vi.fn().mockResolvedValue(null),
		applyAutoFixes: vi.fn().mockResolvedValue([]),
		initialize: vi.fn().mockResolvedValue(undefined),
	};
}

function makeMockPluginRegistry() {
	return {
		activeSidecar: null as { label: string } | null,
		activeSidecarKey: null as string | null,
		sidecarProviders: [] as Array<{ key: string; label: string }>,
		sidecars: [] as Array<{ id: string; label: string }>,
		allSchemas: [],
		checkConflicts: vi.fn().mockReturnValue([]),
		register: vi.fn(),
		setAlias: vi.fn(),
		setActiveSidecar: vi.fn(),
		getPlugin: vi.fn().mockReturnValue(null),
	};
}

function makeMockPluginStore() {
	return {
		installed: [],
		cliToolStatuses: [],
		error: null as string | null,
		loadingRegistry: false,
		loadInstalled: vi.fn().mockResolvedValue(undefined),
		loadCliToolStatuses: vi.fn().mockResolvedValue(undefined),
		listRegistry: vi.fn().mockResolvedValue([]),
		installFromGitHub: vi.fn().mockResolvedValue(undefined),
		installFromLocal: vi.fn().mockResolvedValue(undefined),
		uninstall: vi.fn().mockResolvedValue(undefined),
		runCliTool: vi.fn().mockResolvedValue({ exit_code: 0, stdout: "", stderr: "" }),
		getManifest: vi.fn().mockResolvedValue(null),
	};
}

function makeMockSetupStore() {
	return {
		setupComplete: true,
		cliInfo: null,
		checkSetupStatus: vi.fn().mockResolvedValue(undefined),
		checkCli: vi.fn().mockResolvedValue(undefined),
		checkAuth: vi.fn().mockResolvedValue(undefined),
		reauthenticate: vi.fn().mockResolvedValue(undefined),
	};
}

function makeMockEnforcementStore() {
	return {
		rules: [],
		violationHistory: [],
		loadRules: vi.fn(),
		loadViolationHistory: vi.fn(),
	};
}

function makeMockErrorStore() {
	return {
		initialize: vi.fn(),
		destroy: vi.fn(),
		initBrowserHandlers: vi.fn(),
	};
}

function makeMockToast() {
	return {
		success: vi.fn(),
		error: vi.fn(),
		info: vi.fn(),
		warning: vi.fn(),
	};
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/**
 * Builds a full mock stores object and installs it on globalThis.__orqa_stores.
 * Pass partial overrides to customise individual stores for a specific test.
 * @param overrides - Partial store overrides to merge into the default stubs.
 * @returns The installed mock stores object.
 */
export function installMockStores(overrides: Partial<OrqaStores> = {}): OrqaStores {
	const stores = {
		settingsStore: makeMockSettingsStore(),
		sessionStore: makeMockSessionStore(),
		navigationStore: makeMockNavigationStore(),
		projectStore: makeMockProjectStore(),
		artifactStore: makeMockArtifactStore(),
		conversationStore: makeMockConversationStore(),
		artifactGraphSDK: makeMockArtifactGraphSDK(),
		pluginRegistry: makeMockPluginRegistry(),
		pluginStore: makeMockPluginStore(),
		setupStore: makeMockSetupStore(),
		enforcementStore: makeMockEnforcementStore(),
		errorStore: makeMockErrorStore(),
		lessonStore: {} as any,
		toastStore: {} as any,
		toast: makeMockToast(),
		...overrides,
	} as unknown as OrqaStores;

	(globalThis as any).__orqa_stores = stores;
	return stores;
}

/** Removes the mock stores from globalThis, resetting to an uninitialised state. */
export function clearMockStores(): void {
	delete (globalThis as any).__orqa_stores;
}
