import { invoke, extractErrorMessage } from "../ipc/invoke.js";
import { logger } from "../logger.js";
import type { SidecarStatus, StartupSnapshot, StartupTask } from "@orqastudio/types";
import { assertNever } from "@orqastudio/types";

const log = logger("settings");

export type ThemeMode = "light" | "dark" | "system";
export type DefaultModel = "auto" | "claude-opus-4-6" | "claude-sonnet-4-6" | "claude-haiku-4-5";

export type DaemonState = "connected" | "disconnected" | "degraded";

export interface DaemonHealth {
	state: DaemonState;
	artifacts: number;
	rules: number;
	error: string | null;
}

function defaultApplyTheme(mode: ThemeMode): void {
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

export class SettingsStore {
	themeMode = $state<ThemeMode>("system");
	defaultModel = $state<DefaultModel>("auto");
	fontSize = $state<number>(14);
	lastSessionId = $state<number | null>(null);
	activeSection = $state<string>("provider");

	sidecarStatus = $state<SidecarStatus>({
		state: "not_started",
		pid: null,
		uptime_seconds: null,
		cli_detected: false,
		cli_version: null,
		error_message: null,
	});

	daemonHealth = $state<DaemonHealth>({
		state: "disconnected",
		artifacts: 0,
		rules: 0,
		error: null,
	});

	loading = $state(false);
	error = $state<string | null>(null);
	startupStatus = $state<StartupSnapshot | null>(null);

	private _initialized = false;
	private _pollIntervalId: ReturnType<typeof setInterval> | null = null;
	private _daemonPollIntervalId: ReturnType<typeof setInterval> | null = null;
	private _mediaQueryCleanup: (() => void) | null = null;
	private _onThemeChange: ((mode: ThemeMode) => void) | null = null;

	async initialize(options?: { onThemeChange?: (mode: ThemeMode) => void }): Promise<void> {
		if (this._initialized) return;
		this._initialized = true;

		// Store the callback. If none provided, use the default browser DOM manipulation.
		this._onThemeChange = options?.onThemeChange ?? null;

		await this.loadAllSettings();
		await this.refreshSidecarStatus();

		log.info("settings initialized", {
			theme: this.themeMode,
			model: this.defaultModel,
			fontSize: this.fontSize,
		});

		// Apply theme on init
		this.applyTheme(this.themeMode);

		// Listen for system theme changes when in "system" mode
		if (typeof window !== "undefined") {
			const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
			const handler = () => {
				if (this.themeMode === "system") {
					this.applyTheme("system");
				}
			};
			mediaQuery.addEventListener("change", handler);
			this._mediaQueryCleanup = () => mediaQuery.removeEventListener("change", handler);
		}

		// Start sidecar status polling (every 5 seconds)
		this._pollIntervalId = setInterval(() => {
			this.refreshSidecarStatus();
		}, 5000);

		// Start daemon health polling (every 10 seconds)
		this.refreshDaemonHealth();
		this._daemonPollIntervalId = setInterval(() => {
			this.refreshDaemonHealth();
		}, 10_000);
	}

	private applyTheme(mode: ThemeMode): void {
		if (this._onThemeChange) {
			this._onThemeChange(mode);
		} else {
			defaultApplyTheme(mode);
		}
	}

	destroy(): void {
		if (this._pollIntervalId !== null) {
			clearInterval(this._pollIntervalId);
			this._pollIntervalId = null;
		}
		if (this._daemonPollIntervalId !== null) {
			clearInterval(this._daemonPollIntervalId);
			this._daemonPollIntervalId = null;
		}
		if (this._mediaQueryCleanup) {
			this._mediaQueryCleanup();
			this._mediaQueryCleanup = null;
		}
		this._onThemeChange = null;
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

			if (typeof all["last_session_id"] === "number" && all["last_session_id"] > 0) {
				this.lastSessionId = all["last_session_id"];
			}
		} catch (err) {
			log.error("failed to load settings", err);
			this.error = extractErrorMessage(err);
		} finally {
			this.loading = false;
		}
	}

	async setThemeMode(mode: ThemeMode): Promise<void> {
		this.themeMode = mode;
		this.applyTheme(mode);

		try {
			await invoke("settings_set", {
				key: "theme_mode",
				value: mode,
				scope: "app",
			});
		} catch (err) {
			log.error("failed to persist theme mode", err);
			this.error = extractErrorMessage(err);
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
			log.error("failed to persist default model", err);
			this.error = extractErrorMessage(err);
		}
	}

	async setFontSize(size: number): Promise<void> {
		this.fontSize = Math.max(12, Math.min(20, size));

		try {
			await invoke("settings_set", {
				key: "font_size",
				value: this.fontSize,
				scope: "app",
			});
		} catch (err) {
			log.error("failed to persist font size", err);
			this.error = extractErrorMessage(err);
		}
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
			} catch (err: unknown) {
				log.error("failed to check startup status", err);
				const message = extractErrorMessage(err);
				this.error = `Failed to check startup status: ${message}`;
			}
		}

		const prevState = this.sidecarStatus.state;
		try {
			const status = await invoke<SidecarStatus>("sidecar_status");
			if (status.state !== prevState) {
				log.info("sidecar state transition", { state: status.state, pid: status.pid });
			}
			this.sidecarStatus = status;
		} catch (err) {
			log.error("failed to poll sidecar status", err);
			this.sidecarStatus = {
				state: "error",
				pid: null,
				uptime_seconds: null,
				cli_detected: false,
				cli_version: null,
				error_message: extractErrorMessage(err),
			};
		}
	}

	async restartSidecar(): Promise<void> {
		try {
			const status = await invoke<SidecarStatus>("sidecar_restart");
			this.sidecarStatus = status;
		} catch (err) {
			log.error("failed to restart sidecar", err);
			this.sidecarStatus = {
				state: "error",
				pid: null,
				uptime_seconds: null,
				cli_detected: false,
				cli_version: null,
				error_message: extractErrorMessage(err),
			};
		}
	}

	async refreshDaemonHealth(): Promise<void> {
		const prevState = this.daemonHealth.state;
		try {
			const data = await invoke<{
				status: string;
				artifacts: number;
				rules: number;
			}>("daemon_health");

			if (data.status === "ok") {
				const next: DaemonHealth = {
					state: "connected",
					artifacts: data.artifacts,
					rules: data.rules,
					error: null,
				};
				if (next.state !== prevState) {
					log.info("daemon health transition", { state: next.state, artifacts: next.artifacts });
				}
				this.daemonHealth = next;
			} else {
				const next: DaemonHealth = {
					state: "degraded",
					artifacts: data.artifacts ?? 0,
					rules: data.rules ?? 0,
					error: `Unexpected status: ${data.status}`,
				};
				if (next.state !== prevState) {
					log.info("daemon health transition", { state: next.state, artifacts: next.artifacts });
				}
				this.daemonHealth = next;
			}
		} catch (err: unknown) {
			const next: DaemonHealth = {
				state: "disconnected",
				artifacts: 0,
				rules: 0,
				error: extractErrorMessage(err),
			};
			if (next.state !== prevState) {
				log.info("daemon health transition", { state: next.state, artifacts: next.artifacts });
			}
			this.daemonHealth = next;
		}
	}

	get daemonStateLabel(): string {
		switch (this.daemonHealth.state) {
			case "connected":
				return "Connected";
			case "degraded":
				return "Degraded";
			case "disconnected":
				return "Offline";
			default:
				return assertNever(this.daemonHealth.state);
		}
	}

	get daemonConnected(): boolean {
		return this.daemonHealth.state === "connected";
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
				return assertNever(this.defaultModel);
		}
	}

	get sidecarStateLabel(): string {
		switch (this.sidecarStatus.state) {
			case "connected":
				return "Claude Code";
			case "starting":
				return "Claude Code (Starting)";
			case "error":
				return "Claude Code (Offline)";
			case "stopped":
				return "Claude Code (Offline)";
			case "not_started":
				return "Claude Code (Offline)";
			default:
				return assertNever(this.sidecarStatus.state);
		}
	}

	get sidecarConnected(): boolean {
		return this.sidecarStatus.state === "connected";
	}
}
