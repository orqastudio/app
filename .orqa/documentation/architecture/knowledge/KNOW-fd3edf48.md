---
id: KNOW-fd3edf48
type: knowledge
status: active
title: ".orqa/ Directory Structure and Artifact Organization"
domain: architecture
description: "Target .orqa/ directory structure organized by methodology stage — where each artifact type lives and why"
tier: always
created: 2026-03-28
roles: [governance-steward, writer, implementer, reviewer]
paths: [.orqa/]
tags: [governance, artifacts, directory-structure, organization]
relationships:
  - type: synchronised-with
    target: DOC-fd3edf48
---

# .orqa/ Directory Structure and Artifact Organization

## Core Rule

Every `.orqa/` artifact must answer: "would the finished app have created this file, in this format, in this location?"

If not — it does not belong there, regardless of how long it has been there.

## Target `.orqa/` Structure

```text
.orqa/
  project.json                    # Project config
  manifest.json                   # Installed plugin registry (source/installed hashes)
  schema.composed.json            # Generated: composed schema from all plugins
  prompt-registry.json            # Generated: knowledge registry for prompt pipeline
  search.duckdb                   # Semantic search index

  workflows/                      # Generated: one per stage
    methodology.resolved.yaml
    discovery.resolved.yaml
    planning.resolved.yaml
    documentation.resolved.yaml
    implementation.resolved.yaml
    review.resolved.yaml
    learning.resolved.yaml

  discovery/                      # Discovery stage
    ideas/                        # DISCOVERY-IDEA artifacts
    research/                     # DISCOVERY-RESEARCH artifacts
    personas/                     # PERSONA artifacts
    pillars/                      # PILLAR artifacts
    vision/                       # VISION artifacts
    wireframes/                   # WIREFRAME artifacts

  planning/                       # Planning stage
    ideas/                        # PLANNING-IDEA artifacts
    research/                     # PLANNING-RESEARCH artifacts
    decisions/                    # PLANNING-DECISION artifacts (tactical)
    wireframes/                   # WIREFRAME artifacts (planning-scoped)

  documentation/                  # Documentation stage
    <topic>/                      # Organized by topic (plugin authors choose)
      *.md                        # DOC artifacts (human-readable)
      knowledge/                  # KNOW artifacts (agent-consumable chunks)

  implementation/                 # Implementation stage
    milestones/                   # MS artifacts
    epics/                        # EPIC artifacts
    tasks/                        # TASK artifacts
    ideas/                        # Implementation-scoped ideas

  learning/                       # Learning stage
    lessons/                      # IMPL (lesson) artifacts
    decisions/                    # PRINCIPLE-DECISION artifacts (architectural)
    rules/                        # RULE artifacts
```text

## Key Organizational Principles

- **Stage-first** — directories map to methodology stages, not artifact types
- Knowledge lives **alongside** its parent documentation (inside `knowledge/` subdirs, not a flat store)
- Decisions split by level: `planning/decisions/` (tactical, evolving) vs `learning/decisions/` (principle, architectural, rarely changes)
- Wireframes are their own artifact type — NOT DOC artifacts
- No `process/` nesting — gone entirely
- No `agents/` directory — agents are ephemeral, generated per task, not tracked as artifacts
- No `grounding/` directory — becomes `tier: always` knowledge in plugins
- Composed schema and prompt registry are explicit generated artifacts (tracked, not gitignored)

## What Has Changed from Previous State

| Old | New |
| ----- | ----- |
| `process/` nesting | Removed — categories promoted to top-level |
| `delivery/` | Replaced by `implementation/` (the methodology stage name) |
| `agents/` directory | Deleted — agents are ephemeral |
| `grounding/` | Migrated to `tier: always` knowledge in plugins |
| Flat knowledge dirs | Knowledge lives with parent docs in `knowledge/` subdirs |
| Source workflows in `.orqa/` | Only resolved output here; sources stay in plugin dirs |

## Artifact Type to Location Mapping

| Artifact Type | ID Prefix | Location |
| -------------- | ---------- | --------- |
| doc | DOC- | `documentation/<topic>/` |
| knowledge | KNOW- | `documentation/<topic>/knowledge/` |
| persona | PERSONA- | `discovery/personas/` |
| pillar | PILLAR- | `discovery/pillars/` |
| vision | VISION- | `discovery/vision/` |
| wireframe | WIREFRAME- | `discovery/wireframes/` or `planning/wireframes/` |
| epic | EPIC- | `implementation/epics/` |
| task | TASK- | `implementation/tasks/` |
| milestone | MS- | `implementation/milestones/` |
| lesson | IMPL- | `learning/lessons/` |
| rule | RULE- | `learning/rules/` |
| principle-decision | PD- | `learning/decisions/` |
| planning-decision | PLANNING- | `planning/decisions/` |

## Relationships Define Flow

Relationships (forward-only; graph computes inverses) are the connective tissue within and between workflows. The graph engine provides end-to-end traceability across the entire methodology.
