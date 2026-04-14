---
name: Plugin-canonical architecture
description: Plugins are the source of truth for governance patterns — .orqa/ contains installed copies plus dev-only artifacts
type: project
---

Plugins are the canonical source of truth for all governance patterns. The `.orqa/` directory contains installed copies of plugin content plus dev-only artifacts (tasks, research, decisions created during the project). The engine provides capabilities; plugins provide definitions.

**Why:** This is P1 (Plugin-Composed Everything). No governance pattern is hardcoded in the engine. If a pattern exists in `.orqa/`, it came from a plugin or was created during project work. Decision: PD-plugin-canonical.

**How to apply:** When adding governance patterns, they go in a plugin — not hardcoded in engine crates. When reading `.orqa/` content, understand it's installed from plugins + project-specific artifacts, not a hand-maintained directory.
