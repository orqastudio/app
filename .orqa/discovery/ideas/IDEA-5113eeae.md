---
id: "IDEA-5113eeae"
type: idea
title: "Plugin Distribution via Git Submodules"
description: "An official plugins repository under the GitHub org that embeds each plugin as a git submodule. The app reads the repository catalogue, pulls plugins based on user selection, and manages versioning/updates. Each plugin is its own repo with independent releases."
status: surpassed
created: "2026-03-10"
updated: "2026-03-13"
horizon: "later"
research-needed:
  - "Git submodule vs git subtree for plugin embedding — trade-offs for automated updates"
  - "Plugin catalogue format — how does the app discover available plugins from the repo?"
  - "Versioning strategy — semver per plugin repo, pinned in the main catalogue"
  - "Auto-update mechanism — how does the app detect and pull new plugin versions?"
  - "Plugin isolation — how to prevent a broken plugin from affecting the app"
  - "Plugin signing/trust — how to verify plugins from the official repo vs third-party"
relationships:
  - type: "merged-into"
    target: "IDEA-acc89fdc"
  - type: "merged-into"
    target: "IDEA-0a000bc0"
  - type: "realises"
    target: "EPIC-dcc0bac6"
  - target: "AD-ee2910b1"
    type: "crystallises"
  - target: "PILLAR-2acd86c1"
    type: "grounded"
  - target: "PERSONA-2721ae35"
    type: "benefits"
---
> **Surpassed 2026-03-16**: Core plugin distribution mechanism merged into EPIC-dcc0bac6 via AD-ee2910b1. Plugin catalogue/marketplace concept split into IDEA-acc89fdc. User plugin development workflow split into IDEA-0a000bc0.

## Motivation

Plugins need a distribution channel that supports independent versioning, safe installation, and automatic updates without shipping everything in the main app binary. Git submodules give us this naturally:

- Each plugin is its own repo → independent versioning, CI, releases
- The official `orqa-plugins` repo embeds them as submodules → single catalogue, atomic snapshots
- The app reads the catalogue and pulls what the user selects → no unused plugins shipped
- Updates are git operations → familiar, auditable, rollback-friendly

## Sketch

### Architecture

```
github.com/orqa-studio/orqa-plugins          (catalogue repo)
  ├── plugins/
  │   ├── governance-dashboard/               (submodule → orqa-studio/plugin-governance-dashboard)
  │   ├── dependency-graph/                   (submodule → orqa-studio/plugin-dependency-graph)
  │   ├── sprint-planning/                    (submodule → orqa-studio/plugin-sprint-planning)
  │   └── ...
  ├── catalogue.json                          (plugin metadata: name, description, version, compatibility)
  └── README.md
```

Each plugin repo:

```
github.com/orqa-studio/plugin-governance-dashboard
  ├── plugin.json                             (manifest: name, version, entry point, SDK version required)
  ├── src/                                    (plugin source — Svelte components, store, etc.)
  ├── test-data/                              (seed data for development/testing)
  └── README.md
```

### App Integration

1. App reads `catalogue.json` from the plugins repo (cached locally, refreshed periodically)
2. User selects plugins to install from a marketplace-style UI
3. App clones the plugin repo into a local plugins directory
4. Plugin is loaded via the SDK — same `artifactGraph` API the built-in views use
5. Updates: app checks plugin repo for new tags, offers to update

### Four-Layer Plugin Model

The app ships with **default views only**. Everything else is a plugin, distributed through one of four layers:

| Layer | What | Distribution | Versioning |
|-------|------|-------------|------------|
| **Built-in** | Artifact browser, viewers, navigation, conversation | Shipped with app binary | App version |
| **Official plugins** | Curated plugins maintained by the OrqaStudio team | Official plugins repo (git submodules) | Semver per plugin repo |
| **Community plugins** | Third-party plugins shared publicly | Their own GitHub repos, installable via URL | Author-managed |
| **User plugins** | Local plugins created by the user, for the user | Local file path, git-versioned locally | User-managed |

### User Plugins — The Fourth Layer

User plugins are the most important layer. They represent the core promise: **anyone can use the app to extend the app**. A plugin is just a project that OrqaStudio knows how to help build.

**Workflow:**

1. User asks the AI to build a plugin ("I want a dashboard that shows my milestone progress")
2. The AI (with `orqa-plugin-development` skill loaded) creates a new local project with seed data
3. User develops the plugin collaboratively with the AI, testing against seed data
4. When ready, user installs the plugin into their production project via file path
5. The plugin uses the same Artifact Graph SDK as the built-in views
6. Git is initialised locally for versioning — the user owns their plugin history

**Sharing is optional and progressive:**

| Want to... | Do this |
|------------|---------|
| Keep it private | Just use it locally — nothing else needed |
| Share with others | Push to a public GitHub repo, share the URL |
| Get it into the official catalogue | Create a PR to the official plugins repo to add it as a submodule |

The barrier from "I had an idea" to "I have a working plugin" is a single conversation with the AI. The barrier from "working locally" to "shared with the world" is a git push.

### Development Workflow

Plugin development uses the `orqa-plugin-development` skill [TASK-38b82d46](TASK-38b82d46), which guides the AI to:

1. Create the plugin in a **new standalone project** (not inside the user's production project)
2. Generate **seed data** for testing (mock `.orqa/` directory with representative artifacts)
3. Develop and test against the seed data using the Artifact Graph SDK
4. Only import into the production project once the plugin is working

This keeps production governance data safe during development. A plugin IS a project — it has its own `.orqa/` structure, its own governance, and the AI has the skills to help build it.