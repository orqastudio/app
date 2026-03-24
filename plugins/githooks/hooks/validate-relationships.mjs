#!/usr/bin/env node
// validate-relationships.mjs — Validates relationship type fields in artifact frontmatter.
//
// Thin adapter: delegates to the orqa-validation daemon at localhost:10258.
// The daemon holds the full relationship vocabulary (core.json + plugin manifests)
// in memory — no JS-side vocabulary loading needed.
//
// Used by both:
//   1. Pre-commit hook: node validate-relationships.mjs <file1.md> [file2.md ...]
//   2. PreAction hook (stdin): reads JSON { tool_input: { file_path, content } }
//
// Exit 0 = valid, Exit 1 = errors (pre-commit mode), Exit 2 = block (hook mode).

import { existsSync } from "fs";
import { join, resolve } from "path";

const PORT_BASE = parseInt(process.env.ORQA_PORT_BASE || "10200", 10);
const DAEMON_PORT = (Number.isNaN(PORT_BASE) ? 10200 : PORT_BASE) + 58;
const DAEMON_BASE = `http://localhost:${DAEMON_PORT}`;

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
// Daemon client
// ---------------------------------------------------------------------------

async function callDaemon(path, body) {
  const res = await fetch(`${DAEMON_BASE}${path}`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(body),
    signal: AbortSignal.timeout(8000),
  });
  if (!res.ok) {
    throw new Error(`daemon ${path} returned ${res.status}`);
  }
  return res.json();
}

// ---------------------------------------------------------------------------
// Collect relationship findings from daemon response
// ---------------------------------------------------------------------------

function collectRelationshipErrors(parsed, label) {
  const errors = [];

  // Schema-level validation errors
  if (parsed.validation && !parsed.validation.valid) {
    for (const err of parsed.validation.errors || []) {
      if (err.toLowerCase().includes("relationship")) {
        errors.push(`${label} — ${err}`);
      }
    }
  }

  // File-level findings (if daemon provides them)
  for (const finding of parsed.findings || []) {
    if (
      (finding.severity === "error" || finding.severity === "Error") &&
      finding.message.toLowerCase().includes("relationship")
    ) {
      errors.push(`${label} — ${finding.message}`);
    }
  }

  return errors;
}

// ---------------------------------------------------------------------------
// Pre-commit mode: validate file arguments
// ---------------------------------------------------------------------------

async function runPreCommitMode(files, projectRoot) {
  let totalErrors = 0;

  for (const file of files) {
    if (!file.endsWith(".md")) continue;
    const absFile = resolve(projectRoot, file);
    if (!existsSync(absFile)) continue;

    let parsed;
    try {
      parsed = await callDaemon("/parse", { file: absFile });
    } catch {
      // Daemon unavailable — skip gracefully
      console.log("Daemon unavailable — skipping relationship validation.");
      process.exit(0);
    }

    const errors = collectRelationshipErrors(parsed, file);
    for (const err of errors) {
      console.error(`ERROR: ${err}`);
      totalErrors++;
    }
  }

  if (totalErrors > 0) {
    console.error(`\nRelationship validation failed: ${totalErrors} error(s).`);
    process.exit(1);
  }
}

// ---------------------------------------------------------------------------
// Hook mode: validate from stdin JSON
// ---------------------------------------------------------------------------

async function runHookMode(projectRoot) {
  let input = "";
  process.stdin.setEncoding("utf-8");
  for await (const chunk of process.stdin) {
    input += chunk;
  }

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

  // Delegate to daemon — if the file exists on disk, parse it
  if (existsSync(filePath)) {
    let parsed;
    try {
      parsed = await callDaemon("/parse", { file: filePath });
    } catch {
      // Daemon unavailable — allow the action gracefully
      process.exit(0);
    }

    const errors = collectRelationshipErrors(parsed, filePath);
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
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

const args = process.argv.slice(2);
const projectRoot = findProjectRoot(process.cwd());

if (args.length > 0 && !args[0].startsWith("--stdin")) {
  // Pre-commit mode: file arguments
  await runPreCommitMode(args, projectRoot);
} else if (args.includes("--stdin") || !process.stdin.isTTY) {
  // Hook mode: read from stdin
  await runHookMode(projectRoot);
} else {
  console.log("Usage:");
  console.log("  node validate-relationships.mjs <file1.md> [file2.md ...]  # pre-commit");
  console.log("  echo '{...}' | node validate-relationships.mjs --stdin       # hook mode");
  process.exit(0);
}
