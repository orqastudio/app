/**
 * Tests for HealthTrendWidget.svelte.
 *
 * HealthTrendWidget fetches health snapshots from artifactGraphSDK and
 * renders sparklines for errors, warnings, orphans, and broken refs.
 * The widget is hidden until at least 2 snapshots exist.
 */

import { describe, it, expect, beforeEach, afterEach, vi } from "vitest";
import { render, screen, waitFor } from "@testing-library/svelte";
import { installMockStores, clearMockStores } from "../../shared/__tests__/mock-stores.js";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
vi.mock("@tauri-apps/api/event", () => ({ listen: vi.fn().mockResolvedValue(() => {}) }));

import HealthTrendWidget from "../HealthTrendWidget.svelte";

const makeSnapshot = (overrides: Record<string, number> = {}) => ({
	id: 1,
	timestamp: new Date().toISOString(),
	error_count: 0,
	warning_count: 0,
	orphan_count: 0,
	broken_ref_count: 0,
	artifact_count: 10,
	...overrides,
});

describe("HealthTrendWidget", () => {
	beforeEach(() => {
		installMockStores();
	});

	afterEach(() => {
		clearMockStores();
	});

	it("renders nothing when fewer than 2 snapshots exist", async () => {
		installMockStores({
			artifactGraphSDK: {
				graph: { size: 5 },
				loading: false,
				error: null,
				refresh: vi.fn(),
				resolveByPath: vi.fn(),
				getTraceability: vi.fn(),
				runIntegrityScan: vi.fn(),
				storeHealthSnapshot: vi.fn(),
				getHealthSnapshots: vi.fn().mockResolvedValue([makeSnapshot()]),
				getGraphHealth: vi.fn(),
				applyAutoFixes: vi.fn(),
				initialize: vi.fn(),
			},
		});
		const { container } = render(HealthTrendWidget);
		await new Promise((r) => setTimeout(r, 10));
		// Widget should be hidden when < 2 snapshots
		expect(container.querySelector("svg")).not.toBeInTheDocument();
	});

	it("renders Health Trends heading when 2+ snapshots exist", async () => {
		installMockStores({
			artifactGraphSDK: {
				graph: { size: 5 },
				loading: false,
				error: null,
				refresh: vi.fn(),
				resolveByPath: vi.fn(),
				getTraceability: vi.fn(),
				runIntegrityScan: vi.fn(),
				storeHealthSnapshot: vi.fn(),
				getHealthSnapshots: vi
					.fn()
					.mockResolvedValue([makeSnapshot({ error_count: 2 }), makeSnapshot({ error_count: 1 })]),
				getGraphHealth: vi.fn(),
				applyAutoFixes: vi.fn(),
				initialize: vi.fn(),
			},
		});
		render(HealthTrendWidget);
		await waitFor(() => {
			expect(screen.getByText("Health Trends")).toBeInTheDocument();
		});
	});

	it("renders sparkline labels: Errors, Warnings, Orphans, Broken Refs", async () => {
		installMockStores({
			artifactGraphSDK: {
				graph: { size: 10 },
				loading: false,
				error: null,
				refresh: vi.fn(),
				resolveByPath: vi.fn(),
				getTraceability: vi.fn(),
				runIntegrityScan: vi.fn(),
				storeHealthSnapshot: vi.fn(),
				getHealthSnapshots: vi
					.fn()
					.mockResolvedValue([
						makeSnapshot({
							error_count: 3,
							warning_count: 1,
							orphan_count: 0,
							broken_ref_count: 2,
						}),
						makeSnapshot({
							error_count: 2,
							warning_count: 0,
							orphan_count: 1,
							broken_ref_count: 1,
						}),
					]),
				getGraphHealth: vi.fn(),
				applyAutoFixes: vi.fn(),
				initialize: vi.fn(),
			},
		});
		render(HealthTrendWidget);
		await waitFor(() => {
			expect(screen.getByText("Errors")).toBeInTheDocument();
			expect(screen.getByText("Warnings")).toBeInTheDocument();
			expect(screen.getByText("Orphans")).toBeInTheDocument();
			expect(screen.getByText("Broken Refs")).toBeInTheDocument();
		});
	});

	it("renders scan count text", async () => {
		installMockStores({
			artifactGraphSDK: {
				graph: { size: 10 },
				loading: false,
				error: null,
				refresh: vi.fn(),
				resolveByPath: vi.fn(),
				getTraceability: vi.fn(),
				runIntegrityScan: vi.fn(),
				storeHealthSnapshot: vi.fn(),
				getHealthSnapshots: vi
					.fn()
					.mockResolvedValue([makeSnapshot(), makeSnapshot(), makeSnapshot()]),
				getGraphHealth: vi.fn(),
				applyAutoFixes: vi.fn(),
				initialize: vi.fn(),
			},
		});
		render(HealthTrendWidget);
		await waitFor(() => {
			expect(screen.getByText("Based on 3 scans")).toBeInTheDocument();
		});
	});
});
