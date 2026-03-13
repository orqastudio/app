---
id: IDEA-075
title: "App UI save should validate before overwriting project.json"
description: "Clicking save in the OrqaStudio app UI can overwrite a valid project.json with blank/default values. Need validation before write, or dirty-checking to only save modified fields."
status: captured
created: "2026-03-13"
updated: "2026-03-13"
pillars:
  - PILLAR-001
research-needed:
  - "What fields does the settings editor currently write? Does it serialize the full model or only changed fields?"
  - "Should project.json have a JSON schema that the app validates against before saving?"
promoted-to: null
---

## Motivation

During [EPIC-059](EPIC-059), clicking save in the app UI overwrote a valid project.json with blank defaults — null description, empty artifacts array, empty frameworks. Required `git checkout` to recover. The pre-commit hook didn't catch this because it happened in-app, not through git.

This is a data integrity bug. The app should never write destructive changes to its own config file without validation.

## Sketch

Possible approaches:
1. **Schema validation on write** — project.json validates against a schema before being written to disk
2. **Dirty-checking** — only save fields the user actually modified
3. **Backup before overwrite** — create .bak before saving
4. **Required field enforcement** — reject writes that would blank required fields (artifacts array, description)
