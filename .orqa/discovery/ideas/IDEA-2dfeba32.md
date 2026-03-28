---
id: IDEA-2dfeba32
type: discovery-idea
title: Documentation as Navigation Group
description: "Convert the documentation artifact type from a single tree type to a group with child types (Architecture, Product, Development, Process, UI, Wireframes). This gives dual-level navigation like other sections instead of a single collapsible tree."
status: archived
created: 2026-03-07
updated: 2026-03-07
research-needed:
  - Verify config-only change works without code changes
  - Assess whether doc categories become their own governance concern
relationships:
  - target: EPIC-dcc0bac6
    type: realises
  - target: PILLAR-c9e0a695
    type: grounded
  - target: PERSONA-477971bf
    type: benefits
---

## Motivation

The collapsible tree navigation for documentation is harder to use
than the dual-level navigation (group + type) that other sections
like Planning and Governance use. Converting documentation to a
group with child types would give users the same navigation pattern.

## Feasibility (Initial Assessment)

This appears to be a **config-only change** in `project.json`:

```json
// Current: single type with tree scanning
{ "key": "docs", "label": "Documentation", "icon": "file-text", "path": ".orqa/documentation" }

// Proposed: group with child types
{ "key": "docs", "label": "Documentation", "icon": "file-text", "children": [
  { "key": "architecture", "label": "Architecture", "icon": "layers", "path": ".orqa/documentation/architecture" },
  { "key": "product", "label": "Product", "icon": "compass", "path": ".orqa/documentation/product" },
  { "key": "development", "label": "Development", "icon": "code", "path": ".orqa/documentation/development" },
  { "key": "process", "label": "Process", "icon": "workflow", "path": ".orqa/documentation/process" },
  { "key": "ui", "label": "User Interface", "icon": "layout", "path": ".orqa/documentation/ui" },
  { "key": "wireframes", "label": "Wireframes", "icon": "palette", "path": ".orqa/documentation/wireframes" }
]}
```

The infrastructure already supports groups with child types. README.md
files in subdirectories already have label/icon/sort metadata.

## Risk

Documentation categories should NOT become a separate governance concern.
Documentation is still documentation — the change is purely for display
navigation, not for introducing new artifact lifecycle rules per category.
