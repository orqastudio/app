#!/usr/bin/env node
/**
 * Artifact Rebuild Script — Graph-First Architecture Migration
 *
 * Reads from .orqa-backup/, transforms frontmatter (standalone fields → relationships,
 * old vocabulary → canonical vocabulary), and writes to .orqa/ new structure.
 *
 * Usage:
 *   node scripts/rebuild-artifacts.mjs              # Full run
 *   node scripts/rebuild-artifacts.mjs --dry-run    # Show what would change
 */

import { readFileSync, writeFileSync, readdirSync, statSync, existsSync, mkdirSync } from "fs";
import { join, relative, basename, dirname } from "path";

const ROOT = process.cwd();
const BACKUP = join(ROOT, ".orqa-backup");
const TARGET = join(ROOT, ".orqa");
const DRY_RUN = process.argv.includes("--dry-run");

// ---------------------------------------------------------------------------
// Vocabulary mapping: old relationship type → canonical type
// ---------------------------------------------------------------------------
const RELATIONSHIP_TYPE_MAP = {
  "belongs-to": "delivers",
  "contains": "delivered-by",
  "documents": "informs",
  "documented-by": "informed-by",
  "practices": "grounded-by",
  "practiced-by": "grounded",
  "verifies": "enforces",
  "verified-by": "enforced-by",
  "scoped-to": "enforces",
  "scoped-by": "enforced-by",
};

// ---------------------------------------------------------------------------
// Standalone field → relationship conversion
// ---------------------------------------------------------------------------
const SINGLE_FIELD_TO_REL = {
  milestone: { type: "delivers", targetType: "milestone" },
  epic: { type: "delivers", targetType: "epic" },
  "promoted-to": { type: "evolves-into" },
  supersedes: { type: "evolves-into" },
  "superseded-by": { type: "evolves-from" },
  "surpassed-by": { type: "evolves-from" },
  "spun-off-from": { type: "evolves-from" },
  "promoted-from": { type: "evolves-from" },
};

const ARRAY_FIELD_TO_REL = {
  "depends-on": { type: "depends-on" },
  blocks: { type: "depended-on-by" },
  pillars: { type: "grounded-by" },
  "research-refs": { type: "informed-by" },
  "docs-required": { type: "informs" },
  "docs-produced": { type: "informed-by" },
  skills: { type: "grounded-by" },
};

// Fields to remove from frontmatter after converting to relationships
const FIELDS_TO_REMOVE = new Set([
  "milestone", "epic", "promoted-to", "supersedes", "superseded-by",
  "surpassed-by", "spun-off-from", "promoted-from",
  "depends-on", "blocks", "pillars", "research-refs",
  "docs-required", "docs-produced", "skills",
]);

// Fields to preserve as-is in frontmatter
const PRESERVE_FIELDS = new Set([
  "id", "title", "description", "status", "priority", "created", "updated",
  "assignee", "acceptance", "scoring", "horizon", "maturity", "recurrence",
  "category", "version", "gate", "layer", "model", "enforcement",
  "rule-overrides", "bodyTemplate", "tools", "capabilities",
  "user-invocable", "research-needed", "deadline",
]);

