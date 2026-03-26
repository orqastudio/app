/**
 * Shared I/O helpers for connector hooks.
 *
 * Each hook is a thin adapter: read stdin → call daemon → write stdout/stderr.
 * All enforcement logic lives in the Rust daemon at localhost:10258.
 */

import { spawnSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { createConnection } from "node:net";
import { join, relative } from "node:path";
import type { HookInput } from "../types.js";

/** Resolve daemon port from ORQA_PORT_BASE (default 10200) + offset 58. */
function getDaemonPort(): number {
  const raw = process.env["ORQA_PORT_BASE"];
  const base = (raw !== undefined && raw !== "") ? parseInt(raw, 10) : 10200;
  return (Number.isNaN(base) ? 10200 : base) + 58;
}

const DAEMON_BASE = `http://localhost:${getDaemonPort()}`;

/** Canonical hook event (mirrors @orqastudio/types HookContext). */
export type CanonicalEvent =
  | "PreAction"
  | "PostAction"
  | "PromptSubmit"
  | "PreCompact"
  | "SessionStart"
  | "SessionEnd"
  | "SubagentStop"
  | "PreCommit";

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
  violations: Array<{ rule_id: string; action: string; message: string }>;
}

/** Read Claude Code hook JSON from stdin. */
export async function readInput(): Promise<HookInput> {
  let raw = "";
  for await (const chunk of process.stdin) {
    raw += chunk;
  }
  return JSON.parse(raw) as HookInput;
}

/**
 * Call the daemon HTTP API.
 * Falls back to spawning `orqa-validation hook --stdin` if the daemon is not running.
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

/** Map a Claude Code hook event name to a canonical event name. */
export function mapEvent(ccEvent: string): CanonicalEvent {
  const map: Record<string, CanonicalEvent> = {
    PreToolUse: "PreAction",
    PostToolUse: "PostAction",
    UserPromptSubmit: "PromptSubmit",
    PreCompact: "PreCompact",
    SessionStart: "SessionStart",
    Stop: "SessionEnd",
    SubagentStop: "SubagentStop",
  };
  return map[ccEvent] ?? (ccEvent as CanonicalEvent);
}

/**
 * Output a blocking message to stderr and exit 2.
 * This denies the tool call in Claude Code.
 */
export function outputBlock(messages: string[]): never {
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
 */
export function outputWarn(messages: string[]): void {
  process.stdout.write(JSON.stringify({ systemMessage: messages.join("\n") }));
}

/** Exit silently — tool call proceeds with no message. */
export function outputAllow(): never {
  process.exit(0);
}

// ---------------------------------------------------------------------------
// Path helpers
// ---------------------------------------------------------------------------

/** Check if a file path is an OrqaStudio artifact (.orqa/*.md). */
export function isOrqaArtifact(filePath: string, projectDir: string): boolean {
  if (!filePath.endsWith(".md")) return false;
  const rel = relative(projectDir, filePath).replace(/\\/g, "/");
  return rel.startsWith(".orqa/");
}

// ---------------------------------------------------------------------------
// MCP IPC — shared semantic search client
// ---------------------------------------------------------------------------

export interface SearchResult {
  file: string;
  line: number;
  content: string;
  score: number;
}

/** IPC port file path for the running OrqaStudio app's MCP server. */
export function getIpcPortFilePath(): string {
  const dataDir = process.env["LOCALAPPDATA"]
    ? join(process.env["LOCALAPPDATA"], "com.orqastudio.app")
    : join(process.env["HOME"] ?? "~", ".local", "share", "com.orqastudio.app");
  return join(dataDir, "ipc.port");
}

/** Read the IPC port from disk, or null if unavailable. */
export function readIpcPort(): number | null {
  const portFile = getIpcPortFilePath();
  if (!existsSync(portFile)) return null;
  try {
    const content = readFileSync(portFile, "utf-8").trim();
    const port = parseInt(content, 10);
    return Number.isNaN(port) ? null : port;
  } catch {
    return null;
  }
}

/**
 * Send a JSON-RPC request to the MCP server over IPC and return the response.
 * Connects via TCP, sends the MCP header, then the initialize + tools/call sequence.
 * Times out after 4 seconds to keep the hook fast.
 */
export function mcpSearchCall(port: number, projectDir: string, query: string, limit: number): Promise<SearchResult[]> {
  return new Promise((resolve, reject) => {
    const timeout = setTimeout(() => {
      socket.destroy();
      reject(new Error("MCP search timeout"));
    }, 4000);

    const socket = createConnection({ host: "127.0.0.1", port }, () => {
      socket.write(`MCP ${projectDir}\n`);
      const initReq = JSON.stringify({
        jsonrpc: "2.0",
        id: 1,
        method: "initialize",
        params: {
          protocolVersion: "2024-11-05",
          capabilities: {},
          clientInfo: { name: "orqastudio-hook", version: "1.0.0" },
        },
      });
      socket.write(initReq + "\n");
    });

    let buffer = "";
    let initialized = false;

    socket.on("data", (chunk: Buffer) => {
      buffer += chunk.toString();
      const lines = buffer.split("\n");
      buffer = lines.pop() ?? "";

      for (const line of lines) {
        if (!line.trim()) continue;
        try {
          const msg = JSON.parse(line) as { id?: number; result?: unknown };
          if (msg.id === 1 && !initialized) {
            initialized = true;
            const searchReq = JSON.stringify({
              jsonrpc: "2.0",
              id: 2,
              method: "tools/call",
              params: {
                name: "search_semantic",
                arguments: { query, scope: "artifacts", limit },
              },
            });
            socket.write(searchReq + "\n");
          } else if (msg.id === 2) {
            clearTimeout(timeout);
            socket.destroy();
            try {
              const result = msg.result as { content?: Array<{ text?: string }> };
              const text = result?.content?.[0]?.text ?? "[]";
              resolve(JSON.parse(text) as SearchResult[]);
            } catch {
              resolve([]);
            }
          }
        } catch {
          // Incomplete JSON — wait for more data
        }
      }
    });

    socket.on("error", (err: Error) => {
      clearTimeout(timeout);
      reject(err);
    });

    socket.on("close", () => {
      clearTimeout(timeout);
    });
  });
}

// ---------------------------------------------------------------------------
// Binary fallback
// ---------------------------------------------------------------------------

/**
 * Fall back to `orqa-validation hook --stdin` when the daemon is not running.
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
