![License](https://img.shields.io/badge/license-Apache%202.0-blue)

![OrqaStudio](https://github.com/orqastudio/orqastudio-brand/blob/main/assets/banners/banner-1680x240.png?raw=1)

# OrqaStudio

OrqaStudio is an AI-assisted clarity engine designed to help people turn messy situations into structured understanding and evolving plans.

Rather than focusing purely on task management or software generation, OrqaStudio focuses on improving the quality of thinking that leads to action. It operationalises agile thinking through AI-assisted reasoning, enabling individuals and teams to explore problems, shape ideas, design experiments, and continuously learn through structured retrospection.

---

## Project Philosophy

OrqaStudio focuses on **clarity before execution**. Clear thinking leads to better action.

The platform supports a structured thinking loop:

```
Chaos --> Structured Understanding --> Experiments / Backlog --> Execution --> Retrospective --> Improved Understanding
```

### Core Principles

- **Clarity before execution** — Clear thinking leads to better action
- **Human-led AI** — AI acts as a structured thinking partner rather than replacing human judgement
- **Artifact-driven reasoning** — Markdown artifacts represent structured knowledge that can evolve over time
- **Reflective learning** — Retrospectives and iteration history enable continuous learning

---

## What OrqaStudio Does

- **AI-assisted thinking** — Use AI as a structured reasoning partner to explore problems, challenge assumptions, and build understanding before committing to action
- **Artifact-driven knowledge** — Conversations produce markdown artifacts — plans, decisions, retrospectives — that evolve over time and accumulate into a knowledge base
- **Governance as a living system** — Standards, rules, and agent definitions are not documents collecting dust. They are visible, enforceable, and editable through the UI
- **Self-learning loop** — Every session contributes to improving the governance framework. Mistakes become lessons, lessons become rules, rules become enforcement
- **Process visibility** — Scanner dashboards, task pipelines, retrospective cards, and metrics charts make invisible process tangible and manageable

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

The **OrqaStudio** name and branding are the property of the project maintainers.

Brand assets are maintained separately in the [`orqastudio-brand`](https://github.com/orqastudio/orqastudio-brand) repository and may be subject to additional usage restrictions.

---

## Status

OrqaStudio is currently under active development. APIs and internal structures may change.
