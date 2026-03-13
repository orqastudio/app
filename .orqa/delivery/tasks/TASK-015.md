---
id: TASK-015
title: Model auto-download and startup loading
description: Automatically downloads the embedding model from Hugging Face on first use and pre-loads it at startup, with download progress visible in the status bar.
status: done
created: "2026-03-04"
updated: "2026-03-09"
epic: EPIC-034
assignee: AGENT-002
skills:
  - SKILL-027
  - SKILL-032
acceptance:
  - Model auto-downloads from Hugging Face on first use
  - Pre-download at app startup with progress tracking
  - Status bar shows indexing progress
---
## What

Implement automatic model download from Hugging Face for development environments,
and pre-download at startup with status bar progress tracking.

## Outcome

Model downloads on first use via Hugging Face API. Startup task tracker shows
progress in status bar. Git commits: `4c191f7`, `304a4e6`, `e4c5f69`.

## How

Implementation approach defined by the assignee.

## Verification

Acceptance criteria verified by reviewer.
