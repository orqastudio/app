---
id: "EPIC-a63fde02"
type: "epic"
title: "Prompt Generation Pipeline and Knowledge Architecture"
description: "Build the five-stage prompt generation pipeline and knowledge plugin registration/injection system. Replace monolithic CLAUDE.md loading with generated, role-specific prompts assembled from plugin registries."
status: "captured"
priority: "P1"
created: 2026-03-25T00:00:00.000Z
updated: 2026-03-25T00:00:00.000Z
horizon: "active"
relationships:
  - target: "MS-b1ac0a20"
    type: "fulfils"
    rationale: "Token-efficient prompts are critical for sustainable dogfooding"
  - target: "EPIC-f6da17ed"
    type: "depends-on"
    rationale: "Prompt generation needs workflow stage context from the workflow engine"
---

## Scope

From RES-d6e8ab11 sections 5 (Knowledge Architecture) and 6 (Prompt Generation):

- Knowledge plugin manifest format with injection tiers (always, stage-triggered, on-demand)
- Plugin registry for prompt contributions (built at `orqa plugin install` time, cached)
- Schema assembly — for a (role, workflow-stage, task) tuple, collect applicable prompt sections
- Section resolution — resolve references to compressed summaries, follow cross-refs depth 1
- Token budgeting — P0 (never cut) through P3 (nice-to-have), enforce per-agent budgets
- KV-cache-aware prompt output — static core at top, dynamic at bottom, never reorder
- Conflict resolution — project rules > project knowledge > plugin knowledge > core knowledge
- On-demand knowledge retrieval via semantic search integration
- `orqa summarize` CLI command — generates summary drafts for knowledge artifacts
- Summary field in knowledge artifact frontmatter (authors write summaries, including agent authors)

## Expected Impact

Per-agent prompt size: 1,500-4,000 tokens (down from 9,500-16,500). Session total: ~80-120K tokens (down from ~300K+). Overhead ratio: 2-4x (down from 13.4x).
