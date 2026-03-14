---
id: SKILL-038
title: Artifact Audit Methodology
description: |
  Systematic audit of governance artifacts for reference integrity, cross-layer
  consistency, and codebase alignment. Covers the artifact graph as a primary tool,
  known graph limitations, per-type checklists, cross-layer checks, evidence
  requirements, and output format for audit findings documents.
  Use when: Auditing .orqa/ artifacts for broken references, stale paths, content
  inaccuracies, or structural drift from the codebase.
status: active
created: 2026-03-11
updated: 2026-03-11
layer: project
category: methodology
version: 1.0.0
user-invocable: false
relationships:
  - target: PILLAR-001
    type: grounded
    rationale: Systematic auditing detects broken references, stale paths, and content drift, keeping the artifact graph accurate and navigable
  - type: scoped-to
    target: AGENT-003
    rationale: Migrated from scope field
  - type: scoped-to
    target: AGENT-006
    rationale: Migrated from scope field
---

Methodology for auditing `.orqa/` governance artifacts. An audit answers the question: "Does everything in `.orqa/` still describe reality?" This skill provides a repeatable process for finding broken references, stale paths, content inaccuracies, and drift from the codebase — and for structuring findings so they can be fixed efficiently.

## The Artifact Graph as Primary Tool

The artifact graph (exposed via Tauri commands `get_graph_stats`, `get_references_from`, `get_references_to`) is the most efficient starting point for any reference integrity audit. It tracks which artifacts reference which other artifacts and can surface broken references programmatically — far faster than manually reading every file.

### Fields the Graph Tracks

The graph maintains two field categories for cross-reference tracking:

**SINGLE_REF_FIELDS** — fields that hold a single artifact ID as a string:

```
milestone, epic, supersedes, superseded-by, surpassed-by, assignee
```

**ARRAY_REF_FIELDS** — fields that hold lists of artifact IDs:

```
depends-on, blocks, pillars, research-refs, docs-required, docs-produced, skills, relationships
```

For every artifact in `.orqa/`, the graph resolves these fields to their targets and flags any ID that does not resolve to an existing artifact.

### Using the Graph

Start an audit with `get_graph_stats` to get a count of artifacts and a summary of broken references. Then use `get_references_to` with a specific artifact ID to find all artifacts that reference it — useful for checking whether removing or renaming an artifact will orphan anything. Use `get_references_from` to see all outgoing references from a specific artifact.

## Known Graph Limitations

The graph is a useful first pass, but it has three significant blind spots that require manual follow-up:

**1. `docs-required` and `docs-produced` are treated as artifact ID refs.**

These fields often contain file paths (e.g., `.orqa/documentation/ui/scanner-dashboard.md`) rather than artifact IDs. The graph will report these as broken references because they do not match any artifact ID. They are not broken — they are file paths that need to be verified on disk separately. Filter these out before counting broken ref findings.

**2. SINGLE_REF_FIELDS that contain arrays are silently skipped.**

The graph reads SINGLE_REF_FIELDS via `as_str()`. If a field that is defined as a single ref contains an array (e.g., `supersedes: [AD-005, AD-014]`), `as_str()` returns `None` and the refs are silently dropped — they will not appear in the graph and will not be flagged as broken. Any field in SINGLE_REF_FIELDS that contains array data must be checked manually.

**3. The graph only scans `.orqa/`.**

References to files outside `.orqa/` — such as source code paths in skill examples, component names in rules, or function signatures in agent required reading — are not tracked. These require separate codebase verification using Grep or semantic search.

## Per-Artifact-Type Checklists

Run these checklists after the graph pass has identified broken ID references. Each checklist addresses what the graph cannot check automatically.

### Epics

- `status` is one of `draft | ready | in-progress | review | done`
- `milestone` exists as an MS-NNN file
- `research-refs` entries exist as RES-NNN files in `.orqa/delivery/research/`
- `docs-required` file paths resolve to existing files on disk (not artifact IDs — verify on disk)
- `docs-produced` file paths resolve to existing files on disk
- `scoring` dimensions match the keys defined in `project.json`'s scoring section
- `depends-on` and `blocks` reference existing EPIC-NNN or TASK-NNN files

### Tasks

