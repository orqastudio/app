#!/usr/bin/env node
/**
 * Bulk artifact ID migration: sequential (TYPE-NNN) → hex (TYPE-XXXXXXXX)
 *
 * Usage:
 *   node scripts/migrate-artifact-ids.mjs              # dry run
 *   node scripts/migrate-artifact-ids.mjs --apply       # apply changes
 *   node scripts/migrate-artifact-ids.mjs --manifest    # output mapping only
 *
 * Scans all .md and .json files under .orqa/, app/.orqa/, plugins/, and connectors/.
 * Generates a new hex ID for each artifact, then updates:
 *   1. The `id:` field in YAML frontmatter
 *   2. All `target:` references in relationship arrays
 *   3. All body text references (prose, links, tables)
 *   4. Plugin manifests (orqa-plugin.json skill ID entries)
 *
 * Produces a migration manifest (scripts/id-migration-manifest.json) mapping old → new.
 */

import { readFileSync, writeFileSync, readdirSync, existsSync } from "fs";
import { join, relative } from "path";
import { randomBytes } from "crypto";
import { parse as parseYaml, stringify as stringifyYaml } from "yaml";

const ROOT = process.cwd();
const APPLY = process.argv.includes("--apply");
const MANIFEST_ONLY = process.argv.includes("--manifest");

// ---------------------------------------------------------------------------
// ID generation
// ---------------------------------------------------------------------------

function generateHexId(prefix) {
  const hex = randomBytes(4).toString("hex");
  return `${prefix}-${hex}`;
}

// ---------------------------------------------------------------------------
// File discovery
// ---------------------------------------------------------------------------

function walkFiles(dir, extensions, results = []) {
  if (!existsSync(dir)) return results;
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    if (entry.name.startsWith(".") || entry.name === "node_modules" || entry.name === "dist" || entry.name === "target") continue;
    const full = join(dir, entry.name);
    if (entry.isDirectory()) {
      walkFiles(full, extensions, results);
    } else if (extensions.some((ext) => entry.name.endsWith(ext))) {
      results.push(full);
    }
  }
  return results;
}

// ---------------------------------------------------------------------------
// Frontmatter helpers
// ---------------------------------------------------------------------------

/**
 * Split a markdown file into { fmText, body }.
 * fmText is the raw YAML between the opening and closing --- markers (no markers).
 * body is everything after the closing --- marker (starting with \n if there is content).
 * Returns null if the file has no valid frontmatter.
 */
function splitFrontmatter(content) {
  if (!content.startsWith("---\n")) return null;
  const fmEnd = content.indexOf("\n---", 4);
  if (fmEnd === -1) return null;
  return {
    fmText: content.slice(4, fmEnd),
    // body: everything after the \n--- closing marker (the \n--- itself is 4 chars)
    body: content.slice(fmEnd + 4), // content after the closing ---
  };
}

/**
 * Reassemble a markdown file from a parsed frontmatter object and the raw body text.
 */
function joinFrontmatter(fm, body) {
  const newFm = stringifyYaml(fm, { lineWidth: 0 }).trimEnd();
  return "---\n" + newFm + "\n---" + body;
}

// ---------------------------------------------------------------------------
// ID extraction
// ---------------------------------------------------------------------------

/**
 * Extract the artifact ID from a markdown file by parsing its YAML frontmatter.
 * Returns null if no frontmatter, no id field, or the id is not in sequential format.
 */
function extractArtifactId(content) {
  const parts = splitFrontmatter(content);
  if (!parts) return null;

  let fm;
  try {
    fm = parseYaml(parts.fmText);
  } catch {
    return null;
  }

  if (!fm || typeof fm !== "object") return null;
  const id = fm.id;
  if (!id || typeof id !== "string") return null;
  if (isTemplateId(id)) return null;
  if (isAlreadyHex(id)) return null;
  return id;
}

function getTypePrefix(id) {
  // SKILL-SVE-001 → SKILL-SVE, TASK-580 → TASK, AD-057 → AD
  // For compound prefixes like SKILL-SVE, we want the full type prefix
  const parts = id.split("-");
  if (parts.length === 3) {
    // SKILL-SVE-001 — first two parts are the prefix
    return `${parts[0]}-${parts[1]}`;
  }
  // TASK-580, AD-057, EPIC-094 — first part is the prefix
  return parts[0];
}

