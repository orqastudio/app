---
id: TASK-057
title: Task Dependency Mechanism
description: Add a formal depends-on field to the task schema in the artifact framework, add a dependency gate to the task lifecycle rules, and backfill existing tasks that have implicit dependencies.
status: done
created: 2026-03-09
updated: 2026-03-09
epic: EPIC-045
assignee: orchestrator
skills:
  - orqa-governance
scope:
  - .orqa/documentation/product/artifact-framework.md
  - .orqa/governance/rules/artifact-lifecycle.md
acceptance:
  - depends-on field defined in the Task schema (artifact-framework.md)
  - Task lifecycle rule updated with dependency gate (todo → in-progress blocked if deps not done)
  - Field ordering convention updated to include depends-on
  - Traceability web updated to show task-level dependencies
---
## Context

Tasks within an epic often have execution order dependencies (e.g., can't remove
old agents before extracting their domain knowledge into skills). Currently there
is no formal mechanism — dependencies are written as prose in the task body.

This task adds `depends-on` as a first-class frontmatter field with lifecycle
enforcement.
