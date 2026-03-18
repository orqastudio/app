---
id: TASK-407
title: Enhance artifact search with semantic capability and UI fixes
description: Integrate semantic/AI search for artifact discovery. Fix search results overflow and improve input contrast against modal overlay.
status: active
priority: P2
scoring:
  impact: 3
  urgency: 2
  complexity: 3
  dependencies: 1
created: 2026-03-13
updated: 2026-03-13
assignee: null
acceptance:
  - "Artifact search uses semantic/AI matching, not just substring regex"
  - Search results do not overflow their widget container
  - Search input area has clear visual contrast against the modal overlay
relationships:
  - target: EPIC-063
    type: delivers
    rationale: Theme E — search enhancement from UAT
---

## Scope

### Semantic Search (Finding #29)
- **Current**: `ArtifactSearchOverlay.svelte` uses `.toLowerCase().includes(q)` — pure substring
- **Infrastructure**: ONNX+DuckDB semantic search exists in backend but not wired to artifact search
- **Options**:
  1. Add semantic search toggle in search overlay
  2. Keep regex as fast path, add semantic suggestions below
  3. Replace regex with semantic as primary

### UI Fixes (Finding #30)
- **File**: `ArtifactSearchOverlay.svelte`
- **Overflow**: Results have `max-h-[40vh]` with ScrollArea — verify this works correctly
- **Contrast**: Search input uses `text-muted-foreground` — increase to `foreground` for primary interaction area
