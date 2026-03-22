#!/usr/bin/env node
// ARCHIVED: One-off migration tool — ran once to migrate deprecated frontmatter
// fields into relationship edges. Not intended for repeat use.
// parseFrontmatter intentionally left as-is (hand-rolled) since this script
// is not maintained going forward.
//
// Migrate deprecated fields into relationships and then remove them.
//
// 1. Lesson promoted-to → add grounded-by relationship, then remove field
// 2. Rule promoted-from → add observes relationship, then remove field
// 3. Decision research-refs → add informed-by relationship, then remove field
//
// Usage: node tools/migrate-deprecated-fields.mjs [--dry-run]

import { readFileSync, writeFileSync, readdirSync, existsSync } from "fs";
import { resolve, join } from "path";
import { createRequire } from "module";

const ROOT = resolve(import.meta.dirname, "..");
const require = createRequire(resolve(ROOT, "ui", "package.json"));
const yaml = require("yaml");

const dryRun = process.argv.includes("--dry-run");

// ── Shared Helpers ──────────────────────────────────────────────────────────

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

function rewriteFrontmatter(filePath, mutator) {
  const content = readFileSync(filePath, "utf-8");
  const normalized = content.replace(/\r\n/g, "\n").replace(/\r/g, "\n");
  const lines = normalized.split("\n");

  let fmEnd = -1, delimCount = 0;
  for (let i = 0; i < lines.length; i++) {
    if (lines[i].trim() === "---") { delimCount++; if (delimCount === 2) { fmEnd = i; break; } }
  }
  if (fmEnd === -1) return false;

  const fmBlock = lines.slice(1, fmEnd).join("\n");
  const body = lines.slice(fmEnd + 1).join("\n");
  let fm;
  try { fm = yaml.parse(fmBlock); } catch { return false; }

  const changed = mutator(fm);
  if (!changed) return false;

  // Load schema for propertyOrder
  // Try same-directory schema first, then parent
  let propertyOrder = Object.keys(fm);
  for (const schemaDir of [resolve(filePath, ".."), resolve(filePath, "..", "..")]) {
    const schemaPath = join(schemaDir, "schema.json");
    if (existsSync(schemaPath)) {
      try {
        const schema = JSON.parse(readFileSync(schemaPath, "utf-8"));
        if (schema.propertyOrder) { propertyOrder = schema.propertyOrder; break; }
      } catch {}
    }
  }

  const orderedFm = {};
  for (const key of propertyOrder) { if (key in fm) orderedFm[key] = fm[key]; }
  for (const key of Object.keys(fm)) { if (!(key in orderedFm)) orderedFm[key] = fm[key]; }

  const newFmBlock = yaml.stringify(orderedFm, {
    lineWidth: 0, defaultKeyType: "PLAIN", defaultStringType: "QUOTE_DOUBLE",
  }).trim();

  if (!dryRun) {
    writeFileSync(filePath, `---\n${newFmBlock}\n---\n${body}`, "utf-8");
  }
  return true;
}

// ── 1. Lessons: promoted-to → grounded-by relationship ──────────────────────

console.log("=== Lessons: migrating promoted-to ===");
const lessonsDir = resolve(ROOT, ".orqa/process/lessons");
let lessonCount = 0;

for (const file of readdirSync(lessonsDir).sort()) {
  if (!file.startsWith("IMPL-") || !file.endsWith(".md")) continue;
  const filePath = join(lessonsDir, file);
  const fm = parseFrontmatter(readFileSync(filePath, "utf-8"));
  if (!fm) continue;

  const promotedTo = fm["promoted-to"];
  if (promotedTo === undefined) continue; // field doesn't exist

  if (dryRun) {
    console.log(`${file}: promoted-to=${promotedTo} → ${promotedTo ? `add grounded-by:${promotedTo}` : "remove null field"}`);
  } else {
    const success = rewriteFrontmatter(filePath, (fm) => {
      // Add grounded-by relationship if promoted-to has a value
      if (promotedTo && typeof promotedTo === "string") {
        if (!fm.relationships) fm.relationships = [];
        const exists = fm.relationships.some(r => r.type === "grounded-by" && r.target === promotedTo);
        if (!exists) {
          fm.relationships.push({
            target: promotedTo,
            type: "grounded-by",
            rationale: `Lesson promoted to ${promotedTo}`,
          });
        }
      }
      delete fm["promoted-to"];
      return true;
    });
    if (success) {
      console.log(`${file}: migrated promoted-to=${promotedTo}`);
      lessonCount++;
    }
  }
}
console.log(`${lessonCount} lesson(s) migrated.\n`);

// ── 2. Rules: promoted-from → observes relationship ─────────────────────────

console.log("=== Rules: migrating promoted-from ===");
const rulesDir = resolve(ROOT, ".orqa/process/rules");
let ruleCount = 0;

for (const file of readdirSync(rulesDir).sort()) {
  if (!file.startsWith("RULE-") || !file.endsWith(".md")) continue;
  const filePath = join(rulesDir, file);
  const fm = parseFrontmatter(readFileSync(filePath, "utf-8"));
  if (!fm) continue;

  const promotedFrom = fm["promoted-from"];
  if (promotedFrom === undefined) continue;

  const sources = Array.isArray(promotedFrom) ? promotedFrom.filter(Boolean) : (promotedFrom ? [promotedFrom] : []);

  if (dryRun) {
    if (sources.length > 0) {
      console.log(`${file}: promoted-from=[${sources.join(",")}] → add observes relationships`);
    } else {
      console.log(`${file}: promoted-from is null/empty → remove field`);
    }
  } else {
    const success = rewriteFrontmatter(filePath, (fm) => {
      if (sources.length > 0) {
        if (!fm.relationships) fm.relationships = [];
        for (const src of sources) {
          const exists = fm.relationships.some(r => r.type === "observes" && r.target === src);
          if (!exists) {
            fm.relationships.push({
              target: src,
              type: "observes",
              rationale: `Rule promoted from lesson ${src}`,
            });
          }
        }
      }
      delete fm["promoted-from"];
      return true;
    });
    if (success) {
      console.log(`${file}: migrated promoted-from=[${sources.join(",")}]`);
      ruleCount++;
    }
  }
}
console.log(`${ruleCount} rule(s) migrated.\n`);

// ── 3. Decisions: research-refs → informed-by relationship ──────────────────

console.log("=== Decisions: migrating research-refs ===");
const decisionsDir = resolve(ROOT, ".orqa/process/decisions");
let decisionCount = 0;

for (const file of readdirSync(decisionsDir).sort()) {
  if (!file.startsWith("AD-") || !file.endsWith(".md")) continue;
  const filePath = join(decisionsDir, file);
  const fm = parseFrontmatter(readFileSync(filePath, "utf-8"));
  if (!fm) continue;

  const researchRefs = fm["research-refs"];
  if (researchRefs === undefined) continue;

  const refs = Array.isArray(researchRefs) ? researchRefs.filter(Boolean) : [];

  if (dryRun) {
    if (refs.length > 0) {
      console.log(`${file}: research-refs=[${refs.join(",")}] → add informed-by relationships`);
    } else {
      console.log(`${file}: research-refs is empty → remove field`);
    }
  } else {
    const success = rewriteFrontmatter(filePath, (fm) => {
      if (refs.length > 0) {
        if (!fm.relationships) fm.relationships = [];
        for (const ref of refs) {
          const exists = fm.relationships.some(r => r.type === "informed-by" && r.target === ref);
          if (!exists) {
            fm.relationships.push({
              target: ref,
              type: "informed-by",
              rationale: `Research ${ref} informed this decision`,
            });
          }
        }
      }
      delete fm["research-refs"];
      return true;
    });
    if (success) {
      console.log(`${file}: migrated research-refs=[${refs.join(",")}]`);
      decisionCount++;
    }
  }
}
console.log(`${decisionCount} decision(s) migrated.`);
console.log(`\nTotal: ${lessonCount + ruleCount + decisionCount} artifact(s) migrated.`);
