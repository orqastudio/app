export interface EnforcementRule {
	readonly name: string;
	readonly scope: string; // "system" | "project"
	readonly entries: readonly EnforcementEntry[];
	readonly prose: string;
}

export interface EnforcementEntry {
	readonly event: "File" | "Bash";
	readonly action: "Block" | "Warn";
	readonly conditions: readonly Condition[];
	readonly pattern: string | null;
}

export interface Condition {
	readonly field: string;
	readonly pattern: string;
}

export interface EnforcementViolation {
	readonly rule_name: string;
	readonly action: "Block" | "Warn";
	readonly tool_name: string;
	readonly detail: string;
	readonly timestamp: string;
}

/** A violation record loaded from the SQLite `enforcement_violations` table. */
export interface StoredEnforcementViolation {
	readonly id: number;
	readonly project_id: number;
	readonly rule_name: string;
	/** Lower-case: "block" or "warn" (as stored in SQLite). */
	readonly action: string;
	readonly tool_name: string;
	readonly detail: string | null;
	readonly created_at: string;
}

// ---------------------------------------------------------------------------
// Centralised Enforcement Log (Phase 4)
// ---------------------------------------------------------------------------

/**
 * Result of an enforcement check.
 * - `pass`: no violation found
 * - `fail`: violation detected, enforcement triggered
 * - `warn`: potential issue, not blocking
 * - `error`: enforcement check itself failed (e.g. schema compilation error)
 */
export type EnforcementResult = "pass" | "fail" | "warn" | "error";

/**
 * Resolution status for an enforcement event.
 * Set by the agent after receiving enforcement feedback.
 */
export type EnforcementResolution =
	| "unresolved"
	| "fixed"
	| "deferred"
	| "overridden"
	| "false-positive";

/**
 * A single enforcement event logged to the centralised enforcement log.
 *
 * Every enforcement check — regardless of source (hook, LSP, pre-commit,
 * JSON Schema, lint) — produces one event per check per artifact.
 * The enforcement log is NDJSON (one event per line) at `.state/enforcement-log.jsonl`.
 */
export interface EnforcementEvent {
	/** Unique event ID (UUID v4 or nanoid). */
	readonly id: string;
	/** ISO 8601 timestamp. */
	readonly timestamp: string;
	/** Mechanism key that produced this event (e.g. "json-schema", "hook", "lint"). */
	readonly mechanism: string;
	/** Hook or check type within the mechanism (e.g. "PreToolUse", "frontmatter"). */
	readonly type: string;
	/** Rule ID that triggered this enforcement, if applicable. */
	readonly rule_id: string | null;
	/** Artifact ID being checked, if applicable. */
	readonly artifact_id: string | null;
	/** Check result. */
	readonly result: EnforcementResult;
	/** Human-readable message describing the finding. */
	readonly message: string;
	/** Source that produced this event. */
	readonly source: "validator" | "lsp" | "hook" | "pre-commit" | "cli";
	/** Resolution status (starts as "unresolved" for fail/warn events). */
	readonly resolution: EnforcementResolution;
}

/**
 * An agent's response to an enforcement event.
 * Links back to the original event via `event_id`.
 */
export interface EnforcementResponse {
	/** The enforcement event ID this responds to. */
	readonly event_id: string;
	/** ISO 8601 timestamp of the response. */
	readonly timestamp: string;
	/** Action taken by the agent. */
	readonly action: EnforcementResolution;
	/** Human-readable detail about what was done. */
	readonly detail: string;
}
