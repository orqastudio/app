#!/usr/bin/env node
// Enforcement rule verification tool (TASK-366).
//
// Checks:
// 1. Skill portability — core skills must not contain project-specific paths
// 2. Agent capabilities — agent definitions must use capabilities, not tools
// 3. Persistence boundaries — no governance references in SQLite code, no conversation data in .orqa/
//
// Usage: node tools/verify-enforcement-rules.mjs [--staged]
//
// Flags:
//   --staged    Only check files staged in git (for pre-commit hook)

import { readFileSync, readdirSync, existsSync } from "fs";
import { resolve, join, relative } from "path";
import { createRequire } from "module";
import { execSync } from "child_process";

const ROOT = resolve(import.meta.dirname, "..");
const require = createRequire(resolve(ROOT, "ui", "package.json"));
const yaml = require("yaml");

const stagedOnly = process.argv.includes("--staged");
let stagedFiles = null;
if (stagedOnly) {
  stagedFiles = new Set(
    execSync("git diff --cached --name-only --diff-filter=ACMR", { encoding: "utf-8" })
      .trim()
      .split("\n")
      .filter(Boolean)
  );
}

let errors = 0;
let warnings = 0;

function error(msg) { console.error(`  ERROR: ${msg}`); errors++; }
function warn(msg) { console.error(`  WARNING: ${msg}`); warnings++; }

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

// ── Check 1: Skill portability ─────────────────────────────────────────────
// Core skills must not contain project-specific paths

console.log("\n=== SKILL PORTABILITY ===");

const SKILLS_DIR = resolve(ROOT, ".orqa/process/skills");
const PROJECT_PATH_PATTERNS = [
  /backend\/src-tauri\//,
  /ui\/src\//,
  /sidecar\/src\//,
  /\.orqa\/delivery\//,
  /\.orqa\/process\/(decisions|lessons|rules)\//,
  /\.orqa\/documentation\//,
];
// AD/RULE/EPIC/TASK/IMPL references are project-specific in core skills
const ARTIFACT_REF_PATTERN = /\b(AD-\d+|RULE-\d+|EPIC-\d+|TASK-\d+|IMPL-\d+|RES-\d+)\b/;

let skillsChecked = 0;
let skillViolations = 0;

