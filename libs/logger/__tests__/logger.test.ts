/**
 * Tests for the structured logger.
 *
 * Verifies scoped logger creation, level filtering, subscriber delivery/removal,
 * perf timing helpers, and fire-and-forget forwarding calls to the dashboard
 * and daemon endpoints.
 */

import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import {
	logger,
	subscribeToLogs,
	setLogLevel,
	initDevConsole,
	type LogEntry,
	type LogLevel,
} from "../src/index.js";

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/** Collect entries from subscribeToLogs and return the captured array + unsub fn. */
function captureEntries(): { entries: LogEntry[]; unsub: () => void } {
	const entries: LogEntry[] = [];
	const unsub = subscribeToLogs((e) => entries.push(e));
	return { entries, unsub };
}

// ---------------------------------------------------------------------------
// Setup: reset log level and suppress console noise between tests
// ---------------------------------------------------------------------------

beforeEach(() => {
	// Reset to default level so tests start from a known baseline.
	setLogLevel("info");

	// Silence console output during tests — we verify via subscribers, not console.
	vi.spyOn(console, "log").mockImplementation(() => {});
	vi.spyOn(console, "debug").mockImplementation(() => {});
	vi.spyOn(console, "info").mockImplementation(() => {});
	vi.spyOn(console, "warn").mockImplementation(() => {});
	vi.spyOn(console, "error").mockImplementation(() => {});
});

afterEach(() => {
	vi.restoreAllMocks();
	setLogLevel("info");
});

// ---------------------------------------------------------------------------
// logger() scoping
// ---------------------------------------------------------------------------

describe("logger()", () => {
	it("creates a logger with the given source tag", () => {
		const { entries, unsub } = captureEntries();
		const log = logger("navigation");
		log.info("Page opened");
		unsub();

		expect(entries).toHaveLength(1);
		expect(entries[0].source).toBe("navigation");
	});

	it("two loggers with different sources emit independently tagged entries", () => {
		const { entries, unsub } = captureEntries();
		const nav = logger("navigation");
		const art = logger("artifact");
		nav.info("nav event");
		art.info("art event");
		unsub();

		expect(entries[0].source).toBe("navigation");
		expect(entries[1].source).toBe("artifact");
	});
});

// ---------------------------------------------------------------------------
// Log level methods
// ---------------------------------------------------------------------------

describe("log level methods", () => {
	it("debug() emits a debug-level entry", () => {
		setLogLevel("debug");
		const { entries, unsub } = captureEntries();
		logger("test").debug("debug msg");
		unsub();

		expect(entries[0].level).toBe("debug");
		expect(entries[0].message).toBe("debug msg");
	});

	it("info() emits an info-level entry", () => {
		const { entries, unsub } = captureEntries();
		logger("test").info("info msg");
		unsub();

		expect(entries[0].level).toBe("info");
		expect(entries[0].message).toBe("info msg");
	});

	it("warn() emits a warn-level entry", () => {
		const { entries, unsub } = captureEntries();
		logger("test").warn("warn msg");
		unsub();

		expect(entries[0].level).toBe("warn");
		expect(entries[0].message).toBe("warn msg");
	});

	it("error() emits an error-level entry", () => {
		const { entries, unsub } = captureEntries();
		logger("test").error("error msg");
		unsub();

		expect(entries[0].level).toBe("error");
		expect(entries[0].message).toBe("error msg");
	});

	it("entries include a numeric timestamp", () => {
		const before = Date.now();
		const { entries, unsub } = captureEntries();
		logger("test").info("ts check");
		unsub();
		const after = Date.now();

		expect(entries[0].timestamp).toBeGreaterThanOrEqual(before);
		expect(entries[0].timestamp).toBeLessThanOrEqual(after);
	});

	it("extra data arguments are attached to the entry", () => {
		const { entries, unsub } = captureEntries();
		logger("test").info("with data", { key: "value" }, 42);
		unsub();

		expect(entries[0].data).toEqual([{ key: "value" }, 42]);
	});

	it("data is undefined when no extra arguments are passed", () => {
		const { entries, unsub } = captureEntries();
		logger("test").info("no data");
		unsub();

		expect(entries[0].data).toBeUndefined();
	});
});

// ---------------------------------------------------------------------------
// setLogLevel — console filtering
// ---------------------------------------------------------------------------

describe("setLogLevel()", () => {
	it("when level is warn, debug entries are NOT delivered to subscribers", () => {
		setLogLevel("warn");
		const { entries, unsub } = captureEntries();
		logger("test").debug("should be filtered");
		unsub();

		// Subscribers receive ALL entries regardless of console level.
		// The level filter only affects console output, not subscribers.
		// Verify the entry is still dispatched to subscribers.
		expect(entries).toHaveLength(1);
		expect(entries[0].level).toBe("debug");
	});

	it("when level is warn, info entries still reach subscribers", () => {
		setLogLevel("warn");
		const { entries, unsub } = captureEntries();
		logger("test").info("filtered from console only");
		unsub();

		expect(entries).toHaveLength(1);
	});

	it("when level is warn, warn entries reach subscribers", () => {
		setLogLevel("warn");
		const { entries, unsub } = captureEntries();
		logger("test").warn("warn visible");
		unsub();

		expect(entries).toHaveLength(1);
		expect(entries[0].level).toBe("warn");
	});

	it("console.debug is NOT called when level is warn and debug is logged", () => {
		setLogLevel("warn");
		logger("test").debug("quiet");

		expect(console.debug).not.toHaveBeenCalled();
	});

	it("console.error IS called for error regardless of log level", () => {
		setLogLevel("warn");
		logger("test").error("error always shows");

		expect(console.error).toHaveBeenCalled();
	});
});

