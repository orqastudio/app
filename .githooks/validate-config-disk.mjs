#!/usr/bin/env node
// Validates that project.json artifact paths match actual directories on disk.
// Enforces RULE-003: config-driven scanning — every path in config must resolve.
//
// Called from pre-commit hook when project.json is staged.

import { readFileSync, existsSync, statSync } from "fs";
import { resolve } from "path";

const ROOT = resolve(import.meta.dirname, "..");
const configPath = resolve(ROOT, ".orqa/project.json");

if (!existsSync(configPath)) {
  console.error("  ERROR: .orqa/project.json does not exist");
  process.exit(1);
}

const config = JSON.parse(readFileSync(configPath, "utf-8"));
const artifacts = config.artifacts || [];

let errors = 0;

function checkEntry(entry, parentPath) {
  if (entry.path) {
    const fullPath = resolve(ROOT, entry.path);
    if (!existsSync(fullPath)) {
      console.error(`  ERROR: config path "${entry.path}" does not exist on disk`);
      errors++;
    } else if (!statSync(fullPath).isDirectory()) {
      console.error(`  ERROR: config path "${entry.path}" is not a directory`);
      errors++;
    }
  }

  if (entry.children) {
    for (const child of entry.children) {
      checkEntry(child, entry.path || parentPath);
    }
  }
}

for (const entry of artifacts) {
  checkEntry(entry, "");
}

if (errors > 0) {
  console.error(`\n${errors} config-disk mismatch(es) — commit blocked`);
  process.exit(1);
}
