---
name: orqa-artifact-audit
description: "Schema-driven audit methodology for governance artifacts. Validates reference integrity, relationship vocabulary compliance, cross-layer consistency, and content accuracy against core.json and plugin definitions. Use when: Auditing .orqa/ artifacts for broken references, stale content, structural drift, or relationship constraint violations in any OrqaStudio project."
user-invocable: false
---

Methodology for auditing `.orqa/` governance artifacts. An audit answers the question: **"Does everything in `.orqa/` still describe reality?"**

This skill provides a repeatable, schema-driven process for finding broken references, vocabulary violations, constraint breaches, stale content, and structural drift — and for structuring findings so they can be fixed efficiently.

All audit checks derive from the canonical definitions in `core.json` (platform types and relationships), `project.json` (project-scoped types and configuration), and any active plugin's `orqa-plugin.json`. No checklist is hardcoded — the audit reads the schema and validates artifacts against it.

## Audit Categories

### 1. ID Uniqueness

Scan all artifacts across core (`.orqa/` shipped with the app), plugins, and project `.orqa/` directories for duplicate `id:` fields. Every artifact ID must be globally unique across all layers. Collisions cause graph corruption — the graph will silently drop or merge nodes that share an ID.

### 2. Relationship Vocabulary Compliance

Every `type` value in a relationship entry must exist as a key in `core.json`'s relationship vocabulary or in the relevant plugin's `orqa-plugin.json` relationship extensions. For each relationship entry, verify:

- The `type` is a recognised forward or inverse key
- **From-type constraints** are satisfied: the artifact containing the relationship is of a type permitted as a source (e.g., `enforces` is only valid FROM a rule)
- **To-type constraints** are satisfied: the `target` artifact is of a type permitted as a destination (e.g., `grounded` only targets pillars)
- **Semantic category** is appropriate for the connection being made

Load constraints from `core.json`'s `relationships` section. Do not maintain a separate checklist — if the schema changes, the audit adapts automatically.

### 3. Relationship Integrity

Every `target` in a relationship entry must resolve to an existing artifact. For each relationship, verify:

- The target artifact exists on disk with a matching `id:` field
- The target artifact contains the expected **inverse relationship** back to the source (bidirectional integrity)
- No orphaned inverses exist — if artifact A declares `informs` -> B, then B must declare `informed-by` -> A

### 4. Placement Analysis

Artifacts should live in the correct home based on their `layer` field and content scope:

- `layer: core` artifacts belong in the app's `.orqa/` directory — they ship with the platform
- `layer: project` artifacts belong in the project's `.orqa/` directory — they are project-specific
- Plugin artifacts belong in their plugin's directory structure

Flag any artifact whose `layer` field contradicts its physical location, or whose content is project-specific but is placed in the core layer (or vice versa).

### 5. Duplicate Detection

Compare artifacts across layers for byte-identical or near-identical copies. Common patterns:

- A core skill that was copied into a project's `.orqa/` without modification
- A project rule that duplicates a core rule's content with minor wording changes
- Documentation that exists in both platform docs and project docs with identical content

Duplicates create maintenance burden — when the canonical version is updated, the copy drifts silently.

### 6. Stale Content

Identify artifacts that reference things that no longer exist or describe behaviour that has changed:

- File paths that do not resolve on disk
- Function, component, or module names that have been renamed or removed
- Directory structure descriptions that no longer match reality
- References to deprecated vocabulary, removed features, or old configuration keys
- Skills or docs that describe workflows that have been superseded

This category requires human or AI judgement — automated tools can flag path references, but content accuracy requires reading comprehension.

### 7. Doc/Skill Pair Coverage

Every plugin that introduces artifact types should have a synchronised doc+skill pair:

- A platform documentation page explaining the artifact type and its schema
- A skill teaching agents how to work with that artifact type
- The doc and skill should be linked via `synchronised-with` relationships

Check for missing pairs and for pairs that exist but have drifted out of sync.

## Audit Process

### Phase 1: Automated Validation

