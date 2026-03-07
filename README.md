![License](https://img.shields.io/badge/license-Apache%202.0-blue)

![OrqaStudio](https://github.com/orqastudio/orqastudio-brand/blob/main/assets/banners/banner-1680x240.png?raw=1)

# OrqaStudio

> **OrqaStudio is an AI-assisted clarity engine that helps people turn messy situations into structured understanding and evolving plans through agile thinking and continuous retrospection.**

Rather than focusing primarily on building software or managing tasks, OrqaStudio focuses on **improving the quality of thinking that precedes action**. Projects, backlogs, and execution plans are outputs of improved understanding, not the primary goal.

---

## What OrqaStudio Does

- **AI-assisted thinking** — Use AI as a structured reasoning partner to explore problems, challenge assumptions, and build understanding before committing to action
- **Artifact-driven knowledge** — Conversations produce markdown artifacts — plans, decisions, retrospectives — that evolve over time and accumulate into a knowledge base
- **Reflective learning** — Every session feeds the learning loop. Mistakes become lessons, lessons become rules, rules become enforcement. The system gets smarter with each cycle.
- **Governance as a living system** — Standards, rules, and agent definitions are visible, enforceable, and editable through the UI — not hidden in dotfiles
- **Domain-agnostic core** — Software development is the first domain, but the clarity engine applies to any complex work: research, operations, consulting, personal management

### Core Principles

- **Clarity before execution** — Clear thinking leads to better action
- **Human-led AI** — AI acts as a structured thinking partner rather than replacing human judgement
- **Agile as a thinking system** — Restoring the full agile learning loop, not just backlog management
- **Artifact-driven reasoning** — Markdown artifacts are the canonical state of knowledge, reusable in prompts and portable outside the system

### The Agile Learning Loop

```
Chaos / Input
  --> Structured Understanding
  --> Experiments / Backlog
  --> Execution
  --> Retrospective
  --> Improved Understanding
```

### Entry Modes

Users can enter from four starting points — each supporting new projects or existing work:

| Mode | Starting point |
|------|---------------|
| **Problem** | Something is not working and needs diagnosis |
| **Idea** | A concept needs validation and shaping |
| **Goal** | A desired outcome requires planning |
| **Chaos** | A messy situation needs clarity |

Each path triggers different AI-assisted onboarding flows but converges into the same structured reasoning loop.

---

## Repository Purpose

This is the main application repository containing the OrqaStudio desktop app source code.

---

## Tech Stack

- **Desktop:** Tauri v2 (Rust backend, lightweight native shell)
- **Frontend:** Svelte 5 (runes, component architecture)
- **AI Integration:** Multi-provider — Claude Agent SDK, direct APIs, with architecture for additional providers
- **Persistence:** SQLite (session history, metrics, project config)
- **Target platforms:** Windows, macOS, Linux

---

## Repository Ecosystem

| Repository | Purpose |
|------------|---------|
| [orqastudio-app](https://github.com/orqastudio/orqastudio-app) | Application source code |
| [orqastudio-brand](https://github.com/orqastudio/orqastudio-brand) | Canonical branding assets and guidelines |
| orqastudio-site | Project website (planned) |
| orqastudio-docs | Public documentation (planned) |

---

## Getting Started

See [Getting Started](docs/development/getting-started.md) for prerequisites and setup instructions.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for how to contribute.

## Documentation

Documentation lives in the `docs/` directory. Open any `.md` file directly or browse in OrqaStudio's built-in doc viewer.

---

## License

Copyright (c) 2026 Bobbi Byrne-Graham

The OrqaStudio platform is released under the **Apache License 2.0**.

You are free to use, modify, and distribute this software in accordance with the terms of the license.

See the [LICENSE](./LICENSE) file for the full license text.

Documentation (`docs/`) is licensed under [Creative Commons Attribution 4.0 (CC BY 4.0)](docs/LICENSE).

For third-party dependency licenses, see [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).

---

## Trademark Notice

**OrqaStudio™** is a trademark of Bobbi Byrne-Graham.

- **Domains:** [orqastudio.com](https://orqastudio.com), [orqastudio.ai](https://orqastudio.ai)
- **Namespace:** [github.com/orqastudio](https://github.com/orqastudio)

Brand assets are maintained separately in the [`orqastudio-brand`](https://github.com/orqastudio/orqastudio-brand) repository and may be subject to additional usage restrictions.

---

## Status

OrqaStudio is currently under active development. APIs and internal structures may change.
