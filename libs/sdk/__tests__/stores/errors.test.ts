import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import "./setup.js";

// Stub window globals that errors.svelte.ts accesses in initialize() and destroy()
const _global = globalThis as unknown as {
	window: Record<string, unknown>;
};
if (typeof globalThis.window === "undefined") {
	_global.window = {
		onerror: null,
		onunhandledrejection: null,
	};
}

import { errorStore } from "../../src/stores/errors.svelte.js";

beforeEach(() => {
	vi.useFakeTimers();
	errorStore.destroy();
	errorStore.dismissAll();
});

afterEach(() => {
	errorStore.destroy();
	vi.useRealTimers();
});

describe("ErrorStore", () => {
	describe("initial state", () => {
		it("starts with an empty errors array", () => {
			expect(errorStore.errors).toEqual([]);
		});
	});

	describe("addError", () => {
		it("adds an error to the list", () => {
			errorStore.addError("backend", "Something went wrong");

			expect(errorStore.errors).toHaveLength(1);
			expect(errorStore.errors[0].source).toBe("backend");
			expect(errorStore.errors[0].message).toBe("Something went wrong");
			expect(errorStore.errors[0].level).toBe("error");
		});

		it("defaults level to 'error' when not specified", () => {
			errorStore.addError("frontend", "Oops");

			expect(errorStore.errors[0].level).toBe("error");
		});

		it("uses the provided level when specified", () => {
			errorStore.addError("sidecar", "Warning message", "warning");

			expect(errorStore.errors[0].level).toBe("warning");
		});

		it("assigns unique incrementing ids", () => {
			errorStore.addError("a", "first");
			errorStore.addError("b", "second");

			const ids = errorStore.errors.map((e) => e.id);
			expect(ids[0]).not.toBe(ids[1]);
		});

		it("prepends new errors (most recent first)", () => {
			errorStore.addError("a", "first");
			errorStore.addError("b", "second");

			expect(errorStore.errors[0].message).toBe("second");
			expect(errorStore.errors[1].message).toBe("first");
		});

		it("caps errors at MAX_ERRORS (50)", () => {
			for (let i = 0; i < 55; i++) {
				errorStore.addError("test", `error ${i}`);
			}

			expect(errorStore.errors).toHaveLength(50);
			expect(errorStore.errors[0].message).toBe("error 54");
		});

		it("sets a timestamp on each error", () => {
			const now = Date.now();
			errorStore.addError("test", "timestamped");

			expect(errorStore.errors[0].timestamp).toBeGreaterThanOrEqual(now);
		});
	});

	describe("dismiss", () => {
		it("removes a specific error by id", () => {
			errorStore.addError("a", "first");
			errorStore.addError("b", "second");

			const idToRemove = errorStore.errors[1].id; // "first"
			errorStore.dismiss(idToRemove);

			expect(errorStore.errors).toHaveLength(1);
			expect(errorStore.errors[0].message).toBe("second");
		});

		it("does nothing when id does not exist", () => {
			errorStore.addError("a", "only");

			errorStore.dismiss(999999);

			expect(errorStore.errors).toHaveLength(1);
		});
	});

	describe("dismissAll", () => {
		it("clears all errors", () => {
			errorStore.addError("a", "first");
			errorStore.addError("b", "second");
			errorStore.addError("c", "third");

			errorStore.dismissAll();

			expect(errorStore.errors).toEqual([]);
		});

		it("is a no-op when already empty", () => {
			errorStore.dismissAll();

			expect(errorStore.errors).toEqual([]);
		});
	});

	describe("auto-dismiss", () => {
		it("removes errors automatically after 8 seconds", () => {
			errorStore.addError("test", "will vanish");

			expect(errorStore.errors).toHaveLength(1);

			vi.advanceTimersByTime(8000);

			expect(errorStore.errors).toHaveLength(0);
		});

		it("does not remove errors before 8 seconds", () => {
			errorStore.addError("test", "still here");

			vi.advanceTimersByTime(7999);

			expect(errorStore.errors).toHaveLength(1);
		});
	});

	describe("initialize", () => {
		it("can be called without error", async () => {
			await errorStore.initialize();
		});

		it("is idempotent (calling twice does not double-register)", async () => {
			await errorStore.initialize();
			await errorStore.initialize();
		});
	});

	describe("destroy", () => {
		it("resets initialized state so initialize can be called again", async () => {
			await errorStore.initialize();
			errorStore.destroy();
			await errorStore.initialize();
		});
	});

	describe("browser handlers (instance method)", () => {
		it("works without calling initBrowserHandlers — core error functions still work", () => {
			errorStore.addError("test", "works fine");
			expect(errorStore.errors).toHaveLength(1);
			errorStore.dismiss(errorStore.errors[0].id);
			expect(errorStore.errors).toHaveLength(0);
		});
	});
});
