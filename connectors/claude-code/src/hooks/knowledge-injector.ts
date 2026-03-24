// PreToolUse hook — Agent matcher  (knowledge-injector)
//
// Single enforcement mechanism for knowledge injection when agents are spawned.
// The orchestrator does NOT duplicate this — it relies on the hook exclusively.
// See AGENT-1dab5ebe Delegation Steps for the behavioral contract.
//
// Two knowledge layers:
//   Layer 1 (Declared): Reads the agent definition's `employs` relationships
//     for KNOW-* IDs. Works offline — reads files on disk.
//   Layer 2 (Semantic): Calls search_semantic via MCP TCP IPC to find
//     task-specific knowledge beyond declared relationships.
//
// Availability:
//   Layer 1 always works (file reads only).
//   Layer 2 requires the OrqaStudio app or MCP server running (IPC socket
//   at LOCALAPPDATA/com.orqastudio.app/ipc.port). When unavailable, Layer 2
//   gracefully returns empty — the hook still injects Layer 1 results.
//
// Future: When the search engine is extracted as an HTTP service
//   (TASK-a4d5e6b7 in EPIC-a4c7e9b1), Layer 2 can switch from MCP TCP
//   to a direct HTTP call, removing the app dependency. Alternatively,
//   an `orqa search` CLI command could serve as a fallback.
//
// Non-blocking: injects context via outputWarn(), never denies.

import { existsSync, readFileSync, readdirSync } from "fs";
import { createConnection } from "net";
import { join } from "path";
import { readInput, outputAllow, outputWarn } from "./shared.js";
import { logTelemetry } from "./telemetry.js";

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
// Layer 1 — Declared knowledge from agent employs relationships
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

interface AgentFrontmatter {
  title?: string;
  relationships?: Array<{ target: string; type: string; rationale?: string }>;
}

function parseFrontmatter(content: string): AgentFrontmatter {
  const match = content.match(/^---\n([\s\S]*?)\n---/);
  if (!match) return {};

  const yaml = match[1];
  const result: AgentFrontmatter = {};

  const titleMatch = yaml.match(/^title:\s*"?(.+?)"?\s*$/m);
  if (titleMatch) result.title = titleMatch[1];

  const relStart = yaml.indexOf("relationships:");
  if (relStart !== -1) {
    const relSection = yaml.slice(relStart);
    const rels: AgentFrontmatter["relationships"] = [];
    const relRegex = /- target:\s*(\S+)\s*\n\s*type:\s*(\S+)(?:\s*\n\s*rationale:\s*"?(.+?)"?\s*$)?/gm;
    let relMatch: RegExpExecArray | null;
    while ((relMatch = relRegex.exec(relSection)) !== null) {
      rels.push({
        target: relMatch[1],
        type: relMatch[2],
        rationale: relMatch[3],
      });
    }
    result.relationships = rels;
  }

  return result;
}

function findAgentDefinition(role: string, projectDir: string): { path: string; frontmatter: AgentFrontmatter } | null {
  const agentsDir = join(projectDir, ".orqa", "process", "agents");
  if (!existsSync(agentsDir)) return null;

  try {
    const files = readdirSync(agentsDir).filter(f => f.startsWith("AGENT-") && f.endsWith(".md"));
    for (const file of files) {
      const filePath = join(agentsDir, file);
      const content = readFileSync(filePath, "utf-8");
      const fm = parseFrontmatter(content);
      if (fm.title && fm.title.toLowerCase().includes(role.replace("-", " "))) {
        return { path: filePath, frontmatter: fm };
      }
    }
  } catch {
    // Directory read failed
  }
  return null;
}

function resolveKnowledgeArtifacts(ids: string[], projectDir: string): Array<{ id: string; title: string; path: string }> {
  const knowledgeDir = join(projectDir, ".orqa", "process", "knowledge");
  const results: Array<{ id: string; title: string; path: string }> = [];

  for (const id of ids) {
    const filePath = join(knowledgeDir, `${id}.md`);
    if (existsSync(filePath)) {
      try {
        const content = readFileSync(filePath, "utf-8");
        const titleMatch = content.match(/^title:\s*"?(.+?)"?\s*$/m);
        results.push({
          id,
          title: titleMatch?.[1] ?? id,
          path: `.orqa/process/knowledge/${id}.md`,
        });
      } catch {
        results.push({ id, title: id, path: `.orqa/process/knowledge/${id}.md` });
      }
    }
  }

  return results;
}

function getDeclaredKnowledge(prompt: string, projectDir: string): Array<{ id: string; title: string; path: string }> {
  const role = detectRole(prompt);
  if (!role) return [];

  const agentDef = findAgentDefinition(role, projectDir);
  if (!agentDef?.frontmatter.relationships) return [];

  const knowIds = agentDef.frontmatter.relationships
    .filter(r => r.type === "employs" && r.target.startsWith("KNOW-"))
    .map(r => r.target);

  if (knowIds.length === 0) return [];

  return resolveKnowledgeArtifacts(knowIds, projectDir);
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

  // Layer 1: Declared knowledge from agent employs relationships
  const declared = getDeclaredKnowledge(prompt, projectDir);
  const declaredIds = new Set(declared.map(d => d.id));

  // Layer 2: Semantic search for task-relevant knowledge
  const semantic = await searchSemanticKnowledge(prompt, projectDir, declaredIds);

  if (declared.length === 0 && semantic.length === 0) {
    logTelemetry("knowledge-injector", "PreToolUse:Agent", startTime, "no-results", {
      declared_count: 0,
      semantic_count: 0,
    });
    outputAllow();
  }

  // Build injection message
  const parts: string[] = [];

  if (declared.length > 0) {
    const role = detectRole(prompt) ?? "agent";
    parts.push(`KNOWLEDGE INJECTION — ${role} has ${declared.length} declared knowledge artifact(s):\n`);
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
  });

  outputWarn(parts);
}

main().catch(() => {
  outputAllow();
});