- `epic` field exists and references an existing EPIC-NNN
- `depends-on` tasks exist and their `status` is `done` if this task is `in-progress`
- `assignee` is a valid agent name (exists in `.orqa/process/agents/`)
- `skills` entries exist in `.orqa/process/skills/`

### Ideas

- Ideas with `status: promoted` should have a corresponding EPIC-NNN created
- `pillars` entries exist as PILLAR-NNN files
- No idea jumps from `captured` directly to `promoted` (research step must be documented)

### Research

- `status` is one of `draft | complete | surpassed`
- `surpassed-by` references an existing artifact if `status` is `surpassed`
- Body contains substantive findings, not placeholder text

### Decisions

- `supersedes` and `superseded-by` are both set when a decision is superseded — the relationship must be bidirectional and updated in the same commit
- `status` is one of `proposed | accepted | superseded | deprecated`
- Decisions index at `.orqa/documentation/architecture/decisions.md` has an entry for this AD-NNN

### Rules

- `status` is `active` or `inactive` (not `surpassed` — that is for research/tasks only)
- Related Rules section references are all valid RULE-NNN identifiers
- `relationships` array contains at least one `grounded` relationship to a pillar or decision

### Lessons

- `maturity` is `observation` or `understanding`
- `relationships` array contains pipeline connections (grounded, informs, etc.)
- `recurrence` count is accurate — check against known occurrence patterns
- `status` matches recurrence: `active` (count 0–1), `recurring` (count >= 2 not yet promoted), `promoted` (has grounded-by relationships to rules/skills)

## Cross-Layer Consistency Checks

These checks verify that governance artifacts agree with each other and with the codebase.

### Agent Skills Lists

For each agent in `.orqa/process/agents/`, read its `skills:` YAML list. Every entry must correspond to a directory in `.orqa/process/skills/` containing a `SKILL.md`. Skills listed on agent YAML should be Tier 1 portable skills only — project-specific `orqa-*` skills (except `composability`) should not appear on agent frontmatter; they are orchestrator-injected at task time per [RULE-026](RULE-026).

### Agent Required Reading

For each agent, read the Required Reading section. Every file path listed must resolve to an existing file on disk. Use Grep to check:

```
grep -r "Required Reading" .orqa/process/agents/ --include="*.md" -A 20
```

Then verify each listed path with a file existence check. Broken Required Reading paths silently fail at task start — agents proceed without the governing documentation.

### Orchestrator Skill Injection Table

The orchestrator's skill injection table (in `orchestrator.md` / `CLAUDE.md`) lists which project skills are injected for which task scopes. Verify that every skill name in this table has a corresponding directory in `.orqa/process/skills/`. Missing skills will cause injection failures at delegation time.

### Rule Related Rules Sections

For each rule, read the Related Rules section. Every rule ID referenced (e.g., `[RULE-004](RULE-004)`) must correspond to an existing `.orqa/process/rules/RULE-NNN.md` file. Also check that the referenced rule's `status` is `active` — linking to an `inactive` rule is misleading.

### Milestone Epic Counts

Some milestone files record `epic-count` and `completed-epics` fields. Verify these by counting actual EPIC-NNN files that reference the milestone's ID in their `milestone:` field. Mismatch indicates the count was not updated when epics were added or completed.

## Codebase Alignment Checks

These checks verify that governance artifacts describe reality — not a historical state of the codebase.

### File Paths in Rules and Skills

Rules and skills often reference specific file paths as examples or as enforcement targets. For each path mentioned, verify it exists on disk. Common stale patterns from codebase evolution:

- Directory renames (e.g., `persistence/` renamed to `repo/`)
- Module moves (e.g., a type moved from one file to another)
- File deletions (e.g., a file removed from root that governance artifacts still reference)

Use Grep to find all file path references in governance artifacts:

```
grep -r "backend/src-tauri/src/" .orqa/ --include="*.md" -l
grep -r "ui/src/lib/" .orqa/ --include="*.md" -l
```

Then spot-check the referenced paths against disk reality.

### Function Names in Skills

Project skills (layer: project) describe actual codebase patterns. When a skill names a specific function, verify the function exists with that name:

```
grep -r "fn function_name" backend/src-tauri/src/
```

Function renames are the most common source of stale skill content. Focus on skills that document domain services, repository patterns, and IPC commands — these change most frequently as the codebase evolves.

### Component Names in Rules

