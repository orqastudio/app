---
id: EPIC-00dedf0c
type: epic
title: "Plugin lifecycle — install, enable, disable, uninstall with content ownership"
description: "Mechanical plugin lifecycle: install registers in project.json, copies content to .orqa/, records ownership in manifest.json, runs install hooks. Enable/disable toggles content presence. Uninstall removes everything. Hook engine blocks edits to plugin-owned files."
status: active
priority: P0
relationships:
  - target: PILLAR-c9e0a695
    type: grounded
    rationale: "Clarity Through Structure — plugins own content, project consumes it"
  - target: MS-b1ac0a20
    type: fulfils
    rationale: "Epic fulfils this milestone"
---
# Plugin Lifecycle

## Problem

Plugin content (rules, knowledge, agents, docs) lives in plugin directories (`plugins/*/rules/`) but the engine only scans `.orqa/`. The install process doesn't copy content to `.orqa/` or track ownership. Result: 49 of 54 rules are invisible to the engine.

## Target State

### Install (`orqa plugin install <name>`)

1. **Register** plugin in `.orqa/project.json` → `plugins.<name>.installed: true, enabled: true, path: "plugins/<name>"`
2. **Install dependencies** — read plugin's `package.json` / `Cargo.toml` and install what's needed (npm install, cargo fetch). Check `requires` field in manifest for system deps.
3. **Build** — run the plugin's build step if it has one (npm run build, cargo build). Connectors need tsc, Rust plugins need cargo build.
4. **Copy content** from plugin directory → `.orqa/`:
   - `plugins/<name>/rules/*.md` → `.orqa/process/rules/`
   - `plugins/<name>/knowledge/*.md` → `.orqa/process/knowledge/`
   - `plugins/<name>/agents/*.md` → `.orqa/process/agents/`
   - `plugins/<name>/documentation/*.md` → `.orqa/documentation/`
   - Other artifact directories as declared in plugin manifest
5. **Record ownership** in `.orqa/manifest.json`:
   ```json
   {
     "@orqastudio/plugin-agile-workflow": {
       "files": [
         ".orqa/process/rules/RULE-f609242f.md",
         ".orqa/process/rules/RULE-0be7765e.md"
       ],
       "installed_at": "2026-03-22T...",
       "version": "0.1.0-dev"
     }
   }
   ```
6. **Run install hook** — each plugin can declare an `install` entry in its manifest:
   - Connector: syncs to `.claude/plugins/` cache, sets up symlinks
   - Tooling plugins: install their tools (clippy, eslint config, etc.)
   - No install hook = no-op (just content copy)

### Enable (`orqa plugin enable <name>`)

1. Set `plugins.<name>.enabled: true` in `project.json`
2. Copy content from plugin → `.orqa/` (using manifest to know what files)
3. Content is now visible to the engine

### Disable (`orqa plugin disable <name>`)

1. Set `plugins.<name>.enabled: false` in `project.json`
2. Delete the files listed in manifest from `.orqa/`
3. Keep manifest entries (so re-enable knows what to copy back)
4. Keep plugin registration in project.json
5. Content is now invisible to the engine

### Uninstall (`orqa plugin uninstall <name>`)

1. Disable first (delete content from `.orqa/`)
2. Uninstall plugin-specific dependencies (if not shared with other plugins)
3. Remove manifest entries for this plugin
4. Remove plugin entry from `project.json`
5. Run uninstall hook if declared

### Refresh (`orqa plugin refresh [<name>]`)

For each installed plugin (or a specific one):
1. **Check dependencies** — verify all deps are installed, install any missing
2. **Rebuild** — run the plugin's build step (npm run build, cargo build)
3. **Re-sync content** — diff plugin source against manifest, copy changed files to `.orqa/`, update manifest
4. This is the "I changed the plugin source, apply it" command
5. Used during plugin development workflow and as part of `orqa dev` startup
6. Also useful as a recovery command: `orqa plugin refresh` fixes any drift

### Edit protection

- Hook engine reads `.orqa/manifest.json` on every file write
- If the target file is listed in manifest → **block** with message: "This artifact is owned by plugin X. Edit the plugin source and re-sync."
- This is a rule (RULE-XXXXXXXX) with `mechanism: hook, type: PreToolUse, event: file, source: plugin-manifest`
- The Rust `evaluate_hook` checks the manifest as part of its evaluation

### Content drift detection (`orqa plugin diff [<name>]`)