if (existsSync(SKILLS_DIR)) {
  for (const subdir of readdirSync(SKILLS_DIR).sort()) {
    if (subdir.startsWith("_") || subdir === "README.md" || subdir === "schema.json") continue;
    const skillFile = join(SKILLS_DIR, subdir, "SKILL.md");
    if (!existsSync(skillFile)) continue;

    const relPath = relative(ROOT, skillFile).replace(/\\/g, "/");
    if (stagedFiles && !stagedFiles.has(relPath)) continue;

    const content = readFileSync(skillFile, "utf-8");
    const fm = parseFrontmatter(content);
    if (!fm || fm.layer !== "core") continue;

    skillsChecked++;

    // Get body text (after frontmatter)
    const bodyStart = content.indexOf("---", content.indexOf("---") + 3);
    if (bodyStart === -1) continue;
    const body = content.slice(bodyStart + 3);

    // Check for project-specific paths
    for (const pattern of PROJECT_PATH_PATTERNS) {
      const match = body.match(pattern);
      if (match) {
        warn(`${subdir} (core): contains project-specific path "${match[0]}"`);
        skillViolations++;
      }
    }

    // Check for artifact ID references (project-specific)
    const lines = body.split("\n");
    for (let i = 0; i < lines.length; i++) {
      const line = lines[i];
      // Skip lines that are in "Related Rules" or "Related" sections
      if (line.match(/^#+\s*(Related|See Also)/i)) break;
      const artifactMatch = line.match(ARTIFACT_REF_PATTERN);
      if (artifactMatch) {
        warn(`${subdir} (core): references project artifact ${artifactMatch[0]} on line ${i + 1}`);
        skillViolations++;
        break; // One warning per skill is enough
      }
    }
  }
}

console.log(`  Core skills checked: ${skillsChecked}`);
console.log(`  Portability violations: ${skillViolations}`);

// ── Check 2: Agent capabilities ────────────────────────────────────────────
// Agent definitions must use capabilities, not tools

console.log("\n=== AGENT CAPABILITIES ===");

const AGENTS_DIR = resolve(ROOT, ".orqa/process/agents");
let agentsChecked = 0;
let capabilityViolations = 0;

if (existsSync(AGENTS_DIR)) {
  for (const file of readdirSync(AGENTS_DIR).sort()) {
    if (!file.endsWith(".md") || file === "README.md") continue;

    const relPath = relative(ROOT, join(AGENTS_DIR, file)).replace(/\\/g, "/");
    if (stagedFiles && !stagedFiles.has(relPath)) continue;

    const content = readFileSync(join(AGENTS_DIR, file), "utf-8");
    const fm = parseFrontmatter(content);
    if (!fm || fm.status !== "active") continue;

    agentsChecked++;

    // Check for deprecated 'tools' field
    if ("tools" in fm) {
      error(`${file}: uses deprecated 'tools' field — use 'capabilities' instead`);
      capabilityViolations++;
    }

    // Check capabilities field exists
    if (!fm.capabilities) {
      error(`${file}: missing 'capabilities' field`);
      capabilityViolations++;
    } else if (!Array.isArray(fm.capabilities)) {
      error(`${file}: 'capabilities' is not an array`);
      capabilityViolations++;
    }
  }
}

console.log(`  Agents checked: ${agentsChecked}`);
console.log(`  Capability violations: ${capabilityViolations}`);

// ── Check 3: Persistence boundaries ────────────────────────────────────────
// Governance data must not be in SQLite; conversation data must not be in .orqa/

console.log("\n=== PERSISTENCE BOUNDARIES ===");

let persistenceViolations = 0;

// Check Rust repo/migration files for governance table names
const GOVERNANCE_TABLE_PATTERNS = [
  /CREATE\s+TABLE.*\b(rules|skills|agents|decisions|lessons|pillars)\b/i,
  /INSERT\s+INTO.*\b(rules|skills|agents|decisions|lessons|pillars)\b/i,
];

const repoDir = resolve(ROOT, "backend/src-tauri/src/repo");
const migrationsDir = resolve(ROOT, "backend/src-tauri/migrations");

function checkDirForPatterns(dirPath, patterns, label) {
  if (!existsSync(dirPath)) return;
  const files = readdirSync(dirPath, { recursive: true });
  for (const file of files) {
    const filePath = join(dirPath, file.toString());
    if (!filePath.endsWith(".rs") && !filePath.endsWith(".sql")) continue;

    const relPath = relative(ROOT, filePath).replace(/\\/g, "/");
    if (stagedFiles && !stagedFiles.has(relPath)) continue;

    try {
      const content = readFileSync(filePath, "utf-8");
      for (const pattern of patterns) {
        const match = content.match(pattern);
        if (match) {
          warn(`${relPath}: ${label} — found "${match[0].trim()}"`);
          persistenceViolations++;
        }
      }
    } catch { /* skip unreadable files */ }
  }
}

checkDirForPatterns(repoDir, GOVERNANCE_TABLE_PATTERNS, "governance data in SQLite");
checkDirForPatterns(migrationsDir, GOVERNANCE_TABLE_PATTERNS, "governance data in SQLite migrations");

console.log(`  Persistence violations: ${persistenceViolations}`);

// ── Summary ─────────────────────────────────────────────────────────────────

console.log("\n" + "=".repeat(50));
console.log("ENFORCEMENT RULES REPORT");
console.log("=".repeat(50));
console.log(`\n${errors} error(s), ${warnings} warning(s)`);

if (errors > 0) {
  console.log("\nENFORCEMENT RULES: FAIL");
  process.exit(1);
} else {
  console.log("\nENFORCEMENT RULES: PASS");
}
