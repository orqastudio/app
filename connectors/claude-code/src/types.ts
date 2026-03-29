/**
 * Claude Code connector types.
 *
 * These are specific to the Claude Code hook system — the JSON contract
 * between Claude Code and hook scripts. Core types (artifacts, schemas,
 * enforcement) come from `@orqastudio/types`.
 */

/** JSON input passed to hooks via stdin by Claude Code. */
export interface HookInput {
	tool_name?: string;
	tool_input?: HookToolInput;
	cwd?: string;
	content?: string;
	response?: string;
	/** UserPromptSubmit: the user's message text. */
	user_message?: string;
	/** UserPromptSubmit: alias for user_message used in some hook payloads. */
	prompt?: string;
	/** SubagentStop / UserPromptSubmit: agent role identifier. */
	agent_type?: string;
}

/** Tool input fields that may be present depending on the tool. */
export interface HookToolInput {
	command?: string;
	file_path?: string;
	content?: string;
	old_string?: string;
	new_string?: string;
	pattern?: string;
	search?: string;
}

/** Hook output that blocks the tool call (written to stderr, exit 2). */
export interface HookBlockOutput {
	hookSpecificOutput: { permissionDecision: "deny" };
	systemMessage: string;
}

/** Hook output that warns but allows (written to stdout, exit 0). */
export interface HookWarnOutput {
	systemMessage: string;
}

/** Telemetry event details. */
export interface TelemetryDetails {
	[key: string]: unknown;
}
