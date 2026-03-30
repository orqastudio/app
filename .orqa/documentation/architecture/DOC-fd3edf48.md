---
id: DOC-fd3edf48
type: doc
status: active
title: Governance Artifacts
domain: architecture
description: Target .orqa/ directory structure, artifact organization by methodology stage, and relationship flow
created: 2026-03-28T00:00:00.000Z
updated: 2026-03-29T00:00:00.000Z
---

# Governance Artifacts

> This is part of the OrqaStudio Architecture Reference.

---

## 5. Governance Artifacts (`.orqa/`)

The `.orqa/` directory should contain what **a fully working app would generate**. It is not a dumping ground for hand-crafted files accumulated during development. Every artifact should be judged against: "would the finished app have created this file, in this format, in this location?"

### 5.1 Target Structure

The target structure reflects the methodology's stages and the engine's artifact categories. It should be human-navigable — organized by purpose, not by hash.

```text
.orqa/
  project.json                    # Project configuration
  manifest.json                   # Installed plugin registry (source/installed hashes)
  schema.composed.json            # Generated: composed schema from all definition plugins
  prompt-registry.json            # Generated: knowledge registry for prompt pipeline
  search.duckdb                   # Semantic search index

  workflows/                          # Generated: resolved workflows, one per methodology stage
    agile-methodology.resolved.json  # The full resolved methodology
    discovery.resolved.json          # Resolved discovery workflow (embeds artifact_types state machines)
    planning.resolved.json           # Resolved planning workflow (embeds artifact_types state machines)
    documentation.resolved.json      # Resolved documentation workflow (embeds artifact_types state machines)
    implementation.resolved.json     # Resolved implementation workflow (embeds artifact_types state machines)
    review.resolved.json             # Resolved review workflow (embeds artifact_types state machines)
    learning.resolved.json           # Resolved learning workflow (embeds artifact_types state machines)
    <artifact-type>.resolved.json    # Per-artifact-type resolved files (one per type, same structure)

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
    decisions/                    # PLANNING-DECISION artifacts
    wireframes/                   # WIREFRAME artifacts (planning-scoped)

  documentation/                  # Documentation stage
    <categorized by topic>/       # Organized into meaningful subdirectories by plugin authors
      *.md                        # Full documentation (DOC artifacts)
      knowledge/                  # Agent-consumable chunks derived from the docs (KNOW artifacts)

  implementation/                 # Implementation stage
    milestones/                   # MS artifacts
    epics/                        # EPIC artifacts
    tasks/                        # TASK artifacts
    ideas/                        # Implementation-scoped ideas

  learning/                       # Learning stage
    lessons/                      # IMPL (lesson) artifacts
    decisions/                    # PRINCIPLE-DECISION artifacts (overarching architecture)
    rules/                        # RULE artifacts
```text

**Key organizational principle:** The directory structure maps to methodology stages. Artifacts live within the stage they belong to, organized by type within each stage. This mirrors the navigation structure in the app.

**Key differences from current state:**

- **Stage-first organization** — directories map to methodology stages, not artifact types
- No `process/` nesting — gone entirely
- No `delivery/` — replaced by `implementation/` (the methodology stage name)
- No `agents/` directory — agents are ephemeral (generated and discarded), not tracked
- No `grounding/` directory — grounding content becomes `tier: always` knowledge in plugins
- No source workflow definitions — only resolved output (sources stay in plugin directories)
- Decisions split by level: `planning/decisions/` (tactical) and `learning/decisions/` (architectural/principle)
- Knowledge lives WITH documentation — knowledge is documentation split into agent-consumable chunks with injection metadata
- Wireframes are their own artifact type, not DOC
- Resolved workflows are JSON files named by stage (`.resolved.json`), one per stage plus per-artifact-type files; each stage file embeds per-type state machines under the `artifact_types` key
- Composed schema and prompt registry are explicit generated artifacts

### 5.2 Relationships Define Flow

Relationships are the connective tissue of the governance model:

- **Within a workflow:** relationships define the flow between artifacts (e.g., task delivers epic, research informs decision)
- **Between workflows and methodology:** relationships define how each workflow's outputs connect to the broader methodological flow (e.g., discovery outputs feed planning inputs, implementation delivers against planning)

The graph engine computes inverses from forward-only declared relationships. The relationship types are semantic bonds that make the entire methodology traceable end-to-end.