// IDs that should NOT be migrated (templates, examples in docs)
function isTemplateId(id) {
  return id.includes("NNN") || id === "SKILL-NNN" || id === "TASK-NNN";
}

// Already hex format
function isAlreadyHex(id) {
  const suffix = id.split("-").pop();
  return suffix.length === 8 && /^[0-9a-f]+$/.test(suffix);
}

// ---------------------------------------------------------------------------
// Build migration mapping
// ---------------------------------------------------------------------------

function buildMigrationMap() {
  const scanDirs = [
    join(ROOT, ".orqa"),
    join(ROOT, "app", ".orqa"),
    join(ROOT, "plugins"),
    join(ROOT, "connectors"),
  ];

  const allFiles = [];
  for (const dir of scanDirs) {
    walkFiles(dir, [".md"], allFiles);
  }

  const mapping = {}; // old ID → new ID
  const idLocations = {}; // old ID → file path where it's defined

  for (const file of allFiles) {
    const content = readFileSync(file, "utf-8");
    const id = extractArtifactId(content);
    if (!id) continue;

    const prefix = getTypePrefix(id);
    const newId = generateHexId(prefix);
    mapping[id] = newId;
    idLocations[id] = relative(ROOT, file);
  }

  return { mapping, idLocations };
}

// ---------------------------------------------------------------------------
// YAML-aware frontmatter update for markdown files
// ---------------------------------------------------------------------------

/**
 * Recursively walk frontmatter object fields and replace old IDs in string values.
 * Handles plain string fields and arrays of strings or objects.
 * The `relationships` array is handled separately — only non-relationship fields
 * are processed here.
 * Returns true if any change was made.
 */
function updateFrontmatterStringFields(obj, mapping, skipKey = null) {
  let changed = false;
  for (const key of Object.keys(obj)) {
    if (key === skipKey) continue;
    const val = obj[key];
    if (typeof val === "string" && mapping[val]) {
      obj[key] = mapping[val];
      changed = true;
    } else if (Array.isArray(val)) {
      for (let i = 0; i < val.length; i++) {
        if (typeof val[i] === "string" && mapping[val[i]]) {
          val[i] = mapping[val[i]];
          changed = true;
        } else if (val[i] && typeof val[i] === "object") {
          changed = updateFrontmatterStringFields(val[i], mapping) || changed;
        }
      }
    }
  }
  return changed;
}

/**
 * Replace old IDs in a plain text string (body text, prose, tables) using regex.
 * Safe to use on non-YAML content.
 */
function replaceIdsInText(text, oldIds, mapping) {
  if (oldIds.length === 0) return text;
  const pattern = new RegExp(
    oldIds.map((id) => id.replace(/[-]/g, "\\-")).join("|"),
    "g"
  );
  return text.replace(pattern, (match) => mapping[match] ?? match);
}

/**
 * Update a markdown file using YAML parse/stringify for frontmatter and regex for body.
 * Returns the updated content string, or null if no changes were needed.
 */
function updateMarkdownFile(content, mapping, oldIds) {
  const parts = splitFrontmatter(content);

  if (!parts) {
    // No frontmatter — only update body text via regex
    const updated = replaceIdsInText(content, oldIds, mapping);
    return updated !== content ? updated : null;
  }

  let fm;
  try {
    fm = parseYaml(parts.fmText);
  } catch {
    // Invalid YAML frontmatter — fall back to body-only regex update
    const updated = replaceIdsInText(content, oldIds, mapping);
    return updated !== content ? updated : null;
  }

  let fmChanged = false;

  if (fm && typeof fm === "object") {
    // 1. Update the id field
    if (fm.id && mapping[fm.id]) {
      fm.id = mapping[fm.id];
      fmChanged = true;
    }

    // 2. Update relationship targets
    if (Array.isArray(fm.relationships)) {
      for (const rel of fm.relationships) {
        if (rel && typeof rel === "object" && rel.target && mapping[rel.target]) {
          rel.target = mapping[rel.target];
          fmChanged = true;
        }
      }
    }

    // 3. Update other string fields (depends-on, epic, etc.), skip relationships (handled above)
    const otherChanged = updateFrontmatterStringFields(fm, mapping, "relationships");
    fmChanged = fmChanged || otherChanged;
  }

  // 4. Update body text via regex
  const updatedBody = replaceIdsInText(parts.body, oldIds, mapping);
  const bodyChanged = updatedBody !== parts.body;

  if (!fmChanged && !bodyChanged) return null;

  const newFm = fmChanged ? fm : null;
  const newBody = bodyChanged ? updatedBody : parts.body;

  if (newFm) {
    return joinFrontmatter(newFm, newBody);
  }
  return "---\n" + parts.fmText + "\n---" + newBody;
}

