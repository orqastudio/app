/**
 * Tests for StatusBar.svelte.
 *
 * StatusBar reads from settingsStore (sidecar/daemon state, model name),
 * sessionStore (token counts), and artifactGraphSDK (graph size).
 * All IPC is mocked via globalThis.__orqa_stores.
 *
 * TooltipRoot/TooltipTrigger/TooltipContent are stubbed to avoid the
 * bits-ui Tooltip.Provider context requirement.
 */

import { describe, it, expect, beforeEach, afterEach, vi } from "vitest";
import { render, screen } from "@testing-library/svelte";
import { installMockStores, clearMockStores } from "../../shared/__tests__/mock-stores.js";

// Stub tooltip components to bypass bits-ui Provider context requirement
vi.mock("@orqastudio/svelte-components/pure", async (importActual) => {
	const actual = await importActual<typeof import("@orqastudio/svelte-components/pure")>();
	const { default: TooltipRoot } = await import("../../shared/__tests__/stubs/TooltipRoot.svelte");
	const { default: TooltipTrigger } = await import("../../shared/__tests__/stubs/TooltipTrigger.svelte");
	const { default: TooltipContent } = await import("../../shared/__tests__/stubs/TooltipContent.svelte");
	return { ...actual, TooltipRoot, TooltipTrigger, TooltipContent };
});

// Mock Tauri plugin imports that StatusBar's transitive imports may trigger
vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
vi.mock("@tauri-apps/plugin-dialog", () => ({ open: vi.fn() }));
vi.mock("@tauri-apps/api/window", () => ({ getCurrentWindow: vi.fn(() => ({ startDragging: vi.fn(), isMaximized: vi.fn(), maximize: vi.fn(), unmaximize: vi.fn(), close: vi.fn() })) }));
vi.mock("@tauri-apps/api/app", () => ({ getVersion: vi.fn().mockResolvedValue("0.1.0"), getName: vi.fn().mockResolvedValue("OrqaStudio") }));
vi.mock("@tauri-apps/api/event", () => ({ listen: vi.fn().mockResolvedValue(() => {}) }));

// SVG asset mock
vi.mock("$lib/assets/fin-mark.svg", () => ({ default: "fin-mark.svg" }));

import StatusBar from "../StatusBar.svelte";

describe("StatusBar", () => {
	beforeEach(() => {
		installMockStores();
	});

	afterEach(() => {
		clearMockStores();
	});

	it("renders the brand label", () => {
		render(StatusBar);
		expect(screen.getByText("OrqaStudio")).toBeInTheDocument();
	});

	it("renders the model display name", () => {
		installMockStores({
			settingsStore: {
				...({} as any),
				sidecarStatus: { state: "connected" },
				daemonHealth: { state: "connected", artifacts: 0, rules: 0 },
				modelDisplayName: "claude-sonnet-4-6",
				sidecarStateLabel: "Connected",
				daemonStateLabel: "Connected",
				activeStartupTask: null,
				lastSessionId: null,
				initialize: vi.fn(),
				destroy: vi.fn(),
				setActiveSection: vi.fn(),
				refreshDaemonHealth: vi.fn(),
			},
		});
		render(StatusBar);
		expect(screen.getByText("claude-sonnet-4-6")).toBeInTheDocument();
	});

	it("does not render token counter when session has no tokens", () => {
		installMockStores({
			sessionStore: {
				...({} as any),
				activeSession: null,
				sessions: [],
				hasActiveSession: false,
				isLoading: false,
				error: null,
			} as any,
		});
		render(StatusBar);
		// Token counter should not be visible when session is null
		expect(screen.queryByText(/↑.*↓/)).not.toBeInTheDocument();
	});

	it("renders token counter when session has tokens", () => {
		installMockStores({
			sessionStore: {
				...({} as any),
				activeSession: {
					id: 1,
					total_input_tokens: 1500,
					total_output_tokens: 200,
				},
				sessions: [],
				hasActiveSession: true,
				isLoading: false,
				error: null,
			} as any,
		});
		render(StatusBar);
		// 1500 tokens → "1.5k" and 200 → "200"
		expect(screen.getByText(/1\.5k.*200/)).toBeInTheDocument();
	});

	it("renders artifact count from SDK graph size", () => {
		installMockStores({
			artifactGraphSDK: {
				...({} as any),
				graph: { size: 99 },
				loading: false,
				error: null,
				refresh: vi.fn(),
				resolveByPath: vi.fn(),
				resolve: vi.fn(),
				referencesTo: vi.fn().mockReturnValue([]),
				getTraceability: vi.fn(),
				runIntegrityScan: vi.fn(),
				storeHealthSnapshot: vi.fn(),
				getHealthSnapshots: vi.fn(),
				getGraphHealth: vi.fn(),
				applyAutoFixes: vi.fn(),
				initialize: vi.fn(),
			} as any,
		});
		render(StatusBar);
		expect(screen.getByText("99")).toBeInTheDocument();
	});

	it("renders sidecar state label", () => {
		installMockStores({
			settingsStore: {
				...({} as any),
				sidecarStatus: { state: "error", error_message: "Timeout" },
				daemonHealth: { state: "disconnected" },
				modelDisplayName: "Claude Sonnet",
				sidecarStateLabel: "Error",
				daemonStateLabel: "Offline",
				activeStartupTask: null,
				lastSessionId: null,
				initialize: vi.fn(),
				destroy: vi.fn(),
				setActiveSection: vi.fn(),
				refreshDaemonHealth: vi.fn(),
			} as any,
		});
		render(StatusBar);
		expect(screen.getByText("Error")).toBeInTheDocument();
	});

	it("renders startup task label when active", () => {
		installMockStores({
			settingsStore: {
				...({} as any),
				sidecarStatus: { state: "starting" },
				daemonHealth: { state: "disconnected" },
				modelDisplayName: "Claude Sonnet",
				sidecarStateLabel: "Starting",
				daemonStateLabel: "Offline",
				activeStartupTask: { label: "Loading plugins", detail: null },
				lastSessionId: null,
				initialize: vi.fn(),
				destroy: vi.fn(),
				setActiveSection: vi.fn(),
				refreshDaemonHealth: vi.fn(),
			} as any,
		});
		render(StatusBar);
		expect(screen.getByText(/Loading plugins/)).toBeInTheDocument();
	});
});
