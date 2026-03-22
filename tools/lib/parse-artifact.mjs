#!/usr/bin/env node
/**
 * Shared artifact parser — delegates to the orqa-validation Rust binary.
 *
 * Replaces hand-rolled parseFrontmatter() + yaml.parse() across all tools
 * and githooks scripts. The Rust binary is the single authoritative parser
 * for artifact frontmatter.
 *
 * Usage:
 *   import { parseFrontmatter, parseArtifact } from "../tools/lib/parse-artifact.mjs";
 *
 *   const fm = parseFrontmatter("/abs/path/to/artifact.md");
 *   // → { id: "RULE-...", title: "...", status: "...", ... } | null
 *
 *   const artifact = parseArtifact("/abs/path/to/artifact.md");
 *   // → { id, type, status, title, frontmatter, body, ... } | null
 */

import { execFileSync } from "child_process";
import { existsSync } from "fs";
import { resolve, join } from "path";

// ── Binary discovery ────────────────────────────────────────────────────────

/**
 * Find the orqa-validation binary in common build locations.
 * Returns null if not found (tools degrade gracefully).
 */
function findBinary(projectRoot) {
  const candidates = [
    join(projectRoot, "libs", "validation", "target", "release", "orqa-validation.exe"),
    join(projectRoot, "libs", "validation", "target", "release", "orqa-validation"),
    join(projectRoot, "libs", "validation", "target", "debug", "orqa-validation.exe"),
    join(projectRoot, "libs", "validation", "target", "debug", "orqa-validation"),
    join(projectRoot, "target", "release", "orqa-validation.exe"),
    join(projectRoot, "target", "release", "orqa-validation"),
    join(projectRoot, "target", "debug", "orqa-validation.exe"),
    join(projectRoot, "target", "debug", "orqa-validation"),
  ];
  for (const c of candidates) {
    if (existsSync(c)) return c;
  }
  return null;
}

// Cache the binary path after first lookup
let _binaryPath = undefined;

/**
 * Get the binary path, resolving from the project root (two dirs up from tools/lib/).
 */
function getBinary() {
  if (_binaryPath !== undefined) return _binaryPath;
  const projectRoot = resolve(import.meta.dirname, "..", "..", "..");
  _binaryPath = findBinary(projectRoot);
  return _binaryPath;
}

// ── Public API ──────────────────────────────────────────────────────────────

/**
 * Parse a single artifact file using the Rust binary.
 *
 * Returns the full parsed artifact object (id, type, status, title,
 * frontmatter, body) or null if the file could not be parsed.
 *
 * The filePath must be an absolute path on disk.
 */
export function parseArtifact(filePath) {
  const binary = getBinary();
  if (!binary) return null;

  // Normalise to the platform path the binary expects (Windows backslash)
  const absPath = resolve(filePath).replace(/\//g, "\\");

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

/**
 * Return just the frontmatter object from a parsed artifact, or null.
 *
 * This is the drop-in replacement for the old hand-rolled parseFrontmatter()
 * functions that called yaml.parse() directly.
 */
export function parseFrontmatter(filePath) {
  const artifact = parseArtifact(filePath);
  return artifact?.frontmatter ?? null;
}
