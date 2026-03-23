#!/usr/bin/env node
// validate-relationships.mjs — Validates relationship type fields in artifact frontmatter.
//
// Used by both:
//   1. Pre-commit hook: node validate-relationships.mjs <file1.md> [file2.md ...]
//   2. PreAction hook (stdin): reads JSON { tool_input: { file_path, content } }
//
// Reads valid relationship types from installed plugin manifests.
// Exit 0 = valid, Exit 1 = errors (pre-commit mode), Exit 2 = block (hook mode).

import { readFileSync, readdirSync, existsSync } from "fs";
import { join, resolve } from "path";
import matter from "gray-matter";

// ---------------------------------------------------------------------------
// Find project root
// ---------------------------------------------------------------------------

function findProjectRoot(startDir) {
  let dir = startDir;
  while (dir !== "/" && dir !== "." && !dir.endsWith(":\\")) {
    if (existsSync(join(dir, ".orqa"))) return dir;
    const parent = join(dir, "..");
    if (parent === dir) break;
    dir = parent;
  }
  return startDir;
}

// ---------------------------------------------------------------------------
// Relationship vocabulary loading from plugin manifests
// ---------------------------------------------------------------------------

function loadRelationshipVocabulary(projectRoot) {
  const vocab = new Set();

  for (const container of ["plugins", "connectors"]) {
    const dir = join(projectRoot, container);
    if (!existsSync(dir)) continue;
    let entries;
    try { entries = readdirSync(dir, { withFileTypes: true }); } catch { continue; }

    for (const entry of entries) {
      if (!entry.isDirectory() || entry.name.startsWith(".")) continue;
      const manifestPath = join(dir, entry.name, "orqa-plugin.json");
      if (!existsSync(manifestPath)) continue;

      let manifest;
      try { manifest = JSON.parse(readFileSync(manifestPath, "utf-8")); } catch { continue; }

      for (const rel of manifest.provides?.relationships || []) {
        if (rel.key) vocab.add(rel.key);
        if (rel.inverse) vocab.add(rel.inverse);
      }
    }
  }

  return vocab;
}

// ---------------------------------------------------------------------------
// Validate relationships in frontmatter
// ---------------------------------------------------------------------------

function validateRelationships(relationships, vocab, label) {
  const errors = [];

  if (!Array.isArray(relationships)) return errors;

  for (let i = 0; i < relationships.length; i++) {
    const rel = relationships[i];
    if (!rel || typeof rel !== "object") {
      errors.push(`${label} relationships[${i}] — not an object`);
      continue;
    }
    if (!rel.target) {
      errors.push(`${label} relationships[${i}] — missing 'target' field`);
    }
    if (!rel.type) {
      errors.push(`${label} relationships[${i}] — missing 'type' field`);
      continue;
    }
    if (!vocab.has(rel.type)) {
      errors.push(
        `${label} relationships[${i}].type — invalid type '${rel.type}'` +
        `\n  Valid: ${[...vocab].sort().join(", ")}`
      );
    }
  }

  return errors;
}

// ---------------------------------------------------------------------------
// Pre-commit mode: validate file arguments
// ---------------------------------------------------------------------------

function runPreCommitMode(files, projectRoot) {
  const vocab = loadRelationshipVocabulary(projectRoot);
  if (vocab.size === 0) {
    console.log("No relationship types found in plugin manifests — skipping.");
    process.exit(0);
  }

  let totalErrors = 0;

  for (const file of files) {
    if (!file.endsWith(".md")) continue;
    const absFile = resolve(projectRoot, file);
    if (!existsSync(absFile)) continue;

    let parsed;
    try { parsed = matter(readFileSync(absFile, "utf-8")); } catch { continue; }

    const relationships = parsed.data?.relationships;
    if (!Array.isArray(relationships) || relationships.length === 0) continue;

    const errors = validateRelationships(relationships, vocab, file);
    for (const err of errors) {
      console.error(`ERROR: ${err}`);
      totalErrors++;
    }
  }

  if (totalErrors > 0) {
    console.error(`\nRelationship validation failed: ${totalErrors} error(s).`);
    console.error("Valid types are defined in plugin manifests (provides.relationships[]).");
    process.exit(1);
  }
}

// ---------------------------------------------------------------------------
// Hook mode: validate from stdin JSON
// ---------------------------------------------------------------------------

function runHookMode(projectRoot) {
  let input = "";
  process.stdin.setEncoding("utf-8");
  process.stdin.on("data", (chunk) => { input += chunk; });
  process.stdin.on("end", () => {
    let data;
    try { data = JSON.parse(input); } catch { process.exit(0); }

    const filePath = data.tool_input?.file_path || data.tool_input?.filePath || "";

    // Only check .orqa/ markdown files
    if (!filePath.includes(".orqa/") && !filePath.includes(".orqa\\")) process.exit(0);
    if (!filePath.endsWith(".md")) process.exit(0);

    // For Write tool: content field. For Edit tool: new_string field.
    const content = data.tool_input?.content || "";
    const newString = data.tool_input?.new_string || "";
    const textToCheck = content || newString;

    if (!textToCheck || !textToCheck.includes("relationships:")) process.exit(0);

    // Parse frontmatter from the content (only works for Write with full content)
    if (content) {
      let parsed;
      try { parsed = matter(content); } catch { process.exit(0); }

      const relationships = parsed.data?.relationships;
      if (!Array.isArray(relationships) || relationships.length === 0) process.exit(0);

      const vocab = loadRelationshipVocabulary(projectRoot);
      if (vocab.size === 0) process.exit(0);

      const errors = validateRelationships(relationships, vocab, filePath);
      if (errors.length > 0) {
        const msg = errors.join("; ");
        const result = JSON.stringify({
          decision: "deny",
          reason: `Relationship type validation failed: ${msg}`
        });
        process.stderr.write(result);
        process.exit(2);
      }
    }

    process.exit(0);
  });
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

const args = process.argv.slice(2);
const projectRoot = findProjectRoot(process.cwd());

if (args.length > 0 && !args[0].startsWith("--stdin")) {
  // Pre-commit mode: file arguments
  runPreCommitMode(args, projectRoot);
} else if (args.includes("--stdin") || !process.stdin.isTTY) {
  // Hook mode: read from stdin
  runHookMode(projectRoot);
} else {
  console.log("Usage:");
  console.log("  node validate-relationships.mjs <file1.md> [file2.md ...]  # pre-commit");
  console.log("  echo '{...}' | node validate-relationships.mjs --stdin       # hook mode");
  process.exit(0);
}
