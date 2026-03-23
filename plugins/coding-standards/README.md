![License](https://img.shields.io/badge/license-BSL%201.1-blue)
![Status](https://img.shields.io/badge/status-pre--release-orange)
![TypeScript](https://img.shields.io/badge/TypeScript-3178C6?logo=typescript&logoColor=white)

![OrqaStudio](https://github.com/orqastudio/.github/blob/main/assets/banners/banner-1680x240.png?raw=1)

# Coding Standards

OrqaStudio plugin for unified coding standards enforcement — generates tool config files from governance rules and runs quality checks via `orqa check`.

## Installation

```bash
orqa plugin install @orqastudio/plugin-coding-standards
```

## What It Does

- **Config generation** — reads enforcement rules from `.orqa/process/rules/` and generates tool config files (ESLint, Prettier, clippy, rustfmt, etc.)
- **Quality checks** — `orqa check` runs all configured tools against the codebase via plugin executors
- **Organisation sync** — in org mode, propagates coding standards across all projects with override tracking

## How It Works

1. Define coding standards as OrqaStudio rules with `enforcement` arrays
2. Install language-specific plugins (svelte, rust, typescript) that provide tool executors
3. Run `orqa check configure` to generate tool config files from rules
4. Run `orqa check` to enforce standards

## Extends

This plugin provides the config generator and check runner. Language-specific tooling comes from:
- `@orqastudio/plugin-typescript` — tsconfig, ESLint base
- `@orqastudio/plugin-svelte` — svelte-check, Vitest, ESLint svelte
- `@orqastudio/plugin-rust` — clippy, rustfmt, cargo-test
- `@orqastudio/plugin-tauri` — Tauri-specific patterns

## License

BSL-1.1 — see [LICENSE](LICENSE) for details.
