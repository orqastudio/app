/**
 * Tests for ProviderSettings.svelte.
 *
 * ProviderSettings renders the sidecar status card, CLI status card, and
 * provider switcher. It auto-checks CLI state on mount and exposes
 * re-authenticate and re-check actions.
 */

import { describe, it, expect, beforeEach, afterEach, vi } from "vitest";
import { render, screen, fireEvent, waitFor } from "@testing-library/svelte";
import { installMockStores, clearMockStores } from "../../shared/__tests__/mock-stores.js";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
vi.mock("@tauri-apps/api/event", () => ({ listen: vi.fn().mockResolvedValue(() => {}) }));

import ProviderSettings from "../ProviderSettings.svelte";

describe("ProviderSettings", () => {
	let stores: ReturnType<typeof installMockStores>;

	beforeEach(() => {
		stores = installMockStores({
			setupStore: {
				...({} as any),
				setupComplete: true,
				cliInfo: {
					version: "1.0.0",
					path: "/usr/local/bin/claude",
					authenticated: true,
				},
				checkSetupStatus: vi.fn().mockResolvedValue(undefined),
				checkCli: vi.fn().mockResolvedValue(undefined),
				checkAuth: vi.fn().mockResolvedValue(undefined),
				reauthenticate: vi.fn().mockResolvedValue(undefined),
			} as any,
		});
	});

	afterEach(() => {
		clearMockStores();
	});

	it("auto-checks CLI when cliInfo is null on mount", async () => {
		installMockStores({
			setupStore: {
				...({} as any),
				setupComplete: true,
				cliInfo: null, // triggers auto-check
				checkSetupStatus: vi.fn().mockResolvedValue(undefined),
				checkCli: vi.fn().mockResolvedValue(undefined),
				checkAuth: vi.fn().mockResolvedValue(undefined),
				reauthenticate: vi.fn().mockResolvedValue(undefined),
			} as any,
		});
		render(ProviderSettings);
		await new Promise((r) => setTimeout(r, 10));
		// checkCli should have been called on mount
		expect((globalThis as any).__orqa_stores.setupStore.checkCli).toHaveBeenCalled();
	});
});
