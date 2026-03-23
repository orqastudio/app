/**
 * Mock @tauri-apps/api/event utilities.
 *
 * Provides standalone mock factories for the Tauri event API
 * when you need more control than setupTauriMocks() provides.
 */
import { vi } from "vitest";
import type { Mock } from "vitest";

export interface MockEventApi {
	listen: Mock;
	emit: Mock;
}

/**
 * Create mock event API functions.
 *
 * - `listen` resolves to an unlisten function (also a mock)
 * - `emit` resolves to void
 */
export function createMockEventApi(): MockEventApi {
	const unlisten = vi.fn();
	return {
		listen: vi.fn().mockResolvedValue(unlisten),
		emit: vi.fn().mockResolvedValue(undefined),
	};
}
