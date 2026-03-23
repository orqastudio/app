---
id: KNOW-e3a559c9
title: Plugin Setup
description: "Installs the companion plugin for Claude Code. Detects existing agent infrastructure, migrates to the project governance directory, registers the plugin, and sets up symlinks."
status: active
created: 2026-03-11
updated: 2026-03-23
category: tool
version: 0.2.0
user-invocable: true
relationships:
  - target: DOC-a1b2c3d4
    type: synchronised-with
  - target: AGENT-bedeffd1
    type: employed-by
---

# Plugin Setup

This skill guides the installation of the companion plugin for Claude Code. It handles
both fresh installs and migrations from existing agent infrastructure.

## Detection Phase

Before installing, determine the current state:

### Check 1: Does the project governance directory exist?

- **Yes** → Project already initialized. Proceed to plugin installation.
- **No** → Fresh project. Run `project-setup` and `project-inference` skills first.

### Check 2: Does existing Claude Code infrastructure exist with real files (not symlinks)?

| Finding | Meaning | Action |
|---------|---------|--------|
| No agent infrastructure directory | Fresh install | Bootstrap from scratch |
| `CLAUDE.md` is a real file | Existing Claude Code project | Migrate to governance directory |
| `CLAUDE.md` is a symlink to governance dir | Already set up | Just install plugin |
| `rules/` contains real `.md` files | Existing rules | Migrate to governance directory |
| `agents/` contains real `.md` files | Existing agents | Migrate to governance directory |
| `knowledge/` contains real dirs | Existing knowledge | Migrate to governance directory |

### Check 3: Is the plugin already installed?

Check the Claude Code settings file for the plugin registration entry.

## Migration Path (existing agent infrastructure)

When real files exist in the agent infrastructure directory, migrate them to the project governance directory before installing:

### Step 1: Migrate CLAUDE.md → orchestrator agent

Copy `CLAUDE.md` to `<governance-dir>/process/agents/orchestrator.md`.

Add orchestrator frontmatter if missing:

```yaml
---
id: AGENT-1dab5ebe
role: orchestrator
title: Orchestrator
description: Coordinates work, enforces process, delegates to agents.
capabilities:
  - file_read
  - file_write
  - file_edit
  - file_search
  - content_search
  - code_search_regex
  - code_search_semantic
  - code_research
  - shell_execute
  - skill_load
skills:
  - orqa-code-search
  - composability
  - planning
  - governance-maintenance
  - skills-maintenance
---
```

### Step 2: Migrate rules

Copy existing rule `.md` files to `<governance-dir>/process/rules/`.

Add rule frontmatter if missing. Each rule needs at minimum:

```yaml
---
id: RULE-NNN
title: Rule Title
description: What this rule enforces.
status: active
---
```

### Step 3: Migrate agents

Copy existing agent `.md` files to `<governance-dir>/process/agents/`.

### Step 4: Migrate knowledge

Copy existing knowledge directories to `<governance-dir>/process/knowledge/`.

### Step 5: Back up and remove originals

Move originals to a backup directory before removing, so nothing is lost.

## Plugin Installation

### Step 1: Obtain the plugin

Clone or verify the plugin repository exists locally.

### Step 2: Register the marketplace

Add the local plugin source to the Claude Code plugins configuration as a known marketplace with a directory source pointing to the plugin location.

### Step 3: Install the plugin

Run `orqa plugin install <plugin-name>`. This:
- Copies plugin content to `.orqa/` based on the `content` field in `orqa-plugin.json`
- Records installed files in `.orqa/manifest.json`
- Installs npm dependencies declared in `dependencies.npm`
- Runs the plugin's `build` command if present
- Registers the plugin in the project's plugin registry

### Step 4: Enable the plugin in project settings

Add the plugin to the project's Claude Code settings with `enabledPlugins`.

### Step 5: Verify

Restart Claude Code. The plugin's SessionStart hook will:

1. Create `CLAUDE.md` → `<governance-dir>/process/agents/orchestrator.md` symlink
2. Create `rules/` → `<governance-dir>/process/rules/` symlink
3. Create `agents/` → `<governance-dir>/process/agents/` symlink
4. Create `knowledge/` → `<governance-dir>/process/knowledge/` symlink
5. Run session health checks (stashes, worktrees, uncommitted files)

## Fresh Install Path (no existing agent infrastructure)

1. Run `project-setup` skill to create governance directory structure
2. Run `project-inference` skill to detect project characteristics
3. Follow the Plugin Installation steps above
4. On first session start, the plugin creates all symlinks automatically

## What the Plugin Provides

| Component | Purpose |
|-----------|---------|
| PreToolUse hook | Rule enforcement via pattern matching |
| SessionStart hook | Symlink setup + session health checks |
| Stop hook | Pre-commit checklist reminders |
| Rule enforcement knowledge | Documents how enforcement works |
| `/orqa` command | Governance summary |

## What Stays in the Agent Infrastructure Directory

After installation, the agent infrastructure directory contains only:

| Item | Type | Purpose |
|------|------|---------|
| `settings.json` | Real file | Plugin enablement, Claude Code config |
| `worktrees/` | Real dir | Claude Code worktree state |
| `CLAUDE.md` | Symlink | → orchestrator agent definition |
| `rules/` | Symlink | → governance rules |
| `agents/` | Symlink | → agent definitions |
| `knowledge/` | Symlink | → knowledge artifacts |

The governance directory is the source of truth. The symlinks are managed by the plugin.

## Plugin Lifecycle After Install

Installed plugin files in `.orqa/` are owned by the plugin and protected from direct edits. To update plugin content:

```bash
orqa plugin refresh          # Rebuild all plugins and re-sync content to .orqa/
orqa plugin refresh <name>   # Refresh a specific plugin
orqa plugin diff <name>      # Show content drift between source and .orqa/
orqa plugin disable <name>   # Remove plugin content without uninstalling
orqa plugin enable <name>    # Re-copy content for a disabled plugin
orqa plugin uninstall <name> # Remove plugin and delete all its owned files
```

## Platform Notes

### Windows (MSYS2/Git Bash)

The `ln -s` command in MSYS2 creates copies, not real NTFS symlinks. The plugin's
SessionStart hook uses PowerShell to create proper symlinks on Windows.

This requires Developer Mode enabled or running as administrator.

### macOS / Linux

Standard `ln -sfn` works without special permissions.
