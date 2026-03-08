---
role: artifacts
label: "Skills"
description: "Reusable knowledge packages that agents load before working."
icon: "zap"
sort: 2
---

# Skills

Skills are reusable knowledge packages that agents load before starting work. Each skill teaches patterns, anti-patterns, and domain-specific conventions that help agents write better code for this project.

## Skill Types

- **Generic skills**: Portable knowledge about languages, frameworks, and tools (Rust async, Svelte 5, Tailwind)
- **Project skills**: Patterns specific to this codebase (IPC patterns, store patterns, streaming pipeline)

## Loading

Agents declare their required skills in YAML frontmatter. Skills are loaded at task start, before any implementation begins. The `chunkhound` and `orqa-composability` skills are universal — required by every agent.
