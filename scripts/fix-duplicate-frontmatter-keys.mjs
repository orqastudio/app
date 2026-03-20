#!/usr/bin/env node
/**
 * Fix duplicate YAML frontmatter keys across all .orqa/ artifacts.
 *
 * Uses the `yaml` library for proper parsing and stringification.
 * Merges duplicate `relationships:` arrays and removes duplicate scalar keys.
 *
 * Usage:
 *   node scripts/fix-duplicate-frontmatter-keys.mjs              # dry run
 *   node scripts/fix-duplicate-frontmatter-keys.mjs --apply       # apply fixes
 */

import { readFileSync, writeFileSync, readdirSync } from "fs";
import { join, relative } from "path";
import { parse, stringify } from "yaml";

const ROOT = process.cwd();
const APPLY = process.argv.includes("--apply");

function walkFiles(dir, results = []) {
  let entries;
  try { entries = readdirSync(dir, { withFileTypes: true }); } catch { return results; }
  for (const entry of entries) {
    if (entry.name.startsWith(".") || entry.name === "node_modules" || entry.name === "dist" || entry.name === "target") continue;
    const full = join(dir, entry.name);
    if (entry.isDirectory()) walkFiles(full, results);
    else if (entry.name.endsWith(".md")) results.push(full);
  }
  return results;
}

/**
 * Parse frontmatter manually to detect and merge duplicates.
 * The `yaml` library rejects duplicates, so we pre-process first.
 */
function fixFrontmatter(content) {
  const fmStart = content.indexOf("---\n");
  if (fmStart !== 0) return null;

  const fmEnd = content.indexOf("\n---", 4);
  if (fmEnd === -1) return null;

  const fmText = content.substring(4, fmEnd);
  const body = content.substring(fmEnd + 4); // includes the \n---

  // Check for duplicate top-level keys
  const lines = fmText.split("\n");
  const keyPositions = new Map(); // key → [line indices]

  for (let i = 0; i < lines.length; i++) {
    const match = lines[i].match(/^([a-zA-Z][\w-]*):/);
    if (match) {
      const key = match[1];
      if (!keyPositions.has(key)) keyPositions.set(key, []);
      keyPositions.get(key).push(i);
    }
  }

  // Find keys with duplicates
  const duplicateKeys = [...keyPositions.entries()].filter(([, positions]) => positions.length > 1);
  if (duplicateKeys.length === 0) return null; // no duplicates

  // For each duplicate key, merge the entries
  const processedLines = [];
  const skipRanges = new Set();

  for (const [key, positions] of duplicateKeys) {
    // Keep the first occurrence, merge subsequent ones into it
    const firstPos = positions[0];

    for (let p = 1; p < positions.length; p++) {
      const dupPos = positions[p];

      // Find the range of the duplicate block (the key line + indented lines after it)
      let endOfBlock = dupPos + 1;
      while (endOfBlock < lines.length && (lines[endOfBlock].startsWith("  ") || lines[endOfBlock].startsWith("\t") || lines[endOfBlock].trim() === "")) {
        endOfBlock++;
      }

      // If this is an array key (like relationships:), collect the array items
      if (key === "relationships") {
        // Find where the first block ends
        let firstEnd = firstPos + 1;
        while (firstEnd < lines.length && (lines[firstEnd].startsWith("  ") || lines[firstEnd].startsWith("\t"))) {
          firstEnd++;
        }

        // Collect items from the duplicate block (skip the key line itself)
        const dupItems = [];
        for (let i = dupPos + 1; i < endOfBlock; i++) {
          if (lines[i].trim()) dupItems.push(lines[i]);
        }

        // Insert duplicate items before firstEnd
        if (dupItems.length > 0) {
          // We'll handle this by marking the insert position
          lines[firstEnd - 1] = lines[firstEnd - 1] + "\n" + dupItems.join("\n");
        }
      }

      // Mark the duplicate block for removal
      // Handle empty array syntax: "relationships: []"
      if (lines[dupPos].includes("[]")) {
        skipRanges.add(dupPos);
      } else {
        for (let i = dupPos; i < endOfBlock; i++) {
          skipRanges.add(i);
        }
      }
    }
  }

  // Rebuild frontmatter without skipped lines
  const newLines = [];
  for (let i = 0; i < lines.length; i++) {
    if (!skipRanges.has(i)) {
      newLines.push(lines[i]);
    }
  }

  const newFm = newLines.join("\n");

  // Validate the result parses correctly
  try {
    parse(newFm);
  } catch (e) {
    return { error: e.message.split("\n")[0], file: null };
  }

  return {
    content: "---\n" + newFm + "\n---" + body,
    duplicates: duplicateKeys.map(([k, p]) => `${k} (${p.length}x)`),
  };
}

// Main
const scanDirs = [
  join(ROOT, ".orqa"),
  join(ROOT, "app", ".orqa"),
  join(ROOT, "plugins"),
  join(ROOT, "connectors"),
];

const allFiles = scanDirs.flatMap(d => walkFiles(d));
let fixed = 0;
let errors = 0;

for (const file of allFiles) {
  const content = readFileSync(file, "utf-8");
  const result = fixFrontmatter(content);

  if (!result) continue;

  if (result.error) {
    const rel = relative(ROOT, file);
    console.log(`  ERROR: ${rel} — ${result.error}`);
    errors++;
    continue;
  }

  const rel = relative(ROOT, file);
  console.log(`  ${APPLY ? "fixed" : "would fix"}: ${rel} — ${result.duplicates.join(", ")}`);

  if (APPLY) {
    writeFileSync(file, result.content);
  }
  fixed++;
}

console.log(`\n${APPLY ? "Done" : "Dry run"}: ${fixed} files ${APPLY ? "fixed" : "to fix"}, ${errors} errors`);
if (!APPLY && fixed > 0) console.log("Run with --apply to execute.");
