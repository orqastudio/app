---
id: RES-a2a77d0c
type: planning-research
title: Native Tool Display Names and Icons
description: "Design for presenting AI tool calls in a user-friendly way: human-readable names, Lucide icons, de-duplication of repeated calls, and summary grouping."
status: completed
created: 2026-03-04
updated: 2026-03-09
relationships:
  - target: EPIC-4cec22ea
    type: guides
    rationale: "Research findings informed the design of Native Tool UX & First-Run Setup"
---

## Problem

Raw tool calls from the Agent SDK show technical names like `Read`, `Edit`, `Bash` with
JSON parameters. Users need to understand what the AI is doing without reading raw JSON.

## Design

1. **Friendly names** — `Read` → "Reading file", `Edit` → "Editing file", `Bash` → "Running command"
2. **Lucide icons** — Each tool type gets a consistent icon (FileText, Pencil, Terminal, etc.)
3. **Parameter extraction** — Show the most important parameter (file path, command) in the summary
4. **De-duplication** — Consecutive calls to the same tool are grouped ("Read 3 files")
5. **Collapsible detail** — Click to expand the full tool call with input/output

## Git Evidence

- `b0ee670` — Phase 1: Native tool UX implementation
- `00f2754` — Sidecar tool routing, artifact nav
- `7a954d9` — Tool grouping and output truncation improvements
