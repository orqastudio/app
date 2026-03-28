#!/usr/bin/env node
/**
 * Validate connector generator output against targets/claude-code-plugin/.
 *
 * Runs generatePlugin() in dry-run mode, then structurally compares the
 * generated artifacts against the hand-written target state.
 *
 * Usage:
 *   node scripts/validate-connector-output.mjs [--project-root <path>]
 *
 * Exit codes:
 *   0 — all comparisons pass
 *   1 — one or more mismatches found
 *   2 — setup error (target path missing, generator failed, etc.)
 */

import * as fs from "node:fs";
import * as path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, "..");

// ---------------------------------------------------------------------------
// Argument parsing
// ---------------------------------------------------------------------------

function parseArgs() {
  const args = process.argv.slice(2);
  let projectRoot = repoRoot;
  for (let i = 0; i < args.length; i++) {
    if (args[i] === "--project-root" && args[i + 1]) {
      projectRoot = path.resolve(args[++i]);
    }
  }
  return { projectRoot };
}

// ---------------------------------------------------------------------------
// Structural comparison helpers
// ---------------------------------------------------------------------------

/**
 * Normalize whitespace for text comparison.
 * Collapses multiple blank lines to a single blank line and trims trailing whitespace per line.
 */
function normalizeText(text) {
  return text
    .split("\n")
    .map((line) => line.trimEnd())
    .join("\n")
    .replace(/\n{3,}/g, "\n\n")
    .trim();
}

/**
 * Parse JSON with a descriptive error.
 */
function parseJson(filePath, content) {
  try {
    return JSON.parse(content);
  } catch (err) {
    throw new Error(`Failed to parse JSON at ${filePath}: ${err.message}`);
  }
}

/**
 * Deep structural comparison of two values.
 * Returns an array of difference descriptions (empty = match).
 */
function diffValues(a, b, keyPath) {
  const diffs = [];
  if (typeof a !== typeof b) {
    diffs.push(`${keyPath}: type mismatch (got ${typeof b}, expected ${typeof a})`);
    return diffs;
  }
  if (Array.isArray(a) !== Array.isArray(b)) {
    diffs.push(`${keyPath}: array/object mismatch`);
    return diffs;
  }
  if (Array.isArray(a)) {
    if (a.length !== b.length) {
      diffs.push(`${keyPath}: array length mismatch (got ${b.length}, expected ${a.length})`);
    }
    const len = Math.min(a.length, b.length);
    for (let i = 0; i < len; i++) {
      diffs.push(...diffValues(a[i], b[i], `${keyPath}[${i}]`));
    }
    return diffs;
  }
  if (a !== null && typeof a === "object") {
    const aKeys = Object.keys(a).sort();
    const bKeys = Object.keys(b).sort();
    const aSet = new Set(aKeys);
    const bSet = new Set(bKeys);
    for (const k of aKeys) {
      if (!bSet.has(k)) diffs.push(`${keyPath}.${k}: missing in generated output`);
    }
    for (const k of bKeys) {
      if (!aSet.has(k)) diffs.push(`${keyPath}.${k}: extra key in generated output`);
    }
    for (const k of aKeys) {
      if (bSet.has(k)) {
        diffs.push(...diffValues(a[k], b[k], `${keyPath}.${k}`));
      }
    }
    return diffs;
  }
  if (a !== b) {
    diffs.push(`${keyPath}: value mismatch (got ${JSON.stringify(b)}, expected ${JSON.stringify(a)})`);
  }
  return diffs;
}

// ---------------------------------------------------------------------------
// Comparison tasks
// ---------------------------------------------------------------------------

/** Comparison result for one file pair. */
function makeResult(name, targetPath, generatedPath) {
  return { name, targetPath, generatedPath, diffs: [], error: null };
}

/**
 * Compare two markdown files structurally (normalized whitespace).
 */
function compareMarkdown(result) {
  try {
    const target = normalizeText(fs.readFileSync(result.targetPath, "utf-8"));
    const generated = normalizeText(fs.readFileSync(result.generatedPath, "utf-8"));
    if (target !== generated) {
      // Report the first differing line for context.
      const targetLines = target.split("\n");
      const generatedLines = generated.split("\n");
      const maxLen = Math.max(targetLines.length, generatedLines.length);
      for (let i = 0; i < maxLen; i++) {
        if (targetLines[i] !== generatedLines[i]) {
          result.diffs.push(
            `Line ${i + 1}: expected "${targetLines[i] ?? "(missing)"}" got "${generatedLines[i] ?? "(missing)"}"`,
          );
          if (result.diffs.length >= 5) {
            result.diffs.push("... (truncated after 5 diffs)");
            break;
          }
        }
      }
    }
  } catch (err) {
    result.error = err.message;
  }
}

/**
 * Compare two JSON files structurally (key-value equality, not byte-exact).
 */
