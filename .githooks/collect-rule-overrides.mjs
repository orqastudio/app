#!/usr/bin/env node
// Collects rule-overrides from staged task and epic files.
// Outputs a JSON object: { "RULE-032": "reason", ... }
//
// Resolution: task overrides replace epic overrides (no merge).
// If multiple tasks are staged, all their overrides are collected.
//
// Usage: node collect-rule-overrides.mjs
// Reads staged file list from git.

import { execSync } from "child_process";
import { existsSync } from "fs";
import { resolve, join } from "path";
import { parseFrontmatter } from "../tools/lib/parse-artifact.mjs";

const ROOT = resolve(import.meta.dirname, "..");

function getStagedFiles(pattern) {
  try {
    const output = execSync(
      `git diff --cached --name-only --diff-filter=ACMR -- ${pattern}`,
      { encoding: "utf-8", cwd: ROOT }
    ).trim();
    return output ? output.split("\n") : [];
  } catch {
    return [];
  }
}

function readOverridesFromFile(filePath) {
  try {
    const fm = parseFrontmatter(filePath);
    if (!fm) return [];
    const overrides = fm["rule-overrides"];
    if (!Array.isArray(overrides)) return [];
    return overrides.filter((o) => o.rule && o.reason);
  } catch {
    return [];
  }
}

// Collect overrides from all staged tasks (and their epics as fallback)
const overrides = new Map();

const stagedTasks = getStagedFiles(".orqa/delivery/tasks/TASK-*.md");
for (const taskFile of stagedTasks) {
  const taskPath = resolve(ROOT, taskFile);
  const taskOverrides = readOverridesFromFile(taskPath);

  if (taskOverrides.length > 0) {
    // Task has own overrides — use them (replace, not merge with epic)
    for (const o of taskOverrides) {
      overrides.set(o.rule, o.reason);
    }
  } else {
    // Fall back to epic's overrides
    const fm = parseFrontmatter(taskPath);
    if (fm?.epic) {
      const epicPath = resolve(ROOT, ".orqa", "delivery", "epics", `${fm.epic}.md`);
      if (existsSync(epicPath)) {
        const epicOverrides = readOverridesFromFile(epicPath);
        for (const o of epicOverrides) {
          overrides.set(o.rule, o.reason);
        }
      }
    }
  }
}

// Also check staged epic files directly (for epic-level commits without task files)
const stagedEpics = getStagedFiles(".orqa/delivery/epics/EPIC-*.md");
for (const epicFile of stagedEpics) {
  const epicPath = resolve(ROOT, epicFile);
  const epicOverrides = readOverridesFromFile(epicPath);
  for (const o of epicOverrides) {
    // Don't overwrite task-level overrides
    if (!overrides.has(o.rule)) {
      overrides.set(o.rule, o.reason);
    }
  }
}

// Output as JSON object
const result = Object.fromEntries(overrides);
process.stdout.write(JSON.stringify(result));
