---
id: TASK-040
title: Governance data quality backfill
description: Audits and repairs frontmatter quality across all governance artifacts — adding missing titles, descriptions, and consistent YAML field ordering — with no code changes required.
status: done
created: "2026-03-09"
updated: "2026-03-09"
epic: EPIC-043
assignee: AGENT-003
skills:
  - SKILL-011
  - SKILL-029
scope:
  - .orqa/governance/lessons/
  - .orqa/governance/rules/
  - .orqa/governance/decisions/
  - .orqa/planning/research/
  - .orqa/planning/milestones/MS-000.md
  - .orqa/planning/epics/
  - .orqa/planning/tasks/
acceptance:
  - All lessons have human-readable titles (no code references) and descriptions
  - All rules have descriptions in their frontmatter
  - All research documents have descriptions
  - Research documents either all reference a milestone field or none do (consistent approach)
  - MS-000 restructured to match MS-001 format with phases as epics and associated tasks
  - YAML field order audited across all artifact types for content hierarchy sense
  - Field ordering documented in artifact framework
---
## Findings Addressed

- **F4**: Research audits missing description fields
- **F5**: Research milestone grouping inconsistent
- **F6**: Lessons have code-reference titles, some lack descriptions
- **F7**: Rules not all have descriptions
- **F8**: [MS-000](MS-000) needs restructuring as epics/tasks

## Notes

This is a data-only task — no code changes. All fixes are governance file edits.

The YAML field order audit is critical because [TASK-038](TASK-038) will make the renderer respect YAML source order. If the field order in existing files doesn't make sense from a content hierarchy perspective, the renderer fix will surface that as a display problem.

Recommended field order per type:
- **Milestones**: id, title, status, description, created, updated, deadline, gate, epic-count, completed-epics, tags
- **Epics**: id, title, status, priority, milestone, description, created, updated, research-refs, docs-required, docs-produced, scoring, tags
- **Tasks**: id, title, status, epic, description, created, updated, assignee, skills, scope, acceptance, tags
- **Ideas**: id, title, status, pillar, description, research-needed, promoted-to, tags
- **Lessons**: id, title, category, description, recurrence, promoted-to, tags
- **Rules**: id, title, description, scope
- **Decisions**: id, title, status, description, created, updated, supersedes, superseded_by, tags
- **Research**: id, title, status, description, created, updated, milestone, tags

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
