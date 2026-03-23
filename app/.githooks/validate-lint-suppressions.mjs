#!/usr/bin/env node
/**
 * Lint suppression validator for pre-commit hook.
 *
 * Scans staged files for lint suppression annotations and validates that each
 * one references an accepted decision artifact (AD-NNN).
 *
 * Pattern: every suppression must have a `// AD-NNN` or `<!-- AD-NNN` comment
 * on the same line or the immediately preceding/following line.
 *
 * Exempt paths: vendored/generated code (shadcn-svelte, SvelteKit, etc.)
 *
 * Stable interface: the `// AD-NNN` comment pattern is designed to be consumed
 * by future linter integration plugins. This scanner is the initial enforcement
 * mechanism — see AD-047 for the transportability design.
 *
 * Usage: node validate-lint-suppressions.mjs [files...]
 * Exit code 0 = clean, 1 = violations found.
 */

import { readFileSync, existsSync } from "fs";
import { resolve, relative } from "path";

const ROOT = resolve(import.meta.dirname, "..");

// Patterns that indicate lint suppression annotations
const SUPPRESSION_PATTERNS = [
  /\#\[allow\(/,                        // Rust: #[allow(clippy::...)]
  /\/\/\s*eslint-disable/,              // JS/TS: // eslint-disable
  /<!--\s*eslint-disable/,              // Svelte: <!-- eslint-disable -->
  /@ts-ignore/,                         // TypeScript: @ts-ignore
  /@ts-expect-error/,                   // TypeScript: @ts-expect-error
  /\/\*\s*eslint-disable/,              // JS/TS: /* eslint-disable */
];

// Decision reference pattern: AD-NNN anywhere on the line or adjacent lines
const DECISION_REF = /AD-\d+/;

// Exempt path patterns (vendored/generated code)
const EXEMPT_PATTERNS = [
  /ui\/src\/lib\/components\/ui\//,     // shadcn-svelte vendored components
  /\.svelte-kit\//,                     // SvelteKit generated types
  /node_modules\//,                     // Third-party dependencies
  /target\//,                           // Rust build output
];

/**
 * Check if a file path is exempt from suppression validation.
 */
function isExempt(filePath) {
  const rel = relative(ROOT, filePath).replace(/\\/g, "/");
  return EXEMPT_PATTERNS.some((p) => p.test(rel));
}

/**
 * Verify that a referenced decision artifact exists and is accepted.
 */
function isDecisionValid(adId) {
  const decisionPath = resolve(ROOT, `.orqa/process/decisions/${adId}.md`);
  if (!existsSync(decisionPath)) return false;

  const content = readFileSync(decisionPath, "utf-8");
  // Quick frontmatter check for status: accepted
  const fmMatch = content.match(/^---\n([\s\S]*?)\n---/);
  if (!fmMatch) return false;

  return /status:\s*accepted/.test(fmMatch[1]);
}

/**
 * Scan a file for lint suppressions without valid decision references.
 * Returns an array of violation descriptions.
 */
function scanFile(filePath) {
  const violations = [];
  const content = readFileSync(filePath, "utf-8");
  const lines = content.split("\n");
  const rel = relative(ROOT, filePath).replace(/\\/g, "/");

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];

    // Check if this line contains a suppression pattern
    const hasSuppression = SUPPRESSION_PATTERNS.some((p) => p.test(line));
    if (!hasSuppression) continue;

    // Check for decision reference on same line, line before, or line after
    const context = [
      i > 0 ? lines[i - 1] : "",
      line,
      i < lines.length - 1 ? lines[i + 1] : "",
    ].join(" ");

    const adMatch = context.match(/AD-(\d+)/);
    if (!adMatch) {
      violations.push(
        `  ${rel}:${i + 1}: lint suppression without decision reference\n` +
        `    ${line.trim()}\n` +
        `    Add a comment referencing the decision: // AD-NNN: justification`
      );
      continue;
    }

    const adId = `AD-${adMatch[1]}`;
    if (!isDecisionValid(adId)) {
      violations.push(
        `  ${rel}:${i + 1}: references ${adId} which does not exist or is not accepted\n` +
        `    ${line.trim()}`
      );
    }
  }

  return violations;
}

// --- Main ---

const files = process.argv.slice(2);
if (files.length === 0) {
  process.exit(0);
}

let allViolations = [];

for (const file of files) {
  const abs = resolve(ROOT, file);
  if (!existsSync(abs)) continue;
  if (isExempt(abs)) continue;

  // Only scan source files
  if (!/\.(rs|ts|js|svelte|tsx|jsx)$/.test(file)) continue;

  allViolations.push(...scanFile(abs));
}

if (allViolations.length > 0) {
  console.error(
    "Lint suppression violations found:\n\n" +
    allViolations.join("\n\n") +
    "\n\n" +
    "Every lint suppression must reference an accepted decision (AD-NNN).\n" +
    "See AD-047 for the suppression exception policy."
  );
  process.exit(1);
}

process.exit(0);
