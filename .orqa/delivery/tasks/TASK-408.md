---
id: TASK-408
title: Humanize agent capability display and capture subagent mapping idea
description: Display agent capabilities as human-readable labels instead of raw identifiers. Capture sidecar-specific subagent mapping as an idea.
status: completed
priority: P2
created: 2026-03-13
updated: 2026-03-13
assignee: null
acceptance:
  - "Agent capabilities displayed with human-readable labels (e.g., 'Read Files' not 'file_read')"
  - Capability label mapping is centralized and reusable
  - IDEA-089 created for sidecar-specific subagent mapping
relationships:
  - target: EPIC-063
    type: delivers
    rationale: Theme F — agent display improvements from UAT
  - target: SKILL-030
    type: grounded-by
  - target: SKILL-042
    type: grounded-by
---

## Scope

### Capability Labels (Finding #19)
- **File**: `ui/src/lib/components/artifact/AgentViewer.svelte` (lines 38-56)
- **Current**: Raw identifiers displayed via `MetadataRow`
- **Pattern**: `FrontmatterHeader.svelte` already has `TOOL_LABELS` map (lines 154-165) — extend or reuse
- **Mapping**: `file_read` → "Read Files", `shell_execute` → "Run Commands", `code_search_semantic` → "Semantic Search", etc.

### Subagent Mapping Idea (Finding #18)
- Create IDEA-089 capturing that subagent mapping on agents should be sidecar-specific, not part of the universal agent model
