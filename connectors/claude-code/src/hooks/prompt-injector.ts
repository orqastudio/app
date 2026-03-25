// UserPromptSubmit hook — all matchers
//
// Thin adapter: reads stdin, calls daemon for behavioral rules and agent preamble,
// then builds the systemMessage in Claude Code's format.
// Data comes from daemon; format (systemMessage JSON shape) is connector-specific.

import { existsSync, mkdirSync, readFileSync, statSync, writeFileSync } from "fs";
import { createConnection } from "net";
import { join } from "path";
import { readInput, callDaemon, outputAllow } from "./shared.js";
import { logTelemetry } from "./telemetry.js";

interface BehavioralRule {
  id: string;
  title: string;
  category: string;
  message: string;
}

interface BehavioralMessages {
  messages: string[];
  rules: BehavioralRule[];
  rule_count: number;
  behavioral_count: number;
}

interface AgentContent {
  preamble: string;
}

/** Prompt types that drive rule selection. */
type PromptType =
  | "implementation"
  | "planning"
  | "review"
  | "debugging"
  | "research"
  | "documentation"
  | "governance"
  | "general";

/** Maps prompt types to the rule categories most relevant for that type. */
const CATEGORY_RELEVANCE: Record<PromptType, string[]> = {
  implementation: ["safety", "process", "quality"],
  planning: ["planning", "process"],
  review: ["quality", "safety"],
  debugging: ["safety", "quality"],
  research: ["planning", "process"],
  documentation: ["planning", "quality"],
  governance: ["process", "planning"],
  general: ["process", "safety"],
};

/** Maximum number of rules to inline in systemMessage. */
const MAX_INLINE_RULES = 8;

/**
 * Critical rules that are ALWAYS inlined in systemMessage regardless of prompt type.
 * These are the foundational behavioral constraints the orchestrator must never lose sight of.
 */
const CRITICAL_RULE_IDS = new Set([
  "RULE-99abcea1", // Use Agent tool for delegation (agent teams)
  "RULE-87ba1b81", // Agent delegation — orchestrator coordinates, doesn't implement
  "RULE-0d29fc91", // Semantic search preference
  "RULE-5dd9decd", // Honest reporting — no false "complete"
  "RULE-ec9462d8", // Documentation-first
]);

/**
 * Maps thinking-mode frontmatter values to PromptType.
 * Most map 1:1 by name; these are the exceptions.
 */
const THINKING_MODE_MAP: Record<string, PromptType> = {
  "learning-loop": "governance",
  "dogfood-implementation": "implementation",
};

/** Resolve a thinking-mode value to a PromptType. */
function resolveThinkingMode(mode: string): PromptType | null {
  if (THINKING_MODE_MAP[mode]) return THINKING_MODE_MAP[mode];
  const candidates: PromptType[] = [
    "implementation", "planning", "review", "debugging",
    "research", "documentation", "governance", "general",
  ];
  return candidates.includes(mode as PromptType) ? (mode as PromptType) : null;
}

// ---------------------------------------------------------------------------
// Semantic search classifier — Tier 1 (primary, high-quality matching)
// ---------------------------------------------------------------------------

/** IPC port file path for the running OrqaStudio app's MCP server. */
function getIpcPortFilePath(): string {
  const dataDir = process.env["LOCALAPPDATA"]
    ? join(process.env["LOCALAPPDATA"], "com.orqastudio.app")
    : join(process.env["HOME"] ?? "~", ".local", "share", "com.orqastudio.app");
  return join(dataDir, "ipc.port");
}

/** Read the IPC port from disk, or null if unavailable. */
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

interface SearchResult {
  file: string;
  line: number;
  content: string;
  score: number;
}

/**
 * Send a JSON-RPC request to the MCP server over IPC and return the response.
 * Connects via TCP, sends the MCP header, then the initialize + tools/call sequence.
 * Times out after 4 seconds to keep the hook fast.
 */
