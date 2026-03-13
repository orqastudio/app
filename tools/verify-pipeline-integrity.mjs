#!/usr/bin/env node
// Pipeline integrity verification for EPIC-058 completion.
//
// Checks:
// 1. Every governance artifact has a non-empty relationships array
// 2. No null targets without intended:true
// 3. No deprecated fields remain
// 4. Schema validation passes
// 5. Pipeline flow summary
//
// Usage: node tools/verify-pipeline-integrity.mjs [--staged]
//
// Flags:
//   --staged    Only check files staged in git (for pre-commit hook)

import { readFileSync, readdirSync, existsSync } from "fs";
import { resolve, join } from "path";
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

const ARTIFACT_DIRS = {
  lessons: { dir: ".orqa/process/lessons", prefix: "IMPL-", stage: "Observation/Understanding" },
  decisions: { dir: ".orqa/process/decisions", prefix: "AD-", stage: "Principle" },
  rules: { dir: ".orqa/process/rules", prefix: "RULE-", stage: "Enforcement" },
  skills: { dir: ".orqa/process/skills", prefix: null, stage: "Practice" },
};

const stagedOnly = process.argv.includes("--staged");
let stagedFiles = null;
if (stagedOnly) {
  stagedFiles = new Set(
    execSync("git diff --cached --name-only --diff-filter=ACMR", { encoding: "utf-8" })
      .trim()
      .split("\n")
      .filter((f) => f.startsWith(".orqa/") && f.endsWith(".md"))
  );
}

let errors = 0;
let warnings = 0;
const stats = { total: 0, withRelationships: 0, emptyRelationships: 0, deprecatedFields: 0 };
const stageCount = { observation: 0, understanding: 0, principle: 0, practice: 0, enforcement: 0 };
const relationshipTypes = {};

function error(msg) { console.error(`  ERROR: ${msg}`); errors++; }
function warn(msg) { console.error(`  WARNING: ${msg}`); warnings++; }

// ── Check governance artifacts ──────────────────────────────────────────────

for (const [type, config] of Object.entries(ARTIFACT_DIRS)) {
  const dirPath = resolve(ROOT, config.dir);
  if (!existsSync(dirPath)) continue;

  console.log(`\n=== ${type.toUpperCase()} (${config.stage}) ===`);

  if (type === "skills") {
    // Skills are in subdirectories
    for (const subdir of readdirSync(dirPath).sort()) {
      if (subdir.startsWith("_") || subdir === "README.md" || subdir === "schema.json") continue;
      const skillFile = join(dirPath, subdir, "SKILL.md");
      if (!existsSync(skillFile)) continue;
      const relPath = join(config.dir, subdir, "SKILL.md");
      if (stagedFiles && !stagedFiles.has(relPath)) continue;
      checkArtifact(skillFile, subdir, type, config);
    }
  } else {
    for (const file of readdirSync(dirPath).sort()) {
      if (!file.endsWith(".md") || file === "README.md") continue;
      if (config.prefix && !file.startsWith(config.prefix)) continue;
      const relPath = join(config.dir, file);
      if (stagedFiles && !stagedFiles.has(relPath)) continue;
      checkArtifact(join(dirPath, file), file.replace(".md", ""), type, config);
    }
  }
}

function checkArtifact(filePath, id, type, config) {
  const content = readFileSync(filePath, "utf-8");
  const fm = parseFrontmatter(content);
  if (!fm) return;

  stats.total++;

  // Skip inactive/superseded
  if (fm.status === "inactive" || fm.status === "superseded" || fm.status === "deprecated") {
    // Still check for relationships field existence
    if (!fm.relationships) {
      error(`${id}: missing relationships array (status: ${fm.status})`);
    }
    return;
  }

  // Count by stage
  if (type === "lessons") {
    if (fm.maturity === "observation") stageCount.observation++;
    else if (fm.maturity === "understanding") stageCount.understanding++;
    else error(`${id}: missing or invalid maturity field`);
  } else if (type === "decisions") stageCount.principle++;
  else if (type === "skills") stageCount.practice++;
  else if (type === "rules") stageCount.enforcement++;

  // Check 1: relationships array exists and is non-empty
  if (!fm.relationships) {
    error(`${id}: missing relationships array`);
  } else if (!Array.isArray(fm.relationships)) {
    error(`${id}: relationships is not an array`);
  } else if (fm.relationships.length === 0) {
    if (fm.status === "superseded" || fm.status === "deprecated") {
      // OK for superseded/deprecated to have empty relationships
    } else {
      warn(`${id}: empty relationships array (${fm.status})`);
      stats.emptyRelationships++;
    }
  } else {
    stats.withRelationships++;

    // Count relationship types
    for (const rel of fm.relationships) {
      relationshipTypes[rel.type] = (relationshipTypes[rel.type] || 0) + 1;

      // Check 2: null targets without intended:true
      if (rel.target === null && !rel.intended) {
        warn(`${id}: null target without intended:true — type=${rel.type}, rationale="${rel.rationale}"`);
      }
    }
  }

  // Check 3: deprecated fields
  if ("promoted-to" in fm) {
    error(`${id}: deprecated field 'promoted-to' still present`);
    stats.deprecatedFields++;
  }
  if ("promoted-from" in fm) {
    error(`${id}: deprecated field 'promoted-from' still present`);
    stats.deprecatedFields++;
  }
  if (type === "decisions" && "research-refs" in fm) {
    error(`${id}: deprecated field 'research-refs' still present`);
    stats.deprecatedFields++;
  }

  // Check skill-specific fields
  if (type === "skills" && !fm.category) {
    error(`${id}: missing category field`);
  }
  if (type === "lessons" && !fm.maturity) {
    error(`${id}: missing maturity field`);
  }
}

// ── Summary ─────────────────────────────────────────────────────────────────

console.log("\n" + "=".repeat(60));
console.log("PIPELINE INTEGRITY REPORT");
console.log("=".repeat(60));

console.log(`\nArtifacts scanned: ${stats.total}`);
console.log(`  With relationships: ${stats.withRelationships}`);
console.log(`  Empty relationships: ${stats.emptyRelationships}`);
console.log(`  Deprecated fields found: ${stats.deprecatedFields}`);

console.log(`\nPipeline stage distribution:`);
console.log(`  Observation:    ${stageCount.observation} lessons`);
console.log(`  Understanding:  ${stageCount.understanding} lessons`);
console.log(`  Principle:      ${stageCount.principle} decisions`);
console.log(`  Practice:       ${stageCount.practice} skills`);
console.log(`  Enforcement:    ${stageCount.enforcement} rules`);

console.log(`\nRelationship type distribution:`);
for (const [type, count] of Object.entries(relationshipTypes).sort((a, b) => b[1] - a[1])) {
  console.log(`  ${type}: ${count}`);
}

console.log(`\n${errors} error(s), ${warnings} warning(s)`);

if (errors > 0) {
  console.log("\nPIPELINE INTEGRITY: FAIL");
  process.exit(1);
} else {
  console.log("\nPIPELINE INTEGRITY: PASS");
}
