#!/usr/bin/env node
// Validates YAML frontmatter of .orqa/ markdown files against JSON Schema.
// Schemas are discovered via .orqa/project.json artifact config.
//
// Usage: node validate-schema.mjs <file1.md> [file2.md ...]
// Exit code 0 = all valid, 1 = validation errors found.

import { readFileSync } from "fs";
import { resolve, dirname, relative } from "path";
import Ajv from "ajv";
import addFormats from "ajv-formats";
import yaml from "yaml";

const ROOT = resolve(import.meta.dirname, "..");
const CONFIG_PATH = resolve(ROOT, ".orqa/project.json");

// Parse YAML frontmatter from a markdown file (first --- block only)
function parseFrontmatter(content) {
  // Normalize line endings — CRLF breaks YAML date/number parsing
  const normalized = content.replace(/\r\n/g, "\n").replace(/\r/g, "\n");
  const lines = normalized.split("\n");
  if (lines[0]?.trim() !== "---") return null;
  for (let i = 1; i < lines.length; i++) {
    if (lines[i].trim() === "---") {
      const yamlBlock = lines.slice(1, i).join("\n");
      try {
        return yaml.parse(yamlBlock);
      } catch {
        return null;
      }
    }
  }
  return null;
}

// Collect all leaf artifact paths from the config tree
function collectPaths(entries) {
  const paths = [];
  for (const entry of entries) {
    if (entry.path) paths.push(entry.path);
    if (entry.children) paths.push(...collectPaths(entry.children));
  }
  return paths;
}

// Find which artifact directory a file belongs to
function findArtifactDir(filePath, artifactPaths) {
  const rel = relative(ROOT, resolve(filePath)).replace(/\\/g, "/");
  // Match the longest (most specific) artifact path
  let best = null;
  for (const ap of artifactPaths) {
    if (rel.startsWith(ap + "/") || rel.startsWith(ap + "\\")) {
      if (!best || ap.length > best.length) best = ap;
    }
  }
  return best;
}

// Load schema.json from an artifact directory (returns null if none exists)
function loadSchema(artifactPath) {
  try {
    const schemaPath = resolve(ROOT, artifactPath, "schema.json");
    return JSON.parse(readFileSync(schemaPath, "utf-8"));
  } catch {
    return null;
  }
}

// Main
const files = process.argv.slice(2);
if (files.length === 0) {
  process.exit(0);
}

const config = JSON.parse(readFileSync(CONFIG_PATH, "utf-8"));
const artifactPaths = collectPaths(config.artifacts);

const ajv = new Ajv({ allErrors: true, strict: false });
addFormats(ajv);

// Cache compiled validators per artifact path
const validators = new Map();

let errors = 0;

for (const file of files) {
  // Skip READMEs and non-md files
  if (file.endsWith("README.md") || !file.endsWith(".md")) continue;

  const content = readFileSync(resolve(ROOT, file), "utf-8");
  const frontmatter = parseFrontmatter(content);

  if (!frontmatter) {
    // No frontmatter — skip (could be a plain doc)
    continue;
  }

  const artifactDir = findArtifactDir(file, artifactPaths);
  if (!artifactDir) {
    // File not under a registered artifact directory — skip
    continue;
  }

  // Get or compile validator for this directory
  if (!validators.has(artifactDir)) {
    const schema = loadSchema(artifactDir);
    if (schema) {
      validators.set(artifactDir, ajv.compile(schema));
    } else {
      validators.set(artifactDir, null);
    }
  }

  const validate = validators.get(artifactDir);
  if (!validate) {
    // No schema for this directory — skip
    continue;
  }

  // YAML parses dates as Date objects — convert to ISO strings for validation
  const normalized = JSON.parse(
    JSON.stringify(frontmatter, (_, v) =>
      v instanceof Date ? v.toISOString().split("T")[0] : v
    )
  );

  const valid = validate(normalized);
  if (!valid) {
    for (const err of validate.errors) {
      const path = err.instancePath || "(root)";
      let msg = err.message;
      if (err.keyword === "additionalProperties") {
        msg = `unknown field '${err.params.additionalProperty}'`;
      } else if (err.keyword === "enum") {
        msg = `${msg} (${err.params.allowedValues.join(", ")})`;
      }
      console.error(`ERROR: ${file} ${path} — ${msg}`);
    }
    errors += validate.errors.length;
  }

  // Body template validation: check required sections exist in the markdown body
  const bodySchema = loadSchema(artifactDir);
  if (bodySchema?.bodyTemplate?.sections) {
    // Extract body (everything after closing ---)
    const normalizedContent = content.replace(/\r\n/g, "\n").replace(/\r/g, "\n");
    const lines = normalizedContent.split("\n");
    let bodyStart = -1;
    let delimCount = 0;
    for (let i = 0; i < lines.length; i++) {
      if (lines[i].trim() === "---") {
        delimCount++;
        if (delimCount === 2) {
          bodyStart = i + 1;
          break;
        }
      }
    }
    if (bodyStart > 0) {
      const body = lines.slice(bodyStart).join("\n");
      const requiredSections = bodySchema.bodyTemplate.sections
        .filter((s) => s.required)
        .map((s) => s.heading);
      for (const heading of requiredSections) {
        // Match ## Heading (case-sensitive, at start of line)
        const pattern = new RegExp(`^## ${heading.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}\\s*$`, "m");
        if (!pattern.test(body)) {
          console.error(`ERROR: ${file} — missing required section "## ${heading}"`);
          errors++;
        }
      }
    }
  }

  // Field order validation: frontmatter keys must follow schema propertyOrder
  const schema = loadSchema(artifactDir);
  const canonicalOrder = schema?.propertyOrder ?? (schema?.properties ? Object.keys(schema.properties) : null);
  if (canonicalOrder) {
    const fileKeys = Object.keys(frontmatter);
    const fileKeysInSchema = fileKeys.filter((k) => canonicalOrder.includes(k));
    const expectedOrder = canonicalOrder.filter((k) => fileKeysInSchema.includes(k));
    if (JSON.stringify(fileKeysInSchema) !== JSON.stringify(expectedOrder)) {
      console.error(`ERROR: ${file} — field order does not match schema propertyOrder`);
      console.error(`  expected: ${expectedOrder.join(", ")}`);
      console.error(`  actual:   ${fileKeysInSchema.join(", ")}`);
      errors++;
    }
  }
}

if (errors > 0) {
  console.error(
    `\nSchema validation failed: ${errors} error(s) found.`
  );
  console.error(
    "Schemas defined in schema.json files within each artifact directory."
  );
  process.exit(1);
}
