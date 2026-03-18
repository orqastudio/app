---
id: IMPL-010
title: Documentation Must Be App-Native
description: External documentation tools (like Docsify) create parallel navigation and rendering systems that diverge from the app. First-class features should be rendered natively by the app itself.\n
status: active
created: 2026-03-07
updated: 2026-03-07
maturity: understanding
recurrence: 1
relationships: []
---
## What Happened

The initial project used Docsify (a JavaScript documentation site generator) to render project documentation. This required maintaining a separate `index.html`, `_sidebar.md`, `custom.css`, and `.nojekyll` file. The sidebar had its own synchronization rule to keep it aligned with the file structure. This was all removed in favor of native artifact rendering within OrqaStudio itself.

## Why It Was Unexpected

Docsify seemed like a quick way to get documentation browsing working — it's a mature tool that just works. But it created a parallel system: the app had its own artifact browser AND Docsify had its own sidebar and rendering. The two couldn't share state, navigation, or search. The sidebar-synchronization rule was the symptom — maintaining two parallel navigation structures is inherently fragile.

## The Correct Approach

Documentation browsing is a first-class feature of OrqaStudio, not a bolted-on external tool. The artifact scanner reads markdown files directly, extracts frontmatter, and renders them in the app's artifact panel. This was formalized in [AD-020](AD-020) and [AD-022](AD-022).

## Git Evidence

- `5fb0454` — Docsify removal, sidebar-synchronization rule deleted
- `088a7bf` — Fix artifact viewers for native rendering

## Pattern

See description in frontmatter.

## Fix

Fix approach documented at time of lesson capture.
