---
id: DOC-009
title: "Project Configuration (`.orqa/project.json`)"
description: Schema and usage of the .orqa/project.json configuration file that defines project-level settings.
created: 2026-03-03
updated: 2026-03-09
sort: 3
relationships:
  - target: AD-019
    type: informs
    rationale: Documentation page references AD-019
  - target: PILLAR-001
    type: informed-by
  - target: PILLAR-002
    type: informed-by
---

**Date:** 2026-03-03 | **Status:** Active | **Decision:** [AD-019](AD-019) — File-based project settings

---

## Overview

OrqaStudio™ uses a file-based configuration model for project settings. Each managed project stores its configuration in `.orqa/project.json` at the project root. This file is the **source of truth** for project-specific settings.

The SQLite `projects` table remains as the app-wide registry of known projects (recent list, IDs, timestamps). It does NOT own project configuration — `.orqa/project.json` does.

---

## `.orqa/` Directory Convention

OrqaStudio creates a `.orqa/` directory in each managed project for OrqaStudio-specific configuration. `.orqa/` is the source of truth for all governance artifacts. `.claude/` may exist as an optional symlink compatibility layer for CLI tools (such as Claude Code) that look for governance files in `.claude/`.

| Path | Purpose |
|------|---------|
| `.orqa/project.json` | Project configuration file (this document) |

The `.orqa/` directory is created automatically when the user saves project settings for the first time.

---

## File Format

```json
{
    "name": "orqa-studio",
    "description": "Desktop app for managed agentic development",
    "default_model": "auto",
    "excluded_paths": ["node_modules", ".git", "target", "dist", "build"],
    "stack": {
        "languages": ["rust", "typescript", "svelte"],
        "frameworks": ["tauri", "svelte", "tailwindcss"],
        "package_manager": "npm",
        "has_orqa_config": true,
        "has_design_tokens": false
    },
    "governance": {
        "docs": 26,
        "agents": 15,
        "rules": 20,
        "skills": 9,
        "hooks": 3,
        "has_orqa_config": true
    }
}
```

### Field Reference

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | `string` | Yes | Display name for the project |
| `description` | `string \| null` | No | Brief project description |
| `default_model` | `string` | Yes | Default AI model identifier: `"auto"` defers to the provider default; provider-specific model IDs (e.g. `"claude-sonnet-4-6"`) select a specific model |
| `excluded_paths` | `string[]` | Yes | Directory names to skip during scanning |
| `stack` | `DetectedStack \| null` | No | Detected technology stack (populated by scanner) |
| `governance` | `GovernanceCounts \| null` | No | Governance artifact counts (populated by scanner) |

### `DetectedStack` Object

| Field | Type | Description |
|-------|------|-------------|
| `languages` | `string[]` | Detected programming languages (lowercase) |
| `frameworks` | `string[]` | Detected frameworks and tools |
| `package_manager` | `string \| null` | Primary package manager (`"npm"`, `"cargo"`, `"yarn"`, `"pnpm"`, `"bun"`) |
| `has_orqa_config` | `boolean` | Whether `.orqa/project.json` exists |
| `has_design_tokens` | `boolean` | Whether design token files are present |

### `GovernanceCounts` Object

| Field | Type | Description |
|-------|------|-------------|
| `docs` | `number` | Count of `.md` files in `.orqa/documentation/` |
| `agents` | `number` | Count of `.md` files in `.orqa/agents/` (or `.claude/agents/` as a CLI symlink layer) |
| `rules` | `number` | Count of `.md` files in `.orqa/rules/` (or `.claude/rules/` as a CLI symlink layer) |
| `skills` | `number` | Count of directories in `.orqa/skills/` (or `.claude/skills/` as a CLI symlink layer) |
| `hooks` | `number` | Count of files in `.orqa/process/hooks/` (or `.claude/hooks/` as a CLI symlink layer) |
| `has_orqa_config` | `boolean` | Whether `.orqa/project.json` exists |

---

## Discovery Rules

1. When a project is opened (`project_open`), OrqaStudio checks for `.orqa/project.json` at the project root
2. **File exists** — load it as the source of truth for project settings
3. **File missing** — not an error; the UI shows a setup wizard that scans the project and creates the file
4. The `project_open` command syncs the file-based name to SQLite so the recent projects list stays current

---

## Relationship to SQLite

| Concern | Owner | Why |
|---------|-------|-----|
| Project registry (ID, path, timestamps) | SQLite `projects` table | App needs a cross-project list for recent projects, session associations |
| Project configuration (name, model, stack, governance) | `.orqa/project.json` | User-visible, version-controllable, portable |

When `project_settings_write` is called, the `name` field is synced back to the SQLite `projects` table to keep the recent projects list display current.

---

## Schema Versioning

Reserved for future use. When a `version` field is needed for migrations, it will be added to the JSON root. For now, all fields are additive — missing fields use sensible defaults during deserialization (`#[serde(default)]`).

---

## Error Handling

| Scenario | Error | Behavior |
|----------|-------|----------|
| Malformed JSON in `.orqa/project.json` | `OrqaError::Serialization` | UI shows error, offers to re-scan and overwrite |
| Permission denied reading/writing | `OrqaError::FileSystem` (from `io::Error`) | UI shows error message |
| `.orqa/project.json` does not exist | Not an error | `project_settings_read` returns `None`, UI shows setup wizard |
| `.orqa/` directory does not exist | Not an error | Created automatically on first write |

---

## IPC Commands

Three commands manage project settings:

- `project_settings_read(path)` — reads `.orqa/project.json`, returns `Option<ProjectSettings>`
- `project_settings_write(path, settings)` — writes `.orqa/project.json`, creates `.orqa/` dir if needed
- `project_scan(path, excluded_paths?)` — scans filesystem for stack detection and governance counts

See [IPC Command Catalog](./ipc-commands.md) for full parameter tables.

---

## Related Documents

- [IPC Command Catalog](./ipc-commands.md) — command specifications
- [Settings & Onboarding Wireframe](../wireframes/settings-onboarding.md) — UI design
- [SQLite Schema](./sqlite-schema.md) — projects table definition