// ---------------------------------------------------------------------------
// Path mapping: backup path → new path
// ---------------------------------------------------------------------------
function mapPath(backupRelPath) {
  // delivery/ideas/ → discovery/ideas/
  if (backupRelPath.startsWith("delivery/ideas/"))
    return backupRelPath.replace("delivery/ideas/", "discovery/ideas/");

  // delivery/research/ → discovery/research/
  if (backupRelPath.startsWith("delivery/research/"))
    return backupRelPath.replace("delivery/research/", "discovery/research/");

  // process/pillars/ → principles/pillars/
  if (backupRelPath.startsWith("process/pillars/"))
    return backupRelPath.replace("process/pillars/", "principles/pillars/");

  // documentation/wireframes/ → discovery/wireframes/
  if (backupRelPath.startsWith("documentation/wireframes/"))
    return backupRelPath.replace("documentation/wireframes/", "discovery/wireframes/");

  // documentation/grounding/ → principles/grounding/
  if (backupRelPath.startsWith("documentation/grounding/"))
    return backupRelPath.replace("documentation/grounding/", "principles/grounding/");

  // documentation/about/vision.md → principles/vision/vision.md (single file)
  if (backupRelPath === "documentation/about/vision.md")
    return "principles/vision/vision.md";

  // documentation/about/personas.md → principles/personas/personas.md
  if (backupRelPath === "documentation/about/personas.md")
    return "principles/personas/personas.md";

  // documentation/about/ → documentation/platform/ (other about files)
  if (backupRelPath.startsWith("documentation/about/"))
    return backupRelPath.replace("documentation/about/", "documentation/platform/");

  // documentation/reference/ → documentation/platform/
  if (backupRelPath.startsWith("documentation/reference/"))
    return backupRelPath.replace("documentation/reference/", "documentation/platform/");

  // documentation/development/ → check if platform or project
  // Platform-level dev docs: enforcement, orchestration, plugin-architecture, tool-definitions, ipc-commands
  if (backupRelPath.startsWith("documentation/development/")) {
    const fname = basename(backupRelPath);
    const platformDocs = [
      "enforcement.md", "orchestration.md", "plugin-architecture.md",
      "tool-definitions.md", "ipc-commands.md",
    ];
    if (platformDocs.includes(fname))
      return backupRelPath.replace("documentation/development/", "documentation/platform/");
    return backupRelPath.replace("documentation/development/", "documentation/project/");
  }

  // documentation/how-to/ → documentation/project/
  if (backupRelPath.startsWith("documentation/how-to/"))
    return backupRelPath.replace("documentation/how-to/", "documentation/project/");

  // documentation/product/ → documentation/project/
  if (backupRelPath.startsWith("documentation/product/"))
    return backupRelPath.replace("documentation/product/", "documentation/project/");

  // documentation/ui/ → documentation/project/
  if (backupRelPath.startsWith("documentation/ui/"))
    return backupRelPath.replace("documentation/ui/", "documentation/project/");

  // documentation/architecture/ → documentation/project/
  if (backupRelPath.startsWith("documentation/architecture/"))
    return backupRelPath.replace("documentation/architecture/", "documentation/project/");

  // documentation/marketing/ → documentation/project/
  if (backupRelPath.startsWith("documentation/marketing/"))
    return backupRelPath.replace("documentation/marketing/", "documentation/project/");

  // Everything else stays in the same relative position
  return backupRelPath;
}

// ---------------------------------------------------------------------------
// YAML frontmatter parser (simple — handles the standard format)
// ---------------------------------------------------------------------------
function parseFrontmatter(content) {
  const match = content.match(/^---\n([\s\S]*?)\n---\n?([\s\S]*)$/);
  if (!match) return { frontmatter: null, body: content, raw: null };
  return { raw: match[1], body: match[2], frontmatter: parseYaml(match[1]) };
}

