#!/usr/bin/env node
// Batch apply relationship backfill to all decisions.
// Adds grounded relationships based on domain knowledge,
// enforced-by relationships for rules referenced in the body,
// and informs relationships for decisions referenced in "Related Decisions" sections.
//
// Usage: node tools/apply-decision-backfill.mjs [--dry-run]

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

// ── Domain-Aware Decision → Pillar Mapping ───────────────────────────────────

// Map each decision to its primary grounding based on what principle it establishes.
const DECISION_GROUNDINGS = {
  "AD-001": [
    { target: "PILLAR-001", type: "grounded", rationale: "Thick backend provides structural clarity via typed domain ownership" },
  ],
  "AD-002": [
    { target: "PILLAR-001", type: "grounded", rationale: "IPC boundary creates clear structural separation between frontend and backend" },
  ],
  "AD-003": [
    { target: "PILLAR-001", type: "grounded", rationale: "Result-based error propagation creates predictable, structured error handling" },
  ],
  "AD-004": [
    { target: "PILLAR-001", type: "grounded", rationale: "Svelte 5 runes enforce consistent reactive patterns" },
  ],
  "AD-006": [
    { target: "PILLAR-001", type: "grounded", rationale: "Component purity creates clear structural roles for pages vs components" },
  ],
  "AD-007": [
    { target: "PILLAR-001", type: "grounded", rationale: "Sidecar architecture provides structured AI integration boundary" },
  ],
  "AD-008": [
    { target: "PILLAR-001", type: "grounded", rationale: "Max subscription authentication provides structured access control" },
  ],
  "AD-009": [
    { target: "PILLAR-001", type: "grounded", rationale: "Streaming pipeline creates structured data flow from AI to UI" },
  ],
  "AD-010": [
    { target: "PILLAR-001", type: "grounded", rationale: "MCP-based tools provide structured extensibility" },
  ],
  "AD-011": [
    { target: "PILLAR-001", type: "grounded", rationale: "Security model creates structured permission boundaries" },
  ],
  "AD-012": [
    { target: "PILLAR-001", type: "grounded", rationale: "Plugin selections provide structured framework extensibility" },
  ],
  "AD-013": [
    { target: "PILLAR-001", type: "grounded", rationale: "Frontend library selections create consistent structural foundation" },
  ],
  "AD-016": [
    { target: "PILLAR-002", type: "grounded", rationale: "Onboarding strategy structures the learning path for new users" },
  ],
  "AD-017": [
    { target: "PILLAR-001", type: "grounded", rationale: "Composability is the foundational structural principle" },
  ],
  "AD-019": [
    { target: "PILLAR-001", type: "grounded", rationale: "Three-zone layout creates clear structural UI organization" },
  ],
  "AD-020": [
    { target: "PILLAR-001", type: "grounded", rationale: "Filesystem-driven browsing aligns UI structure with disk structure" },
  ],
  "AD-021": [
    { target: "PILLAR-001", type: "grounded", rationale: ".orqa/ as single source of truth creates one canonical location for governance" },
  ],
  "AD-022": [
    { target: "PILLAR-001", type: "grounded", rationale: "Config-driven scanning creates structured, predictable artifact discovery" },
  ],
  "AD-023": [
    { target: "PILLAR-001", type: "grounded", rationale: "Merging plans into research simplifies the artifact schema structure" },
  ],
  "AD-024": [
    { target: "PILLAR-001", type: "grounded", rationale: "Native search engine provides structured knowledge discovery" },
  ],
  "AD-025": [
    { target: "PILLAR-001", type: "grounded", rationale: "Provider-agnostic architecture creates structured AI abstraction layer" },
  ],
  "AD-026": [
    { target: "PILLAR-001", type: "grounded", rationale: "Domain service extraction creates clear structural boundaries in backend" },
  ],
  "AD-027": [
    { target: "PILLAR-001", type: "grounded", rationale: "Domain-agnostic vision clarifies the structural scope of the product" },
    { target: "PILLAR-002", type: "grounded", rationale: "Domain-agnostic vision enables learning across different domains" },
  ],
  "AD-028": [
    { target: "PILLAR-001", type: "grounded", rationale: "Three-tier skill loading creates structured knowledge injection" },
  ],
  "AD-029": [
    { target: "PILLAR-001", type: "grounded", rationale: "Universal roles with domain skills create structured agent delegation" },
  ],
  "AD-030": [
    { target: "PILLAR-001", type: "grounded", rationale: "Skill-driven setup creates structured project scaffolding" },
  ],
  "AD-031": [
    { target: "PILLAR-001", type: "grounded", rationale: "Pillars as artifacts create structured vision enforcement" },
  ],
  "AD-032": [
    { target: "PILLAR-001", type: "grounded", rationale: "Scoped SQLite creates clear structural data persistence boundaries" },
  ],
  "AD-033": [
    { target: "PILLAR-001", type: "grounded", rationale: "Core UI boundary creates focused structural scope for the app" },
  ],
  "AD-034": [
    { target: "PILLAR-001", type: "grounded", rationale: "Schema-driven filtering creates structured artifact navigation" },
  ],
  "AD-035": [
    { target: "PILLAR-001", type: "grounded", rationale: "Config-driven navigation defaults create consistent structural browsing" },
  ],
  "AD-036": [
    { target: "PILLAR-001", type: "grounded", rationale: "Cross-linking creates structured artifact relationships" },
  ],
  "AD-037": [
    { target: "PILLAR-001", type: "grounded", rationale: "AI-driven search enables structured knowledge discovery across artifacts" },
  ],
  "AD-038": [
    { target: "PILLAR-001", type: "grounded", rationale: "Graph-based injection creates structured knowledge delivery" },
  ],
  "AD-039": [
    { target: "PILLAR-001", type: "grounded", rationale: "Core graph firmware principle protects structural integrity" },
  ],
  "AD-040": [
    { target: "PILLAR-001", type: "grounded", rationale: "Task-first audit trail creates structured work traceability" },
  ],
  "AD-041": [
    { target: "PILLAR-001", type: "grounded", rationale: "CLI rule loading creates structured governance injection" },
  ],
};

