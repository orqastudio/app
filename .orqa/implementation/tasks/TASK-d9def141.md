---
id: TASK-d9def141
type: task
title: "Metadata panel icons, remove duplicate relationship fields, spacing (F43, F44, F45, F46)"
description: "Four metadata panel fixes: add icons to all metadata fields; exclude relationship-specific fields from the metadata panel (they belong only in the relationships section); reduce the gap between metadata and acceptance criteria; add a gap between acceptance criteria and body content."
status: archived
priority: P1
scoring:
  impact: 3
  urgency: 3
  complexity: 2
  dependencies: 2
created: 2026-03-14
updated: 2026-03-14
acceptance:
  - All metadata fields have icons
  - "Relationship-specific fields (epic, milestone, depends-on, blocks, research-refs, docs-required, docs-produced, promoted-to, supersedes, superseded-by) excluded from metadata panel and shown only in relationships section"
  - Gap between metadata and acceptance criteria reduced
  - Gap added between acceptance criteria section and body content
relationships:
  - target: EPIC-c96c9f12
    type: delivers
    rationale: UAT findings F43, F44, F45, F46 — metadata panel icons, field exclusions, and spacing
---
