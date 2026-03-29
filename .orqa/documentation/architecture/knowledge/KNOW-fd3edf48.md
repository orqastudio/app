---
id: KNOW-fd3edf48
type: knowledge
status: active
title: Governance Artifacts Structure
domain: architecture
description: Target .orqa/ directory structure organized by methodology stage — where each artifact type lives and why
tier: always
relationships:
  synchronised-with: DOC-fd3edf48
---

# Governance Artifacts Structure

## Core Rule

Every `.orqa/` artifact must answer: "would the finished app have created this file, in this format, in this location?"

## Target `.orqa/` Structure

```text
.orqa/
  project.json                    # Project config
  manifest.json                   # Installed plugin registry
  schema.composed.json            # Generated: composed schema
  prompt-registry.json            # Generated: knowledge registry
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
    ideas/    research/    personas/    pillars/    vision/    wireframes/

  planning/                       # Planning stage
    ideas/    research/    decisions/    wireframes/

  documentation/                  # Documentation stage
    <topic>/
      *.md                        # DOC artifacts
      knowledge/                  # KNOW artifacts (agent-consumable chunks)

  implementation/                 # Implementation stage
    milestones/    epics/    tasks/    ideas/

  learning/                       # Learning stage
    lessons/    decisions/    rules/
```text

## Key Organizational Principles

- **Stage-first** — directories map to methodology stages, not artifact types
- Knowledge lives **alongside** its parent documentation (not a separate flat store)
- Decisions split by level: `planning/decisions/` (tactical) vs `learning/decisions/` (principle/architectural)
- Wireframes are their own artifact type — NOT DOC
- No `process/` nesting — gone entirely
- No `agents/` directory — agents are ephemeral, not tracked
- No `grounding/` directory — becomes `tier: always` knowledge in plugins
- Composed schema and prompt registry are explicit generated artifacts

## What Has Changed from Previous State

| Old | New |
| ----- | ----- |
| `process/` nesting | Removed — categories promoted to top-level |
| `delivery/` | Replaced by `implementation/` |
| `agents/` directory | Deleted — agents are ephemeral |
| `grounding/` | Migrated to `tier: always` knowledge in plugins |
| Flat knowledge dirs | Knowledge lives with parent docs in subdirs |

## Relationships Define Flow

Relationships (forward-only, graph computes inverses) are the connective tissue within and between workflows. Graph engine provides traceability end-to-end.
