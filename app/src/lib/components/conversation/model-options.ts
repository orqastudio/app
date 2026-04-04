// Canonical list of available Claude models and the default model selection.
// All UI components that need model options import from here.

export interface ModelOption {
	readonly value: string;
	readonly label: string;
	readonly description: string;
}

/** The "auto" sentinel value lets the backend choose the best model for each request. */
export const AUTO_MODEL_OPTION: ModelOption = {
	value: "auto",
	label: "Auto (recommended)",
	description: "Automatically selects the best model",
};

export const CLAUDE_MODEL_OPTIONS: ModelOption[] = [
	AUTO_MODEL_OPTION,
	{ value: "claude-opus-4-6", label: "Opus", description: "Most capable, slower" },
	{ value: "claude-sonnet-4-6", label: "Sonnet", description: "Balanced performance" },
	{ value: "claude-haiku-4-5", label: "Haiku", description: "Fastest responses" },
];

export const DEFAULT_MODEL = "claude-sonnet-4-6";