Run `make verify` (or the project's equivalent integrity checker). This catches:
- Broken `target` references in relationship entries
- Missing inverse relationships
- Vocabulary violations (unknown `type` values)
- Status values outside the canonical set

Record the output. Any errors from the automated checker are guaranteed findings — they do not require further investigation.

### Phase 2: ID Scan

Scan for all `id:` fields across every `.orqa/` directory in scope (app core, project, plugins). Detect collisions:

- Same ID used by two different artifacts
- IDs that do not follow the expected prefix pattern for their artifact type (as defined in `core.json` or `project.json`)

### Phase 3: Relationship Scan

Load the relationship vocabulary from `core.json`. For every relationship entry in every artifact:

1. Validate the `type` against the vocabulary
2. Check from-type and to-type constraints
3. Verify the target artifact exists
4. Verify the inverse relationship exists on the target

This phase is the most systematic — it can be fully automated given the schema.

### Phase 4: Content Review

Per-artifact review for accuracy, relevance, and placement. This phase requires human or AI judgement:

- Read each artifact and assess whether its content describes current reality
- Check file paths, function names, and structural references against the actual codebase
- Evaluate whether the artifact is in the correct layer
- Flag content that is duplicated across artifacts

Prioritise artifacts that are consumed at runtime (agent definitions, required reading lists, skill injection tables) — stale content in these artifacts causes silent failures.

### Phase 5: Gap Analysis

Check for structural gaps in the artifact graph:

- Artifact types defined in `core.json` or plugins that have no corresponding doc/skill pair
- Artifacts with no `grounded` relationship to any pillar (if grounding is expected for their type)
- Decisions with no `enforces` relationship from any rule (unenforced decisions)
- Agents whose skill lists reference non-existent skills
- Required reading paths that do not resolve

## Findings Format

Record each finding in a table:

```
| # | Category | Severity | Artifact | Finding | Fix |
|---|----------|----------|----------|---------|-----|
```

Where:
- **#** is a sequential finding number (F-01, F-02, ...)
- **Category** is one of the seven audit categories above
- **Severity** follows the levels below
- **Artifact** is the artifact ID or file path
- **Finding** is a concise description of the issue
- **Fix** is a specific, actionable remediation

## Severity Levels

- **CRITICAL** — Broken references that cause runtime failures, enforcement gaps where a decision has no enforcing rule, or ID collisions that corrupt the graph
- **HIGH** — Wrong vocabulary usage, misplaced artifacts (wrong layer), constraint violations (e.g., `grounded` targeting a non-pillar)
- **MEDIUM** — Missing doc/skill pairs, stale content that misleads but does not break anything, missing inverse relationships
- **LOW** — Style or naming inconsistencies, minor content drift, empty optional sections

## Systemic Pattern Grouping

When an audit produces many findings, group them by root cause rather than listing each individually:

- **Migration residue** — Multiple stale references caused by a single rename or move event
- **Schema drift** — Multiple artifacts with the same missing or wrong field, caused by a schema change that was not backfilled
- **Orphaned content** — Multiple artifacts referencing a removed concept
- **Content duplication** — The same information duplicated across artifacts in different layers

Name the group by its root cause, not its symptoms. Batch fixes by group — this is far more efficient than fixing findings one at a time.

## Output Format

Audit findings are recorded as a research document in the project's `.orqa/delivery/research/` directory:

```markdown
---
id: RES-NNN
title: "[Scope] Audit Findings"
description: "[Brief description of what was audited and key outcomes.]"
status: complete
created: "YYYY-MM-DD"
updated: "YYYY-MM-DD"
relationships:
  - target: [EPIC-NNN]
    type: delivers  - target: DOC-a1b2c3d4
    type: synchronised-with

---

## Executive Summary

[Two to four sentences: overall health, total finding count, highest-priority issues.]

## Finding Categories and Counts

| Category | Count |
|----------|-------|
| ID uniqueness | N |
| Vocabulary compliance | N |
| Relationship integrity | N |
| Placement | N |
| Duplicates | N |
| Stale content | N |
| Coverage gaps | N |
| **Total** | **N** |

## CRITICAL Findings

### F-01: [Title]

**Impact:** [Why this causes real problems.]

[Root cause and affected artifacts.]

**Fix:** [Specific remediation steps.]

## HIGH Findings

[Same structure.]

## MEDIUM Findings

[Same structure.]

## LOW Findings

[Same structure.]

## Recommended Fix Batches

### Batch 1: [Root cause group]
[What to fix together and why.]

### Batch 2: [Root cause group]
[...]

## Artifacts Examined

[List of directories or artifact sets covered by this audit.]
```

## Related Skills

- `orqa-governance` — artifact types, frontmatter schemas, and directory conventions
- `governance-maintenance` — governance framework custodianship and promotion pipeline
- `orqa-documentation` — internal link format and cross-referencing conventions
