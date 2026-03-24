#!/usr/bin/env node
// Validates YAML frontmatter of .orqa/ markdown files against JSON Schema.
// Delegates to the orqa-validation daemon at localhost:10258.
//
// Usage: node validate-schema.mjs <file1.md> [file2.md ...]
// Exit code 0 = all valid, 1 = validation errors found.

import { resolve, relative } from "path";
import { spawnSync } from "child_process";

const ROOT = resolve(import.meta.dirname, "..");
const DAEMON_BASE = "http://localhost:10258";

// Parse args: files and optional --warn-rules=RULE-032,RULE-004
const args = process.argv.slice(2);
const warnRulesArg = args.find((a) => a.startsWith("--warn-rules="));
const warnRules = warnRulesArg
  ? new Set(warnRulesArg.split("=")[1].split(","))
  : new Set();
const files = args.filter((a) => !a.startsWith("--warn-rules="));

if (files.length === 0) {
  process.exit(0);
}

// When RULE-032 is in warn-rules, schema validation failures become warnings
const schemaWarnOnly = warnRules.has("RULE-032");

let errors = 0;
let warnings = 0;

// Helper: report as error or warning depending on RULE-032 suspension
function reportIssue(msg) {
  if (schemaWarnOnly) {
    console.error(`WARNING (RULE-032 suspended): ${msg}`);
    warnings++;
  } else {
    console.error(`ERROR: ${msg}`);
    errors++;
  }
}

// Call daemon /parse endpoint, returns parsed artifact or null on failure
async function callDaemon(absPath) {
  try {
    const res = await fetch(`${DAEMON_BASE}/parse`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ file: absPath }),
      signal: AbortSignal.timeout(8000),
    });
    if (!res.ok) {
      return null;
    }
    return await res.json();
  } catch {
    return null;
  }
}

// Fallback: call orqa-validation binary directly
// CLI: orqa-validation parse <file> [--project <project-root>]
function callBinary(absPath) {
  const result = spawnSync("orqa-validation", ["parse", absPath, "--project", ROOT], {
    encoding: "utf-8",
    timeout: 10000,
    windowsHide: true,
  });
  if (result.error || !result.stdout) {
    return null;
  }
  try {
    return JSON.parse(result.stdout);
  } catch {
    return null;
  }
}

// Track whether daemon is reachable (checked once)
let daemonChecked = false;
let daemonAvailable = false;

async function checkDaemon() {
  if (daemonChecked) return daemonAvailable;
  daemonChecked = true;
  try {
    const res = await fetch(`${DAEMON_BASE}/health`, {
      signal: AbortSignal.timeout(2000),
    });
    daemonAvailable = res.ok;
  } catch {
    daemonAvailable = false;
  }
  return daemonAvailable;
}

// Main validation loop
let daemonReachable = await checkDaemon();
if (!daemonReachable) {
  console.error("WARNING: orqa-validation daemon not running — falling back to binary");
}

for (const file of files) {
  // Skip READMEs and non-md files
  if (file.endsWith("README.md") || !file.endsWith(".md")) continue;

  const absFile = resolve(ROOT, file);
  const relFile = relative(ROOT, absFile).replace(/\\/g, "/");

  let parsed;
  if (daemonReachable) {
    parsed = await callDaemon(absFile);
  }
  if (!parsed) {
    parsed = callBinary(absFile);
  }

  if (!parsed) {
    // Neither daemon nor binary could parse — skip with warning
    console.error(`WARNING: Could not validate ${relFile} — daemon and binary both unavailable`);
    continue;
  }

  // The daemon returns ParsedArtifact with validation: { valid, errors }
  const validation = parsed.validation;
  if (!validation || validation.valid) {
    continue;
  }

  for (const errMsg of validation.errors ?? []) {
    reportIssue(`${relFile} — ${errMsg}`);
  }
}

if (warnings > 0) {
  console.error(
    `\nSchema validation: ${warnings} warning(s) (RULE-032 suspended — not blocking).`
  );
}

if (errors > 0) {
  console.error(
    `\nSchema validation failed: ${errors} error(s) found.`
  );
  console.error(
    "Run orqa-validation daemon or orqa validate for details."
  );
  process.exit(1);
}
