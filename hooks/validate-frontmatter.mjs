#!/usr/bin/env node
// validate-frontmatter.mjs — JSON Schema validation for artifact frontmatter.
//
// Reads schemas from installed plugin manifests (orqa-plugin.json), matches
// staged artifacts to their schema by ID prefix, validates frontmatter with ajv.
//
// Usage:
//   node validate-frontmatter.mjs                    # validate all staged .md files
//   node validate-frontmatter.mjs --all              # validate all .md files in .orqa/ and plugins/
//   node validate-frontmatter.mjs path/to/file.md    # validate specific file

import { readFileSync, readdirSync, existsSync } from "fs";
import { join, relative } from "path";
import { execSync } from "child_process";
import matter from "gray-matter";
import Ajv from "ajv";
import addFormats from "ajv-formats";

const ajv = new Ajv({ allErrors: true, strict: false });
addFormats(ajv);

// ---------------------------------------------------------------------------
// Schema loading from plugin manifests
// ---------------------------------------------------------------------------

function loadSchemas(projectRoot) {
  const schemas = new Map(); // idPrefix → { key, compiled ajv validator, raw schema }

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

      for (const schema of manifest.provides?.schemas || []) {
        if (!schema.key || !schema.idPrefix) continue;

        // Build JSON Schema from the plugin manifest schema definition
        const jsonSchema = buildJsonSchema(schema);
        const validate = ajv.compile(jsonSchema);

        schemas.set(schema.idPrefix, {
          key: schema.key,
          validate,
          jsonSchema,
          pluginName: manifest.name || entry.name,
        });
      }
    }
  }

  return schemas;
}

function buildJsonSchema(pluginSchema) {
  // The manifest's frontmatter field is already a JSON Schema object.
  // Clone it and enrich with auto-derived id pattern and status enum.
  const fm = pluginSchema.frontmatter || {};
  const schema = {
    type: "object",
    additionalProperties: true,
    ...fm,
  };

  // Ensure properties exists
  if (!schema.properties) schema.properties = {};

  // Auto-derive id pattern if not explicitly declared
  if (!schema.properties.id) {
    schema.properties.id = {
      type: "string",
      pattern: `^${pluginSchema.idPrefix}-[a-f0-9]{8}$`,
    };
  }

  // Auto-derive status enum from statusTransitions keys if not explicitly declared
  if (!schema.properties.status && pluginSchema.statusTransitions && Object.keys(pluginSchema.statusTransitions).length > 0) {
    schema.properties.status = {
      type: "string",
      enum: Object.keys(pluginSchema.statusTransitions).sort(),
    };
  }

  // Ensure id is always required
  if (!schema.required) schema.required = [];
  if (!schema.required.includes("id")) schema.required.unshift("id");

  return schema;
}

// ---------------------------------------------------------------------------
// File discovery
// ---------------------------------------------------------------------------

function getStagedFiles() {
  try {
    const output = execSync("git diff --cached --name-only --diff-filter=ACMR -- '*.md'", {
      encoding: "utf-8",
    });
    return output.split("\n").filter(Boolean);
  } catch {
    return [];
  }
}

function getAllArtifactFiles(projectRoot) {
  const files = [];
  const dirs = [
    join(projectRoot, ".orqa"),
    join(projectRoot, "plugins"),
    join(projectRoot, "connectors"),
  ];

  function walk(dir) {
    if (!existsSync(dir)) return;
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      if (["node_modules", "dist", "target", ".git"].includes(entry.name)) continue;
      const full = join(dir, entry.name);
      if (entry.isDirectory()) walk(full);
      else if (entry.name.endsWith(".md")) files.push(relative(projectRoot, full));
    }
  }

  for (const d of dirs) walk(d);
  return files;
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

function main() {
  const args = process.argv.slice(2);
  const all = args.includes("--all");
  const specificFile = args.find(a => !a.startsWith("--"));

  // Find project root
  let projectRoot = process.cwd();
  while (projectRoot !== "/" && !existsSync(join(projectRoot, ".orqa"))) {
    projectRoot = join(projectRoot, "..");
  }

  const schemas = loadSchemas(projectRoot);
  if (schemas.size === 0) {
    console.log("No plugin schemas found — nothing to validate.");
    process.exit(0);
  }

  // Get files to validate
  let files;
  if (specificFile) {
    files = [specificFile];
  } else if (all) {
    files = getAllArtifactFiles(projectRoot);
  } else {
    files = getStagedFiles();
  }

  if (files.length === 0) {
    process.exit(0);
  }

  let errorCount = 0;
  let validated = 0;

  for (const file of files) {
    const fullPath = join(projectRoot, file);
    if (!existsSync(fullPath)) continue;

    let parsed;
    try {
      parsed = matter(readFileSync(fullPath, "utf-8"));
    } catch {
      continue;
    }

    const data = parsed.data;
    if (!data.id) continue;

    // Match by ID prefix
    const prefix = data.id.match(/^([A-Z]+)-/)?.[1];
    if (!prefix) continue;

    const schema = schemas.get(prefix);
    if (!schema) continue; // No schema for this type — skip

    validated++;
    const valid = schema.validate(data);

    if (!valid) {
      errorCount++;
      console.error(`\n${file} (${schema.key}):`);
      for (const err of schema.validate.errors) {
        const path = err.instancePath ? ` ${err.instancePath}` : "";
        console.error(`  ERROR:${path} ${err.message}`);
      }
    }
  }

  if (errorCount > 0) {
    console.error(`\n${errorCount} file(s) failed schema validation (${validated} checked).`);
    process.exit(1);
  }

  if (validated > 0 && !specificFile) {
    console.log(`${validated} artifact(s) validated against plugin schemas.`);
  }
}

main();
