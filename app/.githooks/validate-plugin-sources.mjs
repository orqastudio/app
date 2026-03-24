#!/usr/bin/env node
/**
 * Pre-commit check: block edits to plugin-sourced files in .orqa/.
 *
 * Files tracked in .orqa/manifest.json are installed copies from plugin source
 * directories. Editing them directly causes drift — the edit is lost on the
 * next `orqa plugin refresh`. Edit the plugin source instead.
 *
 * Behavior:
 *   dogfood=true  → ERROR (block commit)
 *   dogfood=false → WARNING (allow commit)
 *
 * Usage: node validate-plugin-sources.mjs <staged-file> [<staged-file> ...]
 */

import { readFileSync, existsSync } from "fs";
import { resolve, relative, join } from "path";

const ROOT = resolve(import.meta.dirname, "..", "..");
const MANIFEST_PATH = join(ROOT, ".orqa", "manifest.json");
const PROJECT_JSON = join(ROOT, ".orqa", "project.json");

// Read manifest
let trackedFiles = new Set();
if (existsSync(MANIFEST_PATH)) {
  try {
    const manifest = JSON.parse(readFileSync(MANIFEST_PATH, "utf-8"));
    for (const [pluginName, entry] of Object.entries(manifest.plugins || {})) {
      for (const filePath of Object.keys(entry.files || {})) {
        trackedFiles.add(filePath);
      }
    }
  } catch {
    // Manifest unreadable — skip check
    process.exit(0);
  }
}

if (trackedFiles.size === 0) {
  process.exit(0);
}

// Check dogfood mode
let isDogfood = false;
if (existsSync(PROJECT_JSON)) {
  try {
    const pj = JSON.parse(readFileSync(PROJECT_JSON, "utf-8"));
    isDogfood = pj.dogfood === true;
  } catch { /* ignore */ }
}

// Check staged files
const stagedFiles = process.argv.slice(2);
const violations = [];

for (const file of stagedFiles) {
  if (trackedFiles.has(file)) {
    // Find which plugin owns this file
    const manifest = JSON.parse(readFileSync(MANIFEST_PATH, "utf-8"));
    let ownerPlugin = "unknown";
    for (const [pluginName, entry] of Object.entries(manifest.plugins || {})) {
      if (entry.files && entry.files[file]) {
        ownerPlugin = pluginName;
        break;
      }
    }
    violations.push({ file, plugin: ownerPlugin });
  }
}

if (violations.length === 0) {
  process.exit(0);
}

const prefix = isDogfood ? "ERROR" : "WARNING";
console.error(`\n  ${prefix}: ${violations.length} staged file(s) are plugin-sourced (edit plugin source instead):\n`);

for (const v of violations) {
  console.error(`    ${v.file}  (from ${v.plugin})`);
}

console.error(`\n  These files are installed copies from plugin source directories.`);
console.error(`  Edits here will be overwritten by \`orqa plugin refresh\`.`);
console.error(`  Edit the file in the plugin source directory instead.\n`);

if (isDogfood) {
  process.exit(1);
} else {
  process.exit(0);
}
