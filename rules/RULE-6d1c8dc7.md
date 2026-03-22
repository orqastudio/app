---
id: RULE-6d1c8dc7
title: Historical Artifact Preservation
description: Documentation is deleted when outdated. Research and tasks are preserved and marked as surpassed for historical traceability.
status: active
created: 2026-03-07
updated: 2026-03-07
enforcement: "agent system prompt — governance steward and orchestrator are instructed not to delete research or task artifacts; orqa validate checks for surpassed artifacts missing surpassed-by fields"
relationships:
  - target: AD-c8535011
    type: enforces
---

## Two Categories of Written Artifacts

### Documentation (DELETE when outdated)

Documentation pages in `.orqa/documentation/` describe the **current target state**. When a feature, concept, or architecture changes, the old documentation is deleted and replaced. Git history preserves the old version if anyone needs it.

This applies to:
- Architecture docs (`.orqa/documentation/development/`)
- Product docs (`.orqa/documentation/about/`)
- Development docs (`.orqa/documentation/development/`)
- Process docs (`.orqa/documentation/guide/`) and (`.orqa/documentation/about/`)
- UI specs (`.orqa/documentation/reference/`)
- Wireframes (`.orqa/documentation/reference/wireframes/`)

### Research & Task Artifacts (PRESERVE, mark as surpassed)

Research and tasks are historical records of thinking, investigation, and decisions. They have lasting value for understanding *why* the project evolved the way it did. When they are no longer current, they are marked as `status: surpassed` — never deleted.

This applies to:
- Research (`.orqa/delivery/research/`)
- Tasks (`.orqa/delivery/tasks/`)
- Ideas (`.orqa/delivery/ideas/`) — use `status: archived` per existing lifecycle
- Lessons (`.orqa/process/lessons/`) — use `promoted_to` field per existing lifecycle

## The `surpassed` Status

When a research document or task has been overtaken by newer work:

1. Set `status: surpassed` in the YAML frontmatter
2. Add a `surpassed-by` field referencing the replacement artifact (if one exists)
3. Do NOT delete the file
4. Do NOT modify the body content — it is a historical record

```yaml
---
id: TASK-fa777da9
title: "Original nav implementation"
status: surpassed
surpassed-by: TASK-614a6a45
---
```

## Why This Matters

- **Documentation** answers "what is the current state?" — stale docs cause bugs and confusion
- **Research** answers "what did we investigate and learn?" — deleting research loses institutional knowledge
- **Tasks** answer "what was done and by whom?" — traceability from epic to implementation

## Rule Status Vocabulary

Rules use `active` / `inactive` — NOT `surpassed`. Surpassed is for research and tasks.

- **Active**: The rule is enforced. Agents must comply.
- **Inactive**: The rule is no longer enforced. If a rule was superseded by a newer rule, mark it `inactive` and add a leading comment in the body explaining why (e.g., "Superseded by `new-rule.md`"). The file is preserved for historical reference.

## FORBIDDEN

- Deleting research or task files (mark as surpassed instead)
- Leaving outdated documentation pages alive (delete and replace)
- Marking documentation pages as "surpassed" instead of deleting them
- Modifying the body content of surpassed artifacts (the historical record is immutable)
- Using `surpassed` as a rule status (use `inactive` instead)

## Related Rules

- [RULE-9daf29c0](RULE-9daf29c0) (documentation-first) — documentation describes current state, deleted when outdated
- [RULE-7b770593](RULE-7b770593) (artifact-lifecycle) — status transitions for ideas, epics, milestones
