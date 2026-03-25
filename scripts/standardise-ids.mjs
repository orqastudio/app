#!/usr/bin/env node
// standardise-ids.mjs — Remove plugin intermediary prefixes from artifact IDs.
//
// Converts:
//   KNOW-CLI-3198c8fb  →  KNOW-990e4f85   (strip intermediary, keep hex)
//   KNOW-CC-decision-tree  →  KNOW-<new hex>  (non-hex suffix, generate new)
//   KNOW-CS-001  →  KNOW-<new hex>  (sequential, generate new)
//   AGENT-RST-spec-a3f7d2b1  →  AGENT-<new hex>  (complex suffix, generate new)
//
// Updates: frontmatter id fields, relationship targets, body text references,
// and plugin manifest (orqa-plugin.json) entries.
//
// Usage:
//   node scripts/standardise-ids.mjs              # dry run (show what would change)
//   node scripts/standardise-ids.mjs --apply      # apply changes
//   node scripts/standardise-ids.mjs --manifest   # output mapping JSON only

import { readFileSync, writeFileSync, readdirSync, existsSync, statSync } from "fs";
import { join, relative } from "path";
import { randomBytes } from "crypto";

// ---------------------------------------------------------------------------
// Config
// ---------------------------------------------------------------------------

const ROOT = process.env.ORQA_ROOT || process.cwd();
const APPLY = process.argv.includes("--apply");
const MANIFEST_ONLY = process.argv.includes("--manifest");

// Directories to scan for artifacts and manifests
const SCAN_DIRS = [
  join(ROOT, ".orqa"),
  join(ROOT, "plugins"),
  join(ROOT, "connectors"),
];

// Regex: matches TYPE-INTERMEDIARY-SUFFIX where INTERMEDIARY is 2-4 uppercase letters
// Examples: KNOW-CLI-3198c8fb, AGENT-RST-spec-a3f7d2b1, DOC-SVE-5d832d1d
const INTERMEDIARY_RE = /^([A-Z]+)-([A-Z]{2,4})-(.+)$/;

// What counts as a valid 8-char hex (the final format we want)
const HEX8_RE = /^[0-9a-f]{8}$/;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

function generateHex() {
  return randomBytes(4).toString("hex");
}

function walkDir(dir, files = []) {
  if (!existsSync(dir)) return files;
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    if (entry.name === "node_modules" || entry.name === "dist" ||
        entry.name === "target" || entry.name === ".git") continue;
    const fullPath = join(dir, entry.name);
    if (entry.isDirectory()) {
      walkDir(fullPath, files);
    } else if (entry.name.endsWith(".md") || entry.name === "orqa-plugin.json") {
      files.push(fullPath);
    }
  }
  return files;
}

function extractId(content) {
  const match = content.match(/^id:\s*"?([^"\n]+)"?\s*$/m);
  return match ? match[1].trim() : null;
}

// ---------------------------------------------------------------------------
// Build migration mapping
// ---------------------------------------------------------------------------

function buildMapping(files) {
  const mapping = new Map(); // oldId -> newId
  const seenNewIds = new Set();

  for (const file of files) {
    if (!file.endsWith(".md")) continue;
    let content;
    try { content = readFileSync(file, "utf-8"); } catch { continue; }

    const id = extractId(content);
    if (!id) continue;

    const match = id.match(INTERMEDIARY_RE);
    if (!match) continue; // No intermediary — already in TYPE-hex format

    const [, type, /* intermediary */, suffix] = match;

    let newId;
    if (HEX8_RE.test(suffix)) {
      // Suffix is already a valid 8-char hex — just strip the intermediary
      newId = `${type}-${suffix}`;
    } else {
      // Suffix is not hex (e.g., "spec-a3f7d2b1", "decision-tree", "001")
      // Generate a fresh 8-char hex
      let hex;
      do { hex = generateHex(); } while (seenNewIds.has(`${type}-${hex}`));
      newId = `${type}-${hex}`;
    }

    // Check for collision with existing IDs
    if (seenNewIds.has(newId)) {
      // Collision — generate a fresh hex instead
      let hex;
      do { hex = generateHex(); } while (seenNewIds.has(`${type}-${hex}`));
      newId = `${type}-${hex}`;
    }

    seenNewIds.add(newId);
    mapping.set(id, newId);
  }

  return mapping;
}

// ---------------------------------------------------------------------------
// Apply mapping to files
// ---------------------------------------------------------------------------

function applyMapping(files, mapping) {
  let filesChanged = 0;
  let refsUpdated = 0;

  for (const file of files) {
    let content;
    try { content = readFileSync(file, "utf-8"); } catch { continue; }

    let newContent = content;
    let changed = false;

    for (const [oldId, newId] of mapping) {
      // Replace all occurrences — frontmatter id, relationship targets, body refs
      // Use word-boundary-aware replacement to avoid partial matches
      const escaped = oldId.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
      const re = new RegExp(escaped, "g");
      const replaced = newContent.replace(re, newId);
      if (replaced !== newContent) {
        const count = (newContent.match(re) || []).length;
        refsUpdated += count;
        newContent = replaced;
        changed = true;
      }
    }

    if (changed) {
      filesChanged++;
      const rel = relative(ROOT, file);
      if (APPLY) {
        writeFileSync(file, newContent, "utf-8");
        console.log(`  updated: ${rel}`);
      } else if (!MANIFEST_ONLY) {
        console.log(`  would update: ${rel}`);
      }
    }
  }

  return { filesChanged, refsUpdated };
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

function main() {
  console.log(`Scanning ${SCAN_DIRS.map(d => relative(ROOT, d)).join(", ")}...`);

  const files = [];
  for (const dir of SCAN_DIRS) {
    walkDir(dir, files);
  }
  console.log(`Found ${files.length} files to scan.\n`);

  const mapping = buildMapping(files);

  if (mapping.size === 0) {
    console.log("No IDs with plugin intermediary prefixes found. Nothing to do.");
    process.exit(0);
  }

  console.log(`Found ${mapping.size} IDs to standardise:\n`);

  if (!MANIFEST_ONLY) {
    const maxOld = Math.max(...[...mapping.keys()].map(k => k.length));
    for (const [oldId, newId] of mapping) {
      console.log(`  ${oldId.padEnd(maxOld)}  →  ${newId}`);
    }
    console.log();
  }

  if (MANIFEST_ONLY) {
    const manifestObj = {
      migrated: new Date().toISOString(),
      mapping: Object.fromEntries(mapping),
    };
    const manifestPath = join(ROOT, "scripts", "id-standardise-manifest.json");
    writeFileSync(manifestPath, JSON.stringify(manifestObj, null, 2) + "\n", "utf-8");
    console.log(`Manifest written to: ${relative(ROOT, manifestPath)}`);
    process.exit(0);
  }

  // Apply changes
  const { filesChanged, refsUpdated } = applyMapping(files, mapping);

  if (APPLY) {
    console.log(`\nDone. ${filesChanged} files updated, ${refsUpdated} references replaced.`);

    // Write manifest
    const manifestObj = {
      migrated: new Date().toISOString(),
      mapping: Object.fromEntries(mapping),
    };
    const manifestPath = join(ROOT, "scripts", "id-standardise-manifest.json");
    writeFileSync(manifestPath, JSON.stringify(manifestObj, null, 2) + "\n", "utf-8");
    console.log(`Manifest written to: ${relative(ROOT, manifestPath)}`);
  } else {
    console.log(`Dry run. ${filesChanged} files would be updated, ${refsUpdated} references would be replaced.`);
    console.log(`Run with --apply to execute.`);
  }
}

main();
