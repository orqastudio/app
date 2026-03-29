/**
 * Shared utilities for enforcement config generator scripts.
 *
 * Provides argument parsing, recursive Markdown frontmatter scanning,
 * and file writing with dry-run support. All generators use this module
 * to share the common logic described in the generator interface (design
 * doc section 3.1).
 */

import * as fs from "node:fs";
import * as path from "node:path";

/** Parsed CLI arguments for every generator script. */
export interface GeneratorArgs {
  /** Absolute path to the project root. */
  projectRoot: string;
  /** Absolute path to write the generated config file. */
  output: string;
  /** Absolute path to the rules directory (.orqa/learning/rules/). */
  rulesDir: string;
  /**
   * When true, write to .state/dry-run/<engine>/ instead of the live output
   * path.
   */
  dryRun: boolean;
}

/**
 * A single enforcement entry extracted from a rule file's YAML frontmatter.
 * Only fields that generators need are represented here; other fields are
 * ignored.
 */
export interface EnforcementEntry {
  /** The enforcement mechanism — "mechanical", "behavioral", "hook", etc. */
  mechanism?: string;
  /** The engine this entry targets (e.g., "eslint", "clippy"). */
  engine?: string;
  /** Engine-specific rule name (e.g., "@typescript-eslint/no-explicit-any"). */
  rule?: string;
  /** Severity level — "error" or "warn". */
  severity?: string;
  /**
   * Arbitrary engine-specific options. Parsed from the `options` mapping in
   * frontmatter when present.
   */
  options?: Record<string, unknown>;
  /** Human-readable description of the enforcement requirement. */
  description?: string;
  /** Arbitrary additional key/value pairs from the frontmatter entry. */
  [key: string]: unknown;
}

/** A parsed rule file including its id, title, and enforcement entries. */
export interface ParsedRule {
  /** Rule identifier (e.g., "RULE-abc12345"). */
  id: string;
  /** Rule title. */
  title: string;
  /** All enforcement entries from the frontmatter array. */
  enforcement: EnforcementEntry[];
}

/**
 * Parse CLI arguments from process.argv using the standard generator interface.
 *
 * Required flags: --project-root, --output, --rules-dir
 * Optional flag:  --dry-run
 *
 * Exits with code 1 and a usage message when required arguments are missing.
 */
export function parseArgs(): GeneratorArgs {
  const argv = process.argv.slice(2);
  const get = (flag: string): string | undefined => {
    const idx = argv.indexOf(flag);
    if (idx === -1) return undefined;
    return argv[idx + 1];
  };

  const projectRoot = get("--project-root");
  const output = get("--output");
  const rulesDir = get("--rules-dir");
  const dryRun = argv.includes("--dry-run");

  if (!projectRoot || !output || !rulesDir) {
    process.stderr.write(
      "Usage: <generator> --project-root <path> --output <path> --rules-dir <path> [--dry-run]\n",
    );
    process.exit(1);
  }

  return { projectRoot, output, rulesDir, dryRun };
}

/**
 * Recursively scan all .md files under rulesDir and return parsed rules.
 *
 * Files without valid YAML frontmatter are silently skipped. Files whose
 * status field is anything other than "active" (or absent) are skipped.
 */
export function scanRules(rulesDir: string): ParsedRule[] {
  if (!fs.existsSync(rulesDir)) return [];

  const results: ParsedRule[] = [];
  collectMdFiles(rulesDir).forEach((filePath) => {
    const rule = parseRuleFile(filePath);
    if (rule) results.push(rule);
  });
  return results;
}

/**
 * Filter parsed rules to those that have at least one enforcement entry
 * targeting the specified engine.
 */
export function filterByEngine(
  rules: ParsedRule[],
  engine: string,
): ParsedRule[] {
  return rules
    .map((rule) => ({
      ...rule,
      enforcement: rule.enforcement.filter((e) => e.engine === engine),
    }))
    .filter((rule) => rule.enforcement.length > 0);
}

/**
 * Resolve the effective output path, respecting --dry-run.
 *
 * When dryRun is true, the output is redirected to
 * .state/dry-run/<engine>/<basename> under the project root. The directory
 * is created if it does not exist.
 */
export function resolveOutputPath(
  args: GeneratorArgs,
  engine: string,
): string {
  if (!args.dryRun) return args.output;

  const dryRunDir = path.join(args.projectRoot, ".state", "dry-run", engine);
  fs.mkdirSync(dryRunDir, { recursive: true });
  return path.join(dryRunDir, path.basename(args.output));
}

/**
 * Write content to outputPath, creating parent directories as needed.
 *
 * Logs the path written to stderr so the daemon can capture it.
 * Exits with code 1 on write failure.
 */
