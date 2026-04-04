export interface ResolvedTheme {
	readonly project_id: number;
	readonly tokens: Readonly<Record<string, ThemeToken>>;
	readonly source_files: readonly string[];
	readonly has_overrides: boolean;
}

export interface ThemeToken {
	readonly name: string;
	readonly value_light: string;
	readonly value_dark: string | null;
	readonly source: "extracted" | "override" | "default";
}

export interface SidecarStatus {
	readonly state: SidecarState;
	readonly pid: number | null;
	readonly uptime_seconds: number | null;
	readonly cli_detected: boolean;
	readonly cli_version: string | null;
	readonly error_message: string | null;
}

export type SidecarState = "not_started" | "starting" | "connected" | "error" | "stopped";

export interface StartupTask {
	readonly id: string;
	readonly label: string;
	readonly status: "pending" | "in_progress" | "done" | "error";
	readonly detail: string | null;
}

export interface StartupSnapshot {
	readonly tasks: readonly StartupTask[];
	readonly all_done: boolean;
}
