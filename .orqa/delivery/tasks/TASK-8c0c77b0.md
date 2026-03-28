---
id: "TASK-8c0c77b0"
type: "task"
title: "Task Dependency Mechanism"
description: "Add a formal depends-on field to the task schema in the artifact framework, add a dependency gate to the task lifecycle rules, and backfill existing tasks that have implicit dependencies."
status: archived
created: 2026-03-09T00:00:00.000Z
updated: 2026-03-09T00:00:00.000Z
acceptance:
  - "depends-on field defined in the Task schema (artifact-framework.md)"
  - "Task lifecycle rule updated with dependency gate (todo → in-progress blocked if deps not done)"
  - "Field ordering convention updated to include depends-on"
  - "Traceability web updated to show task-level dependencies"
relationships:
  - target: "EPIC-7394ba2a"
    type: "delivers"
    rationale: "Task belongs to this epic"
---

## Context

Tasks within an epic often have execution order dependencies (e.g., can't remove
old agents before extracting their domain knowledge into skills). Currently there
is no formal mechanism — dependencies are written as prose in the task body.

This task adds `depends-on` as a first-class frontmatter field with lifecycle
enforcement.

## What

See task description and acceptance criteria in frontmatter.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