// ── Body Reference Extraction ─────────────────────────────────────────────────

// Extract RULE-NNN references from the decision body as enforced-by relationships.
// Looks for [RULE-NNN](RULE-NNN) markdown links anywhere in the body.
function extractReferencedRules(body) {
  const refs = new Set();
  for (const match of body.matchAll(/\[RULE-(\d+)\]\(RULE-\d+\)/g)) {
    refs.add(`RULE-${match[1]}`);
  }
  // Also catch bare RULE-NNN patterns (plain text references)
  for (const match of body.matchAll(/\bRULE-(\d+)\b/g)) {
    refs.add(`RULE-${match[1]}`);
  }
  return [...refs];
}

// Extract AD-NNN references from "Related Decisions" section as informs relationships.
function extractRelatedDecisions(body, selfId) {
  const section = body.match(/## Related Decisions\n([\s\S]*?)(?=\n## |\n*$)/);
  if (!section) return [];

  const refs = [];
  for (const match of section[1].matchAll(/\[AD-(\d+)\]\(AD-\d+\)/g)) {
    const id = `AD-${match[1]}`;
    if (id !== selfId) refs.push(id);
  }
  return refs;
}

// ── Main ─────────────────────────────────────────────────────────────────────

const decisionsDir = resolve(ROOT, ".orqa/governance/decisions");
let updated = 0;

for (const file of readdirSync(decisionsDir).sort()) {
  if (!file.startsWith("AD-") || !file.endsWith(".md")) continue;

  const decisionId = file.replace(".md", "");
  const filePath = join(decisionsDir, file);
  const content = readFileSync(filePath, "utf-8");
  const fm = parseFrontmatter(content);
  if (!fm) continue;

  // Skip superseded and deprecated decisions
  if (fm.status === "superseded" || fm.status === "deprecated") {
    console.log(`${decisionId}: status is ${fm.status}, skipping`);
    continue;
  }

  // Skip AD-042 — already has relationships
  if (decisionId === "AD-042") {
    console.log(`${decisionId}: already has relationships, skipping`);
    continue;
  }

  // Skip any decision that already has relationships
  if (fm.relationships && fm.relationships.length > 0) {
    console.log(`${decisionId}: already has relationships, skipping`);
    continue;
  }

  const relationships = [];

  // Add grounded relationship(s) from domain mapping
  const groundings = DECISION_GROUNDINGS[decisionId];
  if (groundings) {
    relationships.push(...groundings);
  } else {
    // Default: grounded to PILLAR-001
    relationships.push({
      target: "PILLAR-001",
      type: "grounded",
      rationale: "Establishes a structural principle (default — needs human review)",
    });
  }

  // Extract body (everything after the closing ---)
  const body = content.replace(/^---\n[\s\S]*?\n---\n/, "");

  // Add enforced-by relationships for rules referenced in the body
  const referencedRules = extractReferencedRules(body);
  for (const ruleId of referencedRules) {
    relationships.push({
      target: ruleId,
      type: "enforced-by",
      rationale: `${ruleId} referenced in decision body`,
    });
  }

  // Add informs relationships for decisions in "Related Decisions" section
  const relatedDecisions = extractRelatedDecisions(body, decisionId);
  for (const adId of relatedDecisions) {
    relationships.push({
      target: adId,
      type: "informs",
      rationale: `Listed in Related Decisions section`,
    });
  }

  if (dryRun) {
    console.log(`${decisionId}: would add ${relationships.length} relationship(s)`);
    for (const r of relationships) {
      console.log(`  ${r.type}: ${r.target} — ${r.rationale}`);
    }
  } else {
    const success = addRelationships(filePath, relationships);
    if (success) {
      console.log(`${decisionId}: added ${relationships.length} relationship(s)`);
      updated++;
    } else {
      console.error(`${decisionId}: FAILED to update`);
    }
  }
}

console.log(`\n${updated} decision(s) updated.`);
