/**
 * Tests for IntegrityWidget.svelte.
 *
 * IntegrityWidget shows a "waiting" state before scan, runs an integrity
 * scan via artifactGraphSDK, then displays results in a filterable table.
 *
 * TooltipRoot/TooltipTrigger/TooltipContent are stubbed because IntegrityWidget
 * renders ArtifactLink (connected) which uses Tooltip without a Provider context.
 */

import { describe, it, expect, beforeEach, afterEach, vi } from "vitest";
import { render, screen, waitFor } from "@testing-library/svelte";
import { installMockStores, clearMockStores } from "../../shared/__tests__/mock-stores.js";

// Stub tooltip components to bypass bits-ui Provider context requirement
vi.mock("@orqastudio/svelte-components/pure", async (importActual) => {
	const actual = await importActual<typeof import("@orqastudio/svelte-components/pure")>();
	const { default: TooltipRoot } = await import("../../shared/__tests__/stubs/TooltipRoot.svelte");
	const { default: TooltipTrigger } = await import(
		"../../shared/__tests__/stubs/TooltipTrigger.svelte"
	);
	const { default: TooltipContent } = await import(
		"../../shared/__tests__/stubs/TooltipContent.svelte"
	);
	return { ...actual, TooltipRoot, TooltipTrigger, TooltipContent };
});

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
vi.mock("@tauri-apps/api/event", () => ({ listen: vi.fn().mockResolvedValue(() => {}) }));

import IntegrityWidget from "../IntegrityWidget.svelte";

/**
 * Builds a full artifactGraphSDK mock including all methods used by IntegrityWidget and ArtifactLink.
 * @param overrides - Properties to override in the default mock.
 * @returns The merged mock object.
 */
function makeGraphMock(overrides: Record<string, unknown> = {}): Record<string, unknown> {
	return {
		graph: { size: 0 },
		loading: false,
		error: null,
		refresh: vi.fn().mockResolvedValue(undefined),
		resolveByPath: vi.fn().mockReturnValue(undefined),
		resolve: vi.fn().mockReturnValue(undefined),
		referencesTo: vi.fn().mockReturnValue([]),
		referencesFrom: vi.fn().mockReturnValue([]),
		byType: vi.fn().mockReturnValue([]),
		pathIndex: new Map(),
		getTraceability: vi.fn().mockResolvedValue(null),
		runIntegrityScan: vi.fn().mockResolvedValue([]),
		storeHealthSnapshot: vi.fn().mockResolvedValue(undefined),
		getHealthSnapshots: vi.fn().mockResolvedValue([]),
		getGraphHealth: vi.fn().mockResolvedValue(null),
		applyAutoFixes: vi.fn().mockResolvedValue([]),
		initialize: vi.fn().mockResolvedValue(undefined),
		...overrides,
	};
}

describe("IntegrityWidget", () => {
	beforeEach(() => {
		installMockStores();
	});

	afterEach(() => {
		clearMockStores();
	});

	it("shows waiting text when graph is empty and not scanned", () => {
		render(IntegrityWidget);
		expect(screen.getByText(/Waiting for artifact graph/i)).toBeInTheDocument();
	});

	it("renders 'all clear' when scan returns no checks", async () => {
		installMockStores({
			artifactGraphSDK: makeGraphMock({
				graph: { size: 5 },
				runIntegrityScan: vi.fn().mockResolvedValue([]),
			}),
		});
		render(IntegrityWidget);
		await waitFor(() => {
			expect(screen.getByText("All clear")).toBeInTheDocument();
		});
	});

	it("shows error badge count when scan finds errors", async () => {
		const mockChecks = [
			{
				severity: "Error",
				category: "BrokenLink",
				artifact_id: "EPIC-001",
				message: "Broken link",
				auto_fixable: false,
			},
			{
				severity: "Warning",
				category: "MissingStatus",
				artifact_id: "TASK-002",
				message: "No status",
				auto_fixable: true,
			},
		];
		installMockStores({
			artifactGraphSDK: makeGraphMock({
				graph: { size: 10 },
				runIntegrityScan: vi.fn().mockResolvedValue(mockChecks),
			}),
		});
		render(IntegrityWidget);
		await waitFor(() => {
			expect(screen.getByText("1 Error")).toBeInTheDocument();
			expect(screen.getByText("1 Warning")).toBeInTheDocument();
		});
	});

	it("renders severity filter buttons after scan", async () => {
		const mockChecks = [
			{
				severity: "Error",
				category: "BrokenLink",
				artifact_id: "EPIC-001",
				message: "Broken",
				auto_fixable: false,
			},
		];
		installMockStores({
			artifactGraphSDK: makeGraphMock({
				graph: { size: 5 },
				runIntegrityScan: vi.fn().mockResolvedValue(mockChecks),
			}),
		});
		render(IntegrityWidget);
		await waitFor(() => {
			expect(screen.getByRole("button", { name: /Errors/i })).toBeInTheDocument();
			expect(screen.getByRole("button", { name: /Warnings/i })).toBeInTheDocument();
		});
	});
});
