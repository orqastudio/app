---
id: EPIC-009
title: Frontend Test Suite
description: Establish a frontend test suite using Vitest for Svelte stores and components, covering the gap left by zero frontend tests.
status: draft
priority: P2
created: 2026-03-07
updated: 2026-03-07
milestone: MS-001
pillars:
  - PILLAR-001
research-refs: []
docs-required: []
docs-produced: []
scoring:
  pillar: 3
  impact: 3
  dependency: 1
  effort: 3
  score: 5
---
## Why P2

465 Rust tests exist but zero frontend tests. Changes to stores break components silently. This is a learning gap — without tests, regression patterns can't be detected.

## Tasks

- [ ] Vitest setup for Svelte component and store testing
- [ ] Store unit tests (conversation, session, project, settings — state transitions, reactive updates)
- [ ] Component tests for critical UI (ConversationView, ToolApprovalDialog, SessionDropdown)
- [ ] IPC contract tests — verify invoke calls match actual Tauri commands

## Context

This epic addresses a need identified during project development.

## Implementation Design

Implementation approach to be defined during planning.
