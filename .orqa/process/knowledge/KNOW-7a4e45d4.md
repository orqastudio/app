---
id: KNOW-7a4e45d4
type: knowledge
title: "Thinking Mode: General"
thinking-mode: general
priority-rules: [process, safety]
description: "The user's prompt does not match a specific thinking mode — apply default process and safety rules with no special emphasis."
status: active
created: 2026-03-23
updated: 2026-03-23
relationships: []
tier: "always"
roles:
  - "*"
tags:
  - "thinking-mode"
  - "general"
  - "default"
priority: "P0"
summary: |
  Default thinking mode when no specific mode matches. Applies standard process
  and safety rules without special emphasis. Fallback for ambiguous or
  conversational prompts.
---

# Thinking Mode: General

The user's prompt does not clearly match a specific thinking mode. This is the default mode — apply standard process and safety rules without special emphasis on any particular category.

## Example Signals

"what's the status", "continue", "go ahead", "let's start", "next", short acknowledgements, ambiguous instructions, conversational messages

## What the Agent Needs

- Standard process rules (delegation, honest reporting)
- Standard safety rules (no force push, no stubs)
- Session state awareness — check where we left off

## Distinguishing from Similar Modes

- This is the fallback when no other mode matches clearly
- If the prompt has ANY specific signal (build, debug, plan, review, research, document, govern), use that mode instead
- General mode should rarely be selected by the semantic classifier — it exists primarily for the keyword fallback