function compareJson(result) {
  try {
    const targetContent = fs.readFileSync(result.targetPath, "utf-8");
    const generatedContent = fs.readFileSync(result.generatedPath, "utf-8");
    const targetObj = parseJson(result.targetPath, targetContent);
    const generatedObj = parseJson(result.generatedPath, generatedContent);
    result.diffs.push(...diffValues(targetObj, generatedObj, "root"));
  } catch (err) {
    result.error = err.message;
  }
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

async function main() {
  const { projectRoot } = parseArgs();
  const targetBase = path.join(repoRoot, "targets", "claude-code-plugin");
  const dryRunBase = path.join(projectRoot, ".state", "dry-run");

  // Validate target directory exists.
  if (!fs.existsSync(targetBase)) {
    console.error(`ERROR: Target directory not found: ${targetBase}`);
    process.exit(2);
  }

  // Run the generator in dry-run mode.
  console.log("Running generator in dry-run mode...");
  process.env["ORQA_DRY_RUN"] = "true";

  let generatePlugin;
  try {
    const generatorPath = path.join(repoRoot, "connectors", "claude-code", "dist", "generator.js");
    const module = await import(pathToFileURL(generatorPath).href);
    generatePlugin = module.generatePlugin;
  } catch (err) {
    console.error(`ERROR: Could not import generator: ${err.message}`);
    console.error("Run `npm run build` in connectors/claude-code/ first.");
    process.exit(2);
  }

  let generateResult;
  try {
    generateResult = generatePlugin(projectRoot);
  } catch (err) {
    console.error(`ERROR: Generator failed: ${err.message}`);
    process.exit(2);
  }

  const writtenCount = generateResult.agents.length + 4; // agents + CLAUDE.md + hooks.json + .mcp.json + .lsp.json
  console.log(`Generator wrote ${writtenCount} file(s) to ${dryRunBase}`);
  if (generateResult.errors && generateResult.errors.length > 0) {
    for (const w of generateResult.errors) {
      console.warn(`  warning: ${w}`);
    }
  }

  // ---------------------------------------------------------------------------
  // Define comparison pairs: [target path, generated path, format]
  // ---------------------------------------------------------------------------

  const comparisons = [];

  // .claude/agents/*.md — compare each target agent file.
  const targetAgentsDir = path.join(targetBase, ".claude", "agents");
  const generatedAgentsDir = path.join(dryRunBase, ".claude", "agents");
  if (fs.existsSync(targetAgentsDir)) {
    for (const file of fs.readdirSync(targetAgentsDir)) {
      if (!file.endsWith(".md")) continue;
      const r = makeResult(
        `.claude/agents/${file}`,
        path.join(targetAgentsDir, file),
        path.join(generatedAgentsDir, file),
      );
      comparisons.push({ result: r, format: "markdown" });
    }
  }

  // .claude/CLAUDE.md
  comparisons.push({
    result: makeResult(
      ".claude/CLAUDE.md",
      path.join(targetBase, ".claude", "CLAUDE.md"),
      path.join(dryRunBase, ".claude", "CLAUDE.md"),
    ),
    format: "markdown",
  });

  // plugin/hooks/hooks.json — both target and generated use the same layout.
  comparisons.push({
    result: makeResult(
      "plugin/hooks/hooks.json",
      path.join(targetBase, "plugin", "hooks", "hooks.json"),
      path.join(dryRunBase, "plugin", "hooks", "hooks.json"),
    ),
    format: "json",
  });

  // .mcp.json and .lsp.json are not in the target dir (they are project-level
  // outputs, not part of the plugin bundle). Skip structural comparison here —
  // their content depends on which plugins are installed at runtime.
  // They are still generated in dry-run mode for inspection.

  // ---------------------------------------------------------------------------
  // Run comparisons
  // ---------------------------------------------------------------------------

  let failCount = 0;
  let passCount = 0;
  let skipCount = 0;

  for (const { result, format } of comparisons) {
    // Check if both files exist before comparing.
    if (!fs.existsSync(result.targetPath)) {
      console.log(`  SKIP  ${result.name} — target file not found`);
      skipCount++;
      continue;
    }
    if (!fs.existsSync(result.generatedPath)) {
      console.log(`  FAIL  ${result.name} — generated file not found`);
      failCount++;
      continue;
    }

    if (format === "json") {
      compareJson(result);
    } else {
      compareMarkdown(result);
    }

    if (result.error) {
      console.log(`  ERROR ${result.name} — ${result.error}`);
      failCount++;
    } else if (result.diffs.length > 0) {
      console.log(`  FAIL  ${result.name}`);
      for (const d of result.diffs) {
        console.log(`          ${d}`);
      }
      failCount++;
    } else {
      console.log(`  PASS  ${result.name}`);
      passCount++;
    }
  }

  // ---------------------------------------------------------------------------
  // Summary
  // ---------------------------------------------------------------------------

  console.log(`\nResults: ${passCount} passed, ${failCount} failed, ${skipCount} skipped`);

  if (failCount > 0) {
    console.log("\nGenerated files are in:", dryRunBase);
    console.log("Target files are in:   ", targetBase);
    process.exit(1);
  }

  process.exit(0);
}

main().catch((err) => {
  console.error(`Unexpected error: ${err.message}`);
  process.exit(2);
});
