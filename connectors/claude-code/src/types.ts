/**
 * Claude Code connector types.
 *
 * These are specific to the Claude Code hook system — the JSON contract
 * between Claude Code and hook scripts. Core types (artifacts, schemas,
 * enforcement) come from `@orqastudio/types`.
 */

/** Tool input fields that may be present depending on the tool. */
export interface HookToolInput {
	readonly command?: string;
	readonly file_path?: string;
	readonly content?: string;
	readonly old_string?: string;
	readonly new_string?: string;
	readonly pattern?: string;
	readonly search?: string;
}

/**
 * JSON input passed to hooks via stdin by Claude Code.
 *
 * Claude Code sends different subsets of these fields depending on the event.
 * All fields are optional because each hook script knows its own event type
 * implicitly (by filename) — Claude Code does not inject a discriminant.
 * The connector maps these raw inputs to HookContext (with a typed event field)
 * before forwarding to the daemon.
 */
export interface HookInput {
	readonly tool_name?: string;
	readonly tool_input?: HookToolInput;
	readonly cwd?: string;
	readonly content?: string;
	readonly response?: string;
	/** UserPromptSubmit: the user's message text. */
	readonly user_message?: string;
	/** UserPromptSubmit: alias for user_message used in some hook payloads. */
	readonly prompt?: string;
	/** SubagentStop / UserPromptSubmit: agent role identifier. */
	readonly agent_type?: string;
}

/** Hook output that blocks the tool call (written to stderr, exit 2). */
export interface HookBlockOutput {
	readonly hookSpecificOutput: { readonly permissionDecision: "deny" };
	readonly systemMessage: string;
}

/** Hook output that warns but allows (written to stdout, exit 0). */
export interface HookWarnOutput {
	readonly systemMessage: string;
}

/** Telemetry event details. */
export interface TelemetryDetails {
	[key: string]: unknown;
}
