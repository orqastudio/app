export interface SetupStatus {
	readonly setup_complete: boolean;
	readonly current_version: number;
	readonly stored_version: number;
	readonly steps: readonly SetupStepStatus[];
}

export interface SetupStepStatus {
	readonly id: string;
	readonly label: string;
	readonly status: StepStatus;
	readonly detail: string | null;
}

export type StepStatus = "pending" | "checking" | "complete" | "error" | "action_required";

export interface ClaudeCliInfo {
	readonly installed: boolean;
	readonly version: string | null;
	readonly path: string | null;
	readonly authenticated: boolean;
	readonly subscription_type: string | null;
	readonly rate_limit_tier: string | null;
	readonly scopes: readonly string[];
	readonly expires_at: number | null;
}
