---
id: IDEA-009
title: Integration Ecosystem
description: "Connect OrqaStudio with external tools and workflows including git, issue trackers, documentation platforms, and CI/CD."
status: captured
created: 2026-03-07
updated: 2026-03-13
horizon: someday
research-needed:
  - "Priority integrations (git, issue trackers)"
  - Bidirectional sync architecture
  - Import format support
relationships:
  - target: DOC-071
    type: informed-by
    rationale: "Auto-generated inverse of documented-by relationship from DOC-071"
  - target: PILLAR-001
    type: grounded-by
---
## Candidate Items

- Git integration — branch awareness, commit correlation with decisions
- Issue tracker sync — bidirectional with GitHub Issues, Linear, Jira
- Documentation platform export — Notion, Confluence, static sites
- Calendar integration — experiment timelines and retrospective schedules
- Notification channels — Slack/Teams/email for artifact changes
- CI/CD integration — pull quality gate results into metrics
- Import from existing tools — Notion, Obsidian, markdown repos
