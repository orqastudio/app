#!/usr/bin/env node
// PreCompact hook: saves critical governance context before compaction.
//
// When Claude Code compacts the conversation, important governance context
// (active epic, current tasks, delegation state) could be lost. This hook
// writes a summary to tmp/governance-context.md so the orchestrator can
// recover after compaction.

import { readFileSync, writeFileSync, existsSync, mkdirSync, readdirSync } from "fs";
import { join } from "path";

// Simple YAML frontmatter parser
function parseFrontmatter(content) {
  const match = content.match(/^---\n([\s\S]*?)\n---/);
  if (!match) return null;
  const yaml = match[1];
  const result = {};
  for (const line of yaml.split("\n")) {
    const kvMatch = line.match(/^(\w[\w-]*)\s*:\s*(.+)$/);
    if (kvMatch) {
      let val = kvMatch[2].trim();
      if (val.startsWith('"') && val.endsWith('"')) val = val.slice(1, -1);
      result[kvMatch[1]] = val;
    }
  }
  return result;
}

// Find active epics
function findActiveEpics(projectDir) {
  const epicsDir = join(projectDir, ".orqa", "delivery", "epics");
  if (!existsSync(epicsDir)) return [];

  const epics = [];
  for (const file of readdirSync(epicsDir)) {
    if (!file.endsWith(".md")) continue;
    const content = readFileSync(join(epicsDir, file), "utf-8");
    const fm = parseFrontmatter(content);
    if (fm && fm.status === "active") {
      epics.push({ id: fm.id || file, title: fm.title || fm.id || file });
    }
  }
  return epics;
}

// Find in-progress tasks
function findActiveTasks(projectDir) {
  const tasksDir = join(projectDir, ".orqa", "delivery", "tasks");
  if (!existsSync(tasksDir)) return [];

  const tasks = [];
  for (const file of readdirSync(tasksDir)) {
    if (!file.endsWith(".md")) continue;
    const content = readFileSync(join(tasksDir, file), "utf-8");
    const fm = parseFrontmatter(content);
    if (fm && (fm.status === "active" || fm.status === "review")) {
      tasks.push({ id: fm.id || file, title: fm.title || fm.id || file, status: fm.status });
    }
  }
  return tasks;
}

async function main() {
  let input = "";
  for await (const chunk of process.stdin) {
    input += chunk;
  }

  let hookInput;
  try {
    hookInput = JSON.parse(input);
  } catch {
    process.exit(0);
  }

  const projectDir = hookInput.cwd || process.env.CLAUDE_PROJECT_DIR || ".";
  const tmpDir = join(projectDir, "tmp");

  if (!existsSync(tmpDir)) {
    mkdirSync(tmpDir, { recursive: true });
  }

  // Gather governance context
  const activeEpics = findActiveEpics(projectDir);
  const activeTasks = findActiveTasks(projectDir);

  // Read existing session state if available
  const sessionStatePath = join(tmpDir, "session-state.md");
  const existingState = existsSync(sessionStatePath)
    ? readFileSync(sessionStatePath, "utf-8")
    : "";

  // Write governance context summary
  const lines = [
    "# Governance Context (saved before compaction)",
    "",
    `Saved: ${new Date().toISOString()}`,
    "",
  ];

  if (activeEpics.length > 0) {
    lines.push("## Active Epics", "");
    for (const e of activeEpics) {
      lines.push(`- **${e.id}**: ${e.title}`);
    }
    lines.push("");
  }

  if (activeTasks.length > 0) {
    lines.push("## Active Tasks", "");
    for (const t of activeTasks) {
      lines.push(`- **${t.id}** [${t.status}]: ${t.title}`);
    }
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

  writeFileSync(join(tmpDir, "governance-context.md"), lines.join("\n"));

  // Return the context as a system message so it survives compaction
  const summary = [
    "GOVERNANCE CONTEXT PRESERVED before compaction:",
    activeEpics.length > 0
      ? `Active epics: ${activeEpics.map((e) => e.id).join(", ")}`
      : "No active epics",
    activeTasks.length > 0
      ? `Active tasks: ${activeTasks.map((t) => `${t.id} [${t.status}]`).join(", ")}`
      : "No active tasks",
    "",
    "Full context saved to tmp/governance-context.md — re-read after compaction.",
  ].join("\n");

  process.stdout.write(JSON.stringify({ systemMessage: summary }));
  process.exit(0);
}

main().catch(() => process.exit(0));
