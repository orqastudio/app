![License](https://img.shields.io/badge/license-BSL%201.1-blue)
![Status](https://img.shields.io/badge/status-pre--release-orange)

![OrqaStudio](https://github.com/orqastudio/orqastudio-brand/blob/main/assets/banners/banner-1680x240.png?raw=1)

# OrqaStudio Dev Environment

> **Pre-release** — APIs and interfaces may change without notice until v1.0.0.

Organisation-mode development environment for OrqaStudio. Aggregates all sub-repositories via git submodules and provides product-level governance artifacts.

## Repository Structure

```
orqastudio-dev/
├── .orqa/                        # Product-level governance artifacts
├── app/                          # Tauri v2 desktop app (Rust + Svelte 5 + SQLite)
├── libs/
│   ├── types/                    # @orqastudio/types — shared TypeScript types
│   ├── sdk/                      # @orqastudio/sdk — Svelte 5 stores, graph SDK
│   ├── integrity-validator/      # @orqastudio/integrity-validator — CLI integrity checker
│   ├── svelte-components/        # @orqastudio/svelte-components — shared UI components
│   ├── graph-visualiser/         # @orqastudio/graph-visualiser — Cytoscape graph viz
│   ├── brand/                    # @orqastudio/brand — branding assets
│   ├── eslint-config/            # @orqastudio/eslint-config — shared ESLint config
│   ├── test-config/              # @orqastudio/test-config — shared test config
│   └── plugin-template/          # @orqastudio/plugin-template — plugin starter
├── plugins/
│   └── claude-plugin/            # OrqaStudio Claude Code plugin
├── tools/
│   └── debug-tool/               # Dev controller and dashboard
├── scripts/link-all.sh           # npm link setup (run for fresh checkout)
└── Makefile                      # verify, build, test targets
```

## Getting Started

```bash
# Clone with submodules
git clone --recurse-submodules git@github.com:orqastudio/orqastudio-dev.git

# Wire up npm links between libraries and app
bash scripts/link-all.sh

# Start the dev environment
cd app && make dev
```

## Repository Ecosystem

| Repository | Purpose |
|------------|---------|
| [orqastudio-app](https://github.com/orqastudio/orqastudio-app) | Desktop application (Tauri v2 + Svelte 5) |
| [orqastudio-types](https://github.com/orqastudio/orqastudio-types) | Shared TypeScript type definitions |
| [orqastudio-sdk](https://github.com/orqastudio/orqastudio-sdk) | Svelte 5 stores, graph SDK, plugin registry |
| [orqastudio-integrity-validator](https://github.com/orqastudio/orqastudio-integrity-validator) | CLI artifact graph integrity checker |
| [orqastudio-svelte-components](https://github.com/orqastudio/orqastudio-svelte-components) | Shared Svelte UI components |
| [orqastudio-graph-visualiser](https://github.com/orqastudio/orqastudio-graph-visualiser) | Cytoscape-based graph visualisation |
| [orqastudio-brand](https://github.com/orqastudio/orqastudio-brand) | Branding assets and guidelines |
| [orqastudio-debug-tool](https://github.com/orqastudio/orqastudio-debug-tool) | Dev controller and web dashboard |
| [orqastudio-claude-plugin](https://github.com/orqastudio/orqastudio-claude-plugin) | Claude Code plugin for OrqaStudio |

---

## License

Copyright (c) 2026 Bobbi Byrne-Graham

[BSL 1.1](LICENSE) — converts to Apache 2.0 four years after each version release. Internal business use, non-commercial use, plugin development, and evaluation are expressly permitted.

---

## Trademark Notice

**OrqaStudio** is a trademark of Bobbi Byrne-Graham.

- **Domains:** [orqastudio.com](https://orqastudio.com), [orqastudio.ai](https://orqastudio.ai)
- **Namespace:** [github.com/orqastudio](https://github.com/orqastudio)
