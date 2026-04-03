import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { mockInvoke } from "./setup.js";

import { SettingsStore } from "../../src/stores/settings.svelte.js";
import type { SidecarStatus } from "@orqastudio/types";

// Stub window.matchMedia for Node test environment
if (typeof window !== "undefined" && typeof window.matchMedia !== "function") {
	(window as Record<string, unknown>).matchMedia = vi.fn().mockReturnValue({
		matches: false,
		addEventListener: vi.fn(),
		removeEventListener: vi.fn(),
	});
}

let settingsStore: SettingsStore;

beforeEach(() => {
	mockInvoke.mockReset();
	settingsStore = new SettingsStore();
});

afterEach(() => {
	settingsStore.destroy();
});

describe("SettingsStore", () => {
	describe("initial state", () => {
		it("starts with default values", () => {
			expect(settingsStore.themeMode).toBe("system");
			expect(settingsStore.defaultModel).toBe("auto");
			expect(settingsStore.fontSize).toBe(14);
			expect(settingsStore.lastSessionId).toBeNull();
			expect(settingsStore.loading).toBe(false);
			expect(settingsStore.error).toBeNull();
		});

		it("sidecarStatus starts as not_started", () => {
			expect(settingsStore.sidecarStatus.state).toBe("not_started");
		});

		it("daemonHealth starts disconnected", () => {
			expect(settingsStore.daemonHealth.state).toBe("disconnected");
			expect(settingsStore.daemonHealth.artifacts).toBe(0);
			expect(settingsStore.daemonHealth.rules).toBe(0);
			expect(settingsStore.daemonHealth.error).toBeNull();
		});
	});

	describe("setThemeMode", () => {
		it("updates theme mode and persists", async () => {
			mockInvoke.mockResolvedValueOnce(undefined);

			await settingsStore.setThemeMode("dark");

			expect(settingsStore.themeMode).toBe("dark");
			expect(mockInvoke).toHaveBeenCalledWith("settings_set", {
				key: "theme_mode",
				value: "dark",
				scope: "app",
			});
		});

		it("sets error on persist failure but still updates local state", async () => {
			mockInvoke.mockRejectedValueOnce(new Error("Persist failed"));

			await settingsStore.setThemeMode("light");

			expect(settingsStore.themeMode).toBe("light");
			expect(settingsStore.error).toBe("Persist failed");
		});
	});

	describe("setDefaultModel", () => {
		it("updates model and persists", async () => {
			mockInvoke.mockResolvedValueOnce(undefined);

			await settingsStore.setDefaultModel("claude-opus-4-6");

			expect(settingsStore.defaultModel).toBe("claude-opus-4-6");
			expect(mockInvoke).toHaveBeenCalledWith("settings_set", {
				key: "default_model",
				value: "claude-opus-4-6",
				scope: "app",
			});
		});
	});

	describe("setFontSize", () => {
		it("clamps font size between 12 and 20", async () => {
			mockInvoke.mockResolvedValue(undefined);

			await settingsStore.setFontSize(8);
			expect(settingsStore.fontSize).toBe(12);

			await settingsStore.setFontSize(25);
			expect(settingsStore.fontSize).toBe(20);

			await settingsStore.setFontSize(16);
			expect(settingsStore.fontSize).toBe(16);
		});
	});

	describe("setActiveSection", () => {
		it("updates activeSection", () => {
			settingsStore.setActiveSection("model");
			expect(settingsStore.activeSection).toBe("model");
		});
	});

	describe("derived: modelDisplayName", () => {
		it("maps model IDs to display names", () => {
			settingsStore.defaultModel = "auto";
			expect(settingsStore.modelDisplayName).toBe("Auto");

			settingsStore.defaultModel = "claude-opus-4-6";
			expect(settingsStore.modelDisplayName).toBe("Opus");

			settingsStore.defaultModel = "claude-sonnet-4-6";
			expect(settingsStore.modelDisplayName).toBe("Sonnet");

			settingsStore.defaultModel = "claude-haiku-4-5";
			expect(settingsStore.modelDisplayName).toBe("Haiku");
		});
	});

	describe("derived: daemonStateLabel", () => {
		it("maps daemon states to labels", () => {
			settingsStore.daemonHealth = { ...settingsStore.daemonHealth, state: "connected" };
			expect(settingsStore.daemonStateLabel).toBe("Connected");

			settingsStore.daemonHealth = { ...settingsStore.daemonHealth, state: "degraded" };
			expect(settingsStore.daemonStateLabel).toBe("Degraded");

			settingsStore.daemonHealth = { ...settingsStore.daemonHealth, state: "disconnected" };
			expect(settingsStore.daemonStateLabel).toBe("Offline");
		});
	});

	describe("derived: daemonConnected", () => {
		it("is true only when daemon state is connected", () => {
			settingsStore.daemonHealth = { ...settingsStore.daemonHealth, state: "connected" };
			expect(settingsStore.daemonConnected).toBe(true);

			settingsStore.daemonHealth = { ...settingsStore.daemonHealth, state: "degraded" };
			expect(settingsStore.daemonConnected).toBe(false);

			settingsStore.daemonHealth = { ...settingsStore.daemonHealth, state: "disconnected" };
			expect(settingsStore.daemonConnected).toBe(false);
		});
	});

	describe("derived: sidecarStateLabel", () => {
		it("maps sidecar states to labels", () => {
			settingsStore.sidecarStatus = { ...settingsStore.sidecarStatus, state: "connected" };
			expect(settingsStore.sidecarStateLabel).toBe("Claude Code");

			settingsStore.sidecarStatus = { ...settingsStore.sidecarStatus, state: "starting" };
			expect(settingsStore.sidecarStateLabel).toBe("Claude Code (Starting)");

			settingsStore.sidecarStatus = { ...settingsStore.sidecarStatus, state: "error" };
			expect(settingsStore.sidecarStateLabel).toBe("Claude Code (Offline)");

			settingsStore.sidecarStatus = { ...settingsStore.sidecarStatus, state: "stopped" };
			expect(settingsStore.sidecarStateLabel).toBe("Claude Code (Offline)");

			settingsStore.sidecarStatus = { ...settingsStore.sidecarStatus, state: "not_started" };
			expect(settingsStore.sidecarStateLabel).toBe("Claude Code (Offline)");
		});
	});

	describe("derived: sidecarConnected", () => {
		it("is true only when state is connected", () => {
			settingsStore.sidecarStatus = { ...settingsStore.sidecarStatus, state: "connected" };
			expect(settingsStore.sidecarConnected).toBe(true);

			settingsStore.sidecarStatus = { ...settingsStore.sidecarStatus, state: "starting" };
			expect(settingsStore.sidecarConnected).toBe(false);
		});
	});

	describe("derived: startupDone", () => {
		it("is false when startupStatus is null", () => {
			expect(settingsStore.startupDone).toBe(false);
		});

		it("reflects all_done from startup snapshot", () => {
			settingsStore.startupStatus = { all_done: true, tasks: [] };
			expect(settingsStore.startupDone).toBe(true);

			settingsStore.startupStatus = { all_done: false, tasks: [] };
			expect(settingsStore.startupDone).toBe(false);
		});
	});

	describe("derived: activeStartupTask", () => {
		it("returns the in_progress task", () => {
			settingsStore.startupStatus = {
				all_done: false,
				tasks: [
					{ id: "t1", label: "Task 1", status: "done", detail: null },
					{ id: "t2", label: "Task 2", status: "in_progress", detail: null },
					{ id: "t3", label: "Task 3", status: "pending", detail: null },
				],
			};

			expect(settingsStore.activeStartupTask?.id).toBe("t2");
		});

		it("returns null when no task is in progress", () => {
			settingsStore.startupStatus = {
				all_done: true,
				tasks: [{ id: "t1", label: "Task 1", status: "done", detail: null }],
			};

			expect(settingsStore.activeStartupTask).toBeNull();
		});
	});

	describe("refreshSidecarStatus", () => {
		it("updates sidecar status from backend", async () => {
			const status: SidecarStatus = {
				state: "connected",
				pid: 1234,
				uptime_seconds: 60,
				cli_detected: true,
				cli_version: "1.0.0",
				error_message: null,
			};
			// startupStatus is null — first call is get_startup_status
			mockInvoke
				.mockResolvedValueOnce({ all_done: true, tasks: [] })
				.mockResolvedValueOnce(status);

			await settingsStore.refreshSidecarStatus();

			expect(settingsStore.sidecarStatus).toEqual(status);
		});

		it("sets error state on sidecar status failure", async () => {
			settingsStore.startupStatus = { all_done: true, tasks: [] };
			mockInvoke.mockRejectedValueOnce(new Error("Connection refused"));

			await settingsStore.refreshSidecarStatus();

			expect(settingsStore.sidecarStatus.state).toBe("error");
			expect(settingsStore.sidecarStatus.error_message).toBe("Connection refused");
		});
	});

	describe("restartSidecar", () => {
		it("updates status on success", async () => {
			const status: SidecarStatus = {
				state: "starting",
				pid: 5678,
				uptime_seconds: 0,
				cli_detected: true,
				cli_version: "1.0.0",
				error_message: null,
			};
			mockInvoke.mockResolvedValueOnce(status);

			await settingsStore.restartSidecar();

			expect(mockInvoke).toHaveBeenCalledWith("sidecar_restart", undefined);
			expect(settingsStore.sidecarStatus).toEqual(status);
		});

		it("sets error state on failure", async () => {
			mockInvoke.mockRejectedValueOnce(new Error("Restart failed"));

			await settingsStore.restartSidecar();

			expect(settingsStore.sidecarStatus.state).toBe("error");
		});
	});

	describe("refreshDaemonHealth", () => {
		it("transitions to connected when daemon returns ok", async () => {
			mockInvoke.mockResolvedValueOnce({ status: "ok", artifacts: 42, rules: 7 });

			await settingsStore.refreshDaemonHealth();

			expect(settingsStore.daemonHealth.state).toBe("connected");
			expect(settingsStore.daemonHealth.artifacts).toBe(42);
			expect(settingsStore.daemonHealth.rules).toBe(7);
			expect(settingsStore.daemonHealth.error).toBeNull();
		});

		it("transitions to degraded when daemon returns non-ok status", async () => {
			mockInvoke.mockResolvedValueOnce({ status: "partial", artifacts: 10, rules: 0 });

			await settingsStore.refreshDaemonHealth();

			expect(settingsStore.daemonHealth.state).toBe("degraded");
			expect(settingsStore.daemonHealth.artifacts).toBe(10);
		});

		it("transitions to disconnected on error", async () => {
			mockInvoke.mockRejectedValueOnce(new Error("ECONNREFUSED"));

			await settingsStore.refreshDaemonHealth();

			expect(settingsStore.daemonHealth.state).toBe("disconnected");
			expect(settingsStore.daemonHealth.error).toBe("ECONNREFUSED");
		});
	});

	describe("initialize idempotency", () => {
		it("does not call backend twice when initialize is called multiple times", async () => {
			const themeCallback = vi.fn();
			mockInvoke
				.mockResolvedValueOnce({}) // settings_get_all
				.mockResolvedValueOnce({ all_done: true, tasks: [] }) // get_startup_status
				.mockResolvedValueOnce({ state: "not_started", pid: null, uptime_seconds: null, cli_detected: false, cli_version: null, error_message: null }) // sidecar_status
				.mockResolvedValueOnce({ status: "ok", artifacts: 0, rules: 0 }); // daemon_health (called at end of initialize)

			await settingsStore.initialize({ onThemeChange: themeCallback });
			await settingsStore.initialize({ onThemeChange: themeCallback });

			// 4 invoke calls total on first initialize (settings_get_all, get_startup_status, sidecar_status, daemon_health)
			// second initialize is a no-op due to idempotency guard
			expect(mockInvoke).toHaveBeenCalledTimes(4);
		});
	});

	describe("onThemeChange callback", () => {
		it("calls onThemeChange when provided via initialize", async () => {
			const themeCallback = vi.fn();
			mockInvoke
				.mockResolvedValueOnce({}) // settings_get_all
				.mockResolvedValueOnce({ all_done: true, tasks: [] }) // get_startup_status
				.mockResolvedValueOnce({ state: "not_started", pid: null, uptime_seconds: null, cli_detected: false, cli_version: null, error_message: null }); // sidecar_status

			await settingsStore.initialize({ onThemeChange: themeCallback });

			// initialize calls applyTheme with the current mode ("system")
			expect(themeCallback).toHaveBeenCalledWith("system");

			// Now set a new theme — callback should be called again
			mockInvoke.mockResolvedValueOnce(undefined);
			await settingsStore.setThemeMode("dark");
			expect(themeCallback).toHaveBeenCalledWith("dark");
		});
	});
});
