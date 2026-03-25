---
id: IDEA-057fce3f
type: idea
title: "Plugin lifecycle events — install/uninstall/update hooks"
description: "Trigger events on plugin install, uninstall, and update that the app and other plugins can listen to. Enables plugins to react to ecosystem changes — e.g. regenerate configs when a coding standards plugin is installed."
status: captured
created: 2026-03-20
updated: 2026-03-20
relationships:
  - target: PILLAR-c9e0a695
    type: grounded
  - target: PERSONA-c4afd86b
    type: benefits
---

# IDEA-142: Plugin Lifecycle Events

Events triggered on plugin install/uninstall/update that the app and other plugins can listen to:
- `plugin:installed` — run dependency installation, config generation
- `plugin:uninstalled` — cleanup configs, remove generated files
- `plugin:updated` — re-run setup, migrate configs
- Other plugins can subscribe to react to ecosystem changes