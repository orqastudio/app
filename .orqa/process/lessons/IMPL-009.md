---
id: IMPL-009
title: Domain-Neutral Naming Avoids Renames
description: "Provider-specific names like sdk_session_id spread across the entire stack. When the architecture evolves to support multiple providers, the rename touches every layer including database migrations.\n"
status: active
created: 2026-03-07
updated: 2026-03-07
maturity: understanding
recurrence: 1
relationships:
  - target: "PILLAR-001"
    type: "grounded"
    rationale: "Domain-neutral naming creates stable structural interfaces"
---
## What Happened

The field `sdk_session_id` was used throughout the entire stack — Rust types, TypeScript protocol, SQLite columns, and all callers (13+ files). When the architecture evolved to be provider-agnostic, renaming to `provider_session_id` required touching every layer simultaneously, including a database migration.

## Why It Was Unexpected

"sdk_session_id" seemed like a reasonable name when the only integration was the Claude Agent SDK. The coupling wasn't apparent until the provider abstraction work began — at which point the rename was unavoidable but costly.

## The Correct Approach

Use domain-neutral names for concepts that might have multiple implementations. Instead of `sdk_session_id`, use `provider_session_id` (or just `session_id`) from day one. The naming should describe the concept's role in the domain, not the technology implementing it.

## Git Evidence

- `fa8ecc7` — 13-file rename across Rust, TypeScript, SQLite migration

## Pattern

See description in frontmatter.

## Fix

Fix approach documented at time of lesson capture.
