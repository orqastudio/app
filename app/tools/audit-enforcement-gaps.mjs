#!/usr/bin/env node
// Automated enforcement gap audit (TASK-374).
// Repeatable version of RES-054 — scans all governance artifacts and reports
// enforcement mechanism coverage, chain completeness, and prioritized gaps.
//
// Usage: node tools/audit-enforcement-gaps.mjs
//
// Runs as part of `make verify` and outputs a structured gap report.

import { readFileSync, readdirSync, existsSync } from "fs";
import { resolve, join } from "path";
import { parseFrontmatter } from "./lib/parse-artifact.mjs";

const ROOT = resolve(import.meta.dirname, "..");

// ── Load all artifacts ───────────────────────────────────────────────────────

const artifacts = new Map();

function loadDir(dirPath, type, prefix) {
  if (!existsSync(dirPath)) return;
  for (const file of readdirSync(dirPath).sort()) {
    if (!file.endsWith(".md") || file === "README.md") continue;
    if (prefix && !file.startsWith(prefix)) continue;
    const filePath = join(dirPath, file);
    const content = readFileSync(filePath, "utf-8");
    const fm = parseFrontmatter(filePath);
    if (fm && fm.id) artifacts.set(fm.id, { fm, type, content });
  }
}

function loadSkills(dirPath) {
  if (!existsSync(dirPath)) return;
  for (const subdir of readdirSync(dirPath).sort()) {
    if (subdir.startsWith("_") || subdir === "README.md" || subdir === "schema.json") continue;
    const skillFile = join(dirPath, subdir, "SKILL.md");
    if (!existsSync(skillFile)) continue;
    const content = readFileSync(skillFile, "utf-8");
    const fm = parseFrontmatter(skillFile);
    if (fm && fm.id) artifacts.set(fm.id, { fm, type: "skills", content });
  }
}

loadDir(resolve(ROOT, ".orqa/process/rules"), "rules", "RULE-");
loadDir(resolve(ROOT, ".orqa/process/decisions"), "decisions", "AD-");
loadDir(resolve(ROOT, ".orqa/process/lessons"), "lessons", "IMPL-");
loadSkills(resolve(ROOT, ".orqa/process/skills"));

// ── Audit 1: Rule enforcement mechanisms ─────────────────────────────────────

console.log("=== RULE ENFORCEMENT AUDIT ===\n");

const ruleCategories = { mechanical: 0, behavioral: 0, selfCompliance: 0 };

for (const [id, artifact] of artifacts) {
  if (artifact.type !== "rules") continue;
  if (artifact.fm.status !== "active") continue;

  const hasEnforcement = artifact.fm.enforcement && artifact.fm.enforcement.length > 0;
  const rels = artifact.fm.relationships || [];
  const hasEnforcesRel = rels.some(r => r.type === "enforces" || r.type === "enforced-by");

  if (hasEnforcement) {
    ruleCategories.mechanical++;
  } else if (hasEnforcesRel) {
    ruleCategories.behavioral++;
  } else {
    ruleCategories.selfCompliance++;
    console.log(`  GAP: ${id} (${artifact.fm.title}) — no enforcement mechanism`);
  }
}

console.log(`\n  Mechanical enforcement: ${ruleCategories.mechanical}`);
console.log(`  Behavioral enforcement: ${ruleCategories.behavioral}`);
console.log(`  Self-compliance only: ${ruleCategories.selfCompliance}`);

// ── Audit 2: AD enforcement chain completeness ───────────────────────────────

console.log("\n=== AD ENFORCEMENT CHAIN AUDIT ===\n");

let adsWithChain = 0;
let adsWithoutChain = 0;

for (const [id, artifact] of artifacts) {
  if (artifact.type !== "decisions") continue;
  if (artifact.fm.status !== "accepted") continue;

  const rels = artifact.fm.relationships || [];
  const hasEnforcement = rels.some(r =>
    r.type === "enforced-by" || r.type === "practiced-by"
  );

  if (hasEnforcement) {
    adsWithChain++;
  } else {
    adsWithoutChain++;
    console.log(`  GAP: ${id} (${artifact.fm.title}) — no enforced-by/practiced-by`);
  }
}

