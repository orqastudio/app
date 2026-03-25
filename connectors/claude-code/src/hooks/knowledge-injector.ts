// PreToolUse hook — Agent matcher  (knowledge-injector)
//
// Single enforcement mechanism for knowledge injection when agents are spawned.
// The orchestrator does NOT duplicate this — it relies on the hook exclusively.
// See AGENT-4c94fe14 Delegation Steps for the behavioral contract.
//
// Two knowledge layers:
//   Layer 1 (Declared): Reads the prompt registry for knowledge entries that
//     match the detected agent role. No file parsing — uses the cached registry
//     built at plugin-install time by @orqastudio/cli.
//   Layer 2 (Semantic): Calls search_semantic via MCP TCP IPC to find
//     task-specific knowledge beyond declared relationships.
//
// Availability:
//   Layer 1 always works (reads .orqa/prompt-registry.json).
//   Layer 2 requires the OrqaStudio app or MCP server running (IPC socket
//   at LOCALAPPDATA/com.orqastudio.app/ipc.port). When unavailable, Layer 2
//   gracefully returns empty — the hook still injects Layer 1 results.
//
// Future: When the search engine is extracted as an HTTP service
//   (TASK-aef92af1 in EPIC-9e3d320b), Layer 2 can switch from MCP TCP
//   to a direct HTTP call, removing the app dependency. Alternatively,
//   an `orqa search` CLI command could serve as a fallback.
//
// Non-blocking: injects context via outputWarn(), never denies.

import { existsSync, readFileSync } from "fs";
import { createConnection } from "net";
import { join } from "path";
import { readInput, outputAllow, outputWarn } from "./shared.js";
import { logTelemetry } from "./telemetry.js";
import { readPromptRegistry, queryKnowledge, type RegistryKnowledgeEntry } from "@orqastudio/cli";

/** Minimum semantic search score to include a result. */
const MIN_SCORE = 0.25;

/** Maximum number of semantic search results to inject. */
const MAX_SEMANTIC = 5;

// ---------------------------------------------------------------------------
// MCP IPC — reuses the pattern from prompt-injector.ts
// ---------------------------------------------------------------------------

interface SearchResult {
  file: string;
  line: number;
  content: string;
  score: number;
}

function getIpcPortFilePath(): string {
  const dataDir = process.env["LOCALAPPDATA"]
    ? join(process.env["LOCALAPPDATA"], "com.orqastudio.app")
    : join(process.env["HOME"] ?? "~", ".local", "share", "com.orqastudio.app");
  return join(dataDir, "ipc.port");
}

