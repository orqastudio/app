#!/usr/bin/env node
/**
 * Add synchronised-with relationships from skills to their human-facing docs.
 * Also adds inverse relationships on the doc files.
 *
 * Usage:
 *   node scripts/link-skills-to-docs.mjs              # dry run
 *   node scripts/link-skills-to-docs.mjs --apply       # apply changes
 */

import { readFileSync, writeFileSync, readdirSync, existsSync } from "fs";
import { join } from "path";
import { parse as parseYaml, stringify as stringifyYaml } from "yaml";

const ROOT = process.cwd();
const APPLY = process.argv.includes("--apply");

// Mapping: skill directory → doc ID to link to
const SKILL_TO_DOC = {
  // Core platform skills → platform skill catalog
  "app/.orqa/process/skills": "DOC-bad8e26f",

  // Plugin skills → their plugin's development guide
  "plugins/svelte/skills": "DOC-SVE-5d832d1d",
  "plugins/tauri/skills": "DOC-TAU-d9c0d1c7",
  "plugins/rust/skills": "DOC-RST-27becb92",
  "plugins/software/skills": "DOC-SW-421219ce",
  "plugins/cli/skills": "DOC-CLI-2c9bfdda",
  "plugins/typescript/skills": "DOC-TS-e4f5a6b7",

  // Project-level skills → project coding standards
  ".orqa/process/skills": "DOC-9814ec3c",
};

// ---------------------------------------------------------------------------
// Frontmatter helpers
// ---------------------------------------------------------------------------

/**
 * Split a markdown file into { fmText, body }.
 * fmText is the raw YAML between the opening and closing --- markers (no markers).
 * body is everything after the closing --- marker.
 * Returns null if the file has no valid frontmatter.
 */
function splitFrontmatter(content) {
  if (!content.startsWith("---\n")) return null;
  const fmEnd = content.indexOf("\n---", 4);
  if (fmEnd === -1) return null;
  return {
    fmText: content.slice(4, fmEnd),
    body: content.slice(fmEnd + 4),
  };
}

/**
 * Reassemble a markdown file from a parsed frontmatter object and the raw body text.
 */
function joinFrontmatter(fm, body) {
  const newFm = stringifyYaml(fm, { lineWidth: 0 }).trimEnd();
  return "---\n" + newFm + "\n---" + body;
}

/**
 * Parse the YAML frontmatter of a file content string.
 * Returns { fm, parts } or null on failure.
 */
function parseFrontmatter(content) {
  const parts = splitFrontmatter(content);
  if (!parts) return null;

  let fm;
  try {
    fm = parseYaml(parts.fmText);
  } catch {
    return null;
  }

  if (!fm || typeof fm !== "object") return null;
  return { fm, parts };
}

// ---------------------------------------------------------------------------
// Relationship helpers
// ---------------------------------------------------------------------------

/**
 * Extract the artifact id from a parsed frontmatter object, or null if absent.
 */
function extractId(fm) {
  return (fm.id && typeof fm.id === "string") ? fm.id : null;
}

/**
 * Return true if the relationships array already contains a relationship
 * with the given target and type.
 */
function hasRelationship(fm, targetId, relType) {
  if (!Array.isArray(fm.relationships)) return false;
  return fm.relationships.some(
    (r) => r && r.target === targetId && r.type === relType
  );
}

/**
 * Add a relationship to a frontmatter object in-place if it does not already exist.
 * Returns true if the relationship was added, false if it already existed.
 */
function addRelationshipToFm(fm, targetId, relType) {
  if (hasRelationship(fm, targetId, relType)) return false;
  if (!Array.isArray(fm.relationships)) {
    fm.relationships = [];
  }
  fm.relationships.push({ target: targetId, type: relType });
  return true;
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

let totalSkills = 0;
let totalLinked = 0;
let totalAlready = 0;
const docInverses = {}; // docId → [skillIds to add as inverses]

for (const [skillDir, docId] of Object.entries(SKILL_TO_DOC)) {
  const fullDir = join(ROOT, skillDir);
  if (!existsSync(fullDir)) continue;

  for (const file of readdirSync(fullDir)) {
    if (!file.endsWith(".md")) continue;
    const filePath = join(fullDir, file);
    const content = readFileSync(filePath, "utf-8");
    const parsed = parseFrontmatter(content);
    if (!parsed) continue;

    const skillId = extractId(parsed.fm);
    if (!skillId) continue;

    totalSkills++;

    if (hasRelationship(parsed.fm, docId, "synchronised-with")) {
      totalAlready++;
      continue;
    }

    addRelationshipToFm(parsed.fm, docId, "synchronised-with");
    if (APPLY) {
      writeFileSync(filePath, joinFrontmatter(parsed.fm, parsed.parts.body));
    }
    totalLinked++;
    console.log(`  ${APPLY ? "linked" : "would link"}: ${skillId} → ${docId} (${skillDir}/${file})`);

    // Track inverse to add on doc
    if (!docInverses[docId]) docInverses[docId] = [];
    docInverses[docId].push(skillId);
  }
}

// Add inverse relationships on docs
console.log("\nDoc inverse relationships:");
for (const [docId, skillIds] of Object.entries(docInverses)) {
  // Collect candidate doc files from all known doc locations
  const allDocs = [];

  const platformDocDir = join(ROOT, "app/.orqa/documentation/platform");
  if (existsSync(platformDocDir)) {
    allDocs.push(...readdirSync(platformDocDir, { withFileTypes: true }).map(e => join(platformDocDir, e.name)));
  }

  const projectDocDir = join(ROOT, ".orqa/documentation/project");
  if (existsSync(projectDocDir)) {
    allDocs.push(...readdirSync(projectDocDir, { withFileTypes: true }).map(e => join(projectDocDir, e.name)));
  }

  for (const plugin of ["svelte", "tauri", "rust", "software", "cli", "typescript"]) {
    const docDir = join(ROOT, "plugins", plugin, "documentation");
    if (existsSync(docDir)) {
      allDocs.push(...readdirSync(docDir, { withFileTypes: true }).map(e => join(docDir, e.name)));
    }
  }

  for (const docPath of allDocs) {
    if (typeof docPath !== "string" || !docPath.endsWith(".md")) continue;
    if (!existsSync(docPath)) continue;

    const content = readFileSync(docPath, "utf-8");
    const parsed = parseFrontmatter(content);
    if (!parsed) continue;

    const id = extractId(parsed.fm);
    if (id !== docId) continue;

    let changed = false;
    for (const skillId of skillIds) {
      if (addRelationshipToFm(parsed.fm, skillId, "synchronised-with")) {
        changed = true;
        console.log(`  ${APPLY ? "added" : "would add"}: ${docId} ← ${skillId}`);
      }
    }
    if (changed && APPLY) {
      writeFileSync(docPath, joinFrontmatter(parsed.fm, parsed.parts.body));
    }
    break;
  }
}

console.log(`\n${APPLY ? "Done" : "Dry run"}:`);
console.log(`  ${totalSkills} skills found`);
console.log(`  ${totalLinked} ${APPLY ? "linked" : "would link"}`);
console.log(`  ${totalAlready} already linked`);

if (!APPLY) {
  console.log("\nRun with --apply to execute.");
}
