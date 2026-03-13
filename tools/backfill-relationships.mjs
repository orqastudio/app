#!/usr/bin/env node
// AI-assisted relationship backfill tool for governance artifacts.
//
// Reads artifacts from a specified directory, identifies which relationship
// types are required by the schema, analyses content to propose connections,
// and outputs proposals for human review.
//
// Usage:
//   node tools/backfill-relationships.mjs <artifact-dir> [--dry-run] [--filter=ID]
//
// Examples:
//   node tools/backfill-relationships.mjs .orqa/process/rules
//   node tools/backfill-relationships.mjs .orqa/process/lessons --filter=IMPL-005
//   node tools/backfill-relationships.mjs .orqa/process/skills --dry-run
//
// Output: JSON proposals to stdout, one per artifact.
// The orchestrator or agent reads proposals, presents to user, and applies approved ones.

import { readFileSync, readdirSync, existsSync, writeFileSync } from "fs";
import { resolve, join, basename } from "path";
import { createRequire } from "module";

const ROOT = resolve(import.meta.dirname, "..");
const require = createRequire(resolve(ROOT, "ui", "package.json"));
const yaml = require("yaml");

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

function extractBody(content) {
  const normalized = content.replace(/\r\n/g, "\n").replace(/\r/g, "\n");
  const lines = normalized.split("\n");
  let delimCount = 0;
  for (let i = 0; i < lines.length; i++) {
    if (lines[i].trim() === "---") {
      delimCount++;
      if (delimCount === 2) {
        return lines.slice(i + 1).join("\n").trim();
      }
    }
  }
  return "";
}

// ── Safe Frontmatter Update ─────────────────────────────────────────────────

// Updates frontmatter fields without corrupting the markdown body.
// Parses the existing frontmatter, merges new fields, serializes back.
function updateFrontmatter(filePath, updates) {
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

  // Merge updates
  Object.assign(fm, updates);

  // Load schema to get propertyOrder for serialization
  const schema = loadSchema(filePath);
  const propertyOrder = schema?.propertyOrder || Object.keys(fm);

  // Serialize frontmatter in schema order
  const orderedFm = {};
  for (const key of propertyOrder) {
    if (key in fm) orderedFm[key] = fm[key];
  }
  // Add any keys not in propertyOrder
  for (const key of Object.keys(fm)) {
    if (!(key in orderedFm)) orderedFm[key] = fm[key];
  }

  const newFmBlock = yaml.stringify(orderedFm, {
    lineWidth: 0,
    defaultKeyType: "PLAIN",
    defaultStringType: "QUOTE_DOUBLE",
  }).trim();

  const result = `---\n${newFmBlock}\n---\n${body}`;
  writeFileSync(filePath, result, "utf-8");
  return true;
}

// ── Schema Loading ──────────────────────────────────────────────────────────

function loadSchema(fileOrDir) {
  // If given a file path, get its directory's schema
  const dir = fileOrDir.endsWith(".md")
    ? resolve(fileOrDir, "..")
    : resolve(fileOrDir);

  const schemaPath = join(dir, "schema.json");
  if (!existsSync(schemaPath)) return null;
  try {
    return JSON.parse(readFileSync(schemaPath, "utf-8"));
  } catch {
    return null;
  }
}

// ── Artifact Index ──────────────────────────────────────────────────────────

