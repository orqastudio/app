---
id: TASK-482
title: "Build LessonVelocityWidget and DecisionQueueWidget"
description: "Build two widgets for the 'What's Next' column. LessonVelocityWidget shows the lesson pipeline stages (Draft, Review, Promoted, Active) with item counts per stage. DecisionQueueWidget shows pending decisions and blockers with context and days pending."
status: todo
priority: P1
created: 2026-03-14
updated: 2026-03-14
epic: EPIC-074
depends-on:
  - TASK-479
acceptance:
  - LessonVelocityWidget shows lesson pipeline stages (Draft, Review, Promoted, Active) with counts per stage
  - DecisionQueueWidget shows pending decisions/blockers with context and days pending
  - Both widgets placed in the "What's Next" column
relationships:
  - target: EPIC-074
    type: delivers
    rationale: Lesson velocity and decision queue widgets for the dashboard redesign
  - target: EPIC-074
    type: belongs-to
    rationale: Task belongs to this epic
---