function mcpSearchCall(port: number, projectDir: string, query: string, limit: number): Promise<SearchResult[]> {
  return new Promise((resolve, reject) => {
    const timeout = setTimeout(() => {
      socket.destroy();
      reject(new Error("MCP search timeout"));
    }, 4000);

    const socket = createConnection({ host: "127.0.0.1", port }, () => {
      // Send MCP protocol header
      socket.write(`MCP ${projectDir}\n`);

      // Send initialize request
      const initReq = JSON.stringify({
        jsonrpc: "2.0",
        id: 1,
        method: "initialize",
        params: { protocolVersion: "2024-11-05", capabilities: {}, clientInfo: { name: "prompt-injector", version: "1.0.0" } },
      });
      socket.write(initReq + "\n");
    });

    let buffer = "";
    let initialized = false;

    socket.on("data", (chunk: Buffer) => {
      buffer += chunk.toString();

      // Process complete JSON-RPC lines
      const lines = buffer.split("\n");
      buffer = lines.pop() ?? "";

      for (const line of lines) {
        if (!line.trim()) continue;
        try {
          const msg = JSON.parse(line) as { id?: number; result?: unknown };
          if (msg.id === 1 && !initialized) {
            // Initialize response received — send the search request
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
            // Search response received
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
      // If we haven't resolved yet, the connection closed prematurely
    });
  });
}

/**
 * Classify a prompt using ONNX semantic search against thinking-mode knowledge artifacts.
 * Returns a PromptType if a confident match is found, null otherwise.
 */
async function classifyWithSearch(message: string, projectDir: string): Promise<PromptType | null> {
  const port = readIpcPort();
  if (port === null) return null;

  // Build a search query from the user message — prefix with context for better matching
  const truncated = message.slice(0, 200);
  const query = `thinking mode classification for user prompt: ${truncated}`;

  const results = await mcpSearchCall(port, projectDir, query, 5);
  if (results.length === 0) return null;

  // Find the best-matching thinking-mode knowledge artifact
  for (const result of results) {
    // Must be a knowledge artifact in the right directory
    if (!result.file.includes("process/knowledge/KNOW-")) continue;

    // Extract thinking-mode value from the content
    const modeMatch = /thinking-mode:\s*(\S+)/.exec(result.content);
    if (!modeMatch) continue;

    const mode = modeMatch[1];
    const resolved = resolveThinkingMode(mode);
    if (resolved) return resolved;
  }

  return null;
}

async function main(): Promise<void> {
  const startTime = Date.now();

  let hookInput;
  try {
    hookInput = await readInput();
  } catch {
    process.exit(0);
  }

  const userMessage = hookInput.user_message ?? hookInput.prompt ?? "";
  const projectDir = hookInput.cwd ?? process.env["CLAUDE_PROJECT_DIR"] ?? ".";
  const agentType = hookInput.agent_type ?? "orchestrator";

  if (!userMessage) {
    outputAllow();
  }

  // Fetch behavioral rules and agent preamble from daemon in parallel.
  const [behavioralResult, agentResult] = await Promise.allSettled([
    callDaemon<BehavioralMessages>("/content/behavioral", {}),
    callDaemon<AgentContent>("/content/agent", { match: agentType || "orchestrator" }),
  ]);

  const behavioralData: BehavioralMessages =
    behavioralResult.status === "fulfilled"
      ? behavioralResult.value
      : { messages: [], rules: [], rule_count: 0, behavioral_count: 0 };
  const preamble =
    agentResult.status === "fulfilled"
      ? agentResult.value.preamble
      : `You are a ${agentType || "orchestrator"}. Follow the task delegated to you.`;

  // Three-tier prompt classification:
  //   1. ONNX semantic search against thinking-mode knowledge artifacts (primary)
  //   2. Keyword regex matching (fallback when search unavailable)
  //   3. "general" default (when nothing matches)
  let promptType: PromptType;
  let classificationMethod: "semantic" | "keyword";
  try {
    const semanticResult = await classifyWithSearch(userMessage, projectDir);
    if (semanticResult) {
      promptType = semanticResult;
      classificationMethod = "semantic";
    } else {
      promptType = classifyPrompt(userMessage);
      classificationMethod = "keyword";
    }
  } catch {
    promptType = classifyPrompt(userMessage);
    classificationMethod = "keyword";
  }

  // Select rules: pinned critical rules + prompt-relevant dynamic rules.
  const selectedRules = selectRules(behavioralData.rules, promptType);

  // Session state freshness check (connector-specific UX concern — stays here).
  const sessionReminder = checkSessionState(projectDir);

  // Context line (connector-specific — stays here).
  const contextLine = getContextLine(projectDir);

  // Build inline rules section for systemMessage — critical rules always present.
  const inlineRulesSection = formatInlineRules(selectedRules, promptType, behavioralData.behavioral_count);

  // Build the full preamble document (written to file for reference).
  const sessionSection = sessionReminder ?? "Session state is current.";
  const allRulesSection = formatAllRules(behavioralData.rules);
  const preambleDoc = [
    "# Orchestrator Preamble",
    "",
    "## Agent Identity",
    "",
    preamble,
    "",
    "## Prompt Classification",
    "",
    `Classified as: **${promptType}** (via ${classificationMethod})`,
    "",
    "## Project Context",
    "",
    contextLine,
    "",
    "## Session State",
    "",
    sessionSection,
    "",
    "## Active Behavioral Rules (by category)",
    "",
    allRulesSection,
    "",
  ].join("\n");

  // Write full preamble to .state/orchestrator-preamble.md for reference.
  const tmpDir = join(projectDir, ".state");
  mkdirSync(tmpDir, { recursive: true });
  writeFileSync(join(tmpDir, "orchestrator-preamble.md"), preambleDoc, "utf-8");

  logTelemetry("prompt-injector", "UserPromptSubmit", startTime, "injected", {
    agent_type: agentType,
    prompt_type: promptType,
    classification_method: classificationMethod,
    rules_pinned: selectedRules.pinned.length,
    rules_dynamic: selectedRules.dynamic.length,
    rules_total: behavioralData.behavioral_count,
    query: userMessage.slice(0, 100),
    action: "allow",
    session_state_reminder: !!sessionReminder,
  }, projectDir);

  // Build systemMessage — inline the selected rules directly.
  const parts: string[] = [];

  // Session reminder goes first if present.
  if (sessionReminder) {
    parts.push(sessionReminder);
  }

  // Inline the priority rules — these are the ones that matter for this prompt.
  parts.push(inlineRulesSection);

  // Reference the full preamble file for additional context.
  parts.push("Full session context and all rules: .state/orchestrator-preamble.md");

  const systemMessage = parts.join("\n\n");
  process.stdout.write(JSON.stringify({ systemMessage }));
  process.exit(0);
}

// ---------------------------------------------------------------------------
// Prompt classification — Tier 2 keyword fallback
// ---------------------------------------------------------------------------

/**
 * Classify a user prompt into a type using keyword matching.
 * Tier 2 fallback — used when ONNX semantic search is unavailable.
 */
function classifyPrompt(message: string): PromptType {
  const lower = message.toLowerCase();

  // Implementation signals
  if (
    /\b(implement|build|create|add|write code|fix bug|refactor|migrate|wire up|hook up)\b/.test(lower)
  ) {
    return "implementation";
  }

  // Debugging signals
  if (/\b(debug|investigate|why does|broken|error|crash|failing|not working|trace)\b/.test(lower)) {
    return "debugging";
  }

  // Review signals
  if (/\b(review|audit|check|verify|validate|assess|compliance)\b/.test(lower)) {
    return "review";
  }

  // Planning signals
  if (/\b(plan|design|scope|epic|roadmap|milestone|break down|approach)\b/.test(lower)) {
    return "planning";
  }

  // Documentation signals
  if (/\b(document|docs|write up|describe|explain|specification)\b/.test(lower)) {
    return "documentation";
  }

  // Research signals
  if (/\b(research|explore|investigate options|compare|evaluate|what are the)\b/.test(lower)) {
    return "research";
  }

  // Governance signals
  if (
    /\b(rule|governance|enforce|lesson|artifact|pillar|promote|knowledge)\b/.test(lower)
  ) {
    return "governance";
  }

  return "general";
}

// ---------------------------------------------------------------------------
// Rule selection and formatting
// ---------------------------------------------------------------------------

/**
 * Select rules for inline injection. Critical rules are always pinned first,
 * then remaining slots are filled from prompt-relevant categories.
 */
function selectRules(rules: BehavioralRule[], promptType: PromptType): { pinned: BehavioralRule[]; dynamic: BehavioralRule[] } {
  if (rules.length === 0) return { pinned: [], dynamic: [] };

  const pinned: BehavioralRule[] = [];
  const seen = new Set<string>();

  // Pin critical rules first — these always appear regardless of prompt type.
  for (const rule of rules) {
    if (CRITICAL_RULE_IDS.has(rule.id)) {
      pinned.push(rule);
      seen.add(rule.id);
    }
  }

  // Fill remaining slots from prompt-relevant categories.
  const dynamic: BehavioralRule[] = [];
  const remainingSlots = MAX_INLINE_RULES - pinned.length;
  const relevantCategories = CATEGORY_RELEVANCE[promptType];

  for (const category of relevantCategories) {
    for (const rule of rules) {
      if (!seen.has(rule.id) && rule.category === category) {
        dynamic.push(rule);
        seen.add(rule.id);
        if (dynamic.length >= remainingSlots) return { pinned, dynamic };
      }
    }
  }

  // Fill any remaining slots from any category.
  for (const rule of rules) {
    if (!seen.has(rule.id)) {
      dynamic.push(rule);
      seen.add(rule.id);
      if (dynamic.length >= remainingSlots) return { pinned, dynamic };
    }
  }

  return { pinned, dynamic };
}

/**
 * Format selected rules for inline injection in systemMessage.
 * Pinned critical rules get their own section; dynamic rules grouped by category.
 */
function formatInlineRules(
  selection: { pinned: BehavioralRule[]; dynamic: BehavioralRule[] },
  promptType: PromptType,
  totalRules: number,
): string {
  const { pinned, dynamic } = selection;
  if (pinned.length === 0 && dynamic.length === 0) return "No behavioral rules loaded.";

  const lines: string[] = [];

  // Critical rules — always enforced, self-contained in systemMessage.
  if (pinned.length > 0) {
    lines.push("[Critical — Always Active]");
    for (const rule of pinned) {
      lines.push(`- ${rule.id} (${rule.title}): ${rule.message}`);
    }
  }

  // Dynamic rules — selected based on prompt classification.
  if (dynamic.length > 0) {
    const categoryLabels: Record<string, string> = {
      safety: "Safety & Standards",
      process: "Process & Delegation",
      planning: "Planning & Documentation",
      quality: "Quality & Tooling",
      general: "General",
    };

    const grouped = new Map<string, BehavioralRule[]>();
    for (const rule of dynamic) {
      const existing = grouped.get(rule.category) ?? [];
      existing.push(rule);
      grouped.set(rule.category, existing);
    }

    lines.push("");
    lines.push(`[Relevant for ${promptType} work]`);
    for (const [category, categoryRules] of grouped) {
      const label = categoryLabels[category] ?? category;
      lines.push(`\n  ${label}:`);
      for (const rule of categoryRules) {
        lines.push(`  - ${rule.id} (${rule.title}): ${rule.message}`);
      }
    }
  }

  const inlinedCount = pinned.length + dynamic.length;
  lines.push(`\n(${inlinedCount} of ${totalRules} rules inlined; full list in .state/orchestrator-preamble.md)`);

  return lines.join("\n");
}

/**
 * Format ALL rules grouped by category for the full preamble file.
 */
function formatAllRules(rules: BehavioralRule[]): string {
  if (rules.length === 0) return "No behavioral rules loaded.";

  const grouped = new Map<string, BehavioralRule[]>();
  for (const rule of rules) {
    const existing = grouped.get(rule.category) ?? [];
    existing.push(rule);
    grouped.set(rule.category, existing);
  }

  const categoryLabels: Record<string, string> = {
    safety: "Safety & Standards",
    process: "Process & Delegation",
    planning: "Planning & Documentation",
    quality: "Quality & Tooling",
    general: "General",
  };

  const sections: string[] = [];
  for (const [category, categoryRules] of grouped) {
    const label = categoryLabels[category] ?? category;
    const lines = [`### ${label}`, ""];
    for (const rule of categoryRules) {
      lines.push(`- **${rule.id}** (${rule.title}): ${rule.message}`);
    }
    sections.push(lines.join("\n"));
  }

  return sections.join("\n\n");
}

// ---------------------------------------------------------------------------
// Connector-specific helpers — format/UX concerns that stay in the connector
// ---------------------------------------------------------------------------

/** Read project.json and return a concise context line. */
function getContextLine(projectDir: string): string {
  const p = join(projectDir, ".orqa", "project.json");
  if (!existsSync(p)) {
    return "Project: unknown. Run `orqa plugin list` to check installed plugins if needed.";
  }
  try {
    const s = JSON.parse(readFileSync(p, "utf-8")) as Record<string, unknown>;
    const name = String(s["name"] ?? "unknown");
    const dogfood = s["dogfood"] ? "active — you are editing the app from the CLI" : "inactive";
    return `Project: ${name}. Dogfood: ${dogfood}. Run \`orqa plugin list\` to check installed plugins if needed.`;
  } catch {
    return "Project: unknown. Run `orqa plugin list` to check installed plugins if needed.";
  }
}

/** Return a session-state reminder string if action is needed, else null. */
function checkSessionState(projectDir: string): string | null {
  try {
    const sessionPath = join(projectDir, ".state", "session-state.md");
    if (!existsSync(sessionPath)) {
      return "Session state reminder: .state/session-state.md does not exist. Create a working session state with: scope, step checklist with completion status, and architecture decisions. Update it in real time as decisions happen (RULE-8aadfd6c).";
    }
    const content = readFileSync(sessionPath, "utf-8");
    const isAutoGenerated = content.includes("Session state auto-generated by stop hook");
    const hasSteps = content.includes("### Steps");
    if (isAutoGenerated && !hasSteps) {
      return "Session state reminder: .state/session-state.md is auto-generated. Replace with a working session state containing: scope, step checklist with completion status, and architecture decisions. Update it in real time as decisions happen (RULE-8aadfd6c).";
    }
    if (!hasSteps) {
      return "Session state reminder: .state/session-state.md exists but has no step checklist. Add a ### Steps section with checkboxes tracking current work, and include the scoped epic (EPIC-XXXXXXXX) so the stop hook can check completion (RULE-8aadfd6c).";
    }
    if (!/EPIC-[a-f0-9]{8}/i.test(content)) {
      return "Session state reminder: .state/session-state.md has no scoped epic. Add the epic ID (EPIC-XXXXXXXX) so the stop hook can check completion status (RULE-8aadfd6c).";
    }
    const ageMinutes = (Date.now() - statSync(sessionPath).mtimeMs) / 60000;
    if (ageMinutes > 10) {
      return `Session state reminder: .state/session-state.md hasn't been updated in ${Math.round(ageMinutes)} minutes. If scope has changed or decisions were made, update it now (RULE-8aadfd6c).`;
    }
    return null;
  } catch {
    return null;
  }
}

main().catch(() => process.exit(0));