// Build an index of all governance artifacts for cross-referencing
function buildArtifactIndex() {
  const index = new Map();
  const dirs = [
    { path: ".orqa/process/rules", prefix: "RULE" },
    { path: ".orqa/process/decisions", prefix: "AD" },
    { path: ".orqa/process/lessons", prefix: "IMPL" },
    { path: ".orqa/process/skills", prefix: "SKILL", isSkill: true },
    { path: ".orqa/process/pillars", prefix: "PILLAR" },
  ];

  for (const { path, prefix, isSkill } of dirs) {
    const fullPath = resolve(ROOT, path);
    if (!existsSync(fullPath)) continue;

    if (isSkill) {
      // Skills are in subdirectories
      for (const subdir of readdirSync(fullPath)) {
        const skillFile = join(fullPath, subdir, "SKILL.md");
        if (!existsSync(skillFile)) continue;
        const content = readFileSync(skillFile, "utf-8");
        const fm = parseFrontmatter(content);
        if (!fm) continue;
        const body = extractBody(content);
        index.set(fm.id || subdir, {
          id: fm.id || subdir,
          title: fm.title || subdir,
          type: "skill",
          path: skillFile,
          frontmatter: fm,
          body: body.slice(0, 500), // First 500 chars for matching
        });
      }
    } else {
      for (const file of readdirSync(fullPath)) {
        if (!file.startsWith(prefix + "-") || !file.endsWith(".md")) continue;
        const filePath = join(fullPath, file);
        const content = readFileSync(filePath, "utf-8");
        const fm = parseFrontmatter(content);
        if (!fm) continue;
        const body = extractBody(content);
        index.set(fm.id || file.replace(".md", ""), {
          id: fm.id || file.replace(".md", ""),
          title: fm.title || file,
          type: prefix.toLowerCase(),
          path: filePath,
          frontmatter: fm,
          body: body.slice(0, 500),
        });
      }
    }
  }

  return index;
}

// ── Cross-Reference Detection ───────────────────────────────────────────────

// Find artifact IDs referenced in a body text
function findReferences(text) {
  const refs = new Set();
  const patterns = [
    /\[([A-Z]+-\d+)\]\([A-Z]+-\d+\)/g, // [RULE-001](RULE-001)
    /(?<!\[)\b(RULE-\d+|AD-\d+|IMPL-\d+|PILLAR-\d+|EPIC-\d+|TASK-\d+)\b(?!\])/g, // bare IDs
  ];
  for (const pattern of patterns) {
    for (const match of text.matchAll(pattern)) {
      refs.add(match[1]);
    }
  }
  return [...refs];
}

// ── Proposal Generation ─────────────────────────────────────────────────────