export function writeOutput(outputPath: string, content: string): void {
  try {
    fs.mkdirSync(path.dirname(outputPath), { recursive: true });
    fs.writeFileSync(outputPath, content, "utf-8");
    process.stderr.write(`[generator] wrote ${outputPath}\n`);
  } catch (err) {
    process.stderr.write(`[generator] failed to write ${outputPath}: ${err}\n`);
    process.exit(1);
  }
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/**
 * Collect all .md file paths recursively under a directory.
 */
function collectMdFiles(dir: string): string[] {
  const result: string[] = [];
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    const full = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      result.push(...collectMdFiles(full));
    } else if (entry.isFile() && entry.name.endsWith(".md")) {
      result.push(full);
    }
  }
  return result;
}

/**
 * Parse a single Markdown file and extract its frontmatter as a ParsedRule.
 *
 * Returns null when:
 * - The file has no YAML frontmatter block
 * - The frontmatter has no `id` field
 * - The `status` field is present and not "active"
 * - The `enforcement` array is absent or empty
 */
function parseRuleFile(filePath: string): ParsedRule | null {
  const content = fs.readFileSync(filePath, "utf-8");

  // Extract the frontmatter block between the first pair of --- delimiters.
  const match = content.match(/^---\r?\n([\s\S]*?)\r?\n---/);
  if (!match || !match[1]) return null;
  const yaml = match[1];

  const id = extractScalar(yaml, "id");
  if (!id) return null;

  const status = extractScalar(yaml, "status");
  if (status && status !== "active") return null;

  const title = extractScalar(yaml, "title") ?? id;

  const enforcement = parseEnforcementArray(yaml);
  if (enforcement.length === 0) return null;

  return { id, title, enforcement };
}

/**
 * Extract a scalar string value for a top-level YAML key.
 *
 * Handles both bare values and quoted values. Returns undefined when the key
 * is absent.
 */
function extractScalar(yaml: string, key: string): string | undefined {
  const re = new RegExp(`^${key}:\\s*(.+)`, "m");
  const m = yaml.match(re);
  if (!m || !m[1]) return undefined;
  return m[1].trim().replace(/^["']|["']$/g, "");
}

/**
 * Parse the `enforcement:` array from a YAML block.
 *
 * This is a hand-rolled parser sufficient for the rule frontmatter format.
 * It handles:
 * - Block-style list items starting with `  - `
 * - Scalar key: value pairs within each item
 * - Nested `options:` mappings (one level deep)
 *
 * It does NOT handle multi-line values or anchors. Rule files are expected to
 * keep enforcement entries simple.
 */
function parseEnforcementArray(yaml: string): EnforcementEntry[] {
  const enforcementIdx = yaml.indexOf("enforcement:");
  if (enforcementIdx === -1) return [];

  // Everything after "enforcement:" up to the next top-level key (no indent).
  const afterKey = yaml.slice(enforcementIdx + "enforcement:".length);

  // Split on list item markers at 2-space indentation.
  const rawItems = afterKey.split(/\n(?=\s{0,2}-\s)/);

  const entries: EnforcementEntry[] = [];

  for (const raw of rawItems) {
    if (!raw.trim().startsWith("-")) continue;

    // Stop when we hit the next top-level key (no leading spaces, not a list).
    if (/^\S/.test(raw) && !raw.trimStart().startsWith("-")) break;

    const entry: EnforcementEntry = {};
    let inOptions = false;
    const lines = raw.split("\n");

    for (const line of lines) {
      if (!line.trim()) continue;

      // Detect options: sub-block start
      if (/^\s{4,}options:\s*$/.test(line) || /^\s{2,}options:\s*$/.test(line)) {
        inOptions = true;
        entry.options = {};
        continue;
      }

      // Parse key: value pairs
      const kv = line.match(/^(\s*)(\w[\w-]*):\s*(.*)/);
      if (!kv) continue;

      const indent = kv[1]?.length ?? 0;
      const key = kv[2] ?? "";
      const val = (kv[3] ?? "").trim().replace(/^["']|["']$/g, "");

      if (indent >= 6 && inOptions && entry.options) {
        // Nested option under options:
        entry.options[key] = parseScalarValue(val);
        continue;
      }

      // Any line at base indent resets options context
      if (indent <= 4) inOptions = false;

      if (key && val !== undefined) {
        entry[key] = parseScalarValue(val);
      }
    }

    // Only include entries that have at least one meaningful field.
    if (Object.keys(entry).length > 0) {
      entries.push(entry);
    }
  }

  return entries;
}

/**
 * Convert a YAML scalar string to its JavaScript primitive equivalent.
 *
 * Handles booleans, integers, and strings. Everything else is left as a
 * string.
 */
function parseScalarValue(val: string): unknown {
  if (val === "true") return true;
  if (val === "false") return false;
  if (/^\d+$/.test(val)) return parseInt(val, 10);
  return val;
}
