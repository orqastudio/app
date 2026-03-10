---
id: TASK-048
title: Classify skills with layer field
status: done
epic: EPIC-044
description: Adds a layer field to every skill definition, distinguishing portable canon skills from OrqaStudio-specific project skills, and documents the classification in the skill enforcement rule.
created: 2026-03-09
updated: 2026-03-09
assignee: agent-maintainer
skills: [orqa-governance]
scope: [.orqa/team/skills/]
acceptance: ["All skill SKILL.md files have `layer:` field (canon/project/plugin)", Canon skills are portable platform skills, Project skills are orqa-specific skills, Classification documented in skill-enforcement.md]
---

## Classification Plan

| Skill | Layer | Rationale |
|-------|-------|-----------|
| code-search | canon | Universal search capability |
| chunkhound | canon | Universal search capability |
| planning | canon | Platform process principle |
| skills-maintenance | canon | Platform lifecycle |
| architecture | canon | Portable architecture patterns |
| uat-process | canon | Platform testing methodology |
| orqa-composability | canon | Platform design philosophy |
| svelte5-best-practices | canon | Portable technology skill |
| typescript-advanced-types | canon | Portable technology skill |
| tailwind-design-system | canon | Portable technology skill |
| rust-async-patterns | canon | Portable technology skill |
| tauri-v2 | canon | Portable technology skill |
| orqa-ipc-patterns | project | OrqaStudio-specific IPC patterns |
| orqa-store-patterns | project | OrqaStudio-specific store patterns |
| orqa-store-orchestration | project | OrqaStudio-specific store coordination |
| orqa-streaming | project | OrqaStudio-specific streaming pipeline |
| orqa-domain-services | project | OrqaStudio-specific domain services |
| orqa-repository-pattern | project | OrqaStudio-specific persistence |
| orqa-error-composition | project | OrqaStudio-specific error handling |
| orqa-governance | project | OrqaStudio-specific governance |
| orqa-testing | project | OrqaStudio-specific testing |
| orqa-native-search | project | OrqaStudio-specific search engine |
