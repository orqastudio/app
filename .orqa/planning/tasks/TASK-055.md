---
id: TASK-055
title: "Update product documentation"
description: >
  Update governance.md with the concept taxonomy from AD-029, create a new
  governance-hub.md page for the distribution and coexistence model from
  AD-030, and verify artifact-framework.md alignment.
status: done
epic: EPIC-045
created: 2026-03-09
updated: 2026-03-09
depends-on: [TASK-052, TASK-056]
assignee: orchestrator
skills: [orqa-governance]
scope:
  - .orqa/documentation/product/governance.md
  - .orqa/documentation/product/governance-hub.md
  - .orqa/documentation/product/artifact-framework.md
acceptance:
  - governance.md updated with concept taxonomy (agent/skill/rule/hook/lesson definitions)
  - governance.md updated with agent vs skill decision framework
  - governance-hub.md created covering distribution model, coexistence, drift reconciliation
  - governance-hub.md positions hub capability as contextual, not identity
  - artifact-framework.md verified aligned with AD-029 universal roles
  - All three pages have pillar alignment sections
tags: [documentation, product, AD-029, AD-030]
---

## Reference

- AD-029 has the concept taxonomy and decision framework
- AD-030 has the governance hub model, coexistence architecture, drift reconciliation
- vision.md Platform Identity section sets the framing (clarity engine first)
