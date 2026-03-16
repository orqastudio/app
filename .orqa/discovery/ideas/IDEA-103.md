---
id: IDEA-103
title: Automated document content audit with task-level enforcement
description: Automated system that audits documentation for staleness and accuracy. Tasks that affect documentation have required documentation update acceptance criteria. Ensures docs stay current as code evolves.
status: captured
created: 2026-03-15
updated: 2026-03-15
horizon: next
research-needed: []
relationships:
  - target: PILLAR-002
    type: grounded-by
  - target: PILLAR-003
    type: grounded-by
---

## Motivation

Documentation drift is a persistent problem. Code changes without corresponding doc updates, and there is no automated mechanism to detect when a doc page has become stale relative to the implementation it describes. RULE-008 requires documentation-first, but there is no enforcement that docs were actually updated after implementation. An automated audit system would compare doc content against linked code artifacts, flag pages that reference changed files, and enforce that tasks with a `docs-produced` field have those docs actually updated before the task can close.

## Sketch

Two components: (1) a staleness detector that hashes documentation pages against the code files they reference (via `@orqa` comment links from IDEA-100 or explicit `references` frontmatter), alerting when linked code changes without a corresponding doc update; (2) a task-level gate that blocks task completion unless all `docs-produced` items have a recent commit touching them. The audit surfaces in the OrqaStudio dashboard as a documentation health score. Could integrate with the pre-commit hook to warn when staged code changes have linked docs that were not also staged.
