/**
 * Vitest global setup file.
 *
 * Registers @testing-library/jest-dom matchers and installs a minimal
 * window.__TAURI_INTERNALS__ stub so that SDK code that calls Tauri's
 * `invoke()` (which reads the IPC bridge from that global) does not crash
 * during module import in a jsdom environment.
 *
 * This file is referenced in vite.config.ts → test.setupFiles and runs
 * once before every test suite.
 */

import "@testing-library/jest-dom";

// Tauri v2's core package checks for window.__TAURI_INTERNALS__ to decide
// whether to route IPC calls through the native bridge. Providing a no-op
// stub prevents ReferenceError when SDK modules are imported in tests.
// Individual test files mock @tauri-apps/api/core via vi.mock() for finer
// control over invoke() return values.
Object.defineProperty(window, "__TAURI_INTERNALS__", {
	value: {
		transformCallback: (cb: unknown) => cb,
		invoke: async () => undefined,
		convertFileSrc: (src: string) => src,
	},
	writable: true,
});

// bits-ui's ScrollArea and other layout components use ResizeObserver which is
// not available in jsdom. Provide a no-op stub so components can mount without
// crashing in tests. Tests do not verify scroll behavior.
if (typeof globalThis.ResizeObserver === "undefined") {
	globalThis.ResizeObserver = class ResizeObserver {
		observe() {}
		unobserve() {}
		disconnect() {}
	};
}
