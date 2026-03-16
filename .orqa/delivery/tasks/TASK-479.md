---
id: TASK-479
title: Build MilestoneContextCard and new dashboard layout shell
description: "Replace the existing dashboard layout with a narrative flow structure: milestone context at top, three columns (Where You Are, How You're Improving, What's Next), and a collapsible section at the bottom. Build the MilestoneContextCard component showing the active milestone."
status: completed
priority: P1
created: 2026-03-14
updated: 2026-03-14
acceptance:
  - "Dashboard layout replaced with narrative flow structure (milestone top, three columns, collapsible bottom)"
  - "MilestoneContextCard shows active milestone title, gate question, P1 epic progress bar, and deadline"
  - "Empty state displayed when no active milestone exists, with link to Roadmap"
relationships:
  - target: EPIC-074
    type: delivers
    rationale: Foundation layout task for the dashboard redesign
  - target: TASK-480
    type: depended-on-by
  - target: TASK-481
    type: depended-on-by
  - target: TASK-482
    type: depended-on-by
  - target: TASK-483
    type: depended-on-by
---
