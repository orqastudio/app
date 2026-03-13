---
role: group
label: Documentation
description: Project documentation, architecture references, and UI specs.
icon: file-text
sort: 1
---


![OrqaStudio™](../ui/src/lib/assets/banner.png)

An AI-assisted clarity engine that helps people turn messy situations into structured understanding and evolving plans through agile thinking and continuous retrospection.

## What OrqaStudio Does

- **Structures thinking** — Artifacts, epics, milestones, and lessons turn vague problems into navigable plans
- **Backfills governance** — Point it at an existing codebase, answer questions, and OrqaStudio progressively builds the documentation, rules, and agent definitions through conversation
- **Makes process visible** — Scanner dashboards, task pipelines, retrospective cards, and metrics charts replace terminal output
- **Persists everything** — Conversation history linked to artifacts, searchable across sessions

## Product Pillars

Active pillars are defined in `.orqa/planning/pillars/`. Every feature must serve at least one active pillar. See the **Pillars** section under Planning for the current pillar definitions and their test questions.

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
make dev
```

## Documentation Sections

- Product — Vision, pillars, governance, roadmap
- Architecture — Decisions, IPC design, module structure
- User Interface — UX specs, component library, layout patterns
- Development — Coding standards, setup guide, lessons learned
- Process — Governance framework, team model, workflows
