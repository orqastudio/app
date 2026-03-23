/**
 * Test setup for store unit tests.
 *
 * Mocks `@tauri-apps/api/core` so stores can be tested without a running Tauri app.
 * Each test file should call `mockInvoke.mockReset()` in beforeEach
 * to ensure clean mock state between tests.
 */
import { vi } from "vitest";

// ---------------------------------------------------------------------------
// Mock @tauri-apps/api/core
// ---------------------------------------------------------------------------

// The mock invoke function — tests configure return values via mockResolvedValue / mockRejectedValue
const mockInvoke = vi.fn();

// Minimal Channel mock that captures onmessage callback
class MockChannel<T> {
	onmessage: ((event: T) => void) | null = null;

	/** Helper for tests to simulate events from the backend */
	emit(event: T) {
		if (this.onmessage) {
			this.onmessage(event);
		}
	}
}

vi.mock("@tauri-apps/api/core", () => ({
	invoke: mockInvoke,
	Channel: MockChannel,
}));

// ---------------------------------------------------------------------------
// Mock @tauri-apps/api/event (used by artifact-graph SDK)
// ---------------------------------------------------------------------------

vi.mock("@tauri-apps/api/event", () => ({
	listen: vi.fn().mockResolvedValue(() => {}),
}));

export { mockInvoke, MockChannel };
