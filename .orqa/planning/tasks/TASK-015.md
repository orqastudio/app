---
id: TASK-015
title: Model auto-download and startup loading
status: done
epic: EPIC-034
description: Automatically downloads the embedding model from Hugging Face on first use and pre-loads it at startup, with download progress visible in the status bar.
created: 2026-03-04
updated: 2026-03-09
assignee: backend-engineer
skills: [rust-async-patterns, tauri-v2]
scope: [src-tauri/src/search/embedder.rs, src-tauri/src/commands/search_commands.rs]
acceptance: [Model auto-downloads from Hugging Face on first use, Pre-download at app startup with progress tracking, Status bar shows indexing progress]
---

## What

Implement automatic model download from Hugging Face for development environments,
and pre-download at startup with status bar progress tracking.

## Outcome

Model downloads on first use via Hugging Face API. Startup task tracker shows
progress in status bar. Git commits: `4c191f7`, `304a4e6`, `e4c5f69`.
