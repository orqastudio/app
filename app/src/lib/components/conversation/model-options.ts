export interface ModelOption {
	value: string;
	label: string;
}

export const CLAUDE_MODELS: ModelOption[] = [
	{ value: "claude-opus-4-6", label: "Claude Opus 4" },
	{ value: "claude-sonnet-4-6", label: "Claude Sonnet 4" },
	{ value: "claude-haiku-4-5-20251001", label: "Claude Haiku 4" },
];

export const DEFAULT_MODEL = "claude-sonnet-4-6";