1. For each plugin (or a specific one), compare every file in the manifest against the plugin source
2. Report: **identical**, **modified** (project copy differs from plugin original), **missing** (in manifest but deleted from `.orqa/`), **orphaned** (in `.orqa/` but not in manifest)
3. Output as human-readable diff summary or `--json` for programmatic consumption
4. This is read-only — it doesn't fix anything, just reports
5. The app UI will consume this to show warnings when someone has edited plugin-owned files directly
6. Agents can run this to self-check before committing

```
orqa plugin diff
  @orqastudio/plugin-agile-workflow:
    RULE-f609242f.md: identical
    RULE-0be7765e.md: MODIFIED (3 lines changed)
    KNOW-ee860ed9.md: identical

  @orqastudio/plugin-core:
    AGENT-4c94fe14.md: identical
    RULE-2f64cc63.md: MISSING (deleted from .orqa/)

  1 modified, 1 missing, 47 identical
```

### Content sync on update (`orqa plugin update <name>`)

1. Pull latest plugin content
2. Diff against manifest — detect added/removed/changed files
3. Delete old files from `.orqa/`, copy new files
4. Update manifest with new file list and version
5. Run install hook again

## Plugin manifest additions (`orqa-plugin.json`)

Each plugin declares its lifecycle requirements. Validated by JSON Schema.

```json
{
  "name": "@orqastudio/plugin-agile-workflow",
  "version": "0.1.0-dev",
  "provides": { "schemas": [...], "relationships": [...], ... },

  "content": {
    "rules": { "source": "rules/", "target": ".orqa/process/rules/" },
    "knowledge": { "source": "knowledge/", "target": ".orqa/process/knowledge/" },
    "agents": { "source": "agents/", "target": ".orqa/process/agents/" },
    "documentation": { "source": "documentation/", "target": ".orqa/documentation/" }
  },

  "dependencies": {
    "npm": ["yaml"],
    "system": [{ "binary": "node", "minVersion": "22" }]
  },

  "build": "npm run build",

  "lifecycle": {
    "install": "node scripts/post-install.mjs",
    "uninstall": "node scripts/pre-uninstall.mjs"
  }
}
```

- `content` — maps plugin-local directories to `.orqa/` destinations. The installer reads this to know what to copy. Only files matching known artifact patterns (RULE-*.md, KNOW-*.md, etc.) are copied.
- `dependencies.npm` — npm packages to install in the plugin directory
- `dependencies.system` — system binaries required (checked, not installed)
- `build` — command to run after deps are installed (cwd = plugin directory)
- `lifecycle.install` — custom command after content copy (cwd = plugin directory)
- `lifecycle.uninstall` — custom command before content removal

All fields are optional. A plugin with no `content` field has no artifacts to copy. A plugin with no `build` field doesn't need building.

The manifest schema itself is validated by JSON Schema (added to libs/types or the validator).

## Manifest format

`.orqa/manifest.json`:
```json
{
  "plugins": {
    "@orqastudio/plugin-agile-workflow": {
      "version": "0.1.0-dev",
      "installed_at": "2026-03-22T00:00:00Z",
      "files": [
        ".orqa/process/rules/RULE-f609242f.md",
        ".orqa/process/rules/RULE-0be7765e.md",
        ".orqa/process/knowledge/KNOW-ee860ed9.md",
        ".orqa/process/agents/AGENT-e5dd38e4.md"
      ]
    }
  }
}
```

## Tasks

1. Define manifest.json schema and read/write functions in Rust engine
2. Implement `orqa plugin install` — register, copy content, record manifest, run hook
3. Implement `orqa plugin enable` — copy from plugin, set enabled: true
4. Implement `orqa plugin disable` — delete from .orqa/, set enabled: false
5. Implement `orqa plugin uninstall` — disable + remove manifest + remove from project.json
6. Add edit protection to Rust hook evaluator — read manifest, block writes to owned files
7. Create edit protection rule artifact
8. Implement `orqa plugin update` — diff, re-sync, update manifest
9. Run initial install for all existing plugins (bootstrap the manifest)
10. Update connector install hook to sync to .claude/plugins/ cache
11. Implement `orqa plugin diff` — content drift detection between project and plugin source
12. Implement `orqa plugin refresh` — check deps, rebuild, re-sync content
13. Wire refresh into `orqa dev` startup (refresh all enabled plugins before starting)