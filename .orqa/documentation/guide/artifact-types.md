---
id: DOC-081
title: Artifact Types
description: Overview of all artifact types in OrqaStudio, their purposes, lifecycles, and relationships.
created: "2026-03-14"
updated: "2026-03-14"
---

## Overview

OrqaStudio uses typed markdown artifacts to represent structured knowledge. Each artifact type has a defined schema, lifecycle, and set of relationships to other types. Together, they form the artifact graph that drives the dashboard, navigation, and integrity checking.

## Delivery Artifacts

These track what the project is building and why.

### Ideas

**Directory:** `.orqa/delivery/ideas/` | **Prefix:** IDEA

Ideas capture future possibilities. They follow a structured lifecycle: `captured` (just recorded) to `exploring` (being investigated) to `shaped` (scoped and validated) to `promoted` (converted to an epic) or `archived` (not pursued).

Ideas carry a `research-needed` field listing questions that must be answered before promotion. This ensures ideas are validated before committing resources.

### Research

**Directory:** `.orqa/delivery/research/` | **Prefix:** RES

Research documents capture investigation findings. They answer questions raised by ideas, epics, or decisions. Research has a simple lifecycle: `draft` to `complete` to `surpassed` (when newer research supersedes it).

Research is never deleted, only surpassed. It serves as the historical record of why decisions were made.

### Milestones

**Directory:** `.orqa/delivery/milestones/` | **Prefix:** MS

Milestones represent strategic goals with a gate question. A milestone is `complete` when its gate question can be answered "yes" and all P1 epics within it are done. Milestones flow: `planning` to `active` to `complete`.

### Epics

**Directory:** `.orqa/delivery/epics/` | **Prefix:** EPIC

Epics are the primary unit of planned work. They contain the implementation design, documentation gates (`docs-required`, `docs-produced`), and priority assessment. Epics flow: `draft` to `ready` to `in-progress` to `review` to `done`.

An epic cannot move to `ready` until all its `docs-required` items exist. It cannot be marked `done` until all `docs-produced` items are verified.

### Tasks

**Directory:** `.orqa/delivery/tasks/` | **Prefix:** TASK

Tasks are individual work items within an epic. They carry acceptance criteria that define what "done" means. Tasks flow: `todo` to `in-progress` to `done`.

Tasks support dependency tracking via `depends-on`. A task cannot start until all its dependencies are done.

## Process Artifacts

These define how the project operates.

### Pillars

**Directory:** `.orqa/process/pillars/` | **Prefix:** PILLAR

Pillars are the guiding principles for the project. Every feature must trace to at least one pillar. Pillars carry gate questions used to evaluate alignment.

### Rules

**Directory:** `.orqa/process/rules/` | **Prefix:** RULE

Rules enforce conventions, standards, and constraints. They carry an `enforcement` array that maps to automated checks (pre-commit hooks, linter rules, skill injections). Rules are `active` or `inactive`.

### Decisions

**Directory:** `.orqa/process/decisions/` | **Prefix:** AD

Architecture decisions record choices with rationale and consequences. They follow: `proposed` to `accepted` to `superseded` or `deprecated`. When one decision supersedes another, both must be updated in the same commit.

### Lessons

**Directory:** `.orqa/process/lessons/` | **Prefix:** IMPL

Lessons capture implementation patterns discovered during work. They track recurrence count and can be promoted to rules or skill updates when a pattern recurs frequently.

### Skills

**Directory:** `.orqa/process/skills/` | **Prefix:** SKILL

Skills are reusable knowledge packages that agents load before performing work. They carry domain expertise (e.g., Svelte 5 patterns, Rust error handling, IPC conventions) and are injected into agent context at task start.

### Agents

**Directory:** `.orqa/process/agents/` | **Prefix:** AGENT

Agent definitions declare universal roles (Implementer, Reviewer, Researcher, etc.) with capabilities, skills, and required reading. They define what each role can do and what knowledge it needs.

## Documentation

**Directory:** `.orqa/documentation/` | **Prefix:** DOC

Documentation pages describe the current target state of the project. Unlike research and tasks (which are historical records), documentation is deleted and replaced when it becomes outdated. Git history preserves old versions.

## Relationship Types

Artifacts connect through typed relationships. Each relationship has an inverse:

| Type | Inverse | Usage |
|------|---------|-------|
| `delivers` | `delivered-by` | Task/epic delivers a milestone/epic |
| `depends-on` | `depended-on-by` | Task depends on another task |
| `informs` | `informed-by` | Research informs a decision or epic |
| `enforces` | `enforced-by` | Rule enforces a decision or standard |
| `documents` | `documented-by` | Documentation page describes an artifact |
| `observes` | `observed-by` | Lesson observes a pattern |
| `supersedes` | `superseded-by` | Decision replaces another |
| `scoped-to` | `scoped-by` | Rule/skill scoped to an agent |

The integrity engine validates that all relationships are bidirectional and that targets resolve to existing artifacts.
