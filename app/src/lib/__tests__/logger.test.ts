/**
 * Tests for the @orqastudio/logger module.
 *
 * The logger emits to console, forwards to the dev dashboard (fire-and-forget
 * HTTP), and notifies subscribers. Tests focus on:
 * - Subscriber notification with correct LogEntry shape
 * - Level filtering (setLogLevel gates console output, not subscriber delivery)
 * - Unsubscription
 * - Logger scoping (source tag)
 *
 * Network requests (forwardToDashboard / forwardToDaemonBus) are fire-and-
 * forget and always silently fail in jsdom — no assertions needed there.
 */

import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import {
	logger,
	subscribeToLogs,
	setLogLevel,
	type LogEntry,
	type LogLevel,
} from "@orqastudio/logger";

// ---------------------------------------------------------------------------
// Silence console noise for tests that intentionally emit log output.
// ---------------------------------------------------------------------------

beforeEach(() => {
	vi.spyOn(console, "log").mockImplementation(() => {});
	vi.spyOn(console, "warn").mockImplementation(() => {});
	vi.spyOn(console, "error").mockImplementation(() => {});
	vi.spyOn(console, "debug").mockImplementation(() => {});
	// Reset to default level before each test
	setLogLevel("info");
});

afterEach(() => {
	vi.restoreAllMocks();
});

// ---------------------------------------------------------------------------
// logger() — scoping and subscriber delivery
// ---------------------------------------------------------------------------

describe("logger", () => {
	it("creates a logger that tags entries with the given source", () => {
		const received: LogEntry[] = [];
		const unsub = subscribeToLogs((e) => received.push(e));

		const log = logger("navigation");
		log.info("navigated");
		unsub();

		expect(received[0].source).toBe("navigation");
		expect(received[0].message).toBe("navigated");
	});

	it("delivers info entries to subscribers", () => {
		const received: LogEntry[] = [];
		const unsub = subscribeToLogs((e) => received.push(e));

		logger("test").info("hello");
		unsub();

		expect(received).toHaveLength(1);
		expect(received[0].level).toBe("info");
	});

	it("delivers warn entries to subscribers", () => {
		const received: LogEntry[] = [];
		const unsub = subscribeToLogs((e) => received.push(e));

		logger("test").warn("careful");
		unsub();

		expect(received[0].level).toBe("warn");
		expect(received[0].message).toBe("careful");
	});

	it("delivers error entries to subscribers", () => {
		const received: LogEntry[] = [];
		const unsub = subscribeToLogs((e) => received.push(e));

		logger("test").error("boom");
		unsub();

		expect(received[0].level).toBe("error");
	});

	it("delivers debug entries to subscribers (subscribers always receive)", () => {
		const received: LogEntry[] = [];
		const unsub = subscribeToLogs((e) => received.push(e));

		setLogLevel("info"); // debug below the console threshold
		logger("test").debug("verbose");
		unsub();

		// Subscribers receive all entries regardless of minLevel
		expect(received).toHaveLength(1);
		expect(received[0].level).toBe("debug");
	});

	it("delivers perf entries to subscribers", () => {
		const received: LogEntry[] = [];
		const unsub = subscribeToLogs((e) => received.push(e));

		logger("test").perf("operation");
		unsub();

		expect(received[0].level).toBe("perf");
		expect(received[0].message).toContain("operation");
	});

	it("includes a numeric timestamp in each entry", () => {
		const received: LogEntry[] = [];
		const unsub = subscribeToLogs((e) => received.push(e));
		const before = Date.now();

		logger("test").info("timestamped");
		unsub();

		const after = Date.now();
		expect(received[0].timestamp).toBeGreaterThanOrEqual(before);
		expect(received[0].timestamp).toBeLessThanOrEqual(after);
	});

	it("includes extra data when provided", () => {
		const received: LogEntry[] = [];
		const unsub = subscribeToLogs((e) => received.push(e));

		logger("test").info("with data", { foo: "bar" });
		unsub();

		expect(received[0].data).toEqual([{ foo: "bar" }]);
	});

	it("omits data field when no extra args are passed", () => {
		const received: LogEntry[] = [];
		const unsub = subscribeToLogs((e) => received.push(e));

		logger("test").info("no data");
		unsub();

		expect(received[0].data).toBeUndefined();
	});
});

