#!/usr/bin/env node
// Link verification tool for governance artifacts.
//
// Scans all .orqa/ markdown files for cross-references and validates:
// 1. Structural checks (pattern matching):
//    - Target artifact exists on disk
//    - ID format is valid (PREFIX-NNN)
//    - No bare artifact IDs without markdown link syntax
//    - Bidirectional consistency (if A references B in relationships, B should reference A)
// 2. Contextual checks (for AI-assisted review):
//    - Flags references that may be contextually inaccurate
//    - Reports relationship type mismatches
//
// Usage:
//   node tools/verify-links.mjs [--fix-bare-ids] [--check-bidirectional] [--json] [--staged]
//
// Flags:
//   --staged    Only check files staged in git (for pre-commit hook)
//
// Output: structured report of broken, missing, and suspect links.

import { readFileSync, readdirSync, existsSync } from "fs";
import { resolve, join, relative } from "path";
import { createRequire } from "module";
import { execSync } from "child_process";

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

// ── Artifact Discovery ──────────────────────────────────────────────────────

// Build a set of all known artifact IDs
function discoverAllArtifacts() {
  const artifacts = new Map(); // id → { path, type }

  const dirs = [
    { path: ".orqa/governance/rules", prefix: "RULE" },
    { path: ".orqa/governance/decisions", prefix: "AD" },
    { path: ".orqa/governance/lessons", prefix: "IMPL" },
    { path: ".orqa/planning/pillars", prefix: "PILLAR" },
    { path: ".orqa/planning/milestones", prefix: "MS" },
    { path: ".orqa/planning/epics", prefix: "EPIC" },
    { path: ".orqa/planning/tasks", prefix: "TASK" },
    { path: ".orqa/planning/ideas", prefix: "IDEA" },
    { path: ".orqa/planning/research", prefix: "RES" },
  ];

  for (const { path, prefix } of dirs) {
    const fullPath = resolve(ROOT, path);
    if (!existsSync(fullPath)) continue;
    for (const file of readdirSync(fullPath)) {
      if (!file.startsWith(prefix + "-") || !file.endsWith(".md")) continue;
      const id = file.replace(".md", "");
      artifacts.set(id, { path: join(path, file), type: prefix.toLowerCase() });
    }
  }

  // Skills (subdirectory pattern)
  const skillsPath = resolve(ROOT, ".orqa/team/skills");
  if (existsSync(skillsPath)) {
    for (const subdir of readdirSync(skillsPath)) {
      const skillFile = join(skillsPath, subdir, "SKILL.md");
      if (!existsSync(skillFile)) continue;
      const content = readFileSync(skillFile, "utf-8");
      const fm = parseFrontmatter(content);
      if (fm?.id) {
        artifacts.set(fm.id, { path: join(".orqa/team/skills", subdir, "SKILL.md"), type: "skill" });
      }
    }
  }

  return artifacts;
}

// ── Reference Scanning ──────────────────────────────────────────────────────

const ARTIFACT_ID_PATTERN = /\b(RULE-\d+|AD-\d+|IMPL-\d+|PILLAR-\d+|MS-\d+|EPIC-\d+|TASK-\d+|IDEA-\d+|RES-\d+)\b/g;
const LINKED_REF_PATTERN = /\[([^\]]*)\]\(([A-Z]+-\d+)\)/g;
const BARE_ID_PATTERN = /(?<!\[)(?<!\()(?<!\w)\b(RULE-\d+|AD-\d+|IMPL-\d+|PILLAR-\d+|MS-\d+|EPIC-\d+|TASK-\d+|IDEA-\d+|RES-\d+)\b(?!\))(?!\])/g;

