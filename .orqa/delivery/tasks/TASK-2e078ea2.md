---
id: TASK-2e078ea2
title: Build MilestoneContextCard and new dashboard layout shell
description: "Replace the existing dashboard layout with a narrative flow structure: milestone context at top, three columns (Where You Are, How You're Improving, What's Next), and a collapsible section at the bottom. Build the MilestoneContextCard component showing the active milestone."
status: completed
priority: P1
scoring:
  impact: 4
  urgency: 4
  complexity: 3
  dependencies: 4
created: 2026-03-14
updated: 2026-03-14
acceptance:
  - "Dashboard layout replaced with narrative flow structure (milestone top, three columns, collapsible bottom)"
  - "MilestoneContextCard shows active milestone title, gate question, P1 epic progress bar, and deadline"
  - "Empty state displayed when no active milestone exists, with link to Roadmap"
relationships:
  - target: EPIC-11561c51
    type: delivers
    rationale: Foundation layout task for the dashboard redesign
  - target: TASK-75f8d4bf
    type: depended-on-by
  - target: TASK-c68a16cf
    type: depended-on-by
  - target: TASK-79f51e7d
    type: depended-on-by
  - target: TASK-cae0706a
    type: depended-on-by
---
