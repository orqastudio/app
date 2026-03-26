#!/usr/bin/env node
/**
 * Migration script: bare types → stage-scoped types
 *
 * - type: idea → type: discovery-idea
 * - type: research → type: discovery-research
 * - type: decision → type: discovery-decision
 *
 * Only modifies the `type:` field in YAML frontmatter. Does NOT rename IDs or move files.
 */

import { readFileSync, writeFileSync, readdirSync, statSync } from 'fs';
import { join, extname } from 'path';

const ORQA_DIR = join(process.cwd(), '.orqa');

const TYPE_MAP = {
  'idea': 'discovery-idea',
  'research': 'discovery-research',
  'decision': 'discovery-decision',
};

let stats = { scanned: 0, migrated: 0, skipped: 0, errors: [] };

function walkDir(dir) {
  const entries = readdirSync(dir);
  const files = [];
  for (const entry of entries) {
    const full = join(dir, entry);
    const s = statSync(full);
    if (s.isDirectory()) {
      files.push(...walkDir(full));
    } else if (extname(entry) === '.md') {
      files.push(full);
    }
  }
  return files;
}

function migrateFile(filepath) {
  stats.scanned++;
  const content = readFileSync(filepath, 'utf-8');

  // Must start with YAML frontmatter
  if (!content.startsWith('---')) {
    return;
  }

  const fmEnd = content.indexOf('\n---', 3);
  if (fmEnd === -1) return;

  const frontmatter = content.slice(0, fmEnd);
  const rest = content.slice(fmEnd);

  // Match type: <bare-type> or type: "<bare-type>" in frontmatter
  const typeMatch = frontmatter.match(/^type:\s*"?(idea|research|decision)"?\s*$/m);
  if (!typeMatch) return;

  const oldType = typeMatch[1];
  const newType = TYPE_MAP[oldType];

  const newFrontmatter = frontmatter.replace(
    /^type:\s*"?(idea|research|decision)"?\s*$/m,
    `type: ${newType}`
  );

  const newContent = newFrontmatter + rest;
  writeFileSync(filepath, newContent, 'utf-8');
  stats.migrated++;
  console.log(`  migrated: ${filepath.replace(process.cwd(), '.')} (${oldType} → ${newType})`);
}

console.log('Type Migration: bare types → stage-scoped types');
console.log('================================================\n');

const files = walkDir(ORQA_DIR);
console.log(`Found ${files.length} markdown files in .orqa/\n`);

for (const f of files) {
  try {
    migrateFile(f);
  } catch (err) {
    stats.errors.push({ file: f, error: err.message });
    console.error(`  ERROR: ${f} — ${err.message}`);
  }
}

console.log('\n--- Results ---');
console.log(`Scanned:  ${stats.scanned}`);
console.log(`Migrated: ${stats.migrated}`);
console.log(`Errors:   ${stats.errors.length}`);

if (stats.errors.length > 0) {
  console.log('\nErrors:');
  for (const e of stats.errors) {
    console.log(`  ${e.file}: ${e.error}`);
  }
}

process.exit(stats.errors.length > 0 ? 1 : 0);
