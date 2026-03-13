#!/usr/bin/env node
// Checks documentation pages for required Pillar Alignment sections (RULE-021).
// Only checks docs in .orqa/documentation/ — research, rules, and other exempt
// artifact types are skipped.
//
// Called from pre-commit hook when .orqa/documentation/ markdown files are staged.

import { readFileSync } from "fs";
import { resolve } from "path";
import { execSync } from "child_process";

const ROOT = resolve(import.meta.dirname, "..");

// Exempt directories (per RULE-021)
const EXEMPT_PATHS = [
  ".orqa/documentation/development/", // Dev guidelines
  ".orqa/documentation/product/vision.md",
  ".orqa/documentation/product/governance.md",
];

// Get staged doc files
const stagedDocs = execSync(
  "git diff --cached --name-only --diff-filter=ACMR", { encoding: "utf-8" }
).trim().split("\n")
  .filter(f => f.startsWith(".orqa/documentation/") && f.endsWith(".md") && !f.endsWith("README.md"));

let errors = 0;

for (const file of stagedDocs) {
  // Check exemptions
  if (EXEMPT_PATHS.some(p => file.startsWith(p) || file === p)) continue;

  const content = readFileSync(resolve(ROOT, file), "utf-8");

  // Check for Pillar Alignment section
  if (!content.includes("## Pillar Alignment")) {
    console.error(`  ERROR: ${file}: missing "## Pillar Alignment" section (RULE-021)`);
    errors++;
  }
}

if (errors > 0) {
  console.error(`\n${errors} documentation page(s) missing Pillar Alignment — commit blocked`);
  process.exit(1);
}
