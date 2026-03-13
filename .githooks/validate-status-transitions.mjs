#!/usr/bin/env node
// Validates artifact status transitions in staged files.
// Blocks invalid state changes (e.g., draft→in-progress skipping ready).
//
// Called from pre-commit hook when .orqa/ markdown files are staged.

import { readFileSync } from "fs";
import { resolve } from "path";
import { createRequire } from "module";
import { execSync } from "child_process";

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
      try { return yaml.parse(yamlBlock); } catch { return null; }
    }
  }
  return null;
}

// Valid status transitions per artifact type (prefix-based detection)
const VALID_TRANSITIONS = {
  "EPIC-": {
    "draft": ["ready"],
    "ready": ["in-progress"],
    "in-progress": ["review"],
    "review": ["done", "in-progress"], // review can go back to in-progress on FAIL
  },
  "TASK-": {
    "todo": ["in-progress"],
    "in-progress": ["done"],
  },
  "IDEA-": {
    "captured": ["exploring", "archived"],
    "exploring": ["shaped", "archived"],
    "shaped": ["promoted", "archived"],
  },
  "MS-": {
    "planning": ["active"],
    "active": ["complete"],
  },
  "RES-": {
    "draft": ["complete"],
    "complete": ["surpassed"],
  },
  "AD-": {
    "proposed": ["accepted"],
    "accepted": ["superseded", "deprecated"],
  },
};

// Get staged files that are modifications (not new files)
const stagedModified = execSync(
  "git diff --cached --name-only --diff-filter=M", { encoding: "utf-8" }
).trim().split("\n").filter(f => f.startsWith(".orqa/") && f.endsWith(".md"));

let errors = 0;

for (const file of stagedModified) {
  // Get the old (HEAD) version
  let oldContent;
  try {
    oldContent = execSync(`git show HEAD:${file}`, { encoding: "utf-8" });
  } catch {
    continue; // File didn't exist in HEAD (new file)
  }

  const newContent = readFileSync(resolve(ROOT, file), "utf-8");
  const oldFm = parseFrontmatter(oldContent);
  const newFm = parseFrontmatter(newContent);

  if (!oldFm || !newFm) continue;
  if (!oldFm.status || !newFm.status) continue;
  if (oldFm.status === newFm.status) continue;

  // Determine artifact type from filename
  const filename = file.split("/").pop();
  let transitionMap = null;
  for (const [prefix, transitions] of Object.entries(VALID_TRANSITIONS)) {
    if (filename.startsWith(prefix)) {
      transitionMap = transitions;
      break;
    }
  }

  if (!transitionMap) continue; // Unknown artifact type, skip

  const validNextStatuses = transitionMap[oldFm.status];
  if (!validNextStatuses) {
    // Old status is a terminal state or unknown
    console.error(`  ERROR: ${filename}: cannot transition from terminal status "${oldFm.status}" to "${newFm.status}"`);
    errors++;
  } else if (!validNextStatuses.includes(newFm.status)) {
    console.error(`  ERROR: ${filename}: invalid transition "${oldFm.status}" → "${newFm.status}" (valid: ${validNextStatuses.join(", ")})`);
    errors++;
  }
}

if (errors > 0) {
  console.error(`\n${errors} invalid status transition(s) — commit blocked`);
  process.exit(1);
}
