// UserPromptSubmit hook — all matchers
//
// Thin adapter: reads stdin, calls daemon for behavioral rules and agent preamble,
// then builds the systemMessage in Claude Code's format.
// Data comes from daemon; format (systemMessage JSON shape) is connector-specific.

import { spawnSync } from "node:child_process";
import { existsSync, readFileSync, statSync } from "fs";
import { join } from "path";
import { readInput, callDaemon, outputAllow } from "./shared.js";
import { logTelemetry } from "./telemetry.js";

interface BehavioralMessages {
  messages: string[];
}

interface AgentContent {
  preamble: string;
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

  const behavioralMessages =
    behavioralResult.status === "fulfilled" ? behavioralResult.value.messages : [];
  const preamble =
    agentResult.status === "fulfilled"
      ? agentResult.value.preamble
      : `You are a ${agentType || "orchestrator"}. Follow the task delegated to you.`;

  // Mode classification via MCP semantic search (connector-specific — stays here).
  const mode = classifyThinkingMode(userMessage, projectDir);

  // Session state freshness check (connector-specific UX concern — stays here).
  const sessionReminder = checkSessionState(projectDir);

  // Context line (connector-specific — stays here).
  const contextLine = getContextLine(projectDir);

  // Build systemMessage: preamble + mode + behavioral rules + context + session.
  const behavioralStr = behavioralMessages.join(" ");
  const modeInjection = mode
    ? `Thinking mode: ${mode}. ${behavioralStr}`.trim()
    : `Classify this prompt before responding: implementation | research | learning-loop | planning | review | debugging | documentation. If learning-loop: capture as lesson first. Then proceed with the appropriate approach. ${behavioralStr}`.trim();

  let systemMessage = `${preamble}\n\n${modeInjection}\n\n${contextLine}`;
  if (sessionReminder) {
    systemMessage += `\n\n${sessionReminder}`;
  }

  logTelemetry("prompt-injector", "UserPromptSubmit", startTime, "injected", {
    agent_type: agentType,
    mode,
    query: userMessage.slice(0, 100),
    action: "allow",
    session_state_reminder: !!sessionReminder,
  }, projectDir);

  process.stdout.write(JSON.stringify({ systemMessage }));
  process.exit(0);
}

// ---------------------------------------------------------------------------
// Connector-specific helpers — format/UX concerns that stay in the connector
// ---------------------------------------------------------------------------

/**
 * Classify thinking mode by searching knowledge artifacts via `orqa mcp`.
 * Returns the mode suffix (e.g. "implementation") or null if unknown.
 */
function classifyThinkingMode(userMessage: string, projectDir: string): string | null {
  const query = userMessage.length > 200 ? userMessage.slice(0, 200) : userMessage;
  const initialize = JSON.stringify({
    jsonrpc: "2.0", id: 1, method: "initialize",
    params: { protocolVersion: "2024-11-05", capabilities: {}, clientInfo: { name: "prompt-injector", version: "1.0.0" } },
  });
  const toolCall = JSON.stringify({
    jsonrpc: "2.0", id: 2, method: "tools/call",
    params: { name: "search_semantic", arguments: { query, scope: "artifacts", limit: 10 } },
  });

  let result;
  try {
    result = spawnSync("orqa", ["mcp", projectDir], {
      input: `${initialize}\n${toolCall}\n`,
      encoding: "utf-8",
      timeout: 5000,
      windowsHide: true,
    });
  } catch {
    return null;
  }

  if (result.error || result.status !== 0 || !result.stdout) return null;

  for (const line of (result.stdout as string).split("\n").filter((l) => l.trim())) {
    let parsed: Record<string, unknown>;
    try { parsed = JSON.parse(line) as Record<string, unknown>; } catch { continue; }
    if (parsed["id"] !== 2 || parsed["error"]) continue;

    const content = (parsed["result"] as Record<string, unknown> | undefined)
      ?.["content"] as Array<Record<string, unknown>> | undefined;
    const textContent = content?.[0]?.["text"];
    if (!textContent) continue;

    let hits: unknown;
    try { hits = JSON.parse(String(textContent)); } catch { continue; }
    if (!Array.isArray(hits)) continue;

    for (const hit of hits as Record<string, unknown>[]) {
      const fp = String(hit["file"] ?? hit["file_path"] ?? "").replace(/\\/g, "/");
      const m = fp.match(/thinking-mode-([^/]+)/);
      if (m) return m[1];
    }
  }
  return null;
}

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
    const sessionPath = join(projectDir, "tmp", "session-state.md");
    if (!existsSync(sessionPath)) {
      return "Session state reminder: tmp/session-state.md does not exist. Create a working session state with: scope, step checklist with completion status, and architecture decisions. Update it in real time as decisions happen (RULE-4f7e2a91).";
    }
    const content = readFileSync(sessionPath, "utf-8");
    const isAutoGenerated = content.includes("Session state auto-generated by stop hook");
    const hasSteps = content.includes("### Steps");
    if (isAutoGenerated && !hasSteps) {
      return "Session state reminder: tmp/session-state.md is auto-generated. Replace with a working session state containing: scope, step checklist with completion status, and architecture decisions. Update it in real time as decisions happen (RULE-4f7e2a91).";
    }
    if (!hasSteps) {
      return "Session state reminder: tmp/session-state.md exists but has no step checklist. Add a ### Steps section with checkboxes tracking current work, and include the scoped epic (EPIC-XXXXXXXX) so the stop hook can check completion (RULE-4f7e2a91).";
    }
    if (!/EPIC-[a-f0-9]{8}/i.test(content)) {
      return "Session state reminder: tmp/session-state.md has no scoped epic. Add the epic ID (EPIC-XXXXXXXX) so the stop hook can check completion status (RULE-4f7e2a91).";
    }
    const ageMinutes = (Date.now() - statSync(sessionPath).mtimeMs) / 60000;
    if (ageMinutes > 10) {
      return `Session state reminder: tmp/session-state.md hasn't been updated in ${Math.round(ageMinutes)} minutes. If scope has changed or decisions were made, update it now (RULE-4f7e2a91).`;
    }
    return null;
  } catch {
    return null;
  }
}

main().catch(() => process.exit(0));
