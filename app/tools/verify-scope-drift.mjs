#!/usr/bin/env node
// Scope drift detection (TASK-384, PILLAR-003).
//
// Compares what an epic declared as deliverables (task list, docs-produced)
// against what was actually done. Surfaces when deliverables were silently
// added, removed, or deferred without tracking.
//
// Usage: node tools/verify-scope-drift.mjs [EPIC-NNN]
//
// Without arguments: checks all in-progress and review epics.
// With an epic ID: checks only that epic.

import { readFileSync, readdirSync, existsSync } from "fs";
import { resolve, join } from "path";
import { parseFrontmatter } from "./lib/parse-artifact.mjs";

const ROOT = resolve(import.meta.dirname, "..");

const epicFilter = process.argv[2] || null;
const EPIC_DIR = resolve(ROOT, ".orqa/delivery/epics");
const TASK_DIR = resolve(ROOT, ".orqa/delivery/tasks");

if (!existsSync(EPIC_DIR) || !existsSync(TASK_DIR)) {
  console.log("No epics or tasks directory found.");
  process.exit(0);
}

// Load all tasks
const allTasks = new Map();
for (const file of readdirSync(TASK_DIR).sort()) {
  if (!file.endsWith(".md") || !file.startsWith("TASK-")) continue;
  const fm = parseFrontmatter(join(TASK_DIR, file));
  if (fm) allTasks.set(fm.id, fm);
}

let driftCount = 0;

// Check epics
const checkStatuses = new Set(["in-progress", "review", "done"]);
for (const file of readdirSync(EPIC_DIR).sort()) {
  if (!file.endsWith(".md") || !file.startsWith("EPIC-")) continue;
  const epicFilePath = join(EPIC_DIR, file);
  const content = readFileSync(epicFilePath, "utf-8");
  const fm = parseFrontmatter(epicFilePath);
  if (!fm || !checkStatuses.has(fm.status)) continue;
  if (epicFilter && fm.id !== epicFilter) continue;

  console.log(`\n=== ${fm.id}: ${fm.title} (${fm.status}) ===`);

  // Find all tasks for this epic
  const epicTasks = [...allTasks.values()].filter(t => t.epic === fm.id);
  const doneTasks = epicTasks.filter(t => t.status === "done");
  const todoTasks = epicTasks.filter(t => t.status === "todo");
  const inProgressTasks = epicTasks.filter(t => t.status === "in-progress");

  console.log(`  Tasks: ${epicTasks.length} total, ${doneTasks.length} done, ${inProgressTasks.length} in-progress, ${todoTasks.length} todo`);

  // Check docs-produced (only file paths, not artifact IDs)
  if (fm["docs-produced"] && fm["docs-produced"].length > 0) {
    for (const doc of fm["docs-produced"]) {
      // Skip artifact IDs (AD-NNN, RULE-NNN, etc.) — they are references, not file paths
      if (/^[A-Z]+-\d+$/.test(doc)) continue;
      const docPath = resolve(ROOT, doc);
      if (!existsSync(docPath)) {
        console.error(`  DRIFT: docs-produced "${doc}" does not exist`);
        driftCount++;
      }
    }
  }

  // Check for tasks with no acceptance criteria
  for (const task of epicTasks) {
    if (!task.acceptance || (Array.isArray(task.acceptance) && task.acceptance.length === 0)) {
      console.error(`  DRIFT: ${task.id} has no acceptance criteria`);
      driftCount++;
    }
  }

  // If epic is done, check all tasks are done
  if (fm.status === "done") {
    for (const task of epicTasks) {
      if (task.status !== "done") {
        console.error(`  DRIFT: epic is done but ${task.id} is ${task.status}`);
        driftCount++;
      }
    }
  }

  // If epic is done or review, check for unresolved TBD references
  if (fm.status === "done" || fm.status === "review") {
    const bodyForTbd = content.slice(content.indexOf("---", content.indexOf("---") + 3) + 3);
    const tbdMatches = [...bodyForTbd.matchAll(/\b(?:TASK|EPIC|AD|RULE|IDEA|IMPL|RES|VER|MS|PILLAR)-TBD(?:-\d+)?\b/g)];
    if (tbdMatches.length > 0) {
      const unique = [...new Set(tbdMatches.map(m => m[0]))];
      for (const tbd of unique) {
        console.error(`  DRIFT: ${fm.id} is ${fm.status} but contains unresolved ${tbd}`);
        driftCount++;
      }
    }
  }

  // Extract task IDs mentioned in the epic body's task table
  const bodyStart = content.indexOf("---", content.indexOf("---") + 3);
  if (bodyStart !== -1) {
    const body = content.slice(bodyStart + 3);
    const bodyTaskRefs = new Set();
    for (const match of body.matchAll(/TASK-\d+/g)) {
      bodyTaskRefs.add(match[0]);
    }

    // Tasks in body but not as files
    for (const ref of bodyTaskRefs) {
      if (!allTasks.has(ref)) {
        console.error(`  DRIFT: ${ref} referenced in epic body but no task file exists`);
        driftCount++;
      }
    }
  }
}

// Check done tasks for unresolved TBD references
for (const file of readdirSync(TASK_DIR).sort()) {
  if (!file.endsWith(".md") || !file.startsWith("TASK-")) continue;
  const taskFilePath = join(TASK_DIR, file);
  const content = readFileSync(taskFilePath, "utf-8");
  const fm = parseFrontmatter(taskFilePath);
  if (!fm || fm.status !== "done") continue;

  const bodyStart = content.indexOf("---", content.indexOf("---") + 3);
  if (bodyStart === -1) continue;
  const body = content.slice(bodyStart + 3);
  const tbdMatches = [...body.matchAll(/\b(?:TASK|EPIC|AD|RULE|IDEA|IMPL|RES|VER|MS|PILLAR)-TBD(?:-\d+)?\b/g)];
  if (tbdMatches.length > 0) {
    const unique = [...new Set(tbdMatches.map(m => m[0]))];
    for (const tbd of unique) {
      console.error(`  DRIFT: ${fm.id} is done but contains unresolved ${tbd}`);
      driftCount++;
    }
  }
}

console.log(`\n${"=".repeat(50)}`);
console.log(`SCOPE DRIFT REPORT`);
console.log(`${"=".repeat(50)}`);
console.log(`\nDrift items found: ${driftCount}`);

if (driftCount > 0) {
  console.log("\nSCOPE DRIFT: FINDINGS PRESENT");
} else {
  console.log("\nSCOPE DRIFT: CLEAN");
}
