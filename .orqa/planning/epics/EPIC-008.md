---
id: EPIC-008
title: Code Quality Audit
status: draft
priority: P2
milestone: MS-001
pillars: [PILLAR-001]
description: Audit the codebase against coding standards, fix violations, and feed findings into the PILLAR-002 loop as lessons.
created: 2026-03-07
updated: 2026-03-07
research-refs: []
docs-required: [docs/development/coding-standards.md]
docs-produced: [docs/development/coding-standards.md (update if gaps found), .orqa/lessons/ (new lessons from audit findings)]
scoring:
  pillar: 4
  impact: 3
  dependency: 1
  effort: 3
  score: 6.3
---

## Why P2

Can't credibly enforce quality on managed projects if our own code has violations. The audit is also a learning loop input — findings feed into lessons and coding standards.

## Tasks

- [ ] Coding standards compliance audit against `docs/development/coding-standards.md`
- [ ] Enforcement artifact review — rules/hooks/skills completeness
- [ ] Abstraction pattern audit — identify over-complicated patterns from iterative development
- [ ] Fix function size violations in `tool_executor.rs` (`tool_bash` 97 lines, `execute_tool` 69 lines, `project_root_from_state` 152 lines)
