# Forge

A desktop app for managed agentic development. Claude Code's capabilities with a visual process layer.

**Core idea:** The governance framework that makes agentic development reliable (agents, skills, rules, learning loops, documentation-first workflow) is currently invisible infrastructure. Forge makes it tangible — a workspace where process artifacts live alongside the conversation as interactive, editable documents.

## What Forge Does

- **Wraps Claude Code** — Same model, same capabilities, but with a UI that surfaces the process layer
- **Backfills governance** — Point it at an existing codebase, answer questions, and Forge progressively builds the documentation, rules, and agent definitions through conversation
- **Makes process visible** — Scanner dashboards, task pipelines, retrospective cards, and metrics charts replace terminal output
- **Persists everything** — Conversation history linked to artifacts, searchable across sessions

## Tech Stack

- **Desktop:** Tauri v2 (Rust backend, lightweight native shell)
- **Frontend:** Svelte 5 (runes, component architecture)
- **AI Integration:** Claude API + Claude Max (via Agent SDK or direct API)
- **Persistence:** SQLite (session history, metrics, project config)
- **Target platforms:** Windows, macOS, Linux

## Getting Started

See [Development > Getting Started](docs/development/getting-started.md) for prerequisites and setup instructions.

## Documentation

Run `npx docsify-cli serve docs` to browse the full documentation site locally.