// ---------------------------------------------------------------------------
// Apply migration
// ---------------------------------------------------------------------------

function applyMigration(mapping) {
  const oldIds = Object.keys(mapping).sort((a, b) => b.length - a.length);
  if (oldIds.length === 0) {
    console.log("No IDs to migrate.");
    return { filesChanged: 0, replacements: 0 };
  }

  // Scan ALL files that could contain references
  const scanDirs = [
    join(ROOT, ".orqa"),
    join(ROOT, "app", ".orqa"),
    join(ROOT, "plugins"),
    join(ROOT, "connectors"),
  ];

  const allMdFiles = [];
  const allJsonFiles = [];
  for (const dir of scanDirs) {
    walkFiles(dir, [".md"], allMdFiles);
    walkFiles(dir, [".json"], allJsonFiles);
  }

  let filesChanged = 0;
  let totalReplacements = 0;

  // Process markdown files with YAML-aware frontmatter handling
  for (const file of allMdFiles) {
    const original = readFileSync(file, "utf-8");
    const updated = updateMarkdownFile(original, mapping, oldIds);

    if (updated !== null) {
      const count = countOldIdOccurrences(original, oldIds);
      if (APPLY) {
        writeFileSync(file, updated);
      }
      filesChanged++;
      totalReplacements += count;
      const rel = relative(ROOT, file);
      console.log(`  ${APPLY ? "updated" : "would update"}: ${rel} (${count} replacements)`);
    }
  }

  // Process JSON files with simple text replacement (IDs in JSON are plain strings)
  const jsonPattern = new RegExp(
    oldIds.map((id) => id.replace(/[-]/g, "\\-")).join("|"),
    "g"
  );

  for (const file of allJsonFiles) {
    const original = readFileSync(file, "utf-8");
    let count = 0;
    const updated = original.replace(jsonPattern, (match) => {
      if (mapping[match]) {
        count++;
        return mapping[match];
      }
      return match;
    });

    if (count > 0) {
      if (APPLY) {
        writeFileSync(file, updated);
      }
      filesChanged++;
      totalReplacements += count;
      const rel = relative(ROOT, file);
      console.log(`  ${APPLY ? "updated" : "would update"}: ${rel} (${count} replacements)`);
    }
  }

  return { filesChanged, replacements: totalReplacements };
}

/**
 * Count total occurrences of any old ID in the original content string.
 */
function countOldIdOccurrences(content, oldIds) {
  let count = 0;
  for (const id of oldIds) {
    const escaped = id.replace(/[-]/g, "\\-");
    count += (content.match(new RegExp(escaped, "g")) || []).length;
  }
  return count;
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

console.log("Artifact ID Migration: sequential → hex (TYPE-XXXXXXXX)");
console.log("=========================================================\n");

const { mapping, idLocations } = buildMigrationMap();

console.log(`Found ${Object.keys(mapping).length} artifacts to migrate:\n`);

// Show mapping
const entries = Object.entries(mapping).sort(([a], [b]) => a.localeCompare(b));
for (const [oldId, newId] of entries) {
  const location = idLocations[oldId];
  console.log(`  ${oldId.padEnd(20)} → ${newId.padEnd(20)} (${location})`);
}

// Write manifest
const manifestPath = join(ROOT, "scripts", "id-migration-manifest.json");
writeFileSync(manifestPath, JSON.stringify({ migrated: new Date().toISOString(), mapping }, null, 2));
console.log(`\nManifest written to: ${relative(ROOT, manifestPath)}`);

if (MANIFEST_ONLY) {
  process.exit(0);
}

console.log(`\n${APPLY ? "Applying" : "Dry run (pass --apply to execute)"}...\n`);

const { filesChanged, replacements } = applyMigration(mapping);

console.log(`\n${APPLY ? "Done" : "Dry run complete"}:`);
console.log(`  ${Object.keys(mapping).length} IDs migrated`);
console.log(`  ${filesChanged} files ${APPLY ? "changed" : "would change"}`);
console.log(`  ${replacements} total replacements`);

if (!APPLY) {
  console.log("\nRun with --apply to execute the migration.");
}
