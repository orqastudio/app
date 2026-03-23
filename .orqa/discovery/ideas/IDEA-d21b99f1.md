---
id: IDEA-d21b99f1
type: idea
title: Surface lesson frequency trends on the dashboard
description: Show users whether lesson creation is trending up or down over time. Decreasing lesson frequency is evidence the learning pipeline is working — the system is learning and preventing repeat mistakes. Increasing frequency may signal new domain complexity or process gaps.
status: captured
created: 2026-03-13
updated: 2026-03-13
horizon: later
research-needed:
  - "What metrics best represent learning pipeline health? (lessons/epic, lessons/week, recurrence velocity)"
  - "How should the dashboard visualise the trend — chart, sparkline, summary stat?"
  - "Should it also show promotion rate (observations → rules/skills)?"
relationships:
  - target: PILLAR-569581e0
    type: grounded
  - target: PERSONA-cda6edd6
    type: benefits
---

## Motivation

The Knowledge Maturity Pipeline's goal is to reduce recurring mistakes by promoting observations into enforcement. If the pipeline is working, lesson frequency should decrease over time as more patterns are captured and enforced. Surfacing this trend to the user gives them visibility into whether the system is actually learning, and creates a feedback loop on the process itself.