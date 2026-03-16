---
id: EPIC-058
title: Knowledge Maturity Pipeline — Full Migration
description: "Migrate all governance artifacts to the AD-042 knowledge maturity pipeline: rule-overrides mechanism, relationship backfill on ~150 artifacts, schema enforcement, deprecated field removal. Done in one pass to avoid mid-migration drift."
status: completed
priority: P1
created: 2026-03-12
updated: 2026-03-12
deadline: null
horizon: null
scoring: null
rule-overrides:
  - "rule: RULE-032"
  - "rule: RULE-004"
relationships:
  - target: RES-048
    type: informed-by
    rationale: Auto-generated inverse of informed-by relationship from RES-048
  - target: MS-001
    type: delivers
    rationale: Epic belongs to this milestone
  - target: TASK-271
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-272
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-273
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-274
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-275
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-276
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-277
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-278
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-279
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-280
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-348
    type: delivered-by
    rationale: Epic contains this task
  - target: PILLAR-001
    type: grounded-by
  - target: PILLAR-002
    type: grounded-by
  - target: AD-042
    type: informs
  - target: EPIC-059
    type: depended-on-by
---
## Context

[AD-042](AD-042) defines the Knowledge Maturity Pipeline — restructuring governance artifacts around a six-stage learning lifecycle with mandatory bidirectional relationships. The design is complete. This epic is the full migration.

This must be done in one pass. A half-migrated state — some artifacts with relationships, some without; some schemas requiring fields, others not — creates exactly the kind of drift the pipeline is designed to prevent. Mid-migration hell is worse than the current state.

**What exists today:**
- ~150 governance artifacts (16 lessons, 42 decisions, 44 rules, 48 skills) with no pipeline relationships
- Schemas updated with optional `relationships`, `maturity`, `category` fields (Phase 1 already done)
- `rule-overrides` field added to task and epic schemas
- No mechanism to read `rule-overrides` at enforcement time

**What this epic delivers:**
- Rule-overrides mechanism working in both prompt injection and pre-commit hooks
- Every governance artifact has a `relationships` array populated with pipeline connections
- Lessons have `maturity` field (observation/understanding)
- Skills have `category` field (methodology/domain/tool)
- `relationships` is required in all governance schemas
- Deprecated fields (`evolves-into`, `promoted-from`, `research-refs`) removed
- All changes validated, committed, and stable

## Implementation Design

### Phase 0: Rule-Overrides Mechanism

Build the scoped rule suspension system before touching any artifacts.

**Prompt injection** (CLI plugin): When loading rules into agent context for a task, read the task's `rule-overrides` (falling back to epic's if task has none). Suspended rules are loaded but annotated: "This rule is suspended for this task because: [reason]."

**Pre-commit hook**: When validating staged artifacts, check if the commit references a task with `rule-overrides` that suspend the failing rule. If so, warn but don't block.

**Inheritance**: Task with no `rule-overrides` inherits from epic. Task with its own `rule-overrides` replaces the epic's entirely.

### Phase 1: Schema Changes (ALREADY DONE)

Schemas already updated this session with optional fields:
- `governance/lessons/schema.json` — `maturity`, `relationships`
- `governance/decisions/schema.json` — `relationships`
- `governance/rules/schema.json` — `relationships`
- `team/skills/schema.json` — `category`, `relationships`

### Phase 2: Backfill Relationships

AI proposes relationships for all ~150 artifacts. Human reviews in batches.

**Batch order** (most connected first):
1. **Rules** (44) — add `grounded` to decisions/pillars
2. **Skills** (48) — add `category` and `grounded` to decisions/pillars
3. **Decisions** (42) — add `grounded-by` and `enforces` to skills/rules already backfilled
4. **Lessons** (16) — add `maturity` and `grounded` to decisions

Each batch: AI proposes → human reviews → commit. Later batches use earlier batches' connections.

### Phase 3: Make Fields Required

Single commit after Phase 2:
- Move `relationships` into `required` in all four governance schemas
- Move `maturity` into `required` for lessons
- Move `category` into `required` for skills
- Update orchestrator prompt with relationship guidance for new artifact creation

### Phase 4: Deprecate Old Fields

Single commit after Phase 3 is stable:
- Remove `evolves-into` from lesson frontmatter and schema
- Remove `promoted-from` from rule frontmatter and schema
- Remove `research-refs` from decision frontmatter and schema (data migrated to `informed-by` relationships)

## Tasks

| ID | Title | Depends On | Phase |
|----|-------|-----------|-------|
| [TASK-271](TASK-271) | Implement rule-overrides in CLI plugin prompt injection | — | 0 |
| [TASK-272](TASK-272) | Implement rule-overrides in pre-commit hook | — | 0 |
| [TASK-273](TASK-273) | Build AI-assisted backfill and link verification tooling | [TASK-271](TASK-271), [TASK-272](TASK-272) | 1.5 |
| [TASK-274](TASK-274) | Backfill rules with relationships | [TASK-273](TASK-273) | 2 |
| [TASK-275](TASK-275) | Backfill skills with category and relationships | [TASK-274](TASK-274) | 2 |
| [TASK-276](TASK-276) | Backfill decisions with relationships | [TASK-275](TASK-275) | 2 |
| [TASK-277](TASK-277) | Backfill lessons with maturity and relationships | [TASK-276](TASK-276) | 2 |
| [TASK-278](TASK-278) | Make relationship fields required in schemas | [TASK-277](TASK-277) | 3 |
| [TASK-279](TASK-279) | Remove deprecated fields from schemas and artifacts | [TASK-278](TASK-278) | 4 |
| [TASK-280](TASK-280) | Verify pipeline integrity — all artifacts connected, no orphans | [TASK-279](TASK-279) | 4 |

## Out of Scope

These are separate epics that build on this migration but are not part of it:

- Plugin architecture for verification data collection
- Project initialization tooling detection and plugin suggestion
- Verification definition and scheduling system (VER-NNN records)
- Pipeline dashboard (thread visualization, bottleneck detection)
- AI-assisted plugin generation
- "Friction is the feature" pillar evaluation
