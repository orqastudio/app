/**
 * Shared I/O helpers for connector hooks.
 *
 * Each hook is a thin adapter: read stdin → call daemon → write stdout/stderr.
 * All enforcement logic lives in the Rust daemon. The daemon port is resolved
 * via @orqastudio/constants getPort("daemon") (default: 10100).
 */

import { spawnSync } from "node:child_process";
import { relative } from "node:path";
import { getPort } from "@orqastudio/constants";
import { assertNever } from "@orqastudio/types";
import type { HookInput } from "../types.js";

const DAEMON_BASE = `http://localhost:${getPort("daemon")}`;

/** Canonical hook event (mirrors `@orqastudio/types` HookContext). */
export type CanonicalEvent =
  | "PreAction"
  | "PostAction"
  | "PromptSubmit"
  | "PreCompact"
  | "SessionStart"
  | "SessionEnd"
  | "SubagentStop"
  | "TeammateIdle"
  | "TaskCompleted"
  | "PreCommit";

/** Context sent to the daemon POST /hook endpoint. */
export interface HookContext {
  readonly event: CanonicalEvent;
  readonly tool_name?: string;
  readonly tool_input?: unknown;
  readonly file_path?: string;
  readonly user_message?: string;
  readonly agent_type?: string;
}

/** Result returned by the daemon POST /hook endpoint. */
export interface HookResult {
  readonly action: "allow" | "block" | "warn";
  readonly messages: string[];
  readonly violations: ReadonlyArray<{ readonly rule_id: string; readonly action: string; readonly message: string }>;
}

/**
 * Read Claude Code hook JSON from stdin.
 * Collects all chunks into a buffer and parses the complete JSON payload.
 * @returns Parsed HookInput from the hook event payload.
 */
export async function readInput(): Promise<HookInput> {
  const chunks: Buffer[] = [];
  for await (const chunk of process.stdin) {
    chunks.push(Buffer.isBuffer(chunk) ? chunk : Buffer.from(chunk));
  }
  return JSON.parse(Buffer.concat(chunks).toString("utf-8")) as HookInput;
}

/**
 * Call the daemon HTTP API.
 * Falls back to spawning `orqa-validation hook --stdin` if the daemon is not running.
 * @param path - API path to call on the daemon (e.g. "/hook" or "/prompt").
 * @param body - Request body to send as JSON.
 * @returns Parsed response from the daemon endpoint.
 */
export async function callDaemon<T>(path: string, body: unknown): Promise<T> {
  try {
    const res = await fetch(`${DAEMON_BASE}${path}`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(body),
      signal: AbortSignal.timeout(8000),
    });
    if (!res.ok) {
      throw new Error(`daemon ${path} returned ${res.status}`);
    }
    return (await res.json()) as T;
  } catch {
    return callBinary(path, body);
  }
}

/** Known Claude Code event names that map to canonical OrqaStudio events. */
const CC_EVENT_MAP: Readonly<Record<string, CanonicalEvent>> = {
  PreToolUse: "PreAction",
  PostToolUse: "PostAction",
  UserPromptSubmit: "PromptSubmit",
  PreCompact: "PreCompact",
  SessionStart: "SessionStart",
  Stop: "SessionEnd",
  SubagentStop: "SubagentStop",
  TeammateIdle: "TeammateIdle",
  TaskCompleted: "TaskCompleted",
  PreCommit: "PreCommit",
} as const;

/**
 * Map a Claude Code hook event name to a canonical event name.
 * Throws on unrecognised event names rather than silently passing through.
 * @param ccEvent - The raw Claude Code hook event name (e.g. "PreToolUse").
 * @returns The canonical OrqaStudio event name.
 */
export function mapEvent(ccEvent: string): CanonicalEvent {
  const mapped = CC_EVENT_MAP[ccEvent];
  if (mapped === undefined) {
    throw new Error(`Unrecognised Claude Code event: ${ccEvent}`);
  }
  return mapped;
}

/**
 * Output a blocking message to stderr and exit 2.
 * This denies the tool call in Claude Code. Never returns — exits the process.
 * @param messages - Array of message strings to join and send to the agent.
 */
export function outputBlock(messages: readonly string[]): never {
  process.stderr.write(
    JSON.stringify({
      hookSpecificOutput: { permissionDecision: "deny" },
      systemMessage: messages.join("\n"),
    }),
  );
  process.exit(2);
}

/**
 * Output a non-blocking warning to stdout and exit 0.
 * The tool call proceeds but the agent sees the message.
 * @param messages - Array of message strings to join and send to the agent.
 */
export function outputWarn(messages: readonly string[]): void {
  process.stdout.write(JSON.stringify({ systemMessage: messages.join("\n") }));
}

/**
 * Exit silently — tool call proceeds with no message. Never returns — exits the process.
 */
export function outputAllow(): never {
  process.exit(0);
}

// ---------------------------------------------------------------------------
// Path helpers
// ---------------------------------------------------------------------------

/**
 * Check if a file path is an OrqaStudio artifact (.orqa/*.md).
 * @param filePath - Absolute or relative path to the file.
 * @param projectDir - Absolute path to the project root used for relative resolution.
 * @returns True if the file is a .md file inside the .orqa/ directory.
 */
export function isOrqaArtifact(filePath: string, projectDir: string): boolean {
  if (!filePath.endsWith(".md")) return false;
  const rel = relative(projectDir, filePath).replace(/\\/g, "/");
  return rel.startsWith(".orqa/");
}

// ---------------------------------------------------------------------------
// Binary fallback
// ---------------------------------------------------------------------------

/**
 * Fall back to `orqa-validation hook --stdin` when the daemon is not running.
 * @param path - API path forwarded to the binary (e.g. "/hook").
 * @param body - Request body forwarded as JSON.
 * @returns Parsed response from the binary's stdout.
 */
function callBinary<T>(path: string, body: unknown): T {
  const result = spawnSync("orqa-validation", ["hook", "--stdin"], {
    input: JSON.stringify({ path, body }),
    encoding: "utf-8",
    timeout: 8000,
    windowsHide: true,
  });

  if (result.error || !result.stdout) {
    throw new Error(
      `orqa-validation fallback failed: ${result.error?.message ?? "no output"}`,
    );
  }

  return JSON.parse(result.stdout) as T;
}
