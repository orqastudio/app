#!/usr/bin/env node
// Pipeline integrity verification for EPIC-058 completion.
//
// Checks:
// 1. Every governance artifact has a non-empty relationships array
// 2. No null targets without intended:true
// 3. No deprecated fields remain
// 4. Enforcement chain integrity:
//    a. Accepted ADs without enforced-by/practiced-by relationships
//    b. Promoted lessons without grounded-by relationships
//    c. Rules referencing ADs in body without enforces relationships
// 5. Epic reconciliation task presence
// 6. Pipeline flow summary
//
// Usage: node tools/verify-pipeline-integrity.mjs [--staged]
//
// Flags:
//   --staged    Only check files staged in git (for pre-commit hook)

import { readFileSync, readdirSync, existsSync } from "fs";
import { resolve, join } from "path";
import { execSync } from "child_process";
import { parseFrontmatter } from "./lib/parse-artifact.mjs";

const ROOT = resolve(import.meta.dirname, "..");

const ARTIFACT_DIRS = {
  lessons: { dir: ".orqa/learning/lessons", prefix: "IMPL-", stage: "Observation/Understanding" },
  decisions: { dir: ".orqa/learning/decisions", prefix: "AD-", stage: "Principle" },
  rules: { dir: ".orqa/learning/rules", prefix: "RULE-", stage: "Enforcement" },
  skills: { dir: ".orqa/learning/skills", prefix: null, stage: "Practice" },
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
  const fm = parseFrontmatter(filePath);
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

// ── Enforcement chain checks ──────────────────────────────────────────────

// Collect all parsed artifacts for cross-referencing
const allArtifacts = new Map(); // id -> { fm, filePath, type }

// Re-scan to build the full artifact map
for (const [type, config] of Object.entries(ARTIFACT_DIRS)) {
  const dirPath = resolve(ROOT, config.dir);
  if (!existsSync(dirPath)) continue;

  if (type === "skills") {
    for (const subdir of readdirSync(dirPath).sort()) {
      if (subdir.startsWith("_") || subdir === "README.md" || subdir === "schema.json") continue;
      const skillFile = join(dirPath, subdir, "SKILL.md");
      if (!existsSync(skillFile)) continue;
      const content = readFileSync(skillFile, "utf-8");
      const fm = parseFrontmatter(skillFile);
      if (fm && fm.id) allArtifacts.set(fm.id, { fm, filePath: skillFile, type, content });
    }
  } else {
    for (const file of readdirSync(dirPath).sort()) {
      if (!file.endsWith(".md") || file === "README.md") continue;
      if (config.prefix && !file.startsWith(config.prefix)) continue;
      const filePath = join(dirPath, file);
      const content = readFileSync(filePath, "utf-8");
      const fm = parseFrontmatter(filePath);
      if (fm && fm.id) allArtifacts.set(fm.id, { fm, filePath, type, content });
    }
  }
}

const enforcementStats = { adsWithoutEnforcement: 0, lessonsWithoutPromotion: 0, rulesWithoutEnforces: 0 };

console.log("\n=== ENFORCEMENT CHAIN CHECKS ===");

// Check 1: Accepted ADs without enforced-by or practiced-by relationships
for (const [id, artifact] of allArtifacts) {
  if (artifact.type !== "decisions") continue;
  const fm = artifact.fm;
  if (fm.status !== "accepted") continue;

  const rels = fm.relationships || [];
  const hasEnforcement = rels.some(
    (r) => r.type === "enforced-by" || r.type === "practiced-by"
  );

  if (!hasEnforcement) {
    // Check if any relationship has intended:true (strategy decisions that don't need enforcement)
    const hasIntended = rels.some((r) => r.intended === true);
    if (hasIntended) continue;

    warn(`${id}: accepted AD without enforced-by or practiced-by relationship — no enforcement chain`);
    enforcementStats.adsWithoutEnforcement++;
  }
}

// Check 2: Promoted lessons without grounded-by relationships
for (const [id, artifact] of allArtifacts) {
  if (artifact.type !== "lessons") continue;
  const fm = artifact.fm;
  if (fm.status !== "promoted") continue;

  const rels = fm.relationships || [];
  const hasGroundedBy = rels.some((r) => r.type === "grounded-by");

  if (!hasGroundedBy) {
    error(`${id}: promoted lesson without grounded-by relationship — promotion target not traceable`);
    enforcementStats.lessonsWithoutPromotion++;
  }
}

// Check 3: Rules referencing ADs in body text without enforces relationships
const adRefPattern = /\bAD-\d+\b/g;
for (const [id, artifact] of allArtifacts) {
  if (artifact.type !== "rules") continue;
  const fm = artifact.fm;
  if (fm.status === "inactive") continue;

  // Extract AD references from body text (after frontmatter)
  const bodyStart = artifact.content.indexOf("---", artifact.content.indexOf("---") + 3);
  if (bodyStart === -1) continue;
  const body = artifact.content.slice(bodyStart + 3);

  const adRefs = new Set();
  let match;
  while ((match = adRefPattern.exec(body)) !== null) {
    adRefs.add(match[0]);
  }

  if (adRefs.size === 0) continue;

  // Check which ADs this rule has enforces relationships for
  const rels = fm.relationships || [];
  const enforcesTargets = new Set(
    rels.filter((r) => r.type === "enforces").map((r) => r.target)
  );

  for (const adRef of adRefs) {
    if (!enforcesTargets.has(adRef)) {
      // Only warn if the AD is an accepted decision that could be enforced
      const adArtifact = allArtifacts.get(adRef);
      if (adArtifact && adArtifact.fm.status === "accepted") {
        warn(`${id}: references ${adRef} in body but no enforces relationship exists`);
        enforcementStats.rulesWithoutEnforces++;
      }
    }
  }
}

// ── Pipeline health checks ─────────────────────────────────────────────────
// Stage transition gaps: ADs without skills, skills without rules, etc.

const pipelineHealth = {
  adsWithoutPractice: 0,
  skillsWithoutEnforcement: 0,
  rulesWithoutVerification: 0,
  stuckObservations: 0,
  recurringUnpromoted: 0,
};

console.log("\n=== PIPELINE HEALTH ===");

// Build reverse lookup: which ADs are practiced by skills?
const adsPracticedBy = new Set();
for (const [id, artifact] of allArtifacts) {
  const rels = artifact.fm.relationships || [];
  for (const rel of rels) {
    if (rel.type === "practices" && rel.target) adsPracticedBy.add(rel.target);
    if (rel.type === "practiced-by" && rel.target) adsPracticedBy.add(id);
  }
}

// Build reverse lookup: which skills are enforced by rules?
const skillsEnforcedBy = new Set();
for (const [id, artifact] of allArtifacts) {
  const rels = artifact.fm.relationships || [];
  for (const rel of rels) {
    if (rel.type === "enforces" && rel.target) skillsEnforcedBy.add(rel.target);
    if (rel.type === "enforced-by" && rel.target) skillsEnforcedBy.add(id);
  }
}

// Check: Accepted ADs without corresponding skills (practices relationship)
for (const [id, artifact] of allArtifacts) {
  if (artifact.type !== "decisions") continue;
  if (artifact.fm.status !== "accepted") continue;
  if (!adsPracticedBy.has(id)) {
    const hasEnforcement = (artifact.fm.relationships || []).some(
      (r) => r.type === "enforced-by" || r.type === "practiced-by"
    );
    if (!hasEnforcement) {
      // Already warned in enforcement chain checks, skip duplicate
    }
  }
}

// Check: Lessons with recurrence >= 2 that haven't been promoted
for (const [id, artifact] of allArtifacts) {
  if (artifact.type !== "lessons") continue;
  const fm = artifact.fm;
  if (fm.status === "promoted") continue;
  if ((fm.recurrence || 0) >= 2) {
    warn(`${id}: recurrence=${fm.recurrence} but status="${fm.status}" — should be reviewed for promotion`);
    pipelineHealth.recurringUnpromoted++;
  }
}

// Check: Lessons stuck at observation maturity for a long time
// (created > 14 days ago, still maturity: observation, no advancement signals)
const now = new Date();
for (const [id, artifact] of allArtifacts) {
  if (artifact.type !== "lessons") continue;
  const fm = artifact.fm;
  if (fm.maturity !== "observation") continue;
  if (fm.status === "promoted") continue;

  const created = new Date(fm.created);
  const ageDays = Math.floor((now - created) / (1000 * 60 * 60 * 24));
  if (ageDays > 14) {
    warn(`${id}: observation for ${ageDays} days — consider advancing to understanding or archiving`);
    pipelineHealth.stuckObservations++;
  }
}

console.log(`  Recurring unpromoted lessons: ${pipelineHealth.recurringUnpromoted}`);
console.log(`  Stuck observations (>14 days): ${pipelineHealth.stuckObservations}`);

// ── Epic reconciliation ────────────────────────────────────────────────────

const EPIC_DIR = ".orqa/implementation/epics";
const TASK_DIR = ".orqa/implementation/tasks";
const epicDirPath = resolve(ROOT, EPIC_DIR);
const taskDirPath = resolve(ROOT, TASK_DIR);

const epicStats = { scanned: 0, withReconciliation: 0, missingReconciliation: 0 };

if (existsSync(epicDirPath) && existsSync(taskDirPath)) {
  console.log("\n=== EPIC RECONCILIATION ===");

  // Read all tasks once
  const allTasks = [];
  for (const file of readdirSync(taskDirPath).sort()) {
    if (!file.endsWith(".md") || file === "README.md" || !file.startsWith("TASK-")) continue;
    const relPath = join(TASK_DIR, file);
    if (stagedFiles && !stagedFiles.has(relPath)) continue;
    const fm = parseFrontmatter(join(taskDirPath, file));
    if (fm) allTasks.push(fm);
  }

  const checkStatuses = new Set(["ready", "in-progress", "review", "done"]);

  for (const file of readdirSync(epicDirPath).sort()) {
    if (!file.endsWith(".md") || file === "README.md" || !file.startsWith("EPIC-")) continue;
    const relPath = join(EPIC_DIR, file);
    if (stagedFiles && !stagedFiles.has(relPath)) continue;

    const fm = parseFrontmatter(join(epicDirPath, file));
    if (!fm || !checkStatuses.has(fm.status)) continue;

    epicStats.scanned++;
    const epicId = fm.id;

    // Find all tasks for this epic
    const epicTasks = allTasks.filter((t) => t.epic === epicId);
    const reconciliationTask = epicTasks.find((t) => t.title && t.title.startsWith("Reconcile"));

    if (!reconciliationTask) {
      if (fm.status === "done") {
        warn(`${epicId}: completed without reconciliation task`);
      } else {
        error(`${epicId}: no reconciliation task found (status: ${fm.status})`);
      }
      epicStats.missingReconciliation++;
    } else {
      epicStats.withReconciliation++;

      // Check reconciliation task is done if epic is done
      if (fm.status === "done" && reconciliationTask.status !== "done") {
        error(`${epicId}: epic is done but reconciliation task ${reconciliationTask.id} is ${reconciliationTask.status}`);
      }

      // Check that reconciliation task depends on all other epic tasks
      const otherTaskIds = epicTasks
        .filter((t) => t.id !== reconciliationTask.id)
        .map((t) => t.id);
      const dependsOn = new Set(reconciliationTask["depends-on"] || []);

      for (const taskId of otherTaskIds) {
        if (!dependsOn.has(taskId)) {
          warn(`${epicId}: reconciliation task ${reconciliationTask.id} missing depends-on for ${taskId}`);
        }
      }
    }
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

console.log(`\nEnforcement chains:`);
console.log(`  Accepted ADs without enforcement: ${enforcementStats.adsWithoutEnforcement}`);
console.log(`  Promoted lessons without grounded-by: ${enforcementStats.lessonsWithoutPromotion}`);
console.log(`  Rule→AD body refs without enforces: ${enforcementStats.rulesWithoutEnforces}`);

console.log(`\nPipeline health:`);
console.log(`  Recurring unpromoted lessons: ${pipelineHealth.recurringUnpromoted}`);
console.log(`  Stuck observations (>14 days): ${pipelineHealth.stuckObservations}`);

console.log(`\nEpic reconciliation:`);
console.log(`  Epics scanned: ${epicStats.scanned}`);
console.log(`  With reconciliation task: ${epicStats.withReconciliation}`);
console.log(`  Missing reconciliation task: ${epicStats.missingReconciliation}`);

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
