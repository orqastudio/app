#!/usr/bin/env node
// validate-relationships.mjs — Validates relationship types in .orqa/ artifact frontmatter.
//
// Reads valid relationship types from installed plugin manifests (orqa-plugin.json).
// Each relationship entry in frontmatter must have a `type` field whose value matches
// a `key` or `inverse` from a plugin's `provides.relationships[]`.
//
// Usage:
//   node validate-relationships.mjs <file1.md> [file2.md ...]
//   node validate-relationships.mjs --all
// Exit code 0 = all valid, 1 = validation errors found.

import { readFileSync, readdirSync, existsSync } from "fs";
import { resolve, join, relative } from "path";
import { createRequire } from "module";
import { parseFrontmatter } from "../tools/lib/parse-artifact.mjs";

const ROOT = resolve(import.meta.dirname, "..");

// ---------------------------------------------------------------------------
// Relationship vocabulary loading from plugin manifests
// ---------------------------------------------------------------------------

function loadRelationshipVocabulary(projectRoot) {
  const vocab = new Map(); // type key → { plugin, label, inverse, from, to, semantic }

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

      const pluginName = manifest.name || entry.name;

      for (const rel of manifest.provides?.relationships || []) {
        if (!rel.key) continue;

        // Register the forward key
        vocab.set(rel.key, {
          plugin: pluginName,
          label: rel.label || rel.key,
          inverse: rel.inverse || null,
          from: rel.from || [],
          to: rel.to || [],
          semantic: rel.semantic || "unknown",
        });

        // Register the inverse key
        if (rel.inverse) {
          vocab.set(rel.inverse, {
            plugin: pluginName,
            label: rel.inverseLabel || rel.inverse,
            inverse: rel.key,
            from: rel.to || [],
            to: rel.from || [],
            semantic: rel.semantic || "unknown",
          });
        }
      }
    }
  }

  return vocab;
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

const vocab = loadRelationshipVocabulary(ROOT);

if (vocab.size === 0) {
  console.log("No relationship types found in plugin manifests — nothing to validate.");
  process.exit(0);
}

let errors = 0;

for (const file of files) {
  if (!file.endsWith(".md")) continue;

  const absFile = resolve(ROOT, file);
  if (!existsSync(absFile)) continue;

  const frontmatter = parseFrontmatter(absFile);
  if (!frontmatter) continue;

  const relationships = frontmatter.relationships;
  if (!Array.isArray(relationships) || relationships.length === 0) continue;

  for (let i = 0; i < relationships.length; i++) {
    const rel = relationships[i];

    // Each relationship must be an object with target and type
    if (!rel || typeof rel !== "object") {
      console.error(`ERROR: ${file} relationships[${i}] — not an object`);
      errors++;
      continue;
    }

    if (!rel.target) {
      console.error(`ERROR: ${file} relationships[${i}] — missing 'target' field`);
      errors++;
    }

    if (!rel.type) {
      console.error(`ERROR: ${file} relationships[${i}] — missing 'type' field`);
      errors++;
      continue;
    }

    if (!vocab.has(rel.type)) {
      const validTypes = [...vocab.keys()].sort().join(", ");
      console.error(
        `ERROR: ${file} relationships[${i}].type — invalid type '${rel.type}'\n` +
        `  Valid types: ${validTypes}`
      );
      errors++;
    }
  }
}

if (errors > 0) {
  console.error(`\nRelationship validation failed: ${errors} error(s) found.`);
  console.error("Relationship types are defined in plugin manifests (provides.relationships[]).");
  process.exit(1);
}
