---
id: "IMPL-bb34f3cf"
type: lesson
title: "Extract domain logic before command files become monolithic"
description: "Command files grow incrementally during feature work. Extract domain logic into focused service modules as soon as a command file exceeds 300 lines. Thin commands delegate to domain services.\\n"
status: "active"
created: "2026-03-07"
updated: "2026-03-07"
maturity: "understanding"
recurrence: 1
relationships: []
---
## What Happened

`stream_commands.rs` grew to 2,425 lines containing streaming orchestration, tool execution, system prompt building, and session titling — all in one file. This violated the composability principle and made parallel agent work prone to merge conflicts.

## Why It Was Unexpected

During initial implementation (Sub-Phases 3-12), the priority was getting features working end-to-end. Command files were the natural place to put logic because they had access to app state and the sidecar. The growth was incremental — each feature added 100-200 lines — and wasn't noticed until the file was unmanageable.

## The Correct Approach

Extract domain logic into focused service modules (`domain/*.rs`) as soon as a command file exceeds ~300 lines. Command handlers should be thin orchestrators (30-50 lines) that delegate to domain services. This was formalized in [AD-6cd1ff6f](AD-6cd1ff6f).

## Git Evidence

- `7fd306e` — The extraction: stream_commands.rs decomposed into 4 domain modules
- Multiple follow-up commits extracted other command files similarly

## Pattern

See description in frontmatter.

## Fix

Fix approach documented at time of lesson capture.