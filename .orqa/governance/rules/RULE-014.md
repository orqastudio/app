---
id: RULE-014
title: Historical Artifact Preservation
description: Documentation is deleted when outdated. Research, plans, and tasks are preserved and marked as surpassed for historical traceability.
status: active
created: "2026-03-07"
updated: "2026-03-07"
layer: canon
scope: system
---

## Two Categories of Written Artifacts

### Documentation (DELETE when outdated)

Documentation pages in `.orqa/documentation/` describe the **current target state**. When a feature, concept, or architecture changes, the old documentation is deleted and replaced. Git history preserves the old version if anyone needs it.

This applies to:
- Architecture docs (`.orqa/documentation/architecture/`)
- Product docs (`.orqa/documentation/product/`)
- Development docs (`.orqa/documentation/development/`)
- Process docs (`.orqa/documentation/process/`)
- UI specs (`.orqa/documentation/ui/`)
- Wireframes (`.orqa/documentation/wireframes/`)

### Planning & Research Artifacts (PRESERVE, mark as surpassed)

Research, plans, and tasks are historical records of thinking, investigation, and decisions. They have lasting value for understanding *why* the project evolved the way it did. When they are no longer current, they are marked as `status: surpassed` — never deleted.

This applies to:
- Research (`.orqa/planning/research/`)
- Plans (`.orqa/planning/plans/`)
- Tasks (`.orqa/planning/tasks/`)
- Ideas (`.orqa/planning/ideas/`) — use `status: archived` per existing lifecycle
- Lessons (`.orqa/governance/lessons/`) — use `promoted-to` field per existing lifecycle

## The `surpassed` Status

When a research document, plan, or task has been overtaken by newer work:

1. Set `status: surpassed` in the YAML frontmatter
2. Add a `surpassed-by` field referencing the replacement artifact (if one exists)
3. Do NOT delete the file
4. Do NOT modify the body content — it is a historical record

```yaml
---
id: TASK-003
layer: canon
title: "Original nav implementation"
status: surpassed
surpassed-by: TASK-009
---
```

## Why This Matters

- **Documentation** answers "what is the current state?" — stale docs cause bugs and confusion
- **Research** answers "what did we investigate and learn?" — deleting research loses institutional knowledge
- **Plans** answer "what was the thinking behind this approach?" — future decisions benefit from understanding past reasoning
- **Tasks** answer "what was done and by whom?" — traceability from plan to implementation

## Rule Status Vocabulary

Rules use `active` / `inactive` — NOT `surpassed`. Surpassed is for research, plans, and tasks.

- **Active**: The rule is enforced. Agents must comply.
- **Inactive**: The rule is no longer enforced. If a rule was superseded by a newer rule, mark it `inactive` and add a leading comment in the body explaining why (e.g., "Superseded by `new-rule.md`"). The file is preserved for historical reference.

## FORBIDDEN

- Deleting research, plan, or task files (mark as surpassed instead)
- Leaving outdated documentation pages alive (delete and replace)
- Marking documentation pages as "surpassed" instead of deleting them
- Modifying the body content of surpassed artifacts (the historical record is immutable)
- Using `surpassed` as a rule status (use `inactive` instead)

## Related Rules

- RULE-008 (documentation-first) — documentation describes current state, deleted when outdated
- RULE-004 (artifact-lifecycle) — status transitions for ideas, epics, milestones
