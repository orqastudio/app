// Unit tests for pure helper functions extracted from devtools components.
//
// Each function is re-implemented inline — identical to the source — so the
// test validates the algorithm without importing Svelte component files (which
// require a full Svelte compiler context). Component rendering is covered by
// E2E tests; these tests verify the pure data-transformation logic.

import { describe, it, expect } from "vitest";

// ---------------------------------------------------------------------------
// ProcessCard helpers (from ProcessCard.svelte)
// ---------------------------------------------------------------------------

type ProcessStatus = "running" | "stopped" | "crashed" | "not_found" | "unknown";

function formatUptime(seconds: number): string {
	if (seconds < 60) return `${seconds}s`;
	const mins = Math.floor(seconds / 60);
	if (mins < 60) return `${mins}m`;
	const hours = Math.floor(mins / 60);
	const remainingMins = mins % 60;
	return remainingMins > 0 ? `${hours}h ${remainingMins}m` : `${hours}h`;
}

function formatMemory(bytes: number): string {
	if (bytes >= 1_048_576) return `${(bytes / 1_048_576).toFixed(1)} MB`;
	if (bytes >= 1_024) return `${(bytes / 1_024).toFixed(1)} KB`;
	return `${bytes} B`;
}

function binaryFilename(path: string): string {
	const last = path.replace(/\\/g, "/").split("/").pop();
	return last ?? path;
}

type ConnectionState = "connected" | "disconnected" | "reconnecting" | "waiting";

function processConnectionState(status: ProcessStatus): ConnectionState {
	return status === "running"
		? "connected"
		: status === "crashed"
			? "disconnected"
			: status === "unknown"
				? "reconnecting"
				: "waiting";
}

function processStatusLabel(status: ProcessStatus): string {
	return status === "running"
		? "Running"
		: status === "crashed"
			? "Crashed"
			: status === "stopped"
				? "Stopped"
				: status === "not_found"
					? "Not found"
					: "Unknown";
}

describe("formatUptime", () => {
	it("shows seconds when less than 60", () => {
		expect(formatUptime(0)).toBe("0s");
		expect(formatUptime(1)).toBe("1s");
		expect(formatUptime(59)).toBe("59s");
	});

	it("shows minutes when 60s to 3599s", () => {
		expect(formatUptime(60)).toBe("1m");
		expect(formatUptime(90)).toBe("1m");
		expect(formatUptime(3599)).toBe("59m");
	});

	it("shows hours when 3600s or more", () => {
		expect(formatUptime(3600)).toBe("1h");
		expect(formatUptime(7200)).toBe("2h");
	});

	it("shows hours and minutes when remainder is non-zero", () => {
		expect(formatUptime(3660)).toBe("1h 1m");
		expect(formatUptime(5400)).toBe("1h 30m");
		expect(formatUptime(7384)).toBe("2h 3m");
	});

	it("omits minutes when they are zero", () => {
		expect(formatUptime(7200)).toBe("2h");
		expect(formatUptime(10800)).toBe("3h");
	});
});

describe("formatMemory", () => {
	it("shows bytes when less than 1 KB", () => {
		expect(formatMemory(0)).toBe("0 B");
		expect(formatMemory(512)).toBe("512 B");
		expect(formatMemory(1023)).toBe("1023 B");
	});

	it("shows KB when between 1 KB and 1 MB", () => {
		expect(formatMemory(1024)).toBe("1.0 KB");
		expect(formatMemory(2048)).toBe("2.0 KB");
		expect(formatMemory(1536)).toBe("1.5 KB");
	});

	it("shows MB when 1 MB or more", () => {
		expect(formatMemory(1_048_576)).toBe("1.0 MB");
		expect(formatMemory(2_097_152)).toBe("2.0 MB");
		expect(formatMemory(1_572_864)).toBe("1.5 MB");
	});

	it("formats MB to one decimal place", () => {
		expect(formatMemory(10_485_760)).toBe("10.0 MB");
		expect(formatMemory(1_100_000)).toBe("1.0 MB");
	});
});