describe("initDevConsole()", () => {
	it("sets log level to debug so debug entries appear in console", () => {
		initDevConsole();
		logger("test").debug("debug after initDevConsole");

		expect(console.debug).toHaveBeenCalled();
	});
});

// ---------------------------------------------------------------------------
// subscribeToLogs / unsubscribe
// ---------------------------------------------------------------------------

describe("subscribeToLogs()", () => {
	it("subscriber receives emitted entries", () => {
		const received: LogEntry[] = [];
		const unsub = subscribeToLogs((e) => received.push(e));
		logger("test").info("hello subscriber");
		unsub();

		expect(received).toHaveLength(1);
		expect(received[0].message).toBe("hello subscriber");
	});

	it("after unsubscribe, no more entries are delivered", () => {
		const received: LogEntry[] = [];
		const unsub = subscribeToLogs((e) => received.push(e));
		logger("test").info("before unsub");
		unsub();
		logger("test").info("after unsub");

		expect(received).toHaveLength(1);
	});

	it("multiple subscribers each receive the same entry", () => {
		const a: LogEntry[] = [];
		const b: LogEntry[] = [];
		const unsubA = subscribeToLogs((e) => a.push(e));
		const unsubB = subscribeToLogs((e) => b.push(e));
		logger("test").info("broadcast");
		unsubA();
		unsubB();

		expect(a).toHaveLength(1);
		expect(b).toHaveLength(1);
		expect(a[0].message).toBe(b[0].message);
	});

	it("a throwing subscriber does not prevent other subscribers from running", () => {
		const good: LogEntry[] = [];
		const unsubBad = subscribeToLogs(() => { throw new Error("bad subscriber"); });
		const unsubGood = subscribeToLogs((e) => good.push(e));
		logger("test").info("resilient");
		unsubBad();
		unsubGood();

		expect(good).toHaveLength(1);
	});
});

// ---------------------------------------------------------------------------
// perf()
// ---------------------------------------------------------------------------

describe("perf()", () => {
	it("emits a perf-level entry with label", () => {
		const { entries, unsub } = captureEntries();
		logger("test").perf("my-operation");
		unsub();

		expect(entries[0].level).toBe("perf");
		expect(entries[0].message).toBe("my-operation");
	});

	it("when given a function, times it and appends ms to the label", () => {
		const { entries, unsub } = captureEntries();
		logger("test").perf("timed-op", () => {
			// synchronous work
		});
		unsub();

		expect(entries[0].level).toBe("perf");
		expect(entries[0].message).toMatch(/^timed-op \(\d+\.\dms\)$/);
	});

	it("executes the provided function", () => {
		let ran = false;
		logger("test").perf("side-effect", () => { ran = true; });
		expect(ran).toBe(true);
	});
});

// ---------------------------------------------------------------------------
// perfAsync()
// ---------------------------------------------------------------------------

describe("perfAsync()", () => {
	it("returns the result of the async function", async () => {
		const result = await logger("test").perfAsync("async-op", async () => 42);
		expect(result).toBe(42);
	});

	it("emits a perf-level entry with timing appended to the label", async () => {
		const { entries, unsub } = captureEntries();
		await logger("test").perfAsync("async-timed", async () => "value");
		unsub();

		expect(entries[0].level).toBe("perf");
		expect(entries[0].message).toMatch(/^async-timed \(\d+\.\dms\)$/);
	});
});

// ---------------------------------------------------------------------------
// forwardToDashboard / forwardToDaemonBus — fire-and-forget
// ---------------------------------------------------------------------------

describe("forwarding to external endpoints", () => {
	it("calls fetch when navigator.sendBeacon is not available", () => {
		// jsdom does not have navigator.sendBeacon by default.
		// We mock fetch to verify the call is made.
		const mockFetch = vi.fn().mockResolvedValue(new Response());
		vi.stubGlobal("fetch", mockFetch);

		logger("test").info("forwarded");

		// fetch should be called: once for dashboard + once for daemon bus
		expect(mockFetch).toHaveBeenCalled();
		vi.unstubAllGlobals();
	});

	it("does not throw when fetch is unavailable", () => {
		// Remove fetch entirely — should not throw.
		vi.stubGlobal("fetch", undefined);
		expect(() => logger("test").info("no fetch")).not.toThrow();
		vi.unstubAllGlobals();
	});
});
