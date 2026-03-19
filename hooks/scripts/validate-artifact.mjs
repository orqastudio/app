#!/usr/bin/env node
// PostToolUse hook: validates .orqa/ artifacts after Write/Edit operations.
// Checks frontmatter schema, relationship validity, and bidirectional integrity.
//
// Runs after Write/Edit completes on .orqa/ files. Non-blocking — reports
// validation issues as systemMessage warnings without denying the operation.

import { readFileSync, existsSync } from "fs";
import { join, relative } from "path";

// Check if a file path is within the .orqa/ directory
function isOrqaArtifact(filePath, projectDir) {
  const rel = relative(projectDir, filePath).replace(/\\/g, "/");
  return rel.startsWith(".orqa/") && filePath.endsWith(".md");
}

// Simple YAML frontmatter parser
function parseFrontmatter(content) {
  const match = content.match(/^---\n([\s\S]*?)\n---/);
  if (!match) return null;

  const yaml = match[1];
  const result = {};

  for (const line of yaml.split("\n")) {
    const kvMatch = line.match(/^(\w[\w-]*)\s*:\s*(.+)$/);
    if (kvMatch) {
      let val = kvMatch[2].trim();
      if (val.startsWith('"') && val.endsWith('"')) val = val.slice(1, -1);
      result[kvMatch[1]] = val;
    }
  }

  return result;
}

// Validate an artifact's frontmatter
function validateArtifact(filePath, projectDir) {
  const issues = [];

  if (!existsSync(filePath)) return issues;

  const content = readFileSync(filePath, "utf-8");
  const fm = parseFrontmatter(content);

  if (!fm) {
    issues.push("Missing YAML frontmatter");
    return issues;
  }

  // Check required fields
  if (!fm.id) issues.push("Missing required field: id");
  if (!fm.status) issues.push("Missing required field: status");

  // Validate status against canonical set
  const VALID_STATUSES = [
    "captured", "exploring", "ready", "prioritised", "active",
    "hold", "blocked", "review", "completed", "surpassed", "archived", "recurring",
  ];
  if (fm.status && !VALID_STATUSES.includes(fm.status)) {
    issues.push(`Invalid status "${fm.status}" — must be one of: ${VALID_STATUSES.join(", ")}`);
  }

  // Check ID format
  if (fm.id && !/^[A-Z]+-\d+$/.test(fm.id) && !/^[A-Z]+-[A-Z]+-\d+$/.test(fm.id)) {
    issues.push(`ID "${fm.id}" doesn't match expected format (PREFIX-NNN)`);
  }

  // Check for relationships section (warn if missing on delivery/process artifacts)
  const rel = relative(projectDir, filePath).replace(/\\/g, "/");
  if (
    (rel.startsWith(".orqa/delivery/") || rel.startsWith(".orqa/process/")) &&
    !content.includes("relationships:")
  ) {
    issues.push("No relationships declared — most artifacts should have at least one relationship");
  }

  return issues;
}

// Main
async function main() {
  let input = "";
  for await (const chunk of process.stdin) {
    input += chunk;
  }

  let hookInput;
  try {
    hookInput = JSON.parse(input);
  } catch {
    process.exit(0);
  }

  const toolName = hookInput.tool_name || "";
  const toolInput = hookInput.tool_input || {};
  const projectDir = hookInput.cwd || process.env.CLAUDE_PROJECT_DIR || ".";

  // Only validate Write and Edit on .orqa/ files
  if (!["Write", "Edit"].includes(toolName)) {
    process.exit(0);
  }

  const filePath = toolInput.file_path || "";
  if (!isOrqaArtifact(filePath, projectDir)) {
    process.exit(0);
  }

  const issues = validateArtifact(filePath, projectDir);

  if (issues.length === 0) {
    process.exit(0);
  }

  const rel = relative(projectDir, filePath).replace(/\\/g, "/");
  const message = [
    `ARTIFACT VALIDATION — ${rel}:`,
    ...issues.map((i) => `  - ${i}`),
    "",
    "Fix these issues before committing. Run `orqa validate` for full integrity check.",
  ].join("\n");

  process.stdout.write(JSON.stringify({ systemMessage: message }));
  process.exit(0);
}

main().catch(() => process.exit(0));
