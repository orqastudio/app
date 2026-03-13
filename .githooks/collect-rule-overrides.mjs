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
import { readFileSync, existsSync } from "fs";
import { resolve, join } from "path";
import { createRequire } from "module";

const ROOT = resolve(import.meta.dirname, "..");
const require = createRequire(resolve(ROOT, "ui", "package.json"));
const yaml = require("yaml");

function parseFrontmatter(content) {
  const normalized = content.replace(/\r\n/g, "\n").replace(/\r/g, "\n");
  const lines = normalized.split("\n");
  if (lines[0]?.trim() !== "---") return null;
  for (let i = 1; i < lines.length; i++) {
    if (lines[i].trim() === "---") {
      const yamlBlock = lines.slice(1, i).join("\n");
      try {
        return yaml.parse(yamlBlock);
      } catch {
        return null;
      }
    }
  }
  return null;
}

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
    const content = readFileSync(filePath, "utf-8");
    const fm = parseFrontmatter(content);
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
    const content = readFileSync(taskPath, "utf-8");
    const fm = parseFrontmatter(content);
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