// Generate relationship proposals for a single artifact
function proposeRelationships(artifact, artifactIndex, schema) {
  const proposals = [];
  const existingRels = artifact.frontmatter.relationships || [];
  const existingTargets = new Set(existingRels.map((r) => r.target));

  // Find references in body text
  const bodyRefs = findReferences(artifact.body);
  const titleRefs = findReferences(artifact.frontmatter.title || "");
  const allRefs = [...new Set([...bodyRefs, ...titleRefs])];

  for (const refId of allRefs) {
    if (existingTargets.has(refId)) continue; // Already has this relationship
    const target = artifactIndex.get(refId);
    if (!target) continue; // Referenced artifact doesn't exist

    // Determine relationship type based on artifact types
    const relType = inferRelationType(artifact.type, target.type);
    if (!relType) continue;

    proposals.push({
      target: refId,
      type: relType,
      rationale: `Referenced in body/title — ${target.title}`,
      confidence: "medium",
    });
  }

  // Check for "Related Rules" sections which indicate grounded relationships
  const relatedSection = artifact.body.match(/## Related Rules\n([\s\S]*?)(?=\n## |\n*$)/);
  if (relatedSection) {
    const sectionRefs = findReferences(relatedSection[1]);
    for (const refId of sectionRefs) {
      if (existingTargets.has(refId)) continue;
      if (proposals.some((p) => p.target === refId)) continue;
      const target = artifactIndex.get(refId);
      if (!target) continue;

      proposals.push({
        target: refId,
        type: "informs",
        rationale: `Listed in Related Rules section — ${target.title}`,
        confidence: "high",
      });
    }
  }

  return proposals;
}

// Infer the most likely relationship type between two artifact types
function inferRelationType(sourceType, targetType) {
  const map = {
    "rule-ad": "grounded",
    "rule-pillar": "grounded",
    "rule-rule": "informs",
    "skill-ad": "grounded",
    "skill-pillar": "grounded",
    "skill-rule": "practices",
    "skill-skill": "informs",
    "ad-rule": "enforces",
    "ad-skill": "practices",
    "ad-ad": "informs",
    "ad-pillar": "grounded",
    "impl-ad": "observes",
    "impl-rule": "observes",
    "impl-pillar": "observes",
  };

  const key = `${sourceType}-${targetType}`;
  return map[key] || "informs";
}

// ── Main ────────────────────────────────────────────────────────────────────

const args = process.argv.slice(2);
const artifactDir = args.find((a) => !a.startsWith("--"));
const dryRun = args.includes("--dry-run");
const filterArg = args.find((a) => a.startsWith("--filter="));
const filterId = filterArg ? filterArg.split("=")[1] : null;

if (!artifactDir) {
  console.error("Usage: node tools/backfill-relationships.mjs <artifact-dir> [--dry-run] [--filter=ID]");
  process.exit(1);
}

const schema = loadSchema(resolve(ROOT, artifactDir));
if (!schema) {
  console.error(`No schema.json found in ${artifactDir}`);
  process.exit(1);
}

// Check if relationships field exists in schema
if (!schema.properties?.relationships) {
  console.error(`Schema in ${artifactDir} has no relationships field`);
  process.exit(1);
}

// Build cross-reference index
const artifactIndex = buildArtifactIndex();

// Read artifacts from target directory
const targetDir = resolve(ROOT, artifactDir);
const isSkillDir = artifactDir.includes("skills");
const results = [];

if (isSkillDir) {
  for (const subdir of readdirSync(targetDir)) {
    const skillFile = join(targetDir, subdir, "SKILL.md");
    if (!existsSync(skillFile)) continue;
    const content = readFileSync(skillFile, "utf-8");
    const fm = parseFrontmatter(content);
    if (!fm) continue;
    if (filterId && fm.id !== filterId) continue;

    const body = extractBody(content);
    const artifact = { id: fm.id || subdir, type: "skill", frontmatter: fm, body, path: skillFile };
    const proposals = proposeRelationships(artifact, artifactIndex, schema);

    if (proposals.length > 0 || !fm.relationships) {
      results.push({
        id: artifact.id,
        title: fm.title,
        path: skillFile,
        existing: fm.relationships || [],
        proposals,
        needsCategory: !fm.category && schema.properties?.category,
      });
    }
  }
} else {
  const prefix = basename(artifactDir).toUpperCase().replace(/S$/, "");
  const prefixMap = { RULE: "RULE", DECISION: "AD", LESSON: "IMPL" };
  const filePrefix = prefixMap[prefix] || prefix;

  for (const file of readdirSync(targetDir)) {
    if (!file.endsWith(".md") || file === "README.md" || file === "schema.json") continue;
    const filePath = join(targetDir, file);
    const content = readFileSync(filePath, "utf-8");
    const fm = parseFrontmatter(content);
    if (!fm) continue;
    if (filterId && fm.id !== filterId) continue;

    const body = extractBody(content);
    const typeKey = file.startsWith("RULE-") ? "rule"
      : file.startsWith("AD-") ? "ad"
      : file.startsWith("IMPL-") ? "impl"
      : "unknown";
    const artifact = { id: fm.id || file.replace(".md", ""), type: typeKey, frontmatter: fm, body, path: filePath };
    const proposals = proposeRelationships(artifact, artifactIndex, schema);

    if (proposals.length > 0 || !fm.relationships) {
      results.push({
        id: artifact.id,
        title: fm.title,
        path: filePath,
        existing: fm.relationships || [],
        proposals,
        needsMaturity: !fm.maturity && schema.properties?.maturity,
      });
    }
  }
}

// Output results
if (dryRun) {
  console.log(JSON.stringify(results, null, 2));
  console.error(`\n${results.length} artifact(s) need relationship updates.`);
} else {
  // In non-dry-run mode, output machine-readable proposals
  console.log(JSON.stringify(results));
}
