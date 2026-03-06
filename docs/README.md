---
title: "README"
category: general
tags: []
created: 2026-03-02
updated: 2026-03-04
---

![Orqa Studio](../ui/lib/assets/banner.png)

Desktop app for managed agentic development. Claude Code's capabilities with a visual process layer.

## What Orqa Studio Does

- **Wraps Claude Code** — Same model, same capabilities, but with a UI that surfaces the process layer
- **Backfills governance** — Point it at an existing codebase, answer questions, and Orqa Studio progressively builds the documentation, rules, and agent definitions through conversation
- **Makes process visible** — Scanner dashboards, task pipelines, retrospective cards, and metrics charts replace terminal output
- **Persists everything** — Conversation history linked to artifacts, searchable across sessions

## Product Pillars

**Pillar 1: Self-Learning Loop** — The system improves over time via retrospectives, metrics, and lessons feeding back into governance. Every session makes Orqa Studio smarter.

**Pillar 2: Process Governance** — Automated enforcement of standards, rules, and workflows. Governance is not invisible infrastructure — it is tangible, visible, and manageable.

## Tech Stack

| Layer | Technology |
|-------|------------|
| Desktop Shell | Tauri v2 (Rust) |
| Frontend | Svelte 5 (runes) |
| UI Components | shadcn-svelte / Tailwind CSS |
| AI Integration | Claude API + Agent SDK |
| Database | SQLite |
| Testing | cargo test / Vitest / Playwright |

## Getting Started

```bash
npm install
cargo tauri dev
```

## Documentation Sections

- [Product](/product/) — Vision, pillars, governance, roadmap
- [Architecture](/architecture/) — Decisions, IPC design, module structure
- [User Interface](/ui/) — UX specs, component library, layout patterns
- [Development](/development/) — Coding standards, setup guide, lessons learned
- [Research](/research/) — Tech stack decisions, Phase 0 investigations
- [Process](/process/) — Governance framework, team model, workflows
