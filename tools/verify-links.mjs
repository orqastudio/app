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
//   node tools/verify-links.mjs [--fix-bare-ids] [--check-bidirectional] [--check-paths] [--json] [--staged]
//
// Flags:
//   --staged        Only check files staged in git (for pre-commit hook)
//   --check-paths   Scan source code for stale .orqa/ paths (uses tools/path-manifest.json)
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
    { path: ".orqa/process/rules", prefix: "RULE" },
    { path: ".orqa/process/decisions", prefix: "AD" },
    { path: ".orqa/process/lessons", prefix: "IMPL" },
    { path: ".orqa/process/pillars", prefix: "PILLAR" },
    { path: ".orqa/delivery/milestones", prefix: "MS" },
    { path: ".orqa/delivery/epics", prefix: "EPIC" },
    { path: ".orqa/delivery/tasks", prefix: "TASK" },
    { path: ".orqa/delivery/ideas", prefix: "IDEA" },
    { path: ".orqa/delivery/research", prefix: "RES" },
    { path: ".orqa/delivery/verification", prefix: "VER" },
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
  const skillsPath = resolve(ROOT, ".orqa/process/skills");
  if (existsSync(skillsPath)) {
    for (const subdir of readdirSync(skillsPath)) {
      const skillFile = join(skillsPath, subdir, "SKILL.md");
      if (!existsSync(skillFile)) continue;
      const content = readFileSync(skillFile, "utf-8");
      const fm = parseFrontmatter(content);
      if (fm?.id) {
        artifacts.set(fm.id, { path: join(".orqa/process/skills", subdir, "SKILL.md"), type: "skill" });
      }
    }
  }

  return artifacts;
}

// ── Reference Scanning ──────────────────────────────────────────────────────

const ARTIFACT_ID_PATTERN = /\b(RULE-\d+|AD-\d+|IMPL-\d+|PILLAR-\d+|MS-\d+|EPIC-\d+|TASK-\d+|IDEA-\d+|RES-\d+|VER-\d+)\b/g;
const LINKED_REF_PATTERN = /\[([^\]]*)\]\(([A-Z]+-\d+)\)/g;
const BARE_ID_PATTERN = /(?<!\[)(?<!\()(?<!\w)\b(RULE-\d+|AD-\d+|IMPL-\d+|PILLAR-\d+|MS-\d+|EPIC-\d+|TASK-\d+|IDEA-\d+|RES-\d+|VER-\d+)\b(?!\))(?!\])/g;

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
  // Build a set of line indices that are inside fenced code blocks
  const bodyLines = body.split("\n");
  const codeBlockLines = new Set();
  let inFencedBlock = false;
  for (let bi = 0; bi < bodyLines.length; bi++) {
    if (bodyLines[bi].trimStart().startsWith("```")) {
      inFencedBlock = !inFencedBlock;
      codeBlockLines.add(bi);
      continue;
    }
    if (inFencedBlock) codeBlockLines.add(bi);
  }

  for (const match of body.matchAll(BARE_ID_PATTERN)) {
    const id = match[1];
    // Skip if it appears inside a fenced code block
    const lineIdx = body.slice(0, match.index).split("\n").length - 1;
    if (codeBlockLines.has(lineIdx)) continue;

    const line = lines[bodyStart + lineIdx] || "";
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

// ── Source Code Path Verification ────────────────────────────────────────────

/**
 * Scan source code files for hardcoded .orqa/ paths and verify them against
 * the path manifest. Catches stale references after directory reorganizations.
 */
function checkSourcePaths() {
  const manifestPath = resolve(ROOT, "tools/path-manifest.json");
  if (!existsSync(manifestPath)) return [];

  const manifest = JSON.parse(readFileSync(manifestPath, "utf-8"));
  const validPaths = new Set(manifest.paths.map((p) => p.path));
  const retiredPrefixes = manifest.retired.map((r) => r.path);

  const issues = [];

  // Source file extensions to scan
  const sourceExts = [".rs", ".ts", ".mjs", ".js", ".svelte"];

  // Directories to scan (skip node_modules, target, dist, .git)
  const scanDirs = ["backend/src-tauri/src", "ui/src", "tools", ".githooks"];

  // Test fixture paths — paths used in unit tests as pattern-matching inputs,
  // not as real file references. Listed in path-manifest.json under "test_fixtures".
  const testFixtures = new Set((manifest.test_fixtures || []).map((t) => t.path));

  // Pattern to find .orqa/ path references in source code.
  // Matches directory paths and file paths (with extensions like .md, .json, .sh).
  const pathPattern = /["'`]?(\.orqa\/[a-z_/.-]+[a-z])/g;

  for (const scanDir of scanDirs) {
    const fullDir = resolve(ROOT, scanDir);
    if (!existsSync(fullDir)) continue;
    const files = findSourceFiles(fullDir, sourceExts);

    for (const file of files) {
      const relFile = relative(ROOT, file).replace(/\\/g, "/");
      const content = readFileSync(file, "utf-8");

      for (const match of content.matchAll(pathPattern)) {
        const foundPath = match[1].replace(/\/+$/, ""); // strip trailing slash

        // Strip trailing file extension for directory-level matching
        const dirPath = foundPath.replace(/\/[^/]+\.[a-z]+$/, "");

        // Skip if it's a valid path, a prefix of one, or a known test fixture
        if (validPaths.has(foundPath)) continue;
        if (validPaths.has(dirPath)) continue;
        if ([...validPaths].some((vp) => vp.startsWith(foundPath + "/"))) continue;
        if ([...validPaths].some((vp) => vp.startsWith(dirPath + "/"))) continue;
        if (testFixtures.has(foundPath)) continue;

        // Check if it matches a retired path prefix
        const retiredMatch = retiredPrefixes.find((rp) => foundPath.startsWith(rp));
        if (retiredMatch) {
          const replacement = manifest.retired.find((r) => r.path === retiredMatch);
          issues.push({
            type: "stale-source-path",
            severity: "error",
            file: relFile,
            reference: foundPath,
            message: `Stale path "${foundPath}" in ${relFile} — "${retiredMatch}" was replaced by "${replacement.replaced_by}" (${replacement.reason})`,
          });
        } else {
          // Check if the path actually exists on disk
          const fullPath = resolve(ROOT, foundPath);
          if (!existsSync(fullPath)) {
            // Only flag if it looks like a specific artifact path (not a generic .orqa/ comment)
            const segments = foundPath.split("/").length;
            if (segments >= 3) {
              issues.push({
                type: "broken-source-path",
                severity: "error",
                file: relFile,
                reference: foundPath,
                message: `Path "${foundPath}" referenced in ${relFile} does not exist on disk`,
              });
            }
          }
        }
      }
    }
  }

  return issues;
}

function findSourceFiles(dir, exts) {
  const results = [];
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    if (entry.name.startsWith(".") || entry.name === "node_modules" || entry.name === "target") continue;
    const fullPath = join(dir, entry.name);
    if (entry.isDirectory()) {
      results.push(...findSourceFiles(fullPath, exts));
    } else if (exts.some((ext) => entry.name.endsWith(ext))) {
      results.push(fullPath);
    }
  }
  return results;
}

// ── Main ────────────────────────────────────────────────────────────────────

const args = process.argv.slice(2);
const checkBidir = args.includes("--check-bidirectional");
const checkPaths = args.includes("--check-paths");
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

// Source code path verification
if (checkPaths) {
  const pathIssues = checkSourcePaths();
  allIssues.push(...pathIssues);
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
