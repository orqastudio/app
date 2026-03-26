---
id: KNOW-8d1c4be6
type: knowledge
title: Plugin Artifact Usage
description: How plugins provide and consume governance artifacts within the OrqaStudio ecosystem
summary: "Plugin artifact usage: how plugins declare, provide, and consume governance artifacts. Content flow from plugin source through install-time sync to project-level copies."
---

## Plugin Artifact Usage

Plugins interact with governance artifacts through a defined content flow:

### Content Declaration

Plugins declare content sections in `orqa-plugin.json`:
- `content.knowledge` — knowledge artifacts provided by the plugin
- `content.rules` — rule artifacts provided by the plugin
- `content.documentation` — documentation provided by the plugin
- `content.workflows` — workflow definitions provided by the plugin

### Content Flow

1. **Source** — plugin stores content in its own directory structure
2. **Install** — `orqa install` copies content to `.orqa/` target paths
3. **Three-way diff** — detects changes in plugin source vs installed baseline vs user edits
4. **Consumption** — runtime reads from `.orqa/` exclusively, never from plugin source

### Ownership Rules

- Plugin owns the source copy
- Project owns the installed copy (user may edit)
- Conflicts resolved during `orqa install` with user prompts
