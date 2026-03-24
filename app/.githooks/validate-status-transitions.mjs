#!/usr/bin/env node
// Validates artifact status transitions in staged files.
// Blocks invalid state changes (e.g., captured→completed skipping active).
//
// Called from pre-commit hook when .orqa/ markdown files are staged.
//
// Transition rules are loaded from plugin manifests (plugins/*/orqa-plugin.json)
// which are the canonical source of truth for statusTransitions per artifact type.
// Parsing delegates to the daemon (http://localhost:10258/parse) with fallback to
// the orqa-validation binary.

import { resolve, join } from "path";
import { execSync, execFileSync } from "child_process";
import { readdirSync, readFileSync, existsSync } from "fs";

const ROOT = resolve(import.meta.dirname, "..", "..");
const DAEMON_BASE = "http://localhost:10258";

// ---------------------------------------------------------------------------
// Transition map — loaded from plugin manifests
// ---------------------------------------------------------------------------

/**
 * Scan plugins/star/orqa-plugin.json and connectors/star/orqa-plugin.json to build
 * a map of { idPrefix → { fromStatus → [toStatus, ...] } }.
 *
 * Mirrors the Rust daemon's scan_plugin_manifests() logic so the hook reads
 * the same source of truth without hardcoding any transitions.
 */
function loadTransitionMap() {
  const map = new Map(); // idPrefix → { status → [nextStatuses] }

  for (const searchDir of ["plugins", "connectors"]) {
    const dir = join(ROOT, searchDir);
    let entries;
    try {
      entries = readdirSync(dir, { withFileTypes: true });
    } catch {
      continue;
    }

    for (const entry of entries) {
      if (!entry.isDirectory()) continue;
      const manifestPath = join(dir, entry.name, "orqa-plugin.json");
      if (!existsSync(manifestPath)) continue;

      let manifest;
      try {
        manifest = JSON.parse(readFileSync(manifestPath, "utf-8"));
      } catch {
        continue; // Malformed manifest — skip like the Rust daemon does
      }

      const schemas = manifest?.provides?.schemas;
      if (!Array.isArray(schemas)) continue;

      for (const schema of schemas) {
        const prefix = schema.idPrefix;
        const transitions = schema.statusTransitions;
        if (!prefix || !transitions || typeof transitions !== "object") continue;
        // Don't overwrite if multiple plugins define the same prefix — first wins,
        // which matches the Rust daemon's order-dependent loading.
        if (!map.has(prefix)) {
          map.set(prefix, transitions);
        }
      }
    }
  }

  return map;
}

// ---------------------------------------------------------------------------
// Daemon + binary parsing
// ---------------------------------------------------------------------------

/**
 * Parse a single artifact file via the daemon, falling back to the binary.
 * Returns { id, type, status, ... } or null on failure.
 */
async function parseArtifact(absPath) {
  // Try daemon first
  try {
    const res = await fetch(`${DAEMON_BASE}/parse`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ file: absPath }),
      signal: AbortSignal.timeout(5000),
    });
    if (res.ok) return await res.json();
  } catch {
    // Daemon unavailable — fall through to binary
  }

  // Fall back to binary
  const binary = findBinary();
  if (!binary) return null;

  try {
    const output = execFileSync(binary, ["parse", absPath], {
      encoding: "utf-8",
      timeout: 10000,
      windowsHide: true,
    });
    return JSON.parse(output);
  } catch {
    return null;
  }
}

/** Find the orqa-validation binary in common build locations. */
function findBinary() {
  const candidates = [
    join(ROOT, "libs", "validation", "target", "release", "orqa-validation.exe"),
    join(ROOT, "libs", "validation", "target", "release", "orqa-validation"),
    join(ROOT, "libs", "validation", "target", "debug", "orqa-validation.exe"),
    join(ROOT, "libs", "validation", "target", "debug", "orqa-validation"),
    join(ROOT, "target", "release", "orqa-validation.exe"),
    join(ROOT, "target", "release", "orqa-validation"),
    join(ROOT, "target", "debug", "orqa-validation.exe"),
    join(ROOT, "target", "debug", "orqa-validation"),
  ];
  for (const c of candidates) {
    if (existsSync(c)) return c;
  }
  return null;
}

// ---------------------------------------------------------------------------
// Old status extraction (from committed content via git)
// ---------------------------------------------------------------------------

/**
 * Extract status from the already-committed version of a file.
 * Uses a lightweight regex — the committed content is already validated.
 */
function getOldStatus(file) {
  let oldContent;
  try {
    oldContent = execSync(`git show HEAD:${file}`, { encoding: "utf-8" });
  } catch {
    return null; // File didn't exist in HEAD (new file)
  }
  const normalized = oldContent.replace(/\r\n/g, "\n").replace(/\r/g, "\n");
  const match = normalized.match(/^---\n[\s\S]*?\nstatus:\s*(\S+)[\s\S]*?\n---/m);
  return match?.[1] ?? null;
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

async function main() {
  // Load transition rules from plugin manifests (canonical source of truth)
  const transitionMap = loadTransitionMap();

  if (transitionMap.size === 0) {
    console.error("  WARN: No statusTransitions found in plugin manifests — skipping transition validation");
    process.exit(0);
  }

  // Get staged modifications to .orqa/ markdown files
  const stagedModified = execSync(
    "git diff --cached --name-only --diff-filter=M", { encoding: "utf-8" }
  ).trim().split("\n").filter(f => f.startsWith(".orqa/") && f.endsWith(".md"));

  let errors = 0;

  for (const file of stagedModified) {
    const absPath = resolve(ROOT, file);
    const filename = file.split("/").pop();

    // Determine artifact type prefix and get transition rules
    let transitions = null;
    let matchedPrefix = null;
    for (const [prefix, rules] of transitionMap) {
      // ID format is PREFIX-hash (e.g., EPIC-be023ed2), so match prefix + dash
      if (filename.startsWith(prefix + "-") || filename.startsWith(prefix + ".")) {
        transitions = rules;
        matchedPrefix = prefix;
        break;
      }
    }

    if (!transitions) continue; // Unknown artifact type — no transition rules

    // Get the old (committed) status via git
    const oldStatus = getOldStatus(file);
    if (!oldStatus) continue;

    // Get the new (staged) status via daemon/binary parsing
    const parsed = await parseArtifact(absPath);
    const newStatus = parsed?.status ?? null;
    if (!newStatus) continue;

    // No change — skip
    if (oldStatus === newStatus) continue;

    // Validate the transition
    const validNext = transitions[oldStatus];
    if (!validNext) {
      // Old status is a terminal state or not in the transitions map
      console.error(`  ERROR: ${filename}: cannot transition from terminal status "${oldStatus}" to "${newStatus}"`);
      errors++;
    } else if (!Array.isArray(validNext) || !validNext.includes(newStatus)) {
      console.error(`  ERROR: ${filename}: invalid transition "${oldStatus}" → "${newStatus}" (valid: ${Array.isArray(validNext) ? validNext.join(", ") : "none"})`);
      errors++;
    }
  }

  if (errors > 0) {
    console.error(`\n${errors} invalid status transition(s) — commit blocked`);
    process.exit(1);
  }
}

main().catch((err) => {
  console.error(`  WARN: Status transition validation failed: ${err.message}`);
  // Don't block commits if the validation infrastructure itself fails
  process.exit(0);
});
