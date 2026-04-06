/**
 * Tauri invoke mock factory and setup utilities.
 *
 * Provides two approaches:
 * - `createMockInvoke()` — returns a standalone mock for manual wiring
 * - `setupTauriMocks()` — calls vi.mock() on `@tauri-apps/api/core` and
 *   `@tauri-apps/api/event`, returning the mock invoke for test configuration
 *
 * Pattern matches the existing orqa-studio setup.ts mock structure.
 */
import { vi } from "vitest";
import type { Mock } from "vitest";
import { MockChannel } from "./channel.js";

/**
 * Create a standalone mock invoke function.
 * Useful when you need to control the mock without vi.mock() side effects.
 * @returns An object containing mockInvoke (the mock function) and reset (clears recorded calls).
 */
export function createMockInvoke(): { mockInvoke: Mock; reset: () => void } {
	const mockInvoke = vi.fn();
	return { mockInvoke, reset: () => mockInvoke.mockReset() };
}

/**
 * Set up full Tauri API mocks via vi.mock().
 *
 * Mocks:
 * - `@tauri-apps/api/core` — invoke + Channel
 * - `@tauri-apps/api/event` — listen + emit
 *
 * Returns the mock invoke function so tests can configure return values.
 *
 * Usage in a test setup file:
 * ```ts
 * import { setupTauriMocks } from "@orqastudio/test-config/mocks";
 * const { mockInvoke } = setupTauriMocks();
 * ```
 * @returns An object containing mockInvoke for configuring per-command return values.
 */
export function setupTauriMocks(): { mockInvoke: Mock } {
	const mockInvoke = vi.fn();

	vi.mock("@tauri-apps/api/core", () => ({
		invoke: mockInvoke,
		Channel: MockChannel,
	}));

	vi.mock("@tauri-apps/api/event", () => ({
		listen: vi.fn().mockResolvedValue(() => {}),
		emit: vi.fn(),
	}));

	return { mockInvoke };
}
