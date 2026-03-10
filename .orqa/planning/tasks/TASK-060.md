---
id: TASK-060
title: Update rules to reference pillar artifacts generically
description: Update rules to reference pillar artifacts generically
status: done
created: 2026-03-09
updated: 2026-03-09
epic: EPIC-046
depends-on:
  - TASK-058
assignee: orchestrator
skills:
  - orqa-governance
scope:
  - .orqa/governance/rules/vision-alignment.md
  - .orqa/governance/rules/pillar-alignment-docs.md
acceptance:
  - vision-alignment.md references pillar artifacts by path
  - not hardcoded names
  - pillar-alignment-docs.md reads pillar titles from artifacts directory
  - Rules enforce "serve at least one active pillar" generically
  - No hardcoded pillar names remain in enforcement rules
---