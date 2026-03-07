---
id: EPIC-008
title: "Code Quality Audit"
status: draft
priority: P2
milestone: MS-001
created: 2026-03-07
updated: 2026-03-07
deadline: null
plan: null
depends-on: []
blocks: []
assignee: null
pillar:
  - clarity-through-structure
  - learning-through-reflection
scoring:
  pillar: 4
  impact: 3
  dependency: 1
  effort: 3
score: 6.3
roadmap-ref: "D8"
docs-required:
  - docs/development/coding-standards.md
docs-produced:
  - docs/development/coding-standards.md (update if gaps found)
  - .orqa/lessons/ (new lessons from audit findings)
tags: [audit, quality, standards]
---

# Code Quality Audit

## Why P2

Can't credibly enforce quality on managed projects if our own code has violations. The audit is also a learning loop input — findings feed into lessons and coding standards.

## Tasks

- [ ] Coding standards compliance audit against `docs/development/coding-standards.md`
- [ ] Enforcement artifact review — rules/hooks/skills completeness
- [ ] Abstraction pattern audit — identify over-complicated patterns from iterative development
- [ ] Fix function size violations in `tool_executor.rs` (`tool_bash` 97 lines, `execute_tool` 69 lines, `project_root_from_state` 152 lines)
