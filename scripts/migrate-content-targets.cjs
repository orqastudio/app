#!/usr/bin/env node
// One-shot migration: renames `content.*.target` (path string) to `content.*.installPath`
// and adds a new `content.*.target: "surrealdb" | "runtime"` enum field in every
// orqa-plugin.json under the `plugins/` and `connectors/` directories.
//
// Classification rule (per S2-08 Bobbi decision, 2026-04-17):
//   installPath starts with .orqa/<artifact-dir>/  -> target: "surrealdb"
//   everything else                                -> target: "runtime"
//
// Implementation: surgical line-by-line regex rewrite that preserves all existing
// indentation, whitespace, and inline-array formatting. Does NOT re-serialise the
// whole JSON — only replaces the content.*.target lines that hold path values.
//
// Run once from the repo root: node scripts/migrate-content-targets.cjs
// Idempotent: if the new `target` field is already a string enum value, that line is skipped.

const fs = require('fs');
const path = require('path');

const SURREALDB_PREFIXES = [
  '.orqa/documentation',
  '.orqa/learning',
  '.orqa/planning',
  '.orqa/workflows',
  '.orqa/implementation',
  '.orqa/discovery',
];

function classifyInstallPath(installPath) {
  if (!installPath) return 'runtime';
  return SURREALDB_PREFIXES.some(p => installPath.startsWith(p)) ? 'surrealdb' : 'runtime';
}

function findManifests(rootDir) {
  const results = [];
  function walk(dir) {
    for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
      const full = path.join(dir, entry.name);
      if (entry.isDirectory()) {
        walk(full);
      } else if (entry.isFile() && entry.name === 'orqa-plugin.json') {
        results.push(full);
      }
    }
  }
  walk(rootDir);
  return results;
}

// Matches a `"target": "<value>"` line where <value> is a path (not an enum).
// Captures: (indent)(value)(optional trailing comma).
// We match lines where the value is NOT "surrealdb" or "runtime" (i.e. it's an old path).
const TARGET_LINE_RE = /^(\s*)"target"\s*:\s*"([^"]+)"(,?\s*)$/;

function migrateManifest(manifestPath) {
  const raw = fs.readFileSync(manifestPath, 'utf8');
  const lines = raw.split('\n');

  // We need to track context: are we inside a content entry block?
  // Strategy: scan each line for the target pattern, check if it's a path value.
  // A path value does NOT equal "surrealdb" or "runtime".
  // When we find such a line, replace it with two lines: installPath + target enum.

  let changed = false;
  const summary = [];

  // We operate in two passes:
  // Pass 1: detect if this manifest is inside a `"content"` block when we see target lines.
  // Simple approach: track brace depth from the `"content":` key and only rewrite
  // target lines whose value looks like a path (contains "/" or starts with ".").

  // Track whether we're inside the content object at any depth.
  let inContent = false;
  let contentDepth = 0;
  let currentBraceDepth = 0;

  const newLines = [];

  for (const line of lines) {
    // Count brace depth changes.
    const openBraces = (line.match(/\{/g) || []).length;
    const closeBraces = (line.match(/\}/g) || []).length;

    // Check if this line opens the "content" object.
    if (!inContent && /^\s*"content"\s*:\s*\{/.test(line)) {
      inContent = true;
      contentDepth = currentBraceDepth + openBraces - closeBraces;
      newLines.push(line);
      currentBraceDepth = currentBraceDepth + openBraces - closeBraces;
      continue;
    }

    if (inContent) {
      const match = TARGET_LINE_RE.exec(line);
      if (match) {
        const indent = match[1];
        const pathValue = match[2];
        const trailing = match[3]; // optional trailing comma

        // Only rewrite if this is a path value (not already an enum).
        if (pathValue !== 'surrealdb' && pathValue !== 'runtime') {
          const enumTarget = classifyInstallPath(pathValue);
          // Replace with: "installPath": "<path>",\n<indent>"target": "<enum>"<trailing>
          newLines.push(`${indent}"installPath": "${pathValue}",`);
          newLines.push(`${indent}"target": "${enumTarget}"${trailing}`);
          changed = true;
          summary.push(`  installPath=${pathValue}, target=${enumTarget}`);
          currentBraceDepth = currentBraceDepth + openBraces - closeBraces;
          continue;
        }
      }

      // Check if we've closed out of the content block.
      currentBraceDepth = currentBraceDepth + openBraces - closeBraces;
      if (currentBraceDepth < contentDepth) {
        inContent = false;
      }
    } else {
      currentBraceDepth = currentBraceDepth + openBraces - closeBraces;
    }

    newLines.push(line);
  }

  if (changed) {
    fs.writeFileSync(manifestPath, newLines.join('\n'), 'utf8');
  }

  return { path: manifestPath, changed, summary };
}

// --- Main ---
const repoRoot = path.resolve(__dirname, '..');

const searchDirs = [
  path.join(repoRoot, 'plugins'),
  path.join(repoRoot, 'connectors'),
];

const manifests = [];
for (const dir of searchDirs) {
  if (fs.existsSync(dir)) {
    manifests.push(...findManifests(dir));
  }
}

console.log(`Found ${manifests.length} manifests to process.\n`);

let totalChanged = 0;
let totalSurrealdb = 0;
let totalRuntime = 0;

for (const mPath of manifests) {
  const rel = path.relative(repoRoot, mPath);
  const result = migrateManifest(mPath);

  if (result.changed) {
    console.log(`UPDATED: ${rel}`);
    totalChanged++;
  } else {
    console.log(`SKIPPED: ${rel} (already migrated or no content entries)`);
  }

  if (result.summary) {
    for (const line of result.summary) {
      const kind = line.includes('target=surrealdb') ? 'surrealdb' : line.includes('target=runtime') ? 'runtime' : null;
      if (kind === 'surrealdb') totalSurrealdb++;
      else if (kind === 'runtime') totalRuntime++;
      console.log(line);
    }
  }
  console.log();
}

console.log(`\nSummary: ${totalChanged} manifests updated, ${totalSurrealdb} surrealdb entries, ${totalRuntime} runtime entries.`);