function parseYaml(text) {
  // Simple YAML parser for our frontmatter format
  // Uses a line-by-line approach that handles our artifact format
  const lines = text.split("\n");
  const result = {};
  let currentKey = null;
  let currentValue = null;
  let inArray = false;
  let inMultiline = false;
  let arrayItems = [];
  let multilineText = "";
  let indent = 0;

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];

    // Handle relationships array specially — keep as raw YAML
    if (line.match(/^relationships:/)) {
      // Flush previous key
      if (currentKey !== null) {
        if (inArray) result[currentKey] = arrayItems;
        else if (inMultiline) result[currentKey] = multilineText.trimEnd();
        else result[currentKey] = currentValue;
      }
      currentKey = "relationships";
      inArray = false;
      inMultiline = false;
      // Collect all relationship entries
      const rels = [];
      let j = i + 1;
      while (j < lines.length) {
        const relLine = lines[j];
        if (relLine.match(/^  - target:/)) {
          const rel = { target: relLine.match(/target:\s*(.+)/)?.[1]?.trim() };
          j++;
          while (j < lines.length && lines[j].match(/^\s{4}/)) {
            const kv = lines[j].match(/^\s+(\w[\w-]*):\s*(.+)/);
            if (kv) rel[kv[1]] = kv[2].trim();
            j++;
          }
          rels.push(rel);
        } else if (relLine.match(/^\s*-/) || relLine.match(/^\s+/)) {
          j++;
        } else {
          break;
        }
      }
      result.relationships = rels;
      currentKey = null;
      currentValue = null;
      i = j - 1;
      continue;
    }

    // Top-level key: value
    const kvMatch = line.match(/^([\w][\w-]*):\s*(.*)/);
    if (kvMatch) {
      // Flush previous
      if (currentKey !== null) {
        if (inArray) result[currentKey] = arrayItems;
        else if (inMultiline) result[currentKey] = multilineText.trimEnd();
        else result[currentKey] = currentValue;
      }

      currentKey = kvMatch[1];
      const val = kvMatch[2].trim();

      inArray = false;
      inMultiline = false;
      arrayItems = [];
      multilineText = "";

      if (val === "" || val === "[]") {
        // Could be array or object start
        currentValue = val === "[]" ? [] : null;
        // Check if next line is an array item
        if (i + 1 < lines.length && lines[i + 1].match(/^\s+-/)) {
          inArray = true;
          arrayItems = [];
        }
      } else if (val === "|" || val === ">") {
        inMultiline = true;
        multilineText = "";
      } else if (val === "null" || val === "~") {
        currentValue = null;
      } else if (val === "true") {
        currentValue = true;
      } else if (val === "false") {
        currentValue = false;
      } else if (val.match(/^\d+$/)) {
        currentValue = parseInt(val, 10);
      } else if (val.match(/^\d+\.\d+$/)) {
        currentValue = parseFloat(val);
      } else {
        // Strip quotes if present
        currentValue = val.replace(/^["']|["']$/g, "");
      }
      continue;
    }

    // Array item
    if (inArray && line.match(/^\s+-/)) {
      const itemMatch = line.match(/^\s+-\s*(.*)/);
      if (itemMatch) {
        let val = itemMatch[1].trim();
        val = val.replace(/^["']|["']$/g, "");
        arrayItems.push(val);
      }
      continue;
    }

    // Multiline continuation
    if (inMultiline) {
      multilineText += (multilineText ? "\n" : "") + line.replace(/^  /, "");
      continue;
    }
  }

  // Flush last key
  if (currentKey !== null) {
    if (inArray) result[currentKey] = arrayItems;
    else if (inMultiline) result[currentKey] = multilineText.trimEnd();
    else result[currentKey] = currentValue;
  }

  return result;
}

// ---------------------------------------------------------------------------
// Transform frontmatter
// ---------------------------------------------------------------------------
function transformFrontmatter(fm) {
  if (!fm) return fm;

  const relationships = Array.isArray(fm.relationships) ? [...fm.relationships] : [];
  const existingTargetTypes = new Set(
    relationships.map((r) => `${r.target}:${r.type}`),
  );

  // Convert standalone fields → relationships
  for (const [field, config] of Object.entries(SINGLE_FIELD_TO_REL)) {
    const val = fm[field];
    if (val && val !== "null" && typeof val === "string" && val.trim()) {
      const key = `${val.trim()}:${config.type}`;
      if (!existingTargetTypes.has(key)) {
        relationships.push({ target: val.trim(), type: config.type });
        existingTargetTypes.add(key);
      }
    }
  }

  for (const [field, config] of Object.entries(ARRAY_FIELD_TO_REL)) {
    const val = fm[field];
    if (Array.isArray(val)) {
      for (const item of val) {
        if (item && typeof item === "string" && item.trim()) {
          const key = `${item.trim()}:${config.type}`;
          if (!existingTargetTypes.has(key)) {
            relationships.push({ target: item.trim(), type: config.type });
            existingTargetTypes.add(key);
          }
        }
      }
    }
  }

  // Transform relationship types using vocabulary map
  for (const rel of relationships) {
    if (rel.type && RELATIONSHIP_TYPE_MAP[rel.type]) {
      rel.type = RELATIONSHIP_TYPE_MAP[rel.type];
    }
  }

  // Deduplicate by target:type (keep the one with rationale if both exist)
  const deduped = [];
  const seen = new Map();
  for (const rel of relationships) {
    const key = `${rel.target}:${rel.type}`;
    if (seen.has(key)) {
      // Keep the one with rationale
      if (rel.rationale && !seen.get(key).rationale) {
        const idx = deduped.indexOf(seen.get(key));
        if (idx >= 0) deduped[idx] = rel;
        seen.set(key, rel);
      }
    } else {
      seen.set(key, rel);
      deduped.push(rel);
    }
  }
  const dedupedRelationships = deduped;

  // Remove standalone fields
  const newFm = {};
  for (const [key, val] of Object.entries(fm)) {
    if (FIELDS_TO_REMOVE.has(key)) continue;
    if (key === "relationships") continue; // We'll add it back
    newFm[key] = val;
  }

  if (dedupedRelationships.length > 0) {
    newFm.relationships = dedupedRelationships;
  }

  return newFm;
}

// ---------------------------------------------------------------------------
// Serialize frontmatter back to YAML
// ---------------------------------------------------------------------------
function serializeFrontmatter(fm) {
  const lines = [];

  // Ordered keys: id, title, description, status first, then rest, relationships last
  const orderedKeys = ["id", "title", "description", "status", "priority", "created", "updated"];
  const seenKeys = new Set();

  for (const key of orderedKeys) {
    if (key in fm) {
      lines.push(serializeField(key, fm[key]));
      seenKeys.add(key);
    }
  }

  // All other keys (except relationships)
  for (const key of Object.keys(fm)) {
    if (seenKeys.has(key) || key === "relationships") continue;
    lines.push(serializeField(key, fm[key]));
    seenKeys.add(key);
  }

  // Relationships last
  if (fm.relationships && fm.relationships.length > 0) {
    lines.push("relationships:");
    for (const rel of fm.relationships) {
      lines.push(`  - target: ${rel.target}`);
      lines.push(`    type: ${rel.type}`);
      if (rel.rationale) {
        lines.push(`    rationale: ${rel.rationale}`);
      }
    }
  }

  return lines.join("\n");
}

function serializeField(key, value) {
  if (value === null || value === undefined) return `${key}: null`;
  if (typeof value === "boolean") return `${key}: ${value}`;
  if (typeof value === "number") return `${key}: ${value}`;
  if (typeof value === "string") {
    if (value.includes("\n")) {
      return `${key}: |\n${value.split("\n").map((l) => `  ${l}`).join("\n")}`;
    }
    // Quote if contains special chars
    if (value.match(/[:#{}[\],&*?|>!%@`"']/)) {
      return `${key}: "${value.replace(/"/g, '\\"')}"`;
    }
    return `${key}: ${value}`;
  }
  if (Array.isArray(value)) {
    if (value.length === 0) return `${key}: []`;
    // Check if items are objects (like acceptance criteria)
    if (typeof value[0] === "object") {
      return `${key}:\n${value.map((v) => `  - ${JSON.stringify(v)}`).join("\n")}`;
    }
    return `${key}:\n${value.map((v) => `  - ${serializeValue(v)}`).join("\n")}`;
  }
  if (typeof value === "object") {
    const entries = Object.entries(value);
    if (entries.length === 0) return `${key}: {}`;
    return `${key}:\n${entries.map(([k, v]) => `  ${k}: ${v}`).join("\n")}`;
  }
  return `${key}: ${value}`;
}

function serializeValue(val) {
  if (typeof val === "string") {
    if (val.match(/[:#{}[\],&*?|>!%@`"']/)) return `"${val.replace(/"/g, '\\"')}"`;
    return val;
  }
  return String(val);
}

// ---------------------------------------------------------------------------
// Body text vocabulary replacement
// ---------------------------------------------------------------------------
const BODY_VOCAB_MAP = [
  [/\bbelongs[- ]to\b/gi, "delivers"],
  [/\bcontains\b/gi, "delivered-by"],
  [/\bdocuments\b/gi, "informs"],
  [/\bdocumented[- ]by\b/gi, "informed-by"],
  [/\bpractices\b/gi, "grounded-by"],
  [/\bpracticed[- ]by\b/gi, "grounded"],
  [/\bverifies\b/gi, "enforces"],
  [/\bverified[- ]by\b/gi, "enforced-by"],
  [/\bscoped[- ]to\b/gi, "enforces"],
  [/\bscoped[- ]by\b/gi, "enforced-by"],
];

function transformBodyText(body) {
  // Only transform vocabulary in context of relationship references
  // Be conservative — don't replace common English words in prose
  let result = body;
  // Replace old relationship types when they appear as inline code or in relationship context
  result = result.replace(/`belongs-to`/g, "`delivers`");
  result = result.replace(/`contains`/g, "`delivered-by`");
  result = result.replace(/`documents`/g, "`informs`");
  result = result.replace(/`documented-by`/g, "`informed-by`");
  result = result.replace(/`practices`/g, "`grounded-by`");
  result = result.replace(/`practiced-by`/g, "`grounded`");
  result = result.replace(/`verifies`/g, "`enforces`");
  result = result.replace(/`verified-by`/g, "`enforced-by`");
  result = result.replace(/`scoped-to`/g, "`enforces`");
  result = result.replace(/`scoped-by`/g, "`enforced-by`");
  result = result.replace(/`supersedes`/g, "`evolves-into`");
  result = result.replace(/`superseded-by`/g, "`evolves-from`");
  result = result.replace(/`promoted-to`/g, "`evolves-into`");
  // Replace "the epic field" / "the milestone field" style references
  result = result.replace(/the epic field/gi, "the delivers relationship");
  result = result.replace(/the milestone field/gi, "the delivers relationship");
  return result;
}

// ---------------------------------------------------------------------------
// Inverse relationship tracking
// ---------------------------------------------------------------------------
const CANONICAL_INVERSES = {
  "informs": "informed-by",
  "informed-by": "informs",
  "evolves-into": "evolves-from",
  "evolves-from": "evolves-into",
  "drives": "driven-by",
  "driven-by": "drives",
  "governs": "governed-by",
  "governed-by": "governs",
  "delivers": "delivered-by",
  "delivered-by": "delivers",
  "enforces": "enforced-by",
  "enforced-by": "enforces",
  "grounded": "grounded-by",
  "grounded-by": "grounded",
  "observes": "observed-by",
  "observed-by": "observes",
  "merged-into": "merged-from",
  "merged-from": "merged-into",
  "synchronised-with": "synchronised-with",
  "depends-on": "depended-on-by",
  "depended-on-by": "depends-on",
};

// Track all relationships written so we can add missing inverses
const writtenRelationships = []; // { source, target, type }
const writtenArtifacts = new Map(); // id → { path, frontmatter }

// ---------------------------------------------------------------------------
// Walk and process
// ---------------------------------------------------------------------------
function walkDir(dir) {
  const files = [];
  for (const entry of readdirSync(dir)) {
    if (entry.startsWith(".") || entry.startsWith("_")) continue;
    const full = join(dir, entry);
    const stat = statSync(full);
    if (stat.isDirectory()) {
      // Skip plugins directory
      if (entry === "plugins") continue;
      files.push(...walkDir(full));
    } else if (entry.endsWith(".md") && entry !== "README.md") {
      files.push(full);
    }
  }
  return files;
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------
const manifest = [];
let processed = 0;
let skipped = 0;
let needsReview = [];

const allFiles = walkDir(BACKUP);
console.log(`Found ${allFiles.length} artifacts in backup`);

for (const file of allFiles) {
  const relPath = relative(BACKUP, file).replace(/\\/g, "/");
  const newRelPath = mapPath(relPath);
  const targetPath = join(TARGET, newRelPath);

  const content = readFileSync(file, "utf-8");
  const { frontmatter, body, raw } = parseFrontmatter(content);

  if (!frontmatter || !frontmatter.id) {
    // Doc file without frontmatter ID — copy as-is with body text transform
    const newBody = transformBodyText(body);
    const newContent = raw
      ? `---\n${raw}\n---\n${newBody}`
      : newBody;

    const changes = [];
    if (relPath !== newRelPath) changes.push(`moved: ${relPath} → ${newRelPath}`);
    if (body !== newBody) changes.push("body vocabulary updated");

    if (!DRY_RUN) {
      mkdirSync(dirname(targetPath), { recursive: true });
      writeFileSync(targetPath, newContent, "utf-8");
    }

    manifest.push({ source: relPath, dest: newRelPath, changes, id: null });
    skipped++;
    continue;
  }

  // Transform frontmatter
  const newFm = transformFrontmatter(frontmatter);
  const newBody = transformBodyText(body);

  // Track relationships for inverse checking
  if (newFm.relationships) {
    for (const rel of newFm.relationships) {
      writtenRelationships.push({
        source: newFm.id, target: rel.target, type: rel.type,
      });
    }
  }
  writtenArtifacts.set(newFm.id, { path: newRelPath, frontmatter: newFm });

  // Build new content
  const newRaw = serializeFrontmatter(newFm);
  const newContent = `---\n${newRaw}\n---\n${newBody}`;

  // Track changes
  const changes = [];
  if (relPath !== newRelPath) changes.push(`moved: ${relPath} → ${newRelPath}`);

  // Check which standalone fields were converted
  for (const field of FIELDS_TO_REMOVE) {
    if (frontmatter[field] !== undefined && frontmatter[field] !== null) {
      const val = frontmatter[field];
      if (Array.isArray(val) ? val.length > 0 : val && val !== "null") {
        changes.push(`${field} → relationship`);
      }
    }
  }

  // Check vocabulary changes in relationships
  if (frontmatter.relationships) {
    for (const rel of frontmatter.relationships) {
      if (rel.type && RELATIONSHIP_TYPE_MAP[rel.type]) {
        changes.push(`rel type: ${rel.type} → ${RELATIONSHIP_TYPE_MAP[rel.type]}`);
      }
    }
  }

  if (body !== newBody) changes.push("body vocabulary updated");

  manifest.push({
    source: relPath, dest: newRelPath, changes, id: newFm.id,
  });

  if (!DRY_RUN) {
    mkdirSync(dirname(targetPath), { recursive: true });
    writeFileSync(targetPath, newContent, "utf-8");
  }

  processed++;
}

// ---------------------------------------------------------------------------
// Add missing inverses
// ---------------------------------------------------------------------------
let inversesAdded = 0;

if (!DRY_RUN) {
  for (const { source, target, type } of writtenRelationships) {
    const inverseType = CANONICAL_INVERSES[type];
    if (!inverseType) continue;

    const targetArtifact = writtenArtifacts.get(target);
    if (!targetArtifact) continue;

    const hasInverse = targetArtifact.frontmatter.relationships?.some(
      (r) => r.target === source && r.type === inverseType,
    );

    if (!hasInverse) {
      if (!targetArtifact.frontmatter.relationships) {
        targetArtifact.frontmatter.relationships = [];
      }
      targetArtifact.frontmatter.relationships.push({
        target: source,
        type: inverseType,
      });
      inversesAdded++;
    }
  }

  // Re-write artifacts that got new inverses
  for (const [id, { path: relPath, frontmatter }] of writtenArtifacts) {
    const targetPath = join(TARGET, relPath);
    if (!existsSync(targetPath)) continue;

    const content = readFileSync(targetPath, "utf-8");
    const { body } = parseFrontmatter(content);
    const newRaw = serializeFrontmatter(frontmatter);
    const newContent = `---\n${newRaw}\n---\n${body}`;
    writeFileSync(targetPath, newContent, "utf-8");
  }
}

// ---------------------------------------------------------------------------
// Report
// ---------------------------------------------------------------------------
console.log(`\n${"=".repeat(60)}`);
console.log(`Migration ${DRY_RUN ? "(DRY RUN)" : "COMPLETE"}`);
console.log(`${"=".repeat(60)}`);
console.log(`Processed: ${processed} artifacts with frontmatter`);
console.log(`Copied:    ${skipped} files without artifact IDs`);
console.log(`Total:     ${processed + skipped}`);
console.log(`Inverses:  ${inversesAdded} added`);

const moved = manifest.filter((m) => m.source !== m.dest);
if (moved.length > 0) {
  console.log(`\nMoved files (${moved.length}):`);
  for (const m of moved.slice(0, 20)) {
    console.log(`  ${m.source} → ${m.dest}`);
  }
  if (moved.length > 20) console.log(`  ... and ${moved.length - 20} more`);
}

const fieldConversions = manifest.filter((m) => m.changes.some((c) => c.includes("→ relationship")));
if (fieldConversions.length > 0) {
  console.log(`\nField → Relationship conversions (${fieldConversions.length} artifacts)`);
}

const vocabChanges = manifest.filter((m) => m.changes.some((c) => c.startsWith("rel type:")));
if (vocabChanges.length > 0) {
  console.log(`\nVocabulary changes (${vocabChanges.length} artifacts)`);
}

// Write manifest
if (!DRY_RUN) {
  writeFileSync(
    join(ROOT, "scripts", "migration-manifest.json"),
    JSON.stringify(manifest, null, 2),
    "utf-8",
  );
  console.log(`\nManifest written to scripts/migration-manifest.json`);
}