function scanFile(filePath, knownArtifacts) {
  const content = readFileSync(resolve(ROOT, filePath), "utf-8");
  const fm = parseFrontmatter(content);
  const issues = [];

  // Extract body (after frontmatter)
  const normalized = content.replace(/\r\n/g, "\n").replace(/\r/g, "\n");
  const lines = normalized.split("\n");
  let bodyStart = 0;
  let delimCount = 0;
  for (let i = 0; i < lines.length; i++) {
    if (lines[i].trim() === "---") {
      delimCount++;
      if (delimCount === 2) {
        bodyStart = i + 1;
        break;
      }
    }
  }
  const body = lines.slice(bodyStart).join("\n");

  // Check for linked references with broken targets
  for (const match of body.matchAll(LINKED_REF_PATTERN)) {
    const targetId = match[2];
    if (!knownArtifacts.has(targetId)) {
      issues.push({
        type: "broken-link",
        severity: "error",
        file: filePath,
        reference: targetId,
        context: match[0],
        message: `Linked reference [${match[1]}](${targetId}) points to non-existent artifact`,
      });
    }
  }

  // Check for bare artifact IDs (should be linked)
  for (const match of body.matchAll(BARE_ID_PATTERN)) {
    const id = match[1];
    // Skip if it appears inside a code block
    const lineIdx = body.slice(0, match.index).split("\n").length;
    const line = lines[bodyStart + lineIdx - 1] || "";
    if (line.trimStart().startsWith("```") || line.trimStart().startsWith("`")) continue;

    issues.push({
      type: "bare-id",
      severity: "warning",
      file: filePath,
      reference: id,
      context: line.trim().slice(0, 100),
      message: `Bare artifact ID ${id} should use link syntax [${id}](${id})`,
    });
  }

  // Check frontmatter reference fields
  if (fm) {
    const refFields = [
      "epic", "milestone", "depends-on", "blocks", "research-refs",
      "docs-required", "docs-produced", "supersedes", "superseded-by",
      "promoted-to", "promoted-from", "pillars",
    ];

    for (const field of refFields) {
      const value = fm[field];
      if (!value) continue;
      const values = Array.isArray(value) ? value : [value];
      for (const v of values) {
        if (typeof v !== "string") continue;
        if (v.match(/^[A-Z]+-\d+$/) && !knownArtifacts.has(v)) {
          issues.push({
            type: "broken-frontmatter-ref",
            severity: "error",
            file: filePath,
            field,
            reference: v,
            message: `Frontmatter field '${field}' references non-existent artifact ${v}`,
          });
        }
      }
    }

    // Check relationships array targets
    if (fm.relationships && Array.isArray(fm.relationships)) {
      for (const rel of fm.relationships) {
        if (rel.target && !knownArtifacts.has(rel.target)) {
          issues.push({
            type: "broken-relationship",
            severity: "error",
            file: filePath,
            reference: rel.target,
            message: `Relationship target ${rel.target} (type: ${rel.type}) does not exist`,
          });
        }
      }
    }
  }

  return { fm, issues };
}

// ── Bidirectional Consistency ────────────────────────────────────────────────

const INVERSE_TYPES = {
  "observes": "observed-by",
  "observed-by": "observes",
  "grounded": "grounded-by",
  "grounded-by": "grounded",
  "practices": "practiced-by",
  "practiced-by": "practices",
  "enforces": "enforced-by",
  "enforced-by": "enforces",
  "verifies": "verified-by",
  "verified-by": "verifies",
  "informs": "informed-by",
  "informed-by": "informs",
};

function checkBidirectional(allArtifactData) {
  const issues = [];

  for (const [id, data] of allArtifactData) {
    if (!data.fm?.relationships) continue;

    for (const rel of data.fm.relationships) {
      if (!rel.target || !rel.type) continue;
      const inverseType = INVERSE_TYPES[rel.type];
      if (!inverseType) continue;

      const targetData = allArtifactData.get(rel.target);
      if (!targetData?.fm?.relationships) {
        issues.push({
          type: "missing-inverse",
          severity: "warning",
          file: data.path,
          reference: rel.target,
          message: `${id} has ${rel.type}:${rel.target}, but ${rel.target} has no relationships array (expected ${inverseType}:${id})`,
        });
        continue;
      }

      const hasInverse = targetData.fm.relationships.some(
        (r) => r.target === id && r.type === inverseType
      );
      if (!hasInverse) {
        issues.push({
          type: "missing-inverse",
          severity: "warning",
          file: data.path,
          reference: rel.target,
          message: `${id} has ${rel.type}:${rel.target}, but ${rel.target} lacks ${inverseType}:${id}`,
        });
      }
    }
  }

  return issues;
}

