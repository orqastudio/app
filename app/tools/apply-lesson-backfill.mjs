#!/usr/bin/env node
// ARCHIVED: One-off migration tool — ran once to backfill maturity and
// relationship fields onto existing lesson artifacts. Not intended for repeat
// use. parseFrontmatter intentionally left as-is (hand-rolled) since this
// script is not maintained going forward.
//
// Batch apply maturity and relationship backfill to all lessons.
//
// Usage: node tools/apply-lesson-backfill.mjs [--dry-run]

import { readFileSync, writeFileSync, readdirSync, existsSync } from "fs";
import { resolve, join } from "path";
import { createRequire } from "module";

const ROOT = resolve(import.meta.dirname, "..");
const require = createRequire(resolve(ROOT, "ui", "package.json"));
const yaml = require("yaml");

const dryRun = process.argv.includes("--dry-run");

// ── Frontmatter Parsing ─────────────────────────────────────────────────────

function parseFrontmatter(content) {
  const normalized = content.replace(/\r\n/g, "\n").replace(/\r/g, "\n");
  const lines = normalized.split("\n");
  if (lines[0]?.trim() !== "---") return null;
  for (let i = 1; i < lines.length; i++) {
    if (lines[i].trim() === "---") {
      const yamlBlock = lines.slice(1, i).join("\n");
      try {
        return yaml.parse(yamlBlock);
      } catch {
        return null;
      }
    }
  }
  return null;
}

function updateLessonFrontmatter(filePath, updates) {
  const content = readFileSync(filePath, "utf-8");
  const normalized = content.replace(/\r\n/g, "\n").replace(/\r/g, "\n");
  const lines = normalized.split("\n");

  let fmEnd = -1;
  let delimCount = 0;
  for (let i = 0; i < lines.length; i++) {
    if (lines[i].trim() === "---") {
      delimCount++;
      if (delimCount === 2) { fmEnd = i; break; }
    }
  }
  if (fmEnd === -1) return false;

  const fmBlock = lines.slice(1, fmEnd).join("\n");
  const body = lines.slice(fmEnd + 1).join("\n");

  let fm;
  try { fm = yaml.parse(fmBlock); } catch { return false; }

  Object.assign(fm, updates);

  const schemaPath = join(resolve(filePath, ".."), "schema.json");
  let propertyOrder = Object.keys(fm);
  if (existsSync(schemaPath)) {
    try {
      const schema = JSON.parse(readFileSync(schemaPath, "utf-8"));
      if (schema.propertyOrder) propertyOrder = schema.propertyOrder;
    } catch {}
  }

  // promoted-to is not in schema propertyOrder but exists in files — preserve it
  const orderedFm = {};
  for (const key of propertyOrder) {
    if (key in fm) orderedFm[key] = fm[key];
  }
  for (const key of Object.keys(fm)) {
    if (!(key in orderedFm)) orderedFm[key] = fm[key];
  }

  const newFmBlock = yaml.stringify(orderedFm, {
    lineWidth: 0,
    defaultKeyType: "PLAIN",
    defaultStringType: "QUOTE_DOUBLE",
  }).trim();

  const result = `---\n${newFmBlock}\n---\n${body}`;
  if (!dryRun) {
    writeFileSync(filePath, result, "utf-8");
  }
  return true;
}

// ── Maturity Classification ─────────────────────────────────────────────────

// observation: raw capture of what happened (no root cause identified)
// understanding: root cause identified, pattern recognized, fix known
const LESSON_MATURITY = {
  "IMPL-001": "observation",   // Vite dependency optimization — describes symptom and workaround, not root cause
  "IMPL-002": "observation",   // Port-in-use — describes symptom and workaround
  "IMPL-003": "understanding", // Orchestrator lifecycle — identified systemic issue (agent should manage lifecycle)
  "IMPL-004": "understanding", // $derived.by() — root cause identified (Svelte 5 API misunderstanding)
  "IMPL-005": "understanding", // Config paths — root cause identified (scanner silently returns empty)
  "IMPL-006": "understanding", // Symlinks — root cause identified (copies diverge, symlinks don't)
  "IMPL-007": "understanding", // Don't refactor agentic structure with agents — root cause: stale context
  "IMPL-008": "understanding", // Extract domain logic — root cause: incremental growth pattern
  "IMPL-009": "understanding", // Domain-neutral naming — root cause: technology-specific naming
  "IMPL-010": "understanding", // App-native docs — root cause: parallel systems diverge
  "IMPL-011": "understanding", // Investigate systemically — root cause: isolated fixes miss shared causes
  "IMPL-012": "understanding", // Encode improvements — root cause: conversation-only knowledge is lost
  "IMPL-013": "understanding", // Process skills at orchestration level — root cause: wrong agent ownership
  "IMPL-014": "understanding", // Epic titles describe outcomes — root cause: process-oriented naming
  "IMPL-015": "understanding", // Commit at natural boundaries — root cause: no commit threshold
  "IMPL-016": "understanding", // Deferred deliverable — root cause: scope reduction without user approval
};

