---
id: IDEA-a2c5d8e3
type: discovery-idea
title: "CLI command naming: orqa setup + orqa clean"
description: "Rename orqa install to orqa setup (one-shot dev environment), add orqa clean (remove generated artifacts)"
status: captured
priority: high
created: "2026-03-29"
tags:
  - cli
  - commands
  - naming
---

## Problem

`orqa install` is ambiguous — it's the one-shot dev environment setup command but sounds like the plugin install mechanism. Plugin install is `orqa plugin install`. The naming causes confusion in documentation and architecture docs.

## Proposed Changes

### Rename: `orqa install` → `orqa setup`

- One-shot dev environment setup command
- Runs plugin installs, sets up git hooks, creates .orqa/ structure
- Clear distinction: `orqa setup` (environment) vs `orqa plugin install` (individual plugin)

### New: `orqa clean`

- Removes OrqaStudio-generated artifacts from a project
- Cleans .orqa/configs/ (generated enforcement configs)
- Cleans .orqa/workflows/*.resolved.yaml (generated workflows)
- Cleans .orqa/schema.composed.json (generated schema)
- Does NOT delete user-authored artifacts (rules, decisions, lessons, etc.)
- Inverse of the generation pipeline — useful for fresh regeneration or project cleanup
