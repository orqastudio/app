/**
 * Canonical hook lifecycle types.
 *
 * These define the platform's hook model. Connectors map native events
 * (Claude Code PreToolUse, VS Code onDidSave, etc.) to these canonical types.
 * The Rust engine evaluates hooks using these types.
 */

/** Canonical hook event types — platform-defined, connector-mapped. */
export type CanonicalHookEvent =
	| "PreAction"
	| "PostAction"
	| "PromptSubmit"
	| "PreCompact"
	| "SessionStart"
	| "SessionEnd"
	| "SubagentStop"
	| "PreCommit";

/** Context passed to the hook engine for evaluation. */
export interface HookContext {
	readonly event: CanonicalHookEvent;
	readonly tool_name?: string;
	readonly tool_input?: unknown;
	readonly file_path?: string;
	readonly user_message?: string;
	readonly agent_type?: string;
}

/** Result from the hook engine after evaluating rules. */
export interface HookResult {
	readonly action: "allow" | "block" | "warn";
	readonly messages: readonly string[];
	readonly violations: readonly HookViolation[];
}

/** A single rule violation found during hook evaluation. */
export interface HookViolation {
	readonly rule_id: string;
	readonly action: string;
	readonly message: string;
}
