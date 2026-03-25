/**
 * Remove stored inverse relationship entries from .orqa/ artifact frontmatter.
 *
 * Forward-only storage model: artifacts store the forward direction only.
 * The graph engine computes inverses at query time.
 *
 * Inverse types to remove:
 *   grounded-by, documented-by, upheld-by, promoted-from, driven-by, realised-by
 *
 * Preserved (NOT removed):
 *   served-by  (intentionally kept)
 */

import { readFileSync, writeFileSync, readdirSync, statSync } from 'fs';
import { join, relative } from 'path';

const INVERSE_TYPES = new Set([
  'grounded-by',
  'documented-by',
  'upheld-by',
  'promoted-from',
  'driven-by',
  'realised-by',
]);

const ROOT = join(process.cwd(), '.orqa');

// Recursively find all .md files
function findMdFiles(dir) {
  const results = [];
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const full = join(dir, entry.name);
    if (entry.isDirectory()) {
      results.push(...findMdFiles(full));
    } else if (entry.name.endsWith('.md')) {
      results.push(full);
    }
  }
  return results;
}

// Extract frontmatter boundaries (between first --- and second ---)
function parseFrontmatterBounds(content) {
  // Find the opening ---
  const openMatch = content.match(/^---\s*\r?\n/);
  if (!openMatch) return null;

  const startIdx = openMatch[0].length;

  // Find the closing ---
  const closeMatch = content.substring(startIdx).match(/\r?\n---\s*(\r?\n|$)/);
  if (!closeMatch) return null;

  const endIdx = startIdx + closeMatch.index;
  return {
    before: content.substring(0, startIdx),        // "---\n"
    yaml: content.substring(startIdx, endIdx),       // raw YAML text
    separator: closeMatch[0],                         // "\n---\n"
    after: content.substring(endIdx + closeMatch[0].length), // body
  };
}

// Parse relationships from YAML text and remove inverse entries.
// Returns { newYaml, removedCount } or null if no changes.
function removeInverseRelationships(yamlText) {
  // We need to handle the relationships array in YAML.
  // Strategy: find the relationships block, parse each entry, filter, rebuild.

  // Find the relationships: line
  const relMatch = yamlText.match(/^(relationships:\s*\r?\n)/m);
  if (!relMatch) return null;

  const relStart = relMatch.index;
  const relHeaderEnd = relStart + relMatch[0].length;

  // Collect all relationship entries (blocks starting with "  - target:")
  // Each entry is a group of lines starting with "  - target:" followed by
  // continuation lines (indented more than "  -" level, i.e. "    type:" etc.)
  const afterRel = yamlText.substring(relHeaderEnd);
  const lines = afterRel.split(/\r?\n/);

  const entries = [];
  let currentEntry = null;
  let linesConsumed = 0;

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];

    // Check if this is a new relationship entry
    if (/^\s+-\s+target:/.test(line)) {
      if (currentEntry) {
        entries.push(currentEntry);
      }
      currentEntry = { lines: [line], type: null };
    } else if (currentEntry && /^\s+type:\s*/.test(line)) {
      // This is the type field of the current entry
      const typeMatch = line.match(/type:\s*"?([^"\s]+)"?/);
      if (typeMatch) {
        currentEntry.type = typeMatch[1];
      }
      currentEntry.lines.push(line);
    } else if (currentEntry && /^\s+\S/.test(line) && !/^\s+-\s+target:/.test(line)) {
      // Continuation line (e.g., rationale:)
      currentEntry.lines.push(line);
    } else {
      // End of relationships block
      if (currentEntry) {
        entries.push(currentEntry);
        currentEntry = null;
      }
      linesConsumed = i;
      break;
    }

    if (i === lines.length - 1) {
      if (currentEntry) {
        entries.push(currentEntry);
        currentEntry = null;
      }
      linesConsumed = lines.length;
    }
  }

  // Count removals
  const kept = [];
  let removedCount = 0;

  for (const entry of entries) {
    if (entry.type && INVERSE_TYPES.has(entry.type)) {
      removedCount++;
    } else {
      kept.push(entry);
    }
  }

  if (removedCount === 0) return null;

  // Rebuild the YAML
  const beforeRel = yamlText.substring(0, relStart);
  const remainingLines = lines.slice(linesConsumed);

  let newYaml;
  if (kept.length === 0) {
    // No relationships left - remove the relationships key entirely
    // or keep it as empty array
    newYaml = beforeRel + 'relationships: []\n' +
      (remainingLines.length > 0 ? remainingLines.join('\n') : '');
  } else {
    const relBlock = kept.map(e => e.lines.join('\n')).join('\n');
    newYaml = beforeRel + 'relationships:\n' + relBlock + '\n' +
      (remainingLines.length > 0 ? remainingLines.join('\n') : '');
  }

  return { newYaml, removedCount };
}

// Main
const files = findMdFiles(ROOT);
let totalRemoved = 0;
let filesModified = 0;
const details = [];

for (const file of files) {
  const content = readFileSync(file, 'utf-8');
  const bounds = parseFrontmatterBounds(content);
  if (!bounds) continue;

  const result = removeInverseRelationships(bounds.yaml);
  if (!result) continue;

  const newContent = bounds.before + result.newYaml + bounds.separator + bounds.after;
  writeFileSync(file, newContent, 'utf-8');

  totalRemoved += result.removedCount;
  filesModified++;
  const relPath = relative(process.cwd(), file);
  details.push({ file: relPath, removed: result.removedCount });
  console.log(`  ${relPath}: removed ${result.removedCount} inverse entries`);
}

console.log(`\nSummary:`);
console.log(`  Files modified: ${filesModified}`);
console.log(`  Entries removed: ${totalRemoved}`);
console.log(`\nDetails:`);
for (const d of details) {
  console.log(`  ${d.file}: ${d.removed}`);
}
