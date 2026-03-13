---
id: "SKILL-050"
title: "Migration and Link Verification Tooling"
description: "How to use the backfill-relationships and verify-links tools for governance

  artifact migrations. Covers relationship proposal workflow, link verification,

  bidirectional consistency checks, and safe frontmatter updates.

  Use when: Performing schema migrations, backfilling new fields across artifacts,

  auditing cross-references, or verifying bidirectional relationship integrity.\n"
status: "active"
created: "2026-03-12"
updated: "2026-03-12"
layer: "project"
scope:
  - "AGENT-001"
  - "AGENT-003"
category: "tool"
version: "1.0.0"
user-invocable: true
relationships:
  - target: "PILLAR-001"
    type: "grounded"
    rationale: "Migration tooling enables structured schema evolution"
---

Tools for governance artifact migrations and link integrity verification. These tools live in `tools/` at the project root and are designed for CLI use by agents or humans.

## Backfill Tool

**Purpose:** Propose relationship connections for governance artifacts based on content analysis.

**Location:** `tools/backfill-relationships.mjs`

### Usage

```bash
# Dry run — see proposals without applying
node tools/backfill-relationships.mjs .orqa/governance/rules --dry-run

# Single artifact
node tools/backfill-relationships.mjs .orqa/governance/rules --filter=RULE-006

# Machine-readable output (for agent consumption)
node tools/backfill-relationships.mjs .orqa/governance/rules
```

### Workflow

1. Run the tool against an artifact directory
2. Tool scans each artifact's body for cross-references to other artifacts
3. Tool infers relationship types based on source/target artifact types
4. Tool outputs proposals as JSON with target, type, rationale, and confidence
5. Agent or human reviews proposals — approve, reject, or edit
6. Approved proposals are added to the artifact's `relationships` frontmatter array

### Relationship Type Inference

The tool uses artifact type pairs to infer the most likely relationship:

| Source → Target | Inferred Type |
|----------------|---------------|
| Rule → Decision | `grounded` |
| Rule → Pillar | `grounded` |
| Skill → Decision | `grounded` |
| Skill → Pillar | `grounded` |
| Decision → Rule | `enforces` |
| Decision → Skill | `practices` |
| Lesson → Decision | `observes` |
| Any → Any (same type) | `informs` |

### Proposal Format

```json
{
  "id": "RULE-006",
  "title": "Coding Standards",
  "path": ".orqa/governance/rules/RULE-006.md",
  "existing": [],
  "proposals": [
    {
      "target": "AD-001",
      "type": "grounded",
      "rationale": "Referenced in body — Error Propagation Decision",
      "confidence": "medium"
    }
  ]
}
```

## Link Verification Tool

**Purpose:** Scan all `.orqa/` artifacts for broken, missing, or contextually suspect cross-references.

**Location:** `tools/verify-links.mjs`

### Usage

```bash
# Basic scan — find broken links and bare IDs
node tools/verify-links.mjs

# Include bidirectional consistency checks
node tools/verify-links.mjs --check-bidirectional

# JSON output for agent processing
node tools/verify-links.mjs --json
```

### What It Checks

**Structural (pattern matching):**
- Linked references `[text](ID)` where the target artifact doesn't exist
- Bare artifact IDs in body text that should use link syntax
- Frontmatter reference fields (`epic`, `depends-on`, `research-refs`, etc.) pointing to non-existent artifacts
- Relationship targets that don't exist

**Bidirectional consistency (with `--check-bidirectional`):**
- If artifact A has `grounded:B`, artifact B should have `grounded-by:A`
- Reports missing inverse relationships
- Uses the inverse type map defined in [AD-042](AD-042)

### Issue Types

| Type | Severity | Meaning |
|------|----------|---------|
| `broken-link` | error | Linked reference points to non-existent artifact |
| `broken-frontmatter-ref` | error | Frontmatter field references non-existent artifact |
| `broken-relationship` | error | Relationship target doesn't exist |
| `bare-id` | warning | Artifact ID in body without link syntax |
| `missing-inverse` | warning | Bidirectional relationship is one-sided |

### Output Format

```
=== ERRORS ===
  broken-link: Linked reference RULE-099 points to non-existent artifact
  broken-frontmatter-ref: Frontmatter field 'depends-on' references non-existent artifact TASK-999

=== WARNINGS ===
  bare-id: Bare artifact ID EPIC-045 should use link syntax [EPIC-045](EPIC-045)
  missing-inverse: RULE-006 has grounded:AD-001, but AD-001 lacks grounded-by:RULE-006

2 error(s), 2 warning(s) found.
```

## Migration Workflow

When performing a schema migration (like the [AD-042](AD-042) knowledge maturity pipeline):

1. **Pre-flight:** Run `node tools/verify-links.mjs` to establish a baseline
2. **Batch backfill:** Run backfill tool per artifact type, review proposals in batches
3. **After each batch:** Run `node tools/verify-links.mjs --check-bidirectional` to verify consistency
4. **Final check:** Run both tools with `--check-bidirectional` to confirm all relationships are consistent and all links are valid

## Safe Frontmatter Updates

The backfill tool's `updateFrontmatter()` function:
- Parses existing YAML, merges updates, re-serializes
- Preserves markdown body content exactly
- Follows schema `propertyOrder` for field ordering
- Never uses regex replacement on frontmatter

## Adding New Relationship Types

If the schema adds new relationship types:
1. Update the `inferRelationType()` map in `backfill-relationships.mjs`
2. Update the `INVERSE_TYPES` map in `verify-links.mjs`
3. Both maps should stay in sync with the relationship type enum in schema.json