Rules that reference specific Svelte component names (e.g., [RULE-024](RULE-024)'s shared component inventory) should be verified against the actual files in `ui/src/lib/components/`. Component names change as the UI is refactored. A rule pointing to a component that doesn't exist at the path it specifies creates confusion for implementers.

### Directory Structure Descriptions

The orchestrator definition and some skills contain project structure diagrams. Verify that every directory listed in these diagrams exists on disk. Pay particular attention to directories that were renamed or reorganized during codebase evolution.

## Systemic Pattern Grouping

When an audit produces many findings, resist the urge to list each one individually. Group findings by their root cause — this dramatically reduces noise and enables batch fixes.

### Grouping Approach

After collecting all raw findings, identify which ones share the same underlying cause:

- **Migration artifacts** — Multiple stale references all caused by a single rename or move event (e.g., 20 files referencing `dogfood-mode.md` after it was renamed to `[RULE-009](RULE-009).md`)
- **Schema drift** — Multiple artifacts with the same missing or wrong field, caused by a schema change that was not backfilled
- **Orphaned content** — Multiple artifacts that reference a removed concept or type, caused by deleting that concept without updating its references
- **Content duplication** — The same detailed information appears in both a rule and a skill, created because one was added without updating the other

Name the group by its root cause, not its symptoms. "20 stale `dogfood-mode.md` refs from the rule rename migration" is more actionable than 20 individual findings each saying "wrong path."

### Priority Tiers

Assign each group to a priority tier:

- **P1 — Fix immediately:** Findings that cause silent failures at runtime (broken hooks, nonexistent Required Reading, hooks pointing to empty directories)
- **P2 — Fix soon:** Stale paths and content inaccuracies that mislead agents or implementers without causing immediate failures (wrong function names in skills, outdated component inventories in rules)
- **P3 — Improvements:** Structural issues that reduce quality but do not cause failures (content duplication between rules and skills, empty Related Rules sections, inconsistent field ordering)

## Evidence Requirements

Every finding must be documented with enough information to locate and fix it without re-investigation:

| Field | Content |
|-------|---------|
| **Source file** | Full path relative to project root (e.g., `.orqa/process/agents/implementer.md`) |
| **Field or line** | The specific frontmatter field or section heading where the issue appears |
| **Expected value** | What the field or content should contain |
| **Actual value** | What it currently contains |
| **Classification** | One of: `stale path`, `broken ref`, `content inaccuracy`, `missing artifact` |

For grouped findings, document the pattern once and list the affected files. Do not write out identical finding entries for each file in a group.

## Output Format

Audit findings are recorded as a research document (RES-NNN) in `.orqa/delivery/research/`. The document serves as both the findings record and the implementation guide for the fix tasks.

```markdown
---
id: RES-NNN
title: [Scope] Audit Findings
description: [Brief description of what was audited and the key outcomes.]
status: complete
created: "YYYY-MM-DD"
updated: "YYYY-MM-DD"
---

# [Scope] Audit Findings

**Epic:** EPIC-NNN
**Tasks completed:** [list of TASK-NNN that produced these findings]

---

## Executive Summary

[Two to four sentences describing the overall health of the audited layer,
the total finding count, and the highest-priority issues.]

## Finding Categories and Counts

| Category | [Layer A] | [Layer B] | Total |
|----------|-----------|-----------|-------|
| Stale file paths | N | N | N |
| Content accuracy | N | N | N |
| Broken references | N | N | N |
| Missing artifacts | N | N | N |
| **Total** | **N** | **N** | **N** |

---

## Priority 1: Fix Immediately

### F-01: [Finding title]

**Impact:** [Why this causes real problems right now.]

[Root cause and affected files.]

**Fix:** [Specific, actionable fix instructions.]

---

## Priority 2: Fix Soon

[Same structure as P1.]

---

## Priority 3: Improvements

[Same structure as P1.]

---

## Recommended Implementation Approach

### Batch 1: [Category]
[Description of what to fix together and why it's low-risk.]

### Batch 2: [Category]
[...]

---

## Files Examined

[List of files or directories covered by this audit.]
```

## Related Skills

- `orqa-governance` — artifact types, frontmatter schemas, and directory structure
- `governance-maintenance` — governance framework custodianship and promotion pipeline
- `orqa-documentation` — internal link format and cross-referencing conventions
