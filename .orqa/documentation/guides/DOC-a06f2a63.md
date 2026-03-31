---
id: DOC-a06f2a63
type: doc
status: active
title: "Svelte Plugin Setup"
domain: guides
description: "How to install and configure the Svelte development plugin — dependencies, config generation, and organisation mode."
category: onboarding
created: 2026-03-19
updated: 2026-03-19
relationships:
  - target: KNOW-be54e4de
    type: synchronised-with
  - target: KNOW-f028481d
    type: synchronised-with
---

# Svelte Plugin Setup

## Installation

Install via the plugin browser in OrqaStudio or via CLI:

```bash
orqa plugin install @orqastudio/plugin-svelte
```

The installer:

1. Detects which sub-projects use TypeScript/Svelte
2. Recommends which sub-projects should receive the plugin
3. Adds dev dependencies to each selected project's `package.json`
4. Runs `npm install`
5. Generates initial config files from coding standards rules

## What Gets Installed

Dev dependencies added to `package.json`:

- `eslint`, `@typescript-eslint/eslint-plugin`, `@typescript-eslint/parser`
- `eslint-plugin-svelte`
- `svelte-check`
- `vitest`, `@testing-library/svelte`
- `typescript`

Only missing dependencies are added — existing ones are preserved.

## Organisation Mode

When installed at the org level, the plugin asks which sub-projects apply. AI recommends based on detected languages. Each selected sub-project gets dependencies installed and config generated from the org-level coding standards rules.

Sub-projects can override specific standards with tracked rationale.
