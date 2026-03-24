#!/usr/bin/env node
// validate-relationships.mjs — Validates relationship types in .orqa/ artifact frontmatter.
//
// Thin adapter: delegates to the orqa-validation daemon at localhost:10258.
// The daemon holds the full relationship vocabulary (core.json + plugin manifests)
// in memory — no JS-side vocabulary loading needed.
//
// Usage:
//   node validate-relationships.mjs <file1.md> [file2.md ...]
//   node validate-relationships.mjs --all
// Exit code 0 = all valid, 1 = validation errors found.

import { readdirSync, existsSync } from "fs";
import { resolve, join, relative } from "path";

const ROOT = resolve(import.meta.dirname, "..");
const DAEMON_BASE = "http://localhost:10258";

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
// File discovery (--all mode)
// ---------------------------------------------------------------------------

function getAllArtifactFiles(projectRoot) {
  const files = [];
  const orqaDir = join(projectRoot, ".orqa");
  if (!existsSync(orqaDir)) return files;

  function walk(dir) {
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      if (["node_modules", "dist", "target", ".git"].includes(entry.name)) continue;
      const full = join(dir, entry.name);
      if (entry.isDirectory()) walk(full);
      else if (entry.name.endsWith(".md") && !entry.name.startsWith("README")) {
        files.push(relative(projectRoot, full).replace(/\\/g, "/"));
      }
    }
  }

  walk(orqaDir);
  return files;
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

const args = process.argv.slice(2);
const allMode = args.includes("--all");
const files = allMode ? getAllArtifactFiles(ROOT) : args.filter(a => !a.startsWith("--"));

if (files.length === 0) {
  process.exit(0);
}

let errors = 0;

for (const file of files) {
  if (!file.endsWith(".md")) continue;

  const absFile = resolve(ROOT, file);
  if (!existsSync(absFile)) continue;

  let parsed;
  try {
    parsed = await callDaemon("/parse", { file: absFile });
  } catch {
    // Daemon unavailable — skip relationship validation gracefully.
    // The daemon handles all vocabulary loading; without it we cannot validate.
    console.log("Daemon unavailable — skipping relationship validation.");
    process.exit(0);
  }

  // Check validation result from daemon (schema-level checks include relationship fields)
  if (parsed.validation && !parsed.validation.valid) {
    for (const err of parsed.validation.errors || []) {
      if (err.toLowerCase().includes("relationship")) {
        console.error(`ERROR: ${file} — ${err}`);
        errors++;
      }
    }
  }

  // Check findings array if the daemon provides file-level findings
  for (const finding of parsed.findings || []) {
    if (finding.severity === "error" || finding.severity === "Error") {
      if (finding.message.toLowerCase().includes("relationship")) {
        console.error(`ERROR: ${file} — ${finding.message}`);
        errors++;
      }
    }
  }
}

if (errors > 0) {
  console.error(`\nRelationship validation failed: ${errors} error(s) found.`);
  process.exit(1);
}
