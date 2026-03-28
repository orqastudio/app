---
id: "TASK-ddcab065"
type: "task"
title: "Build MilestoneContextCard and new dashboard layout shell"
description: "Replace the existing dashboard layout with a narrative flow structure: milestone context at top, three columns (Where You Are, How You're Improving, What's Next), and a collapsible section at the bottom. Build the MilestoneContextCard component showing the active milestone."
status: archived
priority: "P1"
scoring:
  impact: 4
  urgency: 4
  complexity: 3
  dependencies: 4
created: 2026-03-14T00:00:00.000Z
updated: 2026-03-14T00:00:00.000Z
acceptance:
  - "Dashboard layout replaced with narrative flow structure (milestone top, three columns, collapsible bottom)"
  - "MilestoneContextCard shows active milestone title, gate question, P1 epic progress bar, and deadline"
  - "Empty state displayed when no active milestone exists, with link to Roadmap"
relationships:
  - target: "EPIC-c353971b"
    type: "delivers"
    rationale: "Foundation layout task for the dashboard redesign"
---