function readIpcPort(): number | null {
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

function mcpSearchCall(port: number, projectDir: string, query: string, limit: number): Promise<SearchResult[]> {
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
          clientInfo: { name: "knowledge-injector", version: "1.0.0" },
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
// Layer 1 — Declared knowledge from prompt registry
// ---------------------------------------------------------------------------

const ROLE_PATTERNS: Array<[RegExp, string]> = [
  [/you are (?:an? )?implementer/i, "implementer"],
  [/you are (?:an? )?researcher/i, "researcher"],
  [/you are (?:an? )?reviewer/i, "reviewer"],
  [/you are (?:an? )?planner/i, "planner"],
  [/you are (?:an? )?writer/i, "writer"],
  [/you are (?:an? )?designer/i, "designer"],
  [/you are (?:an? )?governance steward/i, "governance-steward"],
];

function detectRole(prompt: string): string | null {
  for (const [pattern, role] of ROLE_PATTERNS) {
    if (pattern.test(prompt)) return role;
  }
  return null;
}

/**
 * Query the prompt registry for knowledge entries declared for the given role.
 * Returns entries with their IDs, titles, and file paths.
 *
 * Replaces the old approach of hand-parsing AGENT-*.md frontmatter for
 * employs relationships to KNOW-* IDs.
 */
function getDeclaredKnowledge(
  role: string,
  projectDir: string,
): Array<{ id: string; title: string; path: string }> {
  const registry = readPromptRegistry(projectDir);
  if (!registry) return [];

  const entries: RegistryKnowledgeEntry[] = queryKnowledge(registry, { role });

  return entries
    .filter((entry) => entry.content_file !== null)
    .map((entry) => ({
      id: entry.id,
      title: entry.summary
        ? entry.summary.split("\n")[0].slice(0, 80)
        : entry.id,
      path: entry.content_file!,
    }));
}

// ---------------------------------------------------------------------------
// Layer 2 — Semantic search for task-relevant knowledge
// ---------------------------------------------------------------------------

async function searchSemanticKnowledge(
  prompt: string,
  projectDir: string,
  excludeIds: Set<string>,
): Promise<Array<{ id: string; title: string; path: string; score: number }>> {
  const port = readIpcPort();
  if (!port) return [];

  // Extract a query from the prompt — use the first ~500 chars after role assignment
  const roleEnd = prompt.search(/\n\n/);
  const query = roleEnd > 0 ? prompt.slice(roleEnd + 2, roleEnd + 502) : prompt.slice(0, 500);

  try {
    const results = await mcpSearchCall(port, projectDir, query, MAX_SEMANTIC + excludeIds.size);

    return results
      .filter(r => {
        const idMatch = r.file.match(/(KNOW-[a-f0-9]+)/);
        return idMatch && r.score >= MIN_SCORE && !excludeIds.has(idMatch[1]);
      })
      .slice(0, MAX_SEMANTIC)
      .map(r => {
        const idMatch = r.file.match(/(KNOW-[a-f0-9]+)/)!;
        return {
          id: idMatch[1],
          title: r.content.split("\n")[0].replace(/^#+\s*/, "").trim() || idMatch[1],
          path: r.file,
          score: r.score,
        };
      });
  } catch {
    return [];
  }
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

async function main(): Promise<void> {
  const startTime = Date.now();
  const input = await readInput();

  if (input.tool_name !== "Agent") {
    outputAllow();
  }

  const prompt = (input.tool_input as { prompt?: string })?.prompt ?? "";
  if (!prompt) {
    outputAllow();
  }

  const projectDir = input.cwd ?? process.cwd();

  // Layer 1: Declared knowledge from prompt registry (role-matched)
  const role = detectRole(prompt);
  const declared = role ? getDeclaredKnowledge(role, projectDir) : [];
  const declaredIds = new Set(declared.map(d => d.id));

  // Layer 2: Semantic search for task-relevant knowledge
  const semantic = await searchSemanticKnowledge(prompt, projectDir, declaredIds);

  if (declared.length === 0 && semantic.length === 0) {
    logTelemetry("knowledge-injector", "PreToolUse:Agent", startTime, "no-results", {
      declared_count: 0,
      semantic_count: 0,
      role: role ?? "unknown",
    });
    outputAllow();
  }

  // Build injection message
  const parts: string[] = [];

  if (declared.length > 0) {
    parts.push(`KNOWLEDGE INJECTION — ${role ?? "agent"} has ${declared.length} declared knowledge artifact(s):\n`);
    for (const k of declared) {
      parts.push(`- **${k.id}** (${k.title}): ${k.path}`);
    }
  }

  if (semantic.length > 0) {
    parts.push(`\nSEMANTIC KNOWLEDGE — ${semantic.length} additional artifact(s) found relevant to this task:\n`);
    for (const k of semantic) {
      const pct = Math.round(k.score * 100);
      parts.push(`- **${k.id}** (${k.title}) — relevance: ${pct}%`);
    }
  }

  parts.push("\nRead these knowledge artifacts before starting work.");

  logTelemetry("knowledge-injector", "PreToolUse:Agent", startTime, "injected", {
    declared_count: declared.length,
    declared_ids: declared.map(d => d.id),
    semantic_count: semantic.length,
    semantic_ids: semantic.map(s => s.id),
    role: role ?? "unknown",
  });

  outputWarn(parts);
}

main().catch(() => {
  outputAllow();
});
