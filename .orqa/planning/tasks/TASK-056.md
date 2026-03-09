---
id: TASK-056
title: "Update rules for universal roles"
description: >
  Update agent-delegation.md and all other rules that reference old
  software-specific agent names to use the new universal role names
  (Implementer, Reviewer, etc.) and skill-based delegation.
status: done
epic: EPIC-045
created: 2026-03-09
updated: 2026-03-09
depends-on: [TASK-052, TASK-053]
assignee: orchestrator
skills: [orqa-governance]
scope:
  - .orqa/governance/rules/agent-delegation.md
  - .orqa/governance/rules/skill-enforcement.md
  - .orqa/governance/rules/lessons-learned.md
  - .orqa/governance/rules/honest-reporting.md
acceptance:
  - agent-delegation.md rewritten for 7 universal roles instead of 16 agents
  - Delegation table uses role + skill pattern (e.g. "Implementer + backend skills")
  - Resource safety table updated for universal roles
  - skill-enforcement.md updated with skills mapped to universal roles
  - lessons-learned.md references updated (code-reviewer → Reviewer + code-quality skill)
  - honest-reporting.md references updated
  - No remaining references to deleted agent names in any rule file
tags: [rules, governance, AD-029]
---

