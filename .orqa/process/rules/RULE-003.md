---
id: RULE-6c0496e0
title: Artifact Config Integrity
description: Every path in project.json artifacts must resolve to an actual directory on disk. Config-driven scanning with no hardcoded paths.
status: active
created: 2026-03-07
updated: 2026-03-13
relationships:
  - target: AD-2f1991c2
    type: enforces
    rationale: Auto-generated inverse of enforces relationship from AD-2f1991c2
  - target: AD-20d6719d
    type: enforces
    rationale: Auto-generated inverse of enforces relationship from AD-20d6719d
  - target: AD-8b7c4ac5
    type: enforces
  - target: AD-6ce44025
    type: enforces
  - target: AD-0c56aa90
    type: enforces
---
# Artifact Config Integrity (NON-NEGOTIABLE)

The `artifacts` array in `.orqa/project.json` is the single source of truth for what the app scans, displays, and navigates. Every path in the config MUST resolve to an actual directory on disk. Every directory the app needs to scan MUST be listed in the config.

## Config-Driven Scanning

The artifact scanner (`artifact_reader.rs`) does NOT guess directory structure. It reads the `artifacts` config and scans exactly what's configured. This means:

1. **Config paths must match disk** — If the config says `.orqa/delivery/milestones`, the directory `.orqa/delivery/milestones/` must exist
2. **Disk changes require config updates** — Moving `.orqa/lessons/` to `.orqa/process/lessons/` requires updating the config path
3. **No hardcoded paths in Rust or TypeScript** — All artifact paths come from config, never from constants

## Recursive File Explorer Pattern

The scanner walks directories like a file explorer:

- **Flat directories** (e.g., milestones, epics): Scans `.md` files directly in the configured path
- **Tree directories** (e.g., documentation with subdirectories): Recursively walks subdirectories, building `DocNode` entries with `children` for folders and leaf nodes for files
- **Empty directories are omitted** — No empty folder nodes in the tree
- **README.md is navigation metadata** — Skipped as a browsable artifact, but its YAML frontmatter (icon, label, description, sort) is extracted and used to enrich the nav tree
- **Hidden entries** (starting with `.` or `_`) are skipped at all levels

## README as Primary Metadata Source

Every artifact directory's README.md provides the default icon, label, description, and sort order for that directory's nav tree entry. The scanner extracts these from YAML frontmatter.

**Config is the override layer, not the primary source.** The `artifacts` array in `project.json` should only include `icon`, `label`, or `description` fields when they intentionally differ from the README. This keeps config minimal and enables plugins to override specific entries without duplicating README content.

| Field | Primary source | Override source |
|-------|---------------|-----------------|
| `icon` | README frontmatter | Config entry (only if different from README) |
| `label` | README frontmatter | Config entry (only if different from README) |
| `description` | README frontmatter | Config entry |
| `sort` | README frontmatter | Config ordering (array position) |
| `path` | Config entry only | N/A — always in config |

## Display Label Priority

Every `.md` file's display label follows this priority:

1. **YAML frontmatter `title` field** — Use as the label if present
2. **Humanized filename** — Fallback when no frontmatter title exists (e.g., `coding-standards.md` becomes "Coding Standards")

Artifact IDs in all-caps format (e.g., `[EPIC-e045ab6d](EPIC-e045ab6d)`, `[IDEA-3ff06133](IDEA-3ff06133)`, `[AD-69072318](AD-69072318)`) are preserved as-is — they are NOT humanized.

The same applies to `description` — use frontmatter `description` when present, otherwise omit.

## Verification Checklist

Before committing any change that affects artifact paths or structure:

- [ ] Every path in `project.json` `artifacts` resolves to an existing directory
- [ ] No hardcoded artifact paths exist in Rust commands or TypeScript stores
- [ ] Moving a directory on disk is accompanied by a config path update
- [ ] New artifact types are added to the config before being referenced in code
- [ ] The scanner handles both flat and tree directory structures at the configured path

## ArtifactEntry Config Schema

```jsonc
// Minimal entry — icon, label, description all come from directory README.md
{ "key": "docs", "label": "Documentation", "path": ".orqa/documentation" }

// Override entry — icon differs from README, so it's specified here
{ "key": "process", "label": "Process", "icon": "workflow", "path": ".orqa/documentation/process" }

// Group of types (renders as expandable group in sidebar)
// icon: "target" overrides the README's "clipboard-list"
{ "key": "planning", "label": "Planning", "icon": "target",
  "children": [
    { "key": "ideas", "label": "Ideas", "path": ".orqa/delivery/ideas" },
    { "key": "epics", "label": "Epics", "path": ".orqa/delivery/epics" }
  ]
}
```

## .claude/ Symlink Architecture

`.orqa/` is the single source of truth for ALL governance artifacts. The `.claude/` directory exists only for CLI tool compatibility and contains symlinks — NOT copies.

| Symlink | Target (source of truth) |
|---------|-------------------------|
| `.claude/rules/` | → `.orqa/process/rules/` |
| `.claude/agents/` | → `.orqa/process/agents/` |
| `.claude/skills/` | → `.orqa/process/skills/` |
| `.claude/hooks/` | → `.orqa/process/hooks/` |
| `.claude/CLAUDE.md` | → `.orqa/process/agents/orchestrator.md` |

**Real files in `.claude/`** (not symlinks):
- `settings.json` — CLI-specific configuration
- `worktrees/` — CLI worktree state

### Rules

1. **NEVER write directly to `.claude/` directories** — always write to `.orqa/` source of truth
2. **NEVER create separate copies** — if a symlink is broken, fix the symlink, don't create a duplicate file
3. **All agents writing governance artifacts** must target `.orqa/` paths, not `.claude/` paths
4. **New rules** go in `.orqa/process/rules/`, new agents in `.orqa/process/agents/`, new skills in `.orqa/process/skills/`
5. **If symlinks don't exist**, create them — they're the compatibility layer, not the source

## FORBIDDEN

- Writing governance artifacts directly to `.claude/` (use `.orqa/` source of truth)
- Maintaining separate copies in `.claude/` and `.orqa/` (causes divergence)
- Artifact paths in code that don't come from the config
- Config paths that don't match actual disk structure
- Ignoring subdirectories — if a configured path has subdirectories with `.md` files, they must be scanned recursively
- Skipping frontmatter extraction — every `.md` file gets its YAML frontmatter read for title/description
- Hardcoded display labels that override frontmatter titles

## Related Rules

- [RULE-1acb1602](RULE-1acb1602) (end-to-end-completeness) — config changes must be reflected across all layers
- [RULE-9ba80a19](RULE-9ba80a19) (no-aliases-or-hacks) — no alias paths or fallback path resolution
- [RULE-9daf29c0](RULE-9daf29c0) (documentation-first) — document the config schema before implementing
- [RULE-3eccebf3](RULE-3eccebf3) (enforcement-before-code) — create enforcement artifacts before implementation
