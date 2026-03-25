// PostToolUse hook — TaskUpdate
//
// Enforces RULE-04684a16: when a teammate marks a task as completed,
// verify that a findings file exists at .state/team/*/task-<id>.md.
// If no findings file exists, warn the agent that completion without
// evidence violates the rule.

import { existsSync, readdirSync } from "fs";
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

  if (findingsFile) {
    logTelemetry(
      "findings-check", "PostToolUse", startTime, "found",
      { task_id: taskId, findings_file: findingsFile },
      projectDir,
    );
    outputAllow();
  }

  // No findings file found — warn the agent.
  logTelemetry(
    "findings-check", "PostToolUse", startTime, "missing",
    { task_id: taskId },
    projectDir,
  );

  outputWarn([
    `RULE-04684a16 VIOLATION — Task #${taskId} marked complete without findings file.`,
    "",
    "You MUST write your findings to disk BEFORE marking a task complete.",
    `Expected file: .state/team/<team-name>/task-${taskId}.md`,
    "",
    "The findings file is the evidence of completion. TaskUpdate(completed)",
    "is the signal, not the proof. Write the file now, then re-mark the task.",
  ]);
  process.exit(0);
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

main().catch(() => process.exit(0));
