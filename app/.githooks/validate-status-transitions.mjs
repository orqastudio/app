#!/usr/bin/env node
// Validates artifact status transitions in staged files.
// Blocks invalid state changes (e.g., draft→in-progress skipping ready).
//
// Called from pre-commit hook when .orqa/ markdown files are staged.

import { resolve } from "path";
import { execSync } from "child_process";
import { parseFrontmatter } from "../tools/lib/parse-artifact.mjs";

const ROOT = resolve(import.meta.dirname, "..");

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

// Extract status from already-committed content using a regex.
// The old content comes from `git show HEAD:file` — it is already validated,
// so a lightweight regex extraction is sufficient here.
function parseStatusFromContent(content) {
  const normalized = content.replace(/\r\n/g, "\n").replace(/\r/g, "\n");
  const match = normalized.match(/^---\n[\s\S]*?\nstatus:\s*(\S+)[\s\S]*?\n---/m);
  return match?.[1] ?? null;
}

// Get staged files that are modifications (not new files)
const stagedModified = execSync(
  "git diff --cached --name-only --diff-filter=M", { encoding: "utf-8" }
).trim().split("\n").filter(f => f.startsWith(".orqa/") && f.endsWith(".md"));

let errors = 0;

for (const file of stagedModified) {
  // Get the old (HEAD) version — already committed and validated
  let oldContent;
  try {
    oldContent = execSync(`git show HEAD:${file}`, { encoding: "utf-8" });
  } catch {
    continue; // File didn't exist in HEAD (new file)
  }

  // Parse staged file via the Rust binary (authoritative parser for new content)
  const newFm = parseFrontmatter(resolve(ROOT, file));
  const oldStatus = parseStatusFromContent(oldContent);
  const newStatus = newFm?.status ?? null;

  if (!oldStatus || !newStatus) continue;
  if (oldStatus === newStatus) continue;

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

  const validNextStatuses = transitionMap[oldStatus];
  if (!validNextStatuses) {
    // Old status is a terminal state or unknown
    console.error(`  ERROR: ${filename}: cannot transition from terminal status "${oldStatus}" to "${newStatus}"`);
    errors++;
  } else if (!validNextStatuses.includes(newStatus)) {
    console.error(`  ERROR: ${filename}: invalid transition "${oldStatus}" → "${newStatus}" (valid: ${validNextStatuses.join(", ")})`);
    errors++;
  }
}

if (errors > 0) {
  console.error(`\n${errors} invalid status transition(s) — commit blocked`);
  process.exit(1);
}
