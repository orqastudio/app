export interface Session {
	readonly id: number;
	readonly project_id: number;
	readonly title: string | null;
	readonly model: string;
	readonly system_prompt: string | null;
	readonly status: SessionStatus;
	readonly summary: string | null;
	readonly handoff_notes: string | null;
	readonly total_input_tokens: number;
	readonly total_output_tokens: number;
	readonly total_cost_usd: number;
	readonly title_manually_set?: boolean;
	readonly created_at: string;
	readonly updated_at: string;
}

export interface SessionSummary {
	readonly id: number;
	readonly title: string | null;
	readonly status: SessionStatus;
	readonly message_count: number;
	readonly preview: string | null;
	readonly created_at: string;
	readonly updated_at: string;
}

export type SessionStatus = "active" | "completed" | "abandoned" | "error";
