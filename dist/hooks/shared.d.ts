/**
 * Shared I/O helpers for connector hooks.
 *
 * Each hook is a thin adapter: read stdin → call daemon → write stdout/stderr.
 * All enforcement logic lives in the Rust daemon at localhost:3002.
 */
import type { HookInput } from "../types.js";
/** Canonical hook event (mirrors @orqastudio/types HookContext). */
export type CanonicalEvent = "PreAction" | "PostAction" | "PromptSubmit" | "PreCompact" | "SessionStart" | "SessionEnd" | "SubagentStop" | "PreCommit";
/** Context sent to the daemon POST /hook endpoint. */
export interface HookContext {
    event: CanonicalEvent;
    tool_name?: string;
    tool_input?: unknown;
    file_path?: string;
    user_message?: string;
    agent_type?: string;
}
/** Result returned by the daemon POST /hook endpoint. */
export interface HookResult {
    action: "allow" | "block" | "warn";
    messages: string[];
    violations: Array<{
        rule_id: string;
        action: string;
        message: string;
    }>;
}
/** Read Claude Code hook JSON from stdin. */
export declare function readInput(): Promise<HookInput>;
/**
 * Call the daemon HTTP API.
 * Falls back to spawning `orqa-validation hook --stdin` if the daemon is not running.
 */
export declare function callDaemon<T>(path: string, body: unknown): Promise<T>;
/** Map a Claude Code hook event name to a canonical event name. */
export declare function mapEvent(ccEvent: string): CanonicalEvent;
/**
 * Output a blocking message to stderr and exit 2.
 * This denies the tool call in Claude Code.
 */
export declare function outputBlock(messages: string[]): never;
/**
 * Output a non-blocking warning to stdout and exit 0.
 * The tool call proceeds but the agent sees the message.
 */
export declare function outputWarn(messages: string[]): void;
/** Exit silently — tool call proceeds with no message. */
export declare function outputAllow(): never;
//# sourceMappingURL=shared.d.ts.map