#!/usr/bin/env node
// ARCHIVED: One-off migration tool — ran once to backfill relationship fields
// onto existing rule artifacts. Not intended for repeat use.
// Left here as a reference for what was done. parseFrontmatter intentionally
// left as-is (hand-rolled) since this script is not maintained going forward.
//
// Batch apply relationship backfill to all rules.
// Reads proposals from the backfill tool and applies them,
// plus adds grounded relationships based on domain knowledge.
//
// Usage: node tools/apply-rule-backfill.mjs [--dry-run]

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

// ── Safe Update ─────────────────────────────────────────────────────────────

function addRelationships(filePath, relationships) {
  const content = readFileSync(filePath, "utf-8");
  const normalized = content.replace(/\r\n/g, "\n").replace(/\r/g, "\n");
  const lines = normalized.split("\n");

  let fmEnd = -1;
  let delimCount = 0;
  for (let i = 0; i < lines.length; i++) {
    if (lines[i].trim() === "---") {
      delimCount++;
      if (delimCount === 2) {
        fmEnd = i;
        break;
      }
    }
  }
  if (fmEnd === -1) return false;

  const fmBlock = lines.slice(1, fmEnd).join("\n");
  const body = lines.slice(fmEnd + 1).join("\n");

  let fm;
  try {
    fm = yaml.parse(fmBlock);
  } catch {
    return false;
  }

  // Add relationships
  fm.relationships = relationships;

  // Load schema for propertyOrder
  const schemaPath = join(resolve(filePath, ".."), "schema.json");
  let propertyOrder = Object.keys(fm);
  if (existsSync(schemaPath)) {
    try {
      const schema = JSON.parse(readFileSync(schemaPath, "utf-8"));
      if (schema.propertyOrder) propertyOrder = schema.propertyOrder;
    } catch {}
  }

  // Serialize in order
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

// ── Domain-Aware Rule → Pillar/Decision Mapping ─────────────────────────────

// Map each rule to its primary grounding based on what the rule enforces.
// Rules enforce either a pillar (process/governance) or a decision (architecture).
const RULE_GROUNDINGS = {
  "RULE-001": [{ target: "PILLAR-001", type: "grounded", rationale: "Agent delegation provides clarity through structured roles and boundaries" }],
  "RULE-002": [{ target: "PILLAR-001", type: "grounded", rationale: "Architecture decisions document structural choices that create clarity" }],
  "RULE-003": [{ target: "PILLAR-001", type: "grounded", rationale: "Config integrity ensures artifact structure is reliable and predictable" }],
  "RULE-004": [{ target: "PILLAR-001", type: "grounded", rationale: "Artifact lifecycle enforces structured progression from idea to completion" }],
  "RULE-005": [{ target: "PILLAR-001", type: "grounded", rationale: "Semantic search enables structured knowledge discovery" }],
  "RULE-006": [{ target: "PILLAR-001", type: "grounded", rationale: "Coding standards create structural consistency across the codebase" }],
  "RULE-007": [{ target: "PILLAR-001", type: "grounded", rationale: "Make targets provide a single structured interface for development commands" }],
  "RULE-008": [{ target: "PILLAR-001", type: "grounded", rationale: "Documentation-first ensures knowledge is captured before implementation" }],
  "RULE-009": [{ target: "PILLAR-002", type: "grounded", rationale: "Dogfood mode enforces awareness of self-modifying systems" }],
  "RULE-010": [{ target: "PILLAR-001", type: "grounded", rationale: "End-to-end completeness ensures every feature is fully structured across all layers" }],
  "RULE-011": [{ target: "PILLAR-002", type: "grounded", rationale: "Enforcement before code captures patterns as they emerge, enabling learning" }],
  "RULE-012": [{ target: "PILLAR-001", type: "grounded", rationale: "Error ownership ensures problems are resolved, not deferred" }],
  "RULE-013": [{ target: "PILLAR-001", type: "grounded", rationale: "Git workflow creates structured, traceable version control practices" }],
  "RULE-014": [{ target: "PILLAR-001", type: "grounded", rationale: "Document lifecycle distinguishes current state docs from historical records" }],
  "RULE-015": [{ target: "PILLAR-002", type: "grounded", rationale: "Honest reporting enables accurate reflection on work quality" }],
  "RULE-016": [{ target: "PILLAR-001", type: "grounded", rationale: "Artifact ID semantics prevents confusion between identity and priority" }],
  "RULE-017": [{ target: "PILLAR-002", type: "grounded", rationale: "Lessons learned is the core mechanism for learning through reflection" }],
  "RULE-018": [{ target: "PILLAR-001", type: "grounded", rationale: "No aliases ensures structural consistency across type boundaries" }],
  "RULE-019": [{ target: "PILLAR-001", type: "grounded", rationale: "No deferred deliverables ensures scope clarity and completion integrity" }],
  "RULE-020": [{ target: "PILLAR-001", type: "grounded", rationale: "No stubs ensures real implementations create genuine structural value" }],
  "RULE-021": [{ target: "PILLAR-001", type: "grounded", rationale: "Pillar alignment in docs ensures documentation serves the product vision" }],
  "RULE-022": [{ target: "PILLAR-001", type: "grounded", rationale: "Plan compliance creates structured verification before and during implementation" }],
  "RULE-023": [{ target: "PILLAR-001", type: "grounded", rationale: "Required reading ensures agents have governing context loaded before work" }],
  "RULE-024": [{ target: "PILLAR-001", type: "grounded", rationale: "Reusable components enforce structural consistency in the UI layer" }],
  "RULE-025": [{ target: "PILLAR-001", type: "grounded", rationale: "Root directory discipline maintains structural clarity at project level" }],
  "RULE-026": [{ target: "PILLAR-001", type: "grounded", rationale: "Skill enforcement ensures domain knowledge is loaded before implementation" }],
  "RULE-027": [{ target: "PILLAR-001", type: "grounded", rationale: "Structure before work ensures artifacts exist before implementation begins" }],
  "RULE-028": [{ target: "PILLAR-001", type: "grounded", rationale: "Systems thinking provides the core methodology for achieving clarity through structure" }],
  "RULE-029": [{ target: "PILLAR-002", type: "grounded", rationale: "Testing standards create feedback loops that enable learning from failures" }],
  "RULE-030": [{ target: "PILLAR-002", type: "grounded", rationale: "UAT process structures user feedback into systematic improvement" }],
  "RULE-031": [{ target: "PILLAR-001", type: "grounded", rationale: "Vision alignment ensures all features serve the structural pillars" }],
  "RULE-032": [{ target: "PILLAR-001", type: "grounded", rationale: "Schema validation enforces structural consistency in artifact frontmatter" }],
  "RULE-033": [{ target: "PILLAR-001", type: "grounded", rationale: "Tooltip consistency enforces structural UI patterns" }],
  "RULE-034": [{ target: "PILLAR-001", type: "grounded", rationale: "Cross-reference format ensures navigable, structured artifact links" }],
  "RULE-035": [{ target: "PILLAR-001", type: "grounded", rationale: "Skill portability ensures clean separation between core and project content" }],
  "RULE-036": [{ target: "PILLAR-001", type: "grounded", rationale: "Context window discipline keeps orchestration focused and structured" }],
  "RULE-037": [{ target: "PILLAR-001", type: "grounded", rationale: "Tool access restrictions enforce role boundaries for structural clarity" }],
  "RULE-038": [{ target: "PILLAR-001", type: "grounded", rationale: "User-invocable field creates clear structure for skill surfacing" }],
  "RULE-039": [{ target: "PILLAR-001", type: "grounded", rationale: "Session management ensures structured handoffs between sessions" }],
  "RULE-040": [{ target: "PILLAR-001", type: "grounded", rationale: "Provider-agnostic capabilities create a structured abstraction for tool resolution" }],
  "RULE-041": [{ target: "PILLAR-001", type: "grounded", rationale: "Data persistence boundaries create clear structural separation of concerns" }],
  "RULE-042": [{ target: "PILLAR-001", type: "grounded", rationale: "Skill injection automates knowledge structure loading at the right moments" }],
  "RULE-043": [{ target: "PILLAR-001", type: "grounded", rationale: "Tooling ecosystem manages the structural chain from standards to enforcement" }],
  "RULE-044": [{ target: "PILLAR-001", type: "grounded", rationale: "Core graph protection preserves the fundamental structural integrity of the system" }],
};

// ── Related Rules → informs relationships ───────────────────────────────────

function extractRelatedRules(body) {
  const section = body.match(/## Related Rules\n([\s\S]*?)(?=\n## |\n*$)/);
  if (!section) return [];

  const refs = [];
  for (const match of section[1].matchAll(/\[([A-Z]+-\d+)\]\([A-Z]+-\d+\)/g)) {
    if (match[1].startsWith("RULE-")) {
      refs.push(match[1]);
    }
  }
  return refs;
}

// ── Main ────────────────────────────────────────────────────────────────────

const rulesDir = resolve(ROOT, ".orqa/process/rules");
let updated = 0;

for (const file of readdirSync(rulesDir).sort()) {
  if (!file.startsWith("RULE-") || !file.endsWith(".md")) continue;

  const ruleId = file.replace(".md", "");
  const filePath = join(rulesDir, file);
  const content = readFileSync(filePath, "utf-8");
  const fm = parseFrontmatter(content);
  if (!fm) continue;
  if (fm.status && fm.status !== "active") continue;

  // Skip if already has relationships
  if (fm.relationships && fm.relationships.length > 0) {
    console.log(`${ruleId}: already has relationships, skipping`);
    continue;
  }

  const relationships = [];

  // Add grounded relationship from domain mapping
  const groundings = RULE_GROUNDINGS[ruleId];
  if (groundings) {
    relationships.push(...groundings);
  } else {
    // Default: grounded to PILLAR-001
    relationships.push({
      target: "PILLAR-001",
      type: "grounded",
      rationale: "Enforces structural governance (default — needs human review)",
    });
  }

  // Add informs relationships from Related Rules section
  const body = content.split(/\n---\n/).slice(1).join("\n---\n");
  const relatedRules = extractRelatedRules(body);
  for (const ref of relatedRules) {
    if (ref === ruleId) continue; // No self-references
    relationships.push({
      target: ref,
      type: "informs",
      rationale: `Listed in Related Rules section`,
    });
  }

  if (dryRun) {
    console.log(`${ruleId}: would add ${relationships.length} relationship(s)`);
    for (const r of relationships) {
      console.log(`  ${r.type}: ${r.target} — ${r.rationale}`);
    }
  } else {
    const success = addRelationships(filePath, relationships);
    if (success) {
      console.log(`${ruleId}: added ${relationships.length} relationship(s)`);
      updated++;
    } else {
      console.error(`${ruleId}: FAILED to update`);
    }
  }
}

console.log(`\n${updated} rule(s) updated.`);
