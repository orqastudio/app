#!/usr/bin/env node
// ARCHIVED: One-off migration tool — ran once to backfill category and
// relationship fields onto existing skill artifacts. Not intended for repeat
// use. parseFrontmatter intentionally left as-is (hand-rolled) since this
// script is not maintained going forward.
//
// Batch apply category and relationship backfill to all skills.
//
// Usage: node tools/apply-skill-backfill.mjs [--dry-run]

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

function updateSkillFrontmatter(filePath, updates) {
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

  const schemaPath = join(resolve(filePath, "..", ".."), "schema.json");
  let propertyOrder = Object.keys(fm);
  if (existsSync(schemaPath)) {
    try {
      const schema = JSON.parse(readFileSync(schemaPath, "utf-8"));
      if (schema.propertyOrder) propertyOrder = schema.propertyOrder;
    } catch {}
  }

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

// ── Category Classification ─────────────────────────────────────────────────

// methodology: teaches HOW to think/approach problems
// domain: teaches WHAT to know about a specific area
// tool: teaches HOW TO USE a specific tool or technology
const SKILL_CATEGORIES = {
  "architectural-evaluation": "methodology",
  "architecture": "methodology",
  "backend-best-practices": "domain",
  "code-quality-review": "methodology",
  "component-extraction": "methodology",
  "composability": "methodology",
  "diagnostic-methodology": "methodology",
  "epic-requirement-inference": "tool",
  "frontend-best-practices": "domain",
  "governance-maintenance": "methodology",
  "migration-tooling": "tool",
  "orqa-artifact-audit": "methodology",
  "orqa-code-search": "tool",
  "orqa-documentation": "domain",
  "orqa-domain-services": "domain",
  "orqa-error-composition": "domain",
  "orqa-governance": "domain",
  "orqa-ipc-patterns": "domain",
  "orqa-native-search": "tool",
  "orqa-plugin-development": "domain",
  "orqa-repository-pattern": "domain",
  "orqa-schema-compliance": "methodology",
  "orqa-search-architecture": "domain",
  "orqa-store-orchestration": "domain",
  "orqa-store-patterns": "domain",
  "orqa-streaming": "domain",
  "orqa-testing": "domain",
  "planning": "methodology",
  "plugin-setup": "tool",
  "project-inference": "tool",
  "project-migration": "tool",
  "project-setup": "tool",
  "project-type-software": "tool",
  "qa-verification": "methodology",
  "research-methodology": "methodology",
  "restructuring-methodology": "methodology",
  "rule-enforcement": "domain",
  "rust-async-patterns": "domain",
  "security-audit": "methodology",
  "skills-maintenance": "methodology",
  "svelte5-best-practices": "domain",
  "systems-thinking": "methodology",
  "tailwind-design-system": "domain",
  "tauri-v2": "domain",
  "test-engineering": "methodology",
  "typescript-advanced-types": "domain",
  "uat-process": "methodology",
  "ux-compliance-review": "methodology",
};

// ── Skill → Pillar/Decision Grounding ───────────────────────────────────────

const SKILL_GROUNDINGS = {
  // Methodology skills → grounded to the pillar they serve
  "architectural-evaluation": [{ target: "PILLAR-001", type: "grounded", rationale: "Architectural evaluation creates structural clarity through systematic compliance checking" }],
  "architecture": [{ target: "PILLAR-001", type: "grounded", rationale: "Architecture knowledge enables structured design decisions" }],
  "code-quality-review": [{ target: "PILLAR-002", type: "grounded", rationale: "Code review enables learning from implementation patterns" }],
  "component-extraction": [{ target: "PILLAR-001", type: "grounded", rationale: "Component extraction creates structural reuse" }],
  "composability": [{ target: "PILLAR-001", type: "grounded", rationale: "Composability is the core structural principle — build from small, pure, swappable units" }],
  "diagnostic-methodology": [{ target: "PILLAR-002", type: "grounded", rationale: "Diagnostics systematise learning from failures" }],
  "governance-maintenance": [{ target: "PILLAR-001", type: "grounded", rationale: "Governance maintenance ensures structural consistency of the framework" }],
  "orqa-artifact-audit": [{ target: "PILLAR-001", type: "grounded", rationale: "Artifact auditing verifies structural integrity of governance" }],
  "orqa-schema-compliance": [{ target: "PILLAR-001", type: "grounded", rationale: "Schema compliance enforces structural consistency in artifacts" }],
  "planning": [{ target: "PILLAR-001", type: "grounded", rationale: "Planning methodology creates structured approaches before implementation" }],
  "qa-verification": [{ target: "PILLAR-002", type: "grounded", rationale: "QA verification enables learning through systematic testing" }],
  "research-methodology": [{ target: "PILLAR-002", type: "grounded", rationale: "Research methodology structures the process of learning from investigation" }],
  "restructuring-methodology": [{ target: "PILLAR-001", type: "grounded", rationale: "Restructuring methodology ensures structural changes are safe and incremental" }],
  "security-audit": [{ target: "PILLAR-001", type: "grounded", rationale: "Security auditing provides structured assessment of system safety" }],
  "skills-maintenance": [{ target: "PILLAR-001", type: "grounded", rationale: "Skills lifecycle management maintains structural consistency of knowledge" }],
  "systems-thinking": [{ target: "PILLAR-001", type: "grounded", rationale: "Systems thinking is the foundational methodology for clarity through structure" }],
  "test-engineering": [{ target: "PILLAR-002", type: "grounded", rationale: "Test engineering creates feedback loops for learning" }],
  "uat-process": [{ target: "PILLAR-002", type: "grounded", rationale: "UAT process structures user feedback into systematic improvement" }],
  "ux-compliance-review": [{ target: "PILLAR-001", type: "grounded", rationale: "UX compliance ensures structural consistency in user experience" }],
  "epic-requirement-inference": [{ target: "PILLAR-001", type: "grounded", rationale: "Epic requirement inference structures project workflow decisions" }],

  // Domain skills → grounded to the decisions/pillars they serve
  "backend-best-practices": [{ target: "PILLAR-001", type: "grounded", rationale: "Backend standards create structural consistency in Rust code" }],
  "frontend-best-practices": [{ target: "PILLAR-001", type: "grounded", rationale: "Frontend standards create structural consistency in Svelte code" }],
  "orqa-documentation": [{ target: "PILLAR-001", type: "grounded", rationale: "Documentation conventions create structural consistency in written artifacts" }],
  "orqa-domain-services": [{ target: "PILLAR-001", type: "grounded", rationale: "Domain service patterns create structural clarity in backend architecture" }],
  "orqa-error-composition": [{ target: "PILLAR-001", type: "grounded", rationale: "Error composition creates structured error propagation" }],
  "orqa-governance": [{ target: "PILLAR-001", type: "grounded", rationale: "Governance patterns maintain structural consistency of the framework" }],
  "orqa-ipc-patterns": [{ target: "PILLAR-001", type: "grounded", rationale: "IPC patterns enforce structured communication across layers" }],
  "orqa-plugin-development": [{ target: "PILLAR-001", type: "grounded", rationale: "Plugin development creates structured extensibility" }],
  "orqa-repository-pattern": [{ target: "PILLAR-001", type: "grounded", rationale: "Repository pattern creates structured data access" }],
  "orqa-search-architecture": [{ target: "PILLAR-001", type: "grounded", rationale: "Search architecture enables structured knowledge discovery" }],
  "orqa-store-orchestration": [{ target: "PILLAR-001", type: "grounded", rationale: "Store orchestration creates structured state management" }],
  "orqa-store-patterns": [{ target: "PILLAR-001", type: "grounded", rationale: "Store patterns create structured reactive state" }],
  "orqa-streaming": [{ target: "PILLAR-001", type: "grounded", rationale: "Streaming patterns create structured data flow from AI to UI" }],
  "orqa-testing": [{ target: "PILLAR-002", type: "grounded", rationale: "Testing patterns enable learning through automated verification" }],
  "rule-enforcement": [{ target: "PILLAR-001", type: "grounded", rationale: "Rule enforcement creates structured compliance mechanisms" }],
  "rust-async-patterns": [{ target: "PILLAR-001", type: "grounded", rationale: "Async patterns create structured concurrency" }],
  "svelte5-best-practices": [{ target: "PILLAR-001", type: "grounded", rationale: "Svelte 5 patterns create structured component architecture" }],
  "tailwind-design-system": [{ target: "PILLAR-001", type: "grounded", rationale: "Design system patterns create structural visual consistency" }],
  "tauri-v2": [{ target: "PILLAR-001", type: "grounded", rationale: "Tauri v2 knowledge enables structured desktop app development" }],
  "typescript-advanced-types": [{ target: "PILLAR-001", type: "grounded", rationale: "Advanced types create structural type safety" }],

  // Tool skills → grounded to the pillar they serve
  "migration-tooling": [{ target: "PILLAR-001", type: "grounded", rationale: "Migration tooling enables structured schema evolution" }],
  "orqa-code-search": [{ target: "PILLAR-001", type: "grounded", rationale: "Code search wrapper provides structured context-aware search" }],
  "orqa-native-search": [{ target: "PILLAR-001", type: "grounded", rationale: "Native search enables structured knowledge discovery in the app" }],
  "plugin-setup": [{ target: "PILLAR-001", type: "grounded", rationale: "Plugin setup creates structured CLI integration" }],
  "project-inference": [{ target: "PILLAR-001", type: "grounded", rationale: "Project inference creates structured project understanding" }],
  "project-migration": [{ target: "PILLAR-001", type: "grounded", rationale: "Project migration creates structured governance adoption" }],
  "project-setup": [{ target: "PILLAR-001", type: "grounded", rationale: "Project setup creates structured scaffolding" }],
  "project-type-software": [{ target: "PILLAR-001", type: "grounded", rationale: "Software presets create structured development governance" }],
};

// ── Main ────────────────────────────────────────────────────────────────────

const skillsDir = resolve(ROOT, ".orqa/process/skills");
let updated = 0;

for (const subdir of readdirSync(skillsDir).sort()) {
  if (subdir.startsWith("_") || subdir === "README.md" || subdir === "schema.json") continue;

  const skillFile = join(skillsDir, subdir, "SKILL.md");
  if (!existsSync(skillFile)) continue;

  const content = readFileSync(skillFile, "utf-8");
  const fm = parseFrontmatter(content);
  if (!fm) continue;
  if (fm.status && fm.status !== "active") continue;

  const updates = {};
  let needsUpdate = false;

  // Add category if missing
  if (!fm.category) {
    const category = SKILL_CATEGORIES[subdir] || "domain";
    updates.category = category;
    needsUpdate = true;
  }

  // Add relationships if missing
  if (!fm.relationships || fm.relationships.length === 0) {
    const groundings = SKILL_GROUNDINGS[subdir];
    if (groundings) {
      updates.relationships = groundings;
    } else {
      updates.relationships = [{
        target: "PILLAR-001",
        type: "grounded",
        rationale: "Provides structured knowledge (default — needs human review)",
      }];
    }
    needsUpdate = true;
  }

  if (!needsUpdate) {
    if (!dryRun) console.log(`${subdir}: already up to date, skipping`);
    continue;
  }

  if (dryRun) {
    console.log(`${subdir}:`);
    if (updates.category) console.log(`  category: ${updates.category}`);
    if (updates.relationships) {
      for (const r of updates.relationships) {
        console.log(`  ${r.type}: ${r.target} — ${r.rationale}`);
      }
    }
  } else {
    const success = updateSkillFrontmatter(skillFile, updates);
    if (success) {
      console.log(`${subdir}: updated (category: ${updates.category || fm.category}, relationships: ${(updates.relationships || fm.relationships).length})`);
      updated++;
    } else {
      console.error(`${subdir}: FAILED to update`);
    }
  }
}

console.log(`\n${updated} skill(s) updated.`);
