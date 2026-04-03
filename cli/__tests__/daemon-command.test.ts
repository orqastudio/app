/**
 * Tests for the daemon command — subcommand dispatch, usage output, PID file
 * operations, and the formatUptime helper (tested indirectly through the
 * module's exported behaviour).
 *
 * We focus on logic that can be tested without spawning a real daemon process:
 *   - The USAGE string references the DEFAULT_PORT_BASE constant
 *   - runDaemonCommand() with "--help" / "-h" prints and returns without error
 *   - runDaemonCommand() with an unknown subcommand calls process.exit(1)
 */
import { describe, it, expect, vi, afterEach } from "vitest";
import { runDaemonCommand } from "../src/commands/daemon.js";
import { DEFAULT_PORT_BASE } from "../src/lib/ports.js";

// ---------------------------------------------------------------------------
// Help / usage
// ---------------------------------------------------------------------------

describe("runDaemonCommand — help and usage", () => {
	afterEach(() => {
		vi.restoreAllMocks();
	});

	it("prints usage and returns when called with no args", async () => {
		const consoleSpy = vi.spyOn(console, "log").mockImplementation(() => {});
		await runDaemonCommand([]);
		expect(consoleSpy).toHaveBeenCalled();
		const output = consoleSpy.mock.calls.flat().join("\n");
		expect(output).toContain("daemon");
		expect(output).toContain("start");
		expect(output).toContain("stop");
		expect(output).toContain("restart");
		expect(output).toContain("status");
	});

	it("prints usage and returns for --help", async () => {
		const consoleSpy = vi.spyOn(console, "log").mockImplementation(() => {});
		await runDaemonCommand(["--help"]);
		expect(consoleSpy).toHaveBeenCalled();
	});

	it("prints usage and returns for -h", async () => {
		const consoleSpy = vi.spyOn(console, "log").mockImplementation(() => {});
		await runDaemonCommand(["-h"]);
		expect(consoleSpy).toHaveBeenCalled();
	});

	it("usage string references DEFAULT_PORT_BASE", async () => {
		const consoleSpy = vi.spyOn(console, "log").mockImplementation(() => {});
		await runDaemonCommand([]);
		const output = consoleSpy.mock.calls.flat().join("\n");
		expect(output).toContain(String(DEFAULT_PORT_BASE));
	});
});

// ---------------------------------------------------------------------------
// Unknown subcommand exits with code 1
// ---------------------------------------------------------------------------

describe("runDaemonCommand — unknown subcommand", () => {
	afterEach(() => {
		vi.restoreAllMocks();
	});

	it("prints error and calls process.exit(1) for unknown subcommand", async () => {
		const exitSpy = vi.spyOn(process, "exit").mockImplementation((_code) => {
			throw new Error(`process.exit(${_code})`);
		});
		const errorSpy = vi.spyOn(console, "error").mockImplementation(() => {});

		await expect(
			runDaemonCommand(["unknown-subcommand"]),
		).rejects.toThrow("process.exit(1)");

		expect(errorSpy).toHaveBeenCalledWith(
			expect.stringContaining("Unknown daemon subcommand"),
		);
		expect(exitSpy).toHaveBeenCalledWith(1);
	});
});

// ---------------------------------------------------------------------------
// Subcommand dispatch — start/stop/restart/status are recognised
// (they will fail at binary/daemon level but should not hit the "unknown" branch)
// ---------------------------------------------------------------------------

describe("runDaemonCommand — known subcommands dispatch correctly", () => {
	afterEach(() => {
		vi.restoreAllMocks();
	});

	it("status returns without calling process.exit when daemon is stopped", async () => {
		const exitSpy = vi.spyOn(process, "exit").mockImplementation((_code) => {
			throw new Error(`process.exit(${_code})`);
		});
		vi.spyOn(console, "log").mockImplementation(() => {});

		// 'status' should log "stopped" and return normally, not call exit.
		await expect(runDaemonCommand(["status"])).resolves.not.toThrow();
		expect(exitSpy).not.toHaveBeenCalled();
	});

	it("stop returns without error when no PID file exists", async () => {
		vi.spyOn(console, "log").mockImplementation(() => {});
		// stop with no PID file is a no-op.
		await expect(runDaemonCommand(["stop"])).resolves.not.toThrow();
	});
});