// ── Relationship Mapping ────────────────────────────────────────────────────

const LESSON_RELATIONSHIPS = {
  "IMPL-001": [
    { target: "PILLAR-001", type: "grounded", rationale: "Dev environment reliability is structural clarity" },
  ],
  "IMPL-002": [
    { target: "PILLAR-001", type: "grounded", rationale: "Process cleanup prevents structural confusion" },
  ],
  "IMPL-003": [
    { target: "PILLAR-001", type: "grounded", rationale: "Automated lifecycle management provides structural predictability" },
    { target: "IMPL-002", type: "informs", rationale: "Both address dev server lifecycle management" },
  ],
  "IMPL-004": [
    { target: "PILLAR-001", type: "grounded", rationale: "Correct reactive patterns ensure structural UI consistency" },
  ],
  "IMPL-005": [
    { target: "PILLAR-001", type: "grounded", rationale: "Config-disk alignment is fundamental structural integrity" },
  ],
  "IMPL-006": [
    { target: "PILLAR-001", type: "grounded", rationale: "Single source of truth prevents structural divergence" },
    { target: "IMPL-005", type: "informs", rationale: "Both address source-of-truth alignment" },
  ],
  "IMPL-007": [
    { target: "PILLAR-001", type: "grounded", rationale: "Self-referential modification requires structural awareness" },
  ],
  "IMPL-008": [
    { target: "PILLAR-001", type: "grounded", rationale: "Domain service extraction creates structural clarity" },
  ],
  "IMPL-009": [
    { target: "PILLAR-001", type: "grounded", rationale: "Domain-neutral naming creates stable structural interfaces" },
  ],
  "IMPL-010": [
    { target: "PILLAR-001", type: "grounded", rationale: "Native rendering eliminates structural duplication" },
  ],
  "IMPL-011": [
    { target: "PILLAR-002", type: "grounded", rationale: "Systemic investigation is the core learning methodology" },
  ],
  "IMPL-012": [
    { target: "PILLAR-002", type: "grounded", rationale: "Encoding improvements closes the learning loop" },
    { target: "IMPL-011", type: "informs", rationale: "Both address the learning process — investigation and encoding" },
  ],
  "IMPL-013": [
    { target: "PILLAR-001", type: "grounded", rationale: "Correct skill ownership creates structural clarity in agent roles" },
  ],
  "IMPL-014": [
    { target: "PILLAR-001", type: "grounded", rationale: "Outcome-oriented naming creates structural clarity in planning" },
  ],
  "IMPL-015": [
    { target: "PILLAR-001", type: "grounded", rationale: "Commit discipline maintains structural integrity of version history" },
  ],
  "IMPL-016": [
    { target: "PILLAR-001", type: "grounded", rationale: "Scope integrity prevents structural gaps in deliverables" },
  ],
};

// ── Main ────────────────────────────────────────────────────────────────────

const lessonsDir = resolve(ROOT, ".orqa/process/lessons");
let updated = 0;

for (const file of readdirSync(lessonsDir).sort()) {
  if (!file.startsWith("IMPL-") || !file.endsWith(".md")) continue;

  const lessonId = file.replace(".md", "");
  const filePath = join(lessonsDir, file);
  const content = readFileSync(filePath, "utf-8");
  const fm = parseFrontmatter(content);
  if (!fm) continue;

  const updates = {};
  let needsUpdate = false;

  // Add maturity if missing
  if (!fm.maturity) {
    const maturity = LESSON_MATURITY[lessonId] || "observation";
    updates.maturity = maturity;
    needsUpdate = true;
  }

  // Add relationships if missing
  if (!fm.relationships || fm.relationships.length === 0) {
    const relationships = LESSON_RELATIONSHIPS[lessonId];
    if (relationships) {
      updates.relationships = relationships;
    } else {
      updates.relationships = [{
        target: "PILLAR-001",
        type: "grounded",
        rationale: "Provides structural knowledge (default — needs human review)",
      }];
    }
    needsUpdate = true;
  }

  if (!needsUpdate) {
    if (!dryRun) console.log(`${lessonId}: already up to date, skipping`);
    continue;
  }

  if (dryRun) {
    console.log(`${lessonId}: would update`);
    if (updates.maturity) console.log(`  maturity: ${updates.maturity}`);
    if (updates.relationships) {
      for (const r of updates.relationships) {
        console.log(`  ${r.type}: ${r.target} — ${r.rationale}`);
      }
    }
  } else {
    const success = updateLessonFrontmatter(filePath, updates);
    if (success) {
      console.log(`${lessonId}: updated (maturity: ${updates.maturity || fm.maturity}, relationships: ${(updates.relationships || fm.relationships).length})`);
      updated++;
    } else {
      console.error(`${lessonId}: FAILED to update`);
    }
  }
}

console.log(`\n${updated} lesson(s) updated.`);
