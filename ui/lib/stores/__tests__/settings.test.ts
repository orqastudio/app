import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { mockInvoke } from "./setup";

import { settingsStore } from "../settings.svelte";
import type { SidecarStatus, StartupSnapshot } from "$lib/types/settings";

// Mock document/window APIs
const mockClassList = {
	add: vi.fn(),
	remove: vi.fn(),
};

beforeEach(() => {
	mockInvoke.mockReset();
	mockClassList.add.mockReset();
	mockClassList.remove.mockReset();

	// Reset the store to clean state
	settingsStore.themeMode = "system";
	settingsStore.defaultModel = "auto";
	settingsStore.fontSize = 14;
	settingsStore.lastSessionId = null;
	settingsStore.activeSection = "provider";
	settingsStore.loading = false;
	settingsStore.error = null;
	settingsStore.startupStatus = null;
	settingsStore.sidecarStatus = {
		state: "not_started",
		pid: null,
		uptime_seconds: null,
		cli_detected: false,
		cli_version: null,
		error_message: null,
	};

	// Destroy to clear intervals and reset _initialized
	settingsStore.destroy();
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

	describe("derived: sidecarStateLabel", () => {
		it("maps sidecar states to labels", () => {
			settingsStore.sidecarStatus = { ...settingsStore.sidecarStatus, state: "connected" };
			expect(settingsStore.sidecarStateLabel).toBe("Connected");

			settingsStore.sidecarStatus = { ...settingsStore.sidecarStatus, state: "starting" };
			expect(settingsStore.sidecarStateLabel).toBe("Starting");

			settingsStore.sidecarStatus = { ...settingsStore.sidecarStatus, state: "error" };
			expect(settingsStore.sidecarStateLabel).toBe("Error");

			settingsStore.sidecarStatus = { ...settingsStore.sidecarStatus, state: "stopped" };
			expect(settingsStore.sidecarStateLabel).toBe("Disconnected");

			settingsStore.sidecarStatus = { ...settingsStore.sidecarStatus, state: "not_started" };
			expect(settingsStore.sidecarStateLabel).toBe("Disconnected");
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
					{ id: "t1", label: "Task 1", status: "done" },
					{ id: "t2", label: "Task 2", status: "in_progress" },
					{ id: "t3", label: "Task 3", status: "pending" },
				],
			};

			expect(settingsStore.activeStartupTask?.id).toBe("t2");
		});

		it("returns null when no task is in progress", () => {
			settingsStore.startupStatus = {
				all_done: true,
				tasks: [{ id: "t1", label: "Task 1", status: "done" }],
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
			// First call for startup status (startupDone is false by default)
			mockInvoke
				.mockResolvedValueOnce({ all_done: true, tasks: [] }) // get_startup_status
				.mockResolvedValueOnce(status); // sidecar_status

			await settingsStore.refreshSidecarStatus();

			expect(settingsStore.sidecarStatus).toEqual(status);
		});

		it("sets error state on sidecar status failure", async () => {
			// startupDone = true so it skips startup poll
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
});
