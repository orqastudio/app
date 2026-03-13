#!/usr/bin/env node
/**
 * Pre-commit hook: Auto-link bare artifact IDs in .orqa/ markdown files.
 *
 * Finds bare artifact IDs (EPIC-005, TASK-164, AD-033, etc.) in markdown body
 * text and converts them to proper artifact links: [EPIC-005](EPIC-005).
 *
 * Skips:
 * - YAML frontmatter (between --- delimiters)
 * - Fenced code blocks (``` or ~~~)
 * - IDs already formatted as [ID](ID) links
 * - IDs that don't correspond to existing artifact files
 *
 * Architecture decision: AD-036
 */

import { readFileSync, writeFileSync, existsSync } from "fs";
import { join, dirname } from "path";
import { execSync } from "child_process";

// Artifact ID patterns and their directory mappings
const ARTIFACT_PATTERNS = [
  { prefix: "EPIC", dir: ".orqa/delivery/epics" },
  { prefix: "TASK", dir: ".orqa/delivery/tasks" },
  { prefix: "AD", dir: ".orqa/process/decisions" },
  { prefix: "RES", dir: ".orqa/delivery/research" },
  { prefix: "RULE", dir: ".orqa/process/rules" },
  { prefix: "PILLAR", dir: ".orqa/process/pillars" },
  { prefix: "MS", dir: ".orqa/delivery/milestones" },
  { prefix: "IDEA", dir: ".orqa/delivery/ideas" },
  { prefix: "IMPL", dir: ".orqa/process/lessons" },
  { prefix: "VER", dir: ".orqa/delivery/verification" },
  { prefix: "DOC", dir: ".orqa/documentation" },
];

// Build regex that matches any bare artifact ID
// Negative lookbehind: not preceded by [ or ( or `
// Negative lookahead: not followed by ] or ) or `
const prefixes = ARTIFACT_PATTERNS.map((p) => p.prefix).join("|");
const BARE_ID_REGEX = new RegExp(
  `(?<!\\[)(?<!\\()(?<!\`)\\b((?:${prefixes})-\\d{3,})\\b(?!\\])(?!\\))(?!\`)`,
  "g",
);

// Cache for artifact existence checks
const existsCache = new Map();

function artifactFileExists(id) {
  if (existsCache.has(id)) return existsCache.get(id);

  const prefix = id.split("-")[0];
  const pattern = ARTIFACT_PATTERNS.find((p) => p.prefix === prefix);
  if (!pattern) {
    existsCache.set(id, false);
    return false;
  }

  const filePath = join(pattern.dir, `${id}.md`);
  const exists = existsSync(filePath);
  existsCache.set(id, exists);
  return exists;
}

function processFile(filePath) {
  const content = readFileSync(filePath, "utf-8");
  const lines = content.split("\n");
  let modified = false;
  const result = [];

  let inFrontmatter = false;
  let frontmatterCount = 0;
  let inCodeBlock = false;

  for (const line of lines) {
    // Track frontmatter boundaries
    if (line.trim() === "---") {
      frontmatterCount++;
      if (frontmatterCount === 1) inFrontmatter = true;
      if (frontmatterCount === 2) inFrontmatter = false;
      result.push(line);
      continue;
    }

    // Skip frontmatter lines
    if (inFrontmatter) {
      result.push(line);
      continue;
    }

    // Track fenced code blocks
    if (line.trim().startsWith("```") || line.trim().startsWith("~~~")) {
      inCodeBlock = !inCodeBlock;
      result.push(line);
      continue;
    }

    // Skip code block lines
    if (inCodeBlock) {
      result.push(line);
      continue;
    }

    // Replace bare artifact IDs with links
    const newLine = line.replace(BARE_ID_REGEX, (match) => {
      if (artifactFileExists(match)) {
        return `[${match}](${match})`;
      }
      return match;
    });

    if (newLine !== line) modified = true;
    result.push(newLine);
  }

  if (modified) {
    writeFileSync(filePath, result.join("\n"), "utf-8");
    return true;
  }
  return false;
}

// Get staged .orqa/ markdown files
const files = process.argv.slice(2);

if (files.length === 0) {
  process.exit(0);
}

let fixedCount = 0;

for (const file of files) {
  if (!existsSync(file)) continue;

  try {
    if (processFile(file)) {
      fixedCount++;
      console.log(`  Auto-linked artifact IDs in: ${file}`);
      // Re-stage the modified file
      execSync(`git add "${file}"`);
    }
  } catch (err) {
    console.error(`  Error processing ${file}: ${err.message}`);
    process.exit(1);
  }
}

if (fixedCount > 0) {
  console.log(`  Auto-linked artifact IDs in ${fixedCount} file(s)`);
}