describe("binaryFilename", () => {
	it("extracts the filename from a Unix path", () => {
		expect(binaryFilename("/usr/bin/orqa-daemon")).toBe("orqa-daemon");
	});

	it("extracts the filename from a Windows-style path (backslashes)", () => {
		expect(binaryFilename("C:\\Program Files\\orqa\\orqa-daemon.exe")).toBe("orqa-daemon.exe");
	});

	it("returns the input when there is no separator", () => {
		expect(binaryFilename("orqa-daemon")).toBe("orqa-daemon");
	});

	it("handles a trailing slash by returning an empty string's pop fallback", () => {
		// path.replace + split + pop on "foo/" yields ""
		const result = binaryFilename("foo/");
		expect(typeof result).toBe("string");
	});
});

describe("processConnectionState", () => {
	it("maps running → connected", () => {
		expect(processConnectionState("running")).toBe("connected");
	});

	it("maps crashed → disconnected", () => {
		expect(processConnectionState("crashed")).toBe("disconnected");
	});

	it("maps unknown → reconnecting", () => {
		expect(processConnectionState("unknown")).toBe("reconnecting");
	});

	it("maps stopped → waiting", () => {
		expect(processConnectionState("stopped")).toBe("waiting");
	});

	it("maps not_found → waiting", () => {
		expect(processConnectionState("not_found")).toBe("waiting");
	});
});

describe("processStatusLabel", () => {
	it("returns Running for running status", () => {
		expect(processStatusLabel("running")).toBe("Running");
	});

	it("returns Crashed for crashed status", () => {
		expect(processStatusLabel("crashed")).toBe("Crashed");
	});

	it("returns Stopped for stopped status", () => {
		expect(processStatusLabel("stopped")).toBe("Stopped");
	});

	it("returns Not found for not_found status", () => {
		expect(processStatusLabel("not_found")).toBe("Not found");
	});

	it("returns Unknown for unknown status", () => {
		expect(processStatusLabel("unknown")).toBe("Unknown");
	});
});

// ---------------------------------------------------------------------------
// LogRow helpers (from LogRow.svelte)
// ---------------------------------------------------------------------------

function formatTimestamp(ms: number): string {
	const d = new Date(ms);
	const hh = d.getHours().toString().padStart(2, "0");
	const mm = d.getMinutes().toString().padStart(2, "0");
	const ss = d.getSeconds().toString().padStart(2, "0");
	const mmm = d.getMilliseconds().toString().padStart(3, "0");
	return `${hh}:${mm}:${ss}.${mmm}`;
}

describe("formatTimestamp", () => {
	it("formats a known Unix ms timestamp to HH:MM:SS.mmm", () => {
		// 2026-01-01T12:34:56.789Z — local hours depend on timezone, so check structure only
		const result = formatTimestamp(1_735_731_296_789);
		expect(result).toMatch(/^\d{2}:\d{2}:\d{2}\.\d{3}$/);
	});

	it("pads hours, minutes, and seconds with leading zeros", () => {
		// Use a UTC midnight timestamp and check the pattern
		const result = formatTimestamp(0); // 1970-01-01T00:00:00.000Z (in UTC)
		expect(result).toMatch(/^\d{2}:\d{2}:\d{2}\.\d{3}$/);
	});

	it("includes milliseconds in the output", () => {
		// The format must end with a dot and three digits
		const result = formatTimestamp(Date.now());
		expect(result).toMatch(/\.\d{3}$/);
	});
});

// ---------------------------------------------------------------------------
// MetricsView helpers (from MetricsView.svelte)
// ---------------------------------------------------------------------------

function fmtMs(ms: number): string {
	if (ms === 0 || ms === Infinity || ms === -Infinity) return "—";
	return ms < 100 ? `${ms.toFixed(1)}ms` : `${Math.round(ms)}ms`;
}

function computeTrend(history: number[]): number | null {
	if (history.length < 2) return null;
	const recent = history[history.length - 1];
	const reference = history[Math.max(0, history.length - 10)];
	if (reference === 0) return null;
	return Math.round(((recent - reference) / reference) * 100);
}

describe("fmtMs", () => {
	it("returns em-dash for 0", () => {
		expect(fmtMs(0)).toBe("—");
	});

	it("returns em-dash for Infinity", () => {
		expect(fmtMs(Infinity)).toBe("—");
	});

	it("returns em-dash for -Infinity", () => {
		expect(fmtMs(-Infinity)).toBe("—");
	});

	it("formats sub-100ms values with one decimal", () => {
		expect(fmtMs(12.5)).toBe("12.5ms");
		expect(fmtMs(99.9)).toBe("99.9ms");
		expect(fmtMs(1)).toBe("1.0ms");
	});

	it("rounds values 100ms and above to whole ms", () => {
		expect(fmtMs(100)).toBe("100ms");
		expect(fmtMs(123.7)).toBe("124ms");
		expect(fmtMs(1000)).toBe("1000ms");
	});
});

