// PostToolUse hook — TaskUpdate
//
// Enforces RULE-04684a16: when a teammate marks a task as completed,
// verify that a findings file exists at .state/team/*/task-<id>.md.
// If no findings file exists, warn the agent that completion without
// evidence violates the rule.

import { existsSync, readFileSync, readdirSync } from "fs";
import { join } from "path";
import { readInput, outputAllow, outputWarn } from "./shared.js";
import { logTelemetry } from "./telemetry.js";

async function main(): Promise<void> {
  const startTime = Date.now();

  let hookInput;
  try {
    hookInput = await readInput();
  } catch {
    process.exit(0);
  }

  const toolName = hookInput.tool_name ?? "";
  const toolInput = (hookInput.tool_input ?? {}) as Record<string, unknown>;
  const projectDir = hookInput.cwd ?? process.env["CLAUDE_PROJECT_DIR"] ?? ".";

  // Only fire on TaskUpdate calls.
  if (toolName !== "TaskUpdate") {
    outputAllow();
  }

  const taskId = String(toolInput.taskId ?? "");
  const status = String(toolInput.status ?? "");

  // Only check when marking a task as completed.
  if (status !== "completed" || !taskId) {
    outputAllow();
  }

  // Search for findings file in any team directory under .state/team/.
  const teamDir = join(projectDir, ".state", "team");
  const findingsFile = findFindingsFile(teamDir, taskId);

  if (!findingsFile) {
    // No findings file found — warn the agent.
    logTelemetry(
      "findings-check", "PostToolUse", startTime, "missing",
      { task_id: taskId },
      projectDir,
    );

    outputWarn([
      `FINDINGS VIOLATION — Task #${taskId} marked complete without findings file.`,
      "",
      "You MUST write your findings to disk BEFORE marking a task complete.",
      `Expected file: .state/team/<team-name>/task-${taskId}.md`,
      "",
      "The findings file is the evidence of completion. TaskUpdate(completed)",
      "is the signal, not the proof. Write the file now, then re-mark the task.",
    ]);
    process.exit(0);
  }

  // Findings file exists — check for deferral language.
  const deferrals = checkForDeferrals(findingsFile);

  logTelemetry(
    "findings-check", "PostToolUse", startTime,
    deferrals.length > 0 ? "deferred" : "found",
    { task_id: taskId, findings_file: findingsFile, deferrals_found: deferrals.length },
    projectDir,
  );

  if (deferrals.length > 0) {
    outputWarn([
      `DEFERRAL DETECTED — Task #${taskId} findings contain deferred items:`,
      "",
      ...deferrals.map((d) => `  - ${d}`),
      "",
      "Deferred items are first-class work. The orchestrator MUST either:",
      "1. Address them now before marking the task complete, OR",
      "2. Create follow-up tasks and document the deferral explicitly.",
      "",
      "Do NOT mark a task complete with unaddressed deferrals.",
    ]);
    process.exit(0);
  }

  outputAllow();
}

/**
 * Search all team directories under .state/team/ for a findings file matching
 * the pattern task-<id>.md.
 */
function findFindingsFile(teamDir: string, taskId: string): string | null {
  if (!existsSync(teamDir)) return null;
  try {
    const teams = readdirSync(teamDir, { withFileTypes: true });
    for (const entry of teams) {
      if (!entry.isDirectory()) continue;
      const candidate = join(teamDir, entry.name, `task-${taskId}.md`);
      if (existsSync(candidate)) return candidate;
    }
  } catch {
    // Directory read failed — cannot verify, allow silently.
  }
  return null;
}

/**
 * Deferral patterns that indicate incomplete work in findings files.
 */
const DEFERRAL_PATTERNS: RegExp[] = [
  /\bdeferred?\b/i,
  /\bnot (?:done|complete|implemented|addressed)\b/i,
  /\bskipped?\b/i,
  /\bpostponed?\b/i,
  /\bout of scope\b/i,
  /\bfollow[- ]?up required\b/i,
  /\bleft for (?:later|future|next)\b/i,
  /\bTODO\b/,
  /\bFIXME\b/,
];

/**
 * Check a findings file for deferral language. Returns an array of matching
 * lines (trimmed) that suggest incomplete or deferred work.
 */
function checkForDeferrals(filePath: string): string[] {
  try {
    const content = readFileSync(filePath, "utf-8");
    const lines = content.split("\n");
    const matches: string[] = [];
    for (const line of lines) {
      const trimmed = line.trim();
      if (!trimmed || trimmed.startsWith("#")) continue;
      for (const pattern of DEFERRAL_PATTERNS) {
        if (pattern.test(trimmed)) {
          matches.push(trimmed);
          break;
        }
      }
    }
    return matches;
  } catch {
    return [];
  }
}

main().catch(() => process.exit(0));
