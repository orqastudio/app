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

Two modes to preserve artifact data by default:

**Default: `orqa clean`** (safe)

- Removes ONLY generated outputs — not user-authored artifacts
- Cleans .orqa/configs/ (generated enforcement configs)
- Cleans .orqa/workflows/*.resolved.yaml (generated workflows)
- Cleans .orqa/schema.composed.json (generated schema)
- Cleans .orqa/project.json (regenerated from plugin manifests)
- Does NOT delete user-authored artifacts (rules, decisions, lessons, knowledge, epics, tasks, etc.)
- Safe to run anytime — `orqa setup` or `orqa plugin install` regenerates everything
- Use case: fresh regeneration after plugin changes, troubleshooting stale configs

**Full: `orqa clean --full`** (destructive)

- Removes the entire .orqa/ directory
- Deletes ALL artifacts including user-authored content
- Equivalent to starting from scratch
- Requires confirmation prompt ("This will delete all governance artifacts. Continue? y/N")
- Use case: complete project reset, removing OrqaStudio from a project entirely
