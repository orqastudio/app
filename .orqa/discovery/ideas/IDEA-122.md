---
id: IDEA-122
title: "Remove embedded software plugin from app — load all plugins from disk"
description: "The app currently has a TypeScript copy of the software plugin manifest compiled into the frontend. This is wrong — first-party plugins should be installed during project setup, not bundled in the binary. All plugins load the same way: from orqa-plugin.json on disk."
status: captured
created: 2026-03-19
updated: 2026-03-19
research-needed:
  - How does the frontend currently register plugins? Can it load orqa-plugin.json via IPC instead of static import?
  - What's the Rust-side plugin discovery path? Does it already scan plugins/ at runtime?
  - How does project setup install first-party plugins? From the registry? From a bundled set? From GitHub?
  - Should first-party plugins be pre-downloaded during orqa install (for dev) but not compiled into the app?
  - What happens if a project has no plugins? The app should still work — just no delivery artifacts
relationships:
  - target: PILLAR-001
    type: grounded
    rationale: Loading plugins uniformly from disk makes the plugin architecture visible and consistent — no hidden compiled-in exceptions
  - target: PERSONA-003
    type: benefits
---

# Remove Embedded Software Plugin

## The Problem

`app/ui/src/lib/plugins/software-project/manifest.ts` is a hand-maintained TypeScript copy of `plugins/software/orqa-plugin.json`. It's already out of sync (missing 8 skills and the merged-into extension). This pattern:

- Creates a maintenance burden (two files to keep in sync)
- Treats the software plugin as special (bundled) when it should be equal to any other plugin
- Assumes every project needs the software plugin (not true — research projects, consulting projects, etc.)

## The Fix

1. Delete `app/ui/src/lib/plugins/software-project/` entirely
2. The app loads ALL plugins uniformly via IPC from the Rust backend's plugin discovery
3. The Rust backend already has `plugins/discovery.rs` that scans `plugins/` directories
4. First-party plugins are installed during project setup, not compiled into the app

## Project Setup Flow

When a user creates a new project:

1. AI asks about the project type and goals
2. Based on the conversation, AI recommends plugins (software, research, consulting, custom)
3. Plugins are installed from the official registry (or created on the fly for custom use cases)
4. `project.json` is configured with the installed plugins' navigation, delivery hierarchy, and relationships
5. The setup skills (SKILL-021 through SKILL-024) guide this process

The app binary is domain-agnostic. Domain knowledge comes entirely from plugins.
