---
id: EPIC-023
title: Onboarding Flow Review
description: Audit and improve the first-run setup wizard, new project flow, existing folder initialization, and guided first conversation.
status: draft
priority: P2
created: 2026-03-07
updated: 2026-03-07
milestone: MS-002
pillars:
  - PILLAR-001
depends-on:
  - EPIC-011
blocks: []
docs-required:
  - docs/wireframes/settings-onboarding.md
  - docs/architecture/setup-wizard.md
docs-produced:
  - docs/wireframes/settings-onboarding.md (update with reviewed flows)
scoring:
  pillar: 3
  impact: 4
  dependency: 1
  effort: 2
  score: 10
---
## Tasks

- [ ] Audit first-run setup wizard — verify each step is clear and skippable
- [ ] New Project flow — scaffold sensible project structure with optional templates
- [ ] Initialize Existing Folder — improve scan results presentation
- [ ] Guided first conversation — suggest first task after project setup
- [ ] Open Project validation — graceful handling of corrupted `.orqa/`
