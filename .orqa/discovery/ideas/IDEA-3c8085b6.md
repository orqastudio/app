---
id: IDEA-3c8085b6
title: Auto-fix null relationship targets by scanning for tracking artifacts
description: "When the integrity tool finds null relationship targets, it should scan for related IDEAs/TASKs/EPICs that could be the intended target and either auto-fix or suggest linkages. Currently null targets are only reported as warnings requiring manual resolution."
status: completed
created: 2026-03-13
updated: 2026-03-13
horizon: active
research-needed:
  - "What heuristics reliably match a null-target relationship to its tracking artifact? (keyword matching on rationale text, shared relationship edges, same epic scope?)"
  - "Should the tool auto-fix with a confirmation prompt, or just suggest candidates?"
  - "Should this extend to other integrity issues (broken links, missing inverses) or only null targets?"
relationships:
  - target: EPIC-e37794bf
    type: realises
  - target: PILLAR-569581e0
    type: grounded
  - target: PERSONA-cda6edd6
    type: benefits
---

## Motivation

Null relationship targets indicate planned-but-not-yet-created artifacts. In practice, a tracking artifact (IDEA, TASK, EPIC) almost always exists. The integrity tool currently warns but the fix is manual — an agent reads the warning, searches for the tracking artifact, and updates the relationship. This is exactly the kind of mechanical graph traversal that tooling should handle.

## Sketch

Extend `verify-pipeline-integrity.mjs` with a `--fix` mode:
1. When a null target is found, scan all IDEAs/TASKs/EPICs for references to the same concept (keyword match on the rationale text, shared parent relationships)
2. If a single strong candidate exists: auto-fix and report what was changed
3. If multiple candidates: list them as suggestions for manual resolution
4. If no candidates: keep the warning as-is (genuinely unplanned)
