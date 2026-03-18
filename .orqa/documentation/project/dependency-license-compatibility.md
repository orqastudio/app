---
id: DOC-089
title: "Dependency License Compatibility"
description: "How to ensure that every dependency we use is compatible with our licensing model — what's allowed, what's not, and how to check."
category: how-to
created: 2026-03-18
updated: 2026-03-18
relationships:
  - target: AD-056
    type: documents
  - target: SKILL-056
    type: synchronised-with
---

# Dependency License Compatibility

Every dependency we add must have a license compatible with ours. Getting this wrong can force relicensing the entire project or require removing a dependency after it's deeply integrated.

## What's Compatible

Our code is BSL-1.1 (core/libs/plugins), Apache-2.0 (connectors), or MIT (tools/registries). We **consume** dependencies — we don't relicense them. Each dependency keeps its own license. The question is: does their license allow us to use it in our project?

### Safe — always compatible

| Dependency License | Why it's safe |
|-------------------|---------------|
| **MIT** | No restrictions. Use however you want. |
| **Apache-2.0** | Permissive. Requires attribution in NOTICE files. |
| **BSD-2-Clause** | Minimal restrictions, just keep the copyright notice. |
| **BSD-3-Clause** | Same as BSD-2 plus no endorsement clause. |
| **ISC** | Functionally identical to MIT. |
| **0BSD / Unlicense / CC0** | Public domain equivalent. No restrictions at all. |

### Requires caution

| Dependency License | Issue |
|-------------------|-------|
| **MPL-2.0** (Mozilla Public License) | File-level copyleft — modified files must stay MPL, but you can combine with proprietary code. Usually fine, but check if you're modifying the dependency's source files directly. |
| **LGPL-2.1 / LGPL-3.0** | Library copyleft — fine if dynamically linked (npm/cargo dependency). Problematic if you copy source files into your codebase. |

### Incompatible — never use

| Dependency License | Why |
|-------------------|-----|
| **GPL-2.0 / GPL-3.0** | Strong copyleft. Requires your entire project to be GPL. Incompatible with BSL-1.1. |
| **AGPL-3.0** | Network copyleft. Even running it as part of a service triggers the copyleft. Absolutely incompatible. |
| **SSPL** | MongoDB's license. Incompatible with almost everything. |
| **No license / "All rights reserved"** | No permission to use. Treat as proprietary. |
| **Custom restrictive licenses** | Read carefully. If in doubt, don't use it. |

## How to Check

### Before adding a dependency

1. Check the dependency's `LICENSE` file or `license` field in `package.json` / `Cargo.toml`
2. If it's MIT, Apache-2.0, or BSD — proceed
3. If it's MPL or LGPL — check whether you're modifying their source or just importing
4. If it's GPL, AGPL, SSPL, or unlicensed — stop and find an alternative

### Auditing existing dependencies

```bash
# npm dependencies
npx license-checker --summary

# Rust dependencies (install cargo-license first)
cargo install cargo-license
cargo license
```

### Transitive dependencies matter

A dependency's dependencies also count. If package A is MIT but depends on package B which is GPL, you have a GPL dependency. The audit tools above check the full tree.

## Common Scenarios

### "I found a great library but it's GPL"

Find an alternative. There is almost always an MIT/Apache equivalent. If there truly isn't, consider whether the functionality can be implemented directly.

### "A dependency changed its license"

This happens (Redis, Elasticsearch, HashiCorp all relicensed). Pin the version before the change. The old version retains its old license. Evaluate whether to migrate to an alternative or accept the new license.

### "I want to use code from a Stack Overflow answer"

Stack Overflow content is CC BY-SA 4.0. For small snippets (a few lines), this is generally considered fair use. For substantial code blocks, rewrite rather than copy.

### "A Rust crate is dual-licensed MIT OR Apache-2.0"

This is the most common pattern in the Rust ecosystem. You can choose either license. Choose Apache-2.0 for patent protection, or MIT for simplicity. Both are compatible.

## Current Stack

All current dependencies are compatible:

| Layer | Key Dependencies | Licenses |
|-------|-----------------|----------|
| Rust backend | Tauri, rusqlite, serde, tokio, DuckDB, ONNX Runtime | MIT, Apache-2.0, MIT OR Apache-2.0 |
| Svelte frontend | SvelteKit, Cytoscape, Mermaid, Tailwind | MIT |
| TypeScript libs | yaml, ajv | MIT, ISC |
| Build tools | Vite, ESLint, Vitest | MIT |
