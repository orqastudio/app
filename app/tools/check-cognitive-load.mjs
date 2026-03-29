#!/usr/bin/env node
// Cognitive load indicators (TASK-387, PILLAR-003).
//
// Detects when a working session has accumulated too much complexity:
// - Too many uncommitted files
// - Too many modified files across different areas
// - Stale worktrees
// - Open stashes
//
// Usage: node tools/check-cognitive-load.mjs
//
// Designed to be called from session hooks or manually.

import { execSync } from "child_process";

const THRESHOLDS = {
  uncommittedFiles: 20,        // RULE-013 threshold
  modifiedAreas: 4,            // Distinct top-level directories with changes
  staleWorktrees: 1,           // Any stale worktree is a warning
  openStashes: 1,              // Any stash is a warning (RULE-013)
  untrackedFiles: 10,          // Untracked files accumulating
};

let warnings = 0;

function warn(msg) { console.warn(`  WARNING: ${msg}`); warnings++; }

console.log("=== COGNITIVE LOAD CHECK ===\n");

// Check uncommitted files
try {
  const status = execSync("git status --short", { encoding: "utf-8" }).trim();
  const lines = status ? status.split("\n") : [];
  const modifiedFiles = lines.filter(l => l.match(/^[MADRCU ][MADRCU ]/));
  const untrackedFiles = lines.filter(l => l.startsWith("??"));

  console.log(`  Modified/staged files: ${modifiedFiles.length}`);
  console.log(`  Untracked files: ${untrackedFiles.length}`);

  if (modifiedFiles.length >= THRESHOLDS.uncommittedFiles) {
    warn(`${modifiedFiles.length} uncommitted files (threshold: ${THRESHOLDS.uncommittedFiles}) — commit at a natural boundary`);
  }

  if (untrackedFiles.length >= THRESHOLDS.untrackedFiles) {
    warn(`${untrackedFiles.length} untracked files — commit or gitignore before proceeding`);
  }

  // Check modified areas (distinct top-level directories)
  const areas = new Set();
  for (const line of modifiedFiles) {
    const filePath = line.slice(3).trim();
    const topDir = filePath.split("/")[0];
    areas.add(topDir);
  }

  console.log(`  Modified areas: ${areas.size} (${[...areas].join(", ") || "none"})`);
  if (areas.size >= THRESHOLDS.modifiedAreas) {
    warn(`Changes span ${areas.size} areas — consider committing completed work before expanding scope`);
  }
} catch (e) {
  console.error(`  Could not check git status: ${e.message}`);
}

// Check stashes
try {
  const stashes = execSync("git stash list", { encoding: "utf-8" }).trim();
  const stashCount = stashes ? stashes.split("\n").length : 0;
  console.log(`  Open stashes: ${stashCount}`);
  if (stashCount >= THRESHOLDS.openStashes) {
    warn(`${stashCount} open stash(es) — stashes risk data loss (RULE-013). Commit to a branch instead.`);
  }
} catch {
  /* git stash list may fail in some contexts */
}

// Check worktrees
try {
  const worktrees = execSync("git worktree list", { encoding: "utf-8" }).trim();
  const worktreeLines = worktrees.split("\n").filter(l => !l.includes("(bare)"));
  // Main worktree is always listed, so > 1 means additional worktrees exist
  const extraWorktrees = worktreeLines.length - 1;
  console.log(`  Extra worktrees: ${extraWorktrees}`);
  if (extraWorktrees >= THRESHOLDS.staleWorktrees) {
    warn(`${extraWorktrees} worktree(s) exist — verify they are active, not stale`);
  }
} catch {
  /* git worktree may not be available */
}

// Summary
console.log(`\n${"=".repeat(40)}`);
if (warnings > 0) {
  console.log(`COGNITIVE LOAD: ${warnings} warning(s) — consider reducing complexity before continuing`);
} else {
  console.log("COGNITIVE LOAD: OK — session complexity is manageable");
}
