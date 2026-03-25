// PreCompact hook — all matchers
//
// Thin adapter: calls POST /query for active epics and tasks, then writes
// .state/governance-context.md and returns a systemMessage summarising what was preserved.

import { writeFileSync, readFileSync, existsSync, mkdirSync, statSync } from "fs";
import { join } from "path";
import { readInput, callDaemon } from "./shared.js";
import { logTelemetry } from "./telemetry.js";

interface QueryResult {
  items?: Array<{ id: string; title?: string; status?: string }>;
}

async function main(): Promise<void> {
  const startTime = Date.now();

  let hookInput;
  try {
    hookInput = await readInput();
  } catch {
    process.exit(0);
  }

  const projectDir = hookInput.cwd ?? process.env["CLAUDE_PROJECT_DIR"] ?? ".";
  const tmpDir = join(projectDir, ".state");

  if (!existsSync(tmpDir)) {
    mkdirSync(tmpDir, { recursive: true });
  }

  // Query daemon for active epics and in-progress tasks in parallel.
  const [epicsResult, tasksResult] = await Promise.allSettled([
    callDaemon<QueryResult>("/query", { type: "epic", status: "active" }),
    callDaemon<QueryResult>("/query", { type: "task", status: "in-progress" }),
  ]);

  const activeEpics = epicsResult.status === "fulfilled" ? (epicsResult.value.items ?? []) : [];
  const activeTasks = tasksResult.status === "fulfilled" ? (tasksResult.value.items ?? []) : [];

  // Read existing session state if available.
  const sessionStatePath = join(tmpDir, "session-state.md");
  const existingState = existsSync(sessionStatePath)
    ? readFileSync(sessionStatePath, "utf-8")
    : "";

  const lines = [
    "# Governance Context (saved before compaction)",
    "",
    `Saved: ${new Date().toISOString()}`,
    "",
  ];

  if (activeEpics.length > 0) {
    lines.push("## Active Epics", "");
    for (const e of activeEpics) lines.push(`- **${e.id}**: ${e.title ?? e.id}`);
    lines.push("");
  }

  if (activeTasks.length > 0) {
    lines.push("## Active Tasks", "");
    for (const t of activeTasks) lines.push(`- **${t.id}** [${t.status ?? "active"}]: ${t.title ?? t.id}`);
    lines.push("");
  }

  if (existingState) {
    lines.push("## Previous Session State", "", existingState);
  }

  lines.push(
    "",
    "## Recovery Instructions",
    "",
    "After compaction, re-read:",
    "1. The active epic files listed above",
    "2. The active task files listed above",
    "3. `.orqa/process/agents/orchestrator.md` for your role definition",
    "4. Any skills referenced by the current tasks",
  );

  const contextContent = lines.join("\n");
  const contextPath = join(tmpDir, "governance-context.md");
  writeFileSync(contextPath, contextContent);

  let fileSizeBytes = 0;
  try { fileSizeBytes = statSync(contextPath).size; } catch { /* ignore */ }

  logTelemetry("save-context", "PreCompact", startTime, "saved", {
    epics_preserved: activeEpics.length,
    tasks_preserved: activeTasks.length,
    file_size_bytes: fileSizeBytes,
    had_existing_state: existingState.length > 0,
  }, projectDir);

  const summary = [
    "GOVERNANCE CONTEXT PRESERVED before compaction:",
    activeEpics.length > 0
      ? `Active epics: ${activeEpics.map((e) => e.id).join(", ")}`
      : "No active epics",
    activeTasks.length > 0
      ? `Active tasks: ${activeTasks.map((t) => `${t.id} [${t.status ?? "active"}]`).join(", ")}`
      : "No active tasks",
    "",
    "Full context saved to .state/governance-context.md — re-read after compaction.",
  ].join("\n");

  process.stdout.write(JSON.stringify({ systemMessage: summary }));
  process.exit(0);
}

main().catch(() => process.exit(0));
