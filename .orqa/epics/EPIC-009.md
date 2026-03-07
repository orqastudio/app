---
id: EPIC-009
title: "Frontend Test Suite"
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
  - learning-through-reflection
scoring:
  pillar: 3
  impact: 3
  dependency: 1
  effort: 3
score: 5.0
roadmap-ref: "D9"
docs-required:
  - docs/development/coding-standards.md (testing section)
docs-produced:
  - docs/development/coding-standards.md (update with frontend test patterns)
tags: [testing, vitest, frontend]
---

# Frontend Test Suite

## Why P2

465 Rust tests exist but zero frontend tests. Changes to stores break components silently. This is a learning gap — without tests, regression patterns can't be detected.

## Tasks

- [ ] Vitest setup for Svelte component and store testing
- [ ] Store unit tests (conversation, session, project, settings — state transitions, reactive updates)
- [ ] Component tests for critical UI (ConversationView, ToolApprovalDialog, SessionDropdown)
- [ ] IPC contract tests — verify invoke calls match actual Tauri commands
