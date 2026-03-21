#!/usr/bin/env node
// PreToolUse hook: enforces relationship requirements when creating new .orqa/ artifacts.
//
// Fires on Write/Edit to .orqa/ files. When the target file does NOT yet exist
// (new artifact creation), checks the content for minimum required relationships
// based on the artifact type inferred from the file path and `type:` frontmatter.
//
// Issues a WARN (non-blocking systemMessage) listing any missing relationships.
// Never blocks — governance debt is surfaced, not prevented.

import { existsSync, readFileSync } from "fs";
import { join, relative } from "path";
import { logTelemetry } from "./telemetry.mjs";

// ---------------------------------------------------------------------------
// Type → required relationship rules
// ---------------------------------------------------------------------------

/**
 * Minimum relationship requirements per artifact type.
 * Each entry lists the relationship keys that MUST appear in the relationships array.
 *
 * @type {Record<string, Array<{key: string, label: string}>>}
 */
const TYPE_REQUIREMENTS = {
  task: [{ key: "delivers", label: "delivers → epic" }],
  epic: [{ key: "fulfils", label: "fulfils → milestone" }],
  idea: [{ key: "grounded", label: "grounded → pillar" }],
  agent: [
    { key: "serves", label: "serves → pillar or persona" },
    { key: "employs", label: "employs → knowledge" },
  ],
};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/**
 * Check if a file path is within the .orqa/ directory.
 *
 * @param {string} filePath
 * @param {string} projectDir
 * @returns {boolean}
 */
function isOrqaArtifact(filePath, projectDir) {
  const rel = relative(projectDir, filePath).replace(/\\/g, "/");
  return rel.startsWith(".orqa/") && filePath.endsWith(".md");
}

/**
 * Extract YAML frontmatter text from markdown content.
 *
 * @param {string} content
 * @returns {string | null}
 */
function extractFrontmatterText(content) {
  if (!content.startsWith("---\n")) return null;
  const endIdx = content.indexOf("\n---", 4);
  if (endIdx === -1) return null;
  return content.slice(4, endIdx);
}

/**
 * Extract a scalar frontmatter field value.
 *
 * @param {string} fmText
 * @param {string} field
 * @returns {string | null}
 */
function getFrontmatterField(fmText, field) {
  const lines = fmText.split("\n");
  for (const line of lines) {
    const match = line.match(new RegExp(`^${field}:\\s*(.+)$`));
    if (match) {
      return match[1].trim().replace(/^"|"$/g, "");
    }
  }
  return null;
}

/**
 * Extract all relationship type keys from frontmatter.
 *
 * @param {string} fmText
 * @returns {string[]}
 */
function extractRelationshipTypes(fmText) {
  const types = [];
  const lines = fmText.split("\n");
  for (const line of lines) {
    const match = line.match(/^\s+type:\s*(.+)$/);
    if (match) {
      types.push(match[1].trim().replace(/^"|"$/g, ""));
    }
  }
  return types;
}

/**
 * Infer artifact type from the file path using the same heuristic as the
 * Rust graph builder.
 *
 * @param {string} relPath  Relative path from project root
 * @returns {string | null}
 */
function inferTypeFromPath(relPath) {
  const norm = relPath.replace(/\\/g, "/");
  if (norm.includes("/tasks/")) return "task";
  if (norm.includes("/epics/")) return "epic";
  if (norm.includes("/milestones/")) return "milestone";
  if (norm.includes("/ideas/")) return "idea";
  if (norm.includes("/decisions/")) return "decision";
  if (norm.includes("/rules/")) return "rule";
  if (norm.includes("/agents/")) return "agent";
  if (norm.includes("/knowledge/")) return "knowledge";
  if (norm.includes("/pillars/")) return "pillar";
  return null;
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

async function main() {
  const startTime = Date.now();

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

  if (!["Write", "Edit"].includes(toolName)) {
    process.exit(0);
  }

  const filePath = toolInput.file_path || "";
  if (!isOrqaArtifact(filePath, projectDir)) {
    process.exit(0);
  }

  // Only run on NEW artifacts — if the file already exists, skip.
  if (existsSync(filePath)) {
    process.exit(0);
  }

  // Extract the content that is about to be written.
  const content =
    toolName === "Write"
      ? toolInput.content || ""
      : toolInput.new_string || toolInput.content || "";

  if (!content) {
    process.exit(0);
  }

  const fmText = extractFrontmatterText(content);
  if (!fmText) {
    process.exit(0);
  }

  // Determine artifact type: frontmatter `type:` field takes priority, then path inference.
  const relPath = relative(projectDir, filePath).replace(/\\/g, "/");
  const frontmatterType = getFrontmatterField(fmText, "type");
  const artifactType = frontmatterType || inferTypeFromPath(relPath);

  if (!artifactType) {
    process.exit(0);
  }

  const requirements = TYPE_REQUIREMENTS[artifactType];
  if (!requirements || requirements.length === 0) {
    process.exit(0);
  }

  // Check which required relationships are present.
  const presentTypes = new Set(extractRelationshipTypes(fmText));
  const missing = requirements.filter((req) => !presentTypes.has(req.key));

  logTelemetry(
    "artifact-enforcement",
    "PreToolUse",
    startTime,
    missing.length === 0 ? "ok" : "warned",
    {
      file: relPath,
      artifact_type: artifactType,
      missing_count: missing.length,
      missing: missing.map((r) => r.key),
    },
    projectDir
  );

  if (missing.length === 0) {
    process.exit(0);
  }

  const lines = [
    `ARTIFACT RELATIONSHIP WARNING — ${relPath} (type: ${artifactType}):`,
    `New ${artifactType} artifact is missing required relationships:`,
  ];
  for (const req of missing) {
    lines.push(`  - ${req.label}`);
  }
  lines.push("");
  lines.push(
    "Add these relationships before committing. " +
      "Run `orqa validate` to check the full graph after writing."
  );

  process.stdout.write(JSON.stringify({ systemMessage: lines.join("\n") }));
  process.exit(0);
}

main().catch(() => process.exit(0));