console.log(`\n  ADs with enforcement chain: ${adsWithChain}`);
console.log(`  ADs without enforcement chain: ${adsWithoutChain}`);

// ── Audit 3: Lesson promotion status ─────────────────────────────────────────

console.log("\n=== LESSON PROMOTION AUDIT ===\n");

const lessonStats = { active: 0, promoted: 0, recurringUnpromoted: 0, missingGroundedBy: 0 };

for (const [id, artifact] of artifacts) {
  if (artifact.type !== "lessons") continue;

  if (artifact.fm.status === "promoted") {
    lessonStats.promoted++;
    const rels = artifact.fm.relationships || [];
    if (!rels.some(r => r.type === "grounded-by")) {
      lessonStats.missingGroundedBy++;
      console.log(`  GAP: ${id} — promoted but no grounded-by relationship`);
    }
  } else {
    lessonStats.active++;
    if ((artifact.fm.recurrence || 0) >= 2) {
      lessonStats.recurringUnpromoted++;
      console.log(`  GAP: ${id} — recurrence=${artifact.fm.recurrence}, needs promotion review`);
    }
  }
}

console.log(`\n  Active lessons: ${lessonStats.active}`);
console.log(`  Promoted lessons: ${lessonStats.promoted}`);
console.log(`  Recurring unpromoted: ${lessonStats.recurringUnpromoted}`);
console.log(`  Promoted without grounded-by: ${lessonStats.missingGroundedBy}`);

// ── Audit 4: Pipeline stage coverage ─────────────────────────────────────────

console.log("\n=== PIPELINE STAGE COVERAGE ===\n");

const stages = {
  observation: 0,
  understanding: 0,
  principle: 0,
  practice: 0,
  enforcement: 0,
};

for (const [id, artifact] of artifacts) {
  if (artifact.type === "lessons") {
    if (artifact.fm.maturity === "observation") stages.observation++;
    else if (artifact.fm.maturity === "understanding") stages.understanding++;
  } else if (artifact.type === "decisions") {
    if (artifact.fm.status === "accepted") stages.principle++;
  } else if (artifact.type === "skills") {
    stages.practice++;
  } else if (artifact.type === "rules") {
    if (artifact.fm.status === "active") stages.enforcement++;
  }
}

console.log(`  Observation:    ${stages.observation} lessons`);
console.log(`  Understanding:  ${stages.understanding} lessons`);
console.log(`  Principle:      ${stages.principle} decisions`);
console.log(`  Practice:       ${stages.practice} skills`);
console.log(`  Enforcement:    ${stages.enforcement} rules`);

// Ratio checks
const totalPrinciples = stages.principle;
const totalEnforcement = stages.enforcement;
const ratio = totalPrinciples > 0 ? (totalEnforcement / totalPrinciples).toFixed(1) : "N/A";
console.log(`\n  Enforcement/Principle ratio: ${ratio} (ideal: >0.5)`);

// ── Summary ──────────────────────────────────────────────────────────────────

const totalGaps =
  ruleCategories.selfCompliance +
  adsWithoutChain +
  lessonStats.recurringUnpromoted +
  lessonStats.missingGroundedBy;

console.log(`\n${"=".repeat(60)}`);
console.log("ENFORCEMENT GAP AUDIT SUMMARY");
console.log("=".repeat(60));
console.log(`\nTotal gaps found: ${totalGaps}`);
console.log(`  Rules without enforcement: ${ruleCategories.selfCompliance}`);
console.log(`  ADs without enforcement chain: ${adsWithoutChain}`);
console.log(`  Lessons needing promotion: ${lessonStats.recurringUnpromoted}`);
console.log(`  Promoted lessons without tracking: ${lessonStats.missingGroundedBy}`);

if (totalGaps > 0) {
  console.log("\nENFORCEMENT GAPS: FINDINGS PRESENT");
} else {
  console.log("\nENFORCEMENT GAPS: CLEAN");
}
