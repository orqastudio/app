#!/usr/bin/env node
// Mid-cycle orientation check (TASK-386, PILLAR-003).
//
// Re-grounds the agent in the original purpose by reading the active
// epic/task and comparing current work against stated goals.
//
// Usage: node tools/check-orientation.mjs
//
// Designed to be called periodically during extended sessions or
// integrated into a session hook.

import { readFileSync, readdirSync, existsSync } from "fs";
import { resolve, join } from "path";
import { execSync } from "child_process";
import { parseFrontmatter } from "./lib/parse-artifact.mjs";

const ROOT = resolve(import.meta.dirname, "..");

console.log("=== MID-CYCLE ORIENTATION CHECK ===\n");

// Find active work: in-progress epics and tasks
const EPIC_DIR = resolve(ROOT, ".orqa/delivery/epics");
const TASK_DIR = resolve(ROOT, ".orqa/delivery/tasks");

const activeEpics = [];
const activeTasks = [];

if (existsSync(EPIC_DIR)) {
  for (const file of readdirSync(EPIC_DIR).sort()) {
    if (!file.endsWith(".md") || !file.startsWith("EPIC-")) continue;
    const fm = parseFrontmatter(join(EPIC_DIR, file));
    if (fm && fm.status === "in-progress") {
      activeEpics.push(fm);
    }
  }
}

if (existsSync(TASK_DIR)) {
  for (const file of readdirSync(TASK_DIR).sort()) {
    if (!file.endsWith(".md") || !file.startsWith("TASK-")) continue;
    const fm = parseFrontmatter(join(TASK_DIR, file));
    if (fm && fm.status === "in-progress") {
      activeTasks.push(fm);
    }
  }
}

// Report active work
if (activeEpics.length > 0) {
  console.log("ACTIVE EPICS:");
  for (const epic of activeEpics) {
    console.log(`  ${epic.id}: ${epic.title}`);
    console.log(`    ${epic.description || "(no description)"}`);
  }
} else {
  console.log("  No in-progress epics.");
}

console.log("");

if (activeTasks.length > 0) {
  console.log("ACTIVE TASKS:");
  for (const task of activeTasks) {
    console.log(`  ${task.id}: ${task.title} (epic: ${task.epic || "none"})`);
  }
} else {
  console.log("  No in-progress tasks.");
}

// Check session state
const sessionStatePath = resolve(ROOT, "tmp/session-state.md");
if (existsSync(sessionStatePath)) {
  console.log("\nSESSION STATE EXISTS:");
  const content = readFileSync(sessionStatePath, "utf-8");
  // Show first 10 lines as context
  const lines = content.split("\n").slice(0, 10);
  for (const line of lines) {
    console.log(`  ${line}`);
  }
  if (content.split("\n").length > 10) {
    console.log("  ...");
  }
} else {
  console.log("\n  No session state file (tmp/session-state.md).");
}

// Check git log for recent commit context
console.log("\nRECENT COMMITS (last 5):");
try {
  const log = execSync('git log --oneline -5', { encoding: "utf-8" }).trim();
  for (const line of log.split("\n")) {
    console.log(`  ${line}`);
  }
} catch {
  console.log("  (could not read git log)");
}

console.log(`\n${"=".repeat(40)}`);
console.log("ORIENTATION QUESTIONS:");
console.log("  1. Is the current work serving the active epic's goal?");
console.log("  2. Has scope drifted from the original intent?");
console.log("  3. Are there pending decisions that need resolution?");
console.log("  4. Should any work be committed before continuing?");