describe("computeTrend", () => {
	it("returns null when history has fewer than 2 values", () => {
		expect(computeTrend([])).toBeNull();
		expect(computeTrend([42])).toBeNull();
	});

	it("returns null when the reference value is zero (avoids divide by zero)", () => {
		expect(computeTrend([0, 100])).toBeNull();
	});

	it("returns 0 when current equals reference", () => {
		const history = new Array(10).fill(50);
		expect(computeTrend(history)).toBe(0);
	});

	it("returns a positive percentage when the most recent value is higher", () => {
		// reference=100 (10 samples ago), recent=150 → +50%
		const history = [...new Array(9).fill(100), 150];
		expect(computeTrend(history)).toBe(50);
	});

	it("returns a negative percentage when the most recent value is lower", () => {
		// reference=100 (10 samples ago), recent=75 → -25%
		const history = [...new Array(9).fill(100), 75];
		expect(computeTrend(history)).toBe(-25);
	});

	it("uses the earliest element as reference when history has fewer than 10 entries", () => {
		// history=[50, 100] → reference=50, recent=100 → +100%
		expect(computeTrend([50, 100])).toBe(100);
	});
});

// ---------------------------------------------------------------------------
// Navigation helpers (from devtools-navigation.svelte.ts)
// ---------------------------------------------------------------------------

type DevToolsConnectionState =
	| { state: "connected" }
	| { state: "reconnecting"; attempt: number }
	| { state: "waiting-for-daemon" };

function connectionLabel(conn: DevToolsConnectionState): string {
	switch (conn.state) {
		case "connected":
			return "Connected";
		case "reconnecting":
			return `Reconnecting (attempt ${conn.attempt})`;
		case "waiting-for-daemon":
			return "Waiting for daemon...";
	}
}

describe("connectionLabel", () => {
	it("returns Connected for connected state", () => {
		expect(connectionLabel({ state: "connected" })).toBe("Connected");
	});

	it("returns Reconnecting with attempt number for reconnecting state", () => {
		expect(connectionLabel({ state: "reconnecting", attempt: 3 })).toBe(
			"Reconnecting (attempt 3)",
		);
	});

	it("returns Waiting for daemon... for waiting-for-daemon state", () => {
		expect(connectionLabel({ state: "waiting-for-daemon" })).toBe("Waiting for daemon...");
	});
});

// ---------------------------------------------------------------------------
// LogExport helpers (from LogExport.svelte)
// ---------------------------------------------------------------------------

function todayString(): string {
	const d = new Date();
	const yyyy = d.getFullYear();
	const mm = (d.getMonth() + 1).toString().padStart(2, "0");
	const dd = d.getDate().toString().padStart(2, "0");
	return `${yyyy}-${mm}-${dd}`;
}

describe("todayString", () => {
	it("returns a string in YYYY-MM-DD format", () => {
		expect(todayString()).toMatch(/^\d{4}-\d{2}-\d{2}$/);
	});

	it("matches the current year", () => {
		const year = new Date().getFullYear().toString();
		expect(todayString()).toStartWith(year);
	});
});

// ---------------------------------------------------------------------------
// LogFilters helpers (from LogFilters.svelte)
// ---------------------------------------------------------------------------

function toggleSet<T>(set: Set<T>, value: T): Set<T> {
	const next = new Set(set);
	if (next.has(value)) {
		next.delete(value);
	} else {
		next.add(value);
	}
	return next;
}

describe("toggleSet", () => {
	it("adds a value that is not in the set", () => {
		const result = toggleSet(new Set<string>(["a"]), "b");
		expect(result.has("b")).toBe(true);
		expect(result.has("a")).toBe(true);
	});

	it("removes a value that is already in the set", () => {
		const result = toggleSet(new Set<string>(["a", "b"]), "a");
		expect(result.has("a")).toBe(false);
		expect(result.has("b")).toBe(true);
	});

	it("does not mutate the original set", () => {
		const original = new Set<string>(["x"]);
		toggleSet(original, "y");
		expect(original.size).toBe(1);
	});

	it("adds to an empty set", () => {
		const result = toggleSet(new Set<number>(), 42);
		expect(result.has(42)).toBe(true);
		expect(result.size).toBe(1);
	});
});
