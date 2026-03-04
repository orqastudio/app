import { invoke } from "$lib/ipc/invoke";
import type { SidecarStatus, StartupSnapshot, StartupTask } from "$lib/types/settings";

export type ThemeMode = "light" | "dark" | "system";
export type DefaultModel = "auto" | "claude-opus-4-6" | "claude-sonnet-4-6" | "claude-haiku-4-5";

function applyThemeToDocument(mode: ThemeMode): void {
	if (typeof document === "undefined") return;

	if (mode === "dark") {
		document.documentElement.classList.add("dark");
	} else if (mode === "light") {
		document.documentElement.classList.remove("dark");
	} else {
		// System mode: follow OS preference
		const prefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
		if (prefersDark) {
			document.documentElement.classList.add("dark");
		} else {
			document.documentElement.classList.remove("dark");
		}
	}
}

class SettingsStore {
	themeMode = $state<ThemeMode>("system");
	defaultModel = $state<DefaultModel>("auto");
	fontSize = $state<number>(14);
	activeSection = $state<string>("provider");

	sidecarStatus = $state<SidecarStatus>({
		state: "not_started",
		pid: null,
		uptime_seconds: null,
		cli_detected: false,
		cli_version: null,
		error_message: null,
	});

	loading = $state(false);
	error = $state<string | null>(null);
	startupStatus = $state<StartupSnapshot | null>(null);

	private _initialized = false;
	private _pollIntervalId: ReturnType<typeof setInterval> | null = null;
	private _mediaQueryCleanup: (() => void) | null = null;

	async initialize(): Promise<void> {
		if (this._initialized) return;
		this._initialized = true;

		await this.loadAllSettings();
		await this.refreshSidecarStatus();

		// Apply theme on init
		applyThemeToDocument(this.themeMode);

		// Listen for system theme changes when in "system" mode
		if (typeof window !== "undefined") {
			const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
			const handler = () => {
				if (this.themeMode === "system") {
					applyThemeToDocument("system");
				}
			};
			mediaQuery.addEventListener("change", handler);
			this._mediaQueryCleanup = () => mediaQuery.removeEventListener("change", handler);
		}

		// Start sidecar status polling (every 5 seconds)
		this._pollIntervalId = setInterval(() => {
			this.refreshSidecarStatus();
		}, 5000);
	}

	destroy(): void {
		if (this._pollIntervalId !== null) {
			clearInterval(this._pollIntervalId);
			this._pollIntervalId = null;
		}
		if (this._mediaQueryCleanup) {
			this._mediaQueryCleanup();
			this._mediaQueryCleanup = null;
		}
		this._initialized = false;
	}

	private async loadAllSettings(): Promise<void> {
		this.loading = true;
		this.error = null;

		try {
			const all = await invoke<Record<string, unknown>>("settings_get_all", {
				scope: "app",
			});

			if (all["theme_mode"] && typeof all["theme_mode"] === "string") {
				const mode = all["theme_mode"] as ThemeMode;
				if (mode === "light" || mode === "dark" || mode === "system") {
					this.themeMode = mode;
				}
			}

			if (all["default_model"] && typeof all["default_model"] === "string") {
				const model = all["default_model"] as DefaultModel;
				if (
					model === "auto" ||
					model === "claude-opus-4-6" ||
					model === "claude-sonnet-4-6" ||
					model === "claude-haiku-4-5"
				) {
					this.defaultModel = model;
				}
			}

			if (all["font_size"] && typeof all["font_size"] === "number") {
				this.fontSize = Math.max(12, Math.min(20, all["font_size"]));
			}
		} catch (err) {
			this.error = err instanceof Error ? err.message : String(err);
		} finally {
			this.loading = false;
		}
	}

	async setThemeMode(mode: ThemeMode): Promise<void> {
		this.themeMode = mode;
		applyThemeToDocument(mode);

		try {
			await invoke("settings_set", {
				key: "theme_mode",
				value: mode,
				scope: "app",
			});
		} catch (err) {
			this.error = err instanceof Error ? err.message : String(err);
		}
	}

	async setDefaultModel(model: DefaultModel): Promise<void> {
		this.defaultModel = model;

		try {
			await invoke("settings_set", {
				key: "default_model",
				value: model,
				scope: "app",
			});
		} catch (err) {
			this.error = err instanceof Error ? err.message : String(err);
		}
	}

	setFontSize(size: number) {
		this.fontSize = Math.max(12, Math.min(20, size));
	}

	setActiveSection(section: string) {
		this.activeSection = section;
	}

	get startupDone(): boolean {
		return this.startupStatus?.all_done ?? false;
	}

	get activeStartupTask(): StartupTask | null {
		return this.startupStatus?.tasks.find((t) => t.status === "in_progress") ?? null;
	}

	async refreshSidecarStatus(): Promise<void> {
		// Poll startup status until all tasks are done
		if (!this.startupDone) {
			try {
				const status = await invoke<StartupSnapshot>("get_startup_status");
				this.startupStatus = status;
			} catch {
				// Startup status is best-effort; don't fail the poll
			}
		}

		try {
			const status = await invoke<SidecarStatus>("sidecar_status");
			this.sidecarStatus = status;
		} catch (err) {
			this.sidecarStatus = {
				state: "error",
				pid: null,
				uptime_seconds: null,
				cli_detected: false,
				cli_version: null,
				error_message: err instanceof Error ? err.message : String(err),
			};
		}
	}

	async restartSidecar(): Promise<void> {
		try {
			const status = await invoke<SidecarStatus>("sidecar_restart");
			this.sidecarStatus = status;
		} catch (err) {
			this.sidecarStatus = {
				state: "error",
				pid: null,
				uptime_seconds: null,
				cli_detected: false,
				cli_version: null,
				error_message: err instanceof Error ? err.message : String(err),
			};
		}
	}

	get modelDisplayName(): string {
		switch (this.defaultModel) {
			case "auto":
				return "Auto";
			case "claude-opus-4-6":
				return "Opus";
			case "claude-sonnet-4-6":
				return "Sonnet";
			case "claude-haiku-4-5":
				return "Haiku";
			default:
				return "Auto";
		}
	}

	get sidecarStateLabel(): string {
		switch (this.sidecarStatus.state) {
			case "connected":
				return "Connected";
			case "starting":
				return "Starting";
			case "error":
				return "Error";
			case "stopped":
				return "Disconnected";
			case "not_started":
				return "Disconnected";
			default:
				return "Unknown";
		}
	}

	get sidecarConnected(): boolean {
		return this.sidecarStatus.state === "connected";
	}
}

export const settingsStore = new SettingsStore();