// ---------------------------------------------------------------------------
// perf / perfAsync
// ---------------------------------------------------------------------------

describe("logger.perf", () => {
	it("emits a perf entry without a function (label-only call)", () => {
		const received: LogEntry[] = [];
		const unsub = subscribeToLogs((e) => received.push(e));

		logger("test").perf("my-label");
		unsub();

		expect(received[0].level).toBe("perf");
		expect(received[0].message).toBe("my-label");
	});

	it("emits a perf entry with timing suffix when a function is provided", () => {
		const received: LogEntry[] = [];
		const unsub = subscribeToLogs((e) => received.push(e));

		logger("test").perf("my-op", () => {
			// Synchronous work
		});
		unsub();

		// Message should contain timing info: "my-op (Xms)"
		expect(received[0].message).toMatch(/^my-op \(\d+\.\dms\)$/);
	});

	it("perfAsync emits a perf entry after the async function resolves", async () => {
		const received: LogEntry[] = [];
		const unsub = subscribeToLogs((e) => received.push(e));

		await logger("test").perfAsync("async-op", async () => "result");
		unsub();

		expect(received[0].level).toBe("perf");
		expect(received[0].message).toMatch(/^async-op \(\d+\.\dms\)$/);
	});

	it("perfAsync returns the resolved value from the function", async () => {
		const result = await logger("test").perfAsync("sum", async () => 42);
		expect(result).toBe(42);
	});
});

// ---------------------------------------------------------------------------
// subscribeToLogs — lifecycle
// ---------------------------------------------------------------------------

describe("subscribeToLogs", () => {
	it("unsubscribe removes the subscriber", () => {
		const received: LogEntry[] = [];
		const unsub = subscribeToLogs((e) => received.push(e));
		unsub();

		logger("test").info("after unsubscribe");

		expect(received).toHaveLength(0);
	});

	it("multiple subscribers all receive the same entry", () => {
		const received1: LogEntry[] = [];
		const received2: LogEntry[] = [];
		const unsub1 = subscribeToLogs((e) => received1.push(e));
		const unsub2 = subscribeToLogs((e) => received2.push(e));

		logger("test").info("broadcast");

		unsub1();
		unsub2();

		expect(received1).toHaveLength(1);
		expect(received2).toHaveLength(1);
		expect(received1[0]).toBe(received2[0]);
	});

	it("a crashing subscriber does not break other subscribers", () => {
		const received: LogEntry[] = [];
		const unsub1 = subscribeToLogs(() => {
			throw new Error("subscriber crash");
		});
		const unsub2 = subscribeToLogs((e) => received.push(e));

		// Should not throw
		expect(() => logger("test").warn("resilience")).not.toThrow();
		expect(received).toHaveLength(1);

		unsub1();
		unsub2();
	});
});

// ---------------------------------------------------------------------------
// setLogLevel — console gating
// ---------------------------------------------------------------------------

describe("setLogLevel", () => {
	it("debug level does not produce console output when minLevel is info", () => {
		setLogLevel("info");
		const spy = vi.spyOn(console, "debug");

		const unsub = subscribeToLogs(() => {});
		logger("test").debug("silent");
		unsub();

		expect(spy).not.toHaveBeenCalled();
	});

	it("debug level produces console output when minLevel is debug", () => {
		setLogLevel("debug");
		const spy = vi.spyOn(console, "debug");

		const unsub = subscribeToLogs(() => {});
		logger("test").debug("verbose");
		unsub();

		expect(spy).toHaveBeenCalled();
	});

	it("warn level suppresses info from console output", () => {
		setLogLevel("warn");
		const spy = vi.spyOn(console, "log");

		const unsub = subscribeToLogs(() => {});
		logger("test").info("suppressed info");
		unsub();

		expect(spy).not.toHaveBeenCalled();
	});
});