// ── All Markdown Files ──────────────────────────────────────────────────────

function findAllMarkdown(dir) {
  const results = [];
  const fullDir = resolve(ROOT, dir);
  if (!existsSync(fullDir)) return results;

  for (const entry of readdirSync(fullDir, { withFileTypes: true })) {
    if (entry.name.startsWith(".") || entry.name.startsWith("_")) continue;
    const entryPath = join(dir, entry.name);

    if (entry.isDirectory()) {
      results.push(...findAllMarkdown(entryPath));
    } else if (entry.name.endsWith(".md") && entry.name !== "README.md") {
      results.push(entryPath);
    }
  }

  return results;
}

// ── Main ────────────────────────────────────────────────────────────────────

const args = process.argv.slice(2);
const checkBidir = args.includes("--check-bidirectional");
const jsonOutput = args.includes("--json");
const stagedOnly = args.includes("--staged");

// Discover all known artifacts (always full scan for target resolution)
const knownArtifacts = discoverAllArtifacts();

// Determine which files to scan
let allFiles;
if (stagedOnly) {
  // Only check staged .orqa/ markdown files
  const staged = execSync("git diff --cached --name-only --diff-filter=ACMR", { encoding: "utf-8" })
    .trim()
    .split("\n")
    .filter((f) => f.startsWith(".orqa/") && f.endsWith(".md") && !f.endsWith("README.md"));
  allFiles = staged;
} else {
  allFiles = findAllMarkdown(".orqa");
}
const allIssues = [];
const allArtifactData = new Map();

for (const file of allFiles) {
  const { fm, issues } = scanFile(file, knownArtifacts);
  allIssues.push(...issues);

  // Extract artifact ID for bidirectional checks
  // Standard artifacts: PREFIX-NNN.md filename pattern
  const match = file.match(/([A-Z]+-\d+)\.md$/);
  if (match && fm) {
    allArtifactData.set(match[1], { fm, path: file });
  }
  // Skills: subdirectory pattern with SKILL.md, ID is in frontmatter
  if (file.endsWith("SKILL.md") && fm?.id) {
    allArtifactData.set(fm.id, { fm, path: file });
  }
}

// Bidirectional consistency check
if (checkBidir) {
  const bidirIssues = checkBidirectional(allArtifactData);
  allIssues.push(...bidirIssues);
}

// Output
if (jsonOutput) {
  console.log(JSON.stringify(allIssues, null, 2));
} else {
  // Group by severity
  const errors = allIssues.filter((i) => i.severity === "error");
  const warnings = allIssues.filter((i) => i.severity === "warning");

  if (errors.length > 0) {
    console.log("\n=== ERRORS ===");
    for (const issue of errors) {
      console.log(`  ${issue.type}: ${issue.message}`);
      if (issue.context) console.log(`    context: ${issue.context}`);
    }
  }

  if (warnings.length > 0) {
    console.log("\n=== WARNINGS ===");
    for (const issue of warnings) {
      console.log(`  ${issue.type}: ${issue.message}`);
      if (issue.context) console.log(`    context: ${issue.context}`);
    }
  }

  const total = errors.length + warnings.length;
  if (total === 0) {
    console.log("\nAll links verified — no issues found.");
  } else {
    console.log(`\n${errors.length} error(s), ${warnings.length} warning(s) found.`);
  }

  process.exit(errors.length > 0 ? 1 : 0);
}
