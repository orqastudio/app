---
id: TASK-469
title: "Restructure main nav: Exploring → Delivery → Documentation → Process"
description: "Overhaul the main navigation to match the conceptual model. Split Exploring (ideas, research) from Delivery (roadmap, milestones, epics, tasks). Reorder to Exploring → Delivery → Documentation → Process. Fix missing icons. Remove or repurpose Verification."
status: todo
priority: P1
created: 2026-03-14
updated: 2026-03-14
epic: EPIC-073
depends-on: []
assignee: null
skills:
  - SKILL-030
  - SKILL-042
acceptance:
  - Main nav has 4 sections in order: Exploring, Delivery, Documentation, Process
  - Exploring contains ideas and research
  - Delivery contains roadmap (top), milestones, epics, tasks
  - Documentation has an icon
  - Verification either removed or has an icon and clear purpose
  - All nav items render correctly with correct sub-items
relationships:
  - target: EPIC-073
    type: delivers
    rationale: Navigation architecture overhaul (F17-F24)
  - target: EPIC-073
    type: belongs-to
    rationale: Task belongs to this epic
---
