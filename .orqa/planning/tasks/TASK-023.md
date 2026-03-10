---
id: TASK-023
title: Streaming and Conversation UX Fixes
status: done
epic: EPIC-038
description: "Fixes a set of UX issues found during early dogfooding: broken streaming, unreadable tool output, cluttered tool display, unnamed sessions, and a generic titlebar."
created: 2026-03-06
updated: 2026-03-09
assignee: frontend-engineer
skills: [svelte5-best-practices, orqa-streaming]
scope: [ui/lib/components/conversation/, ui/lib/stores/conversation.svelte.ts, sidecar/src/]
acceptance: [NDJSON streaming handles partial messages correctly, Tool output truncated at 500 chars with "Show more", Consecutive same-tool calls grouped, Sessions auto-named after first response, Custom branded titlebar]
---

## What

Fix UX issues discovered during early dogfooding: broken streaming, unreadable
tool output, cluttered tool display, unnamed sessions, and generic titlebar.

## Outcome

All issues fixed in a single sprint commit. Streaming robust, tool display
clean, sessions named, titlebar branded. Git commits: `0aab794`, `7a954d9`.
