---
id: IDEA-e8b2f4a7
type: discovery-idea
title: "Rewrite CLI and connectors in Rust — language boundary violations"
description: "CLI and connectors are TypeScript but DOC-62969bc3 mandates Rust for everything except the frontend UI. TypeScript is frontend-only."
status: captured
priority: critical
created: "2026-03-29"
tags:
  - architecture-violation
  - cli
  - rust
  - language-boundary
---

## Violation

DOC-62969bc3 line 96: "Rust is the base language for all libraries, the CLI, and the daemon. TypeScript is the frontend language only (SvelteKit UI)."

DOC-62969bc3 line 151: "CLI: A thin Rust wrapper around the engine libraries. An access method, not business logic."

Two components violate the language boundary:

### CLI (cli/)

Full TypeScript/Node.js application with ~30 command files and ~20 library modules. Architecture says: "CLI: A thin Rust wrapper around the engine libraries."

Current: @orqastudio/cli TypeScript package, runs as node dist/cli.js
Target: Rust binary crate, thin wrapper delegating to engine crates

### Connectors (connectors/claude-code/)

TypeScript application with generator, hooks, skills, commands. Connectors are below the UI surface — they generate configuration and wire into external tools. Should be Rust per the language boundary.

Current: TypeScript with ~15 source files, hooks, generator
Target: Rust binary/library, generating Claude Code configuration from engine data

## TypeScript correctly used in

- app/ (SvelteKit frontend)
- libs/types/ (serves frontend)
- libs/sdk/ (serves frontend stores)
- libs/svelte-components/ (frontend components)
- plugins with custom views (frontend views)

## Migration Path

### CLI to Rust

1. Create cli/ as Rust binary crate
2. Port each command as thin wrapper around engine crate calls
3. Port lib modules — most should already be engine crate functionality
4. Remove TypeScript CLI once Rust version is feature-complete

### Connector to Rust generator, TypeScript/JS output

1. Connector is a Rust binary/library that GENERATES a Claude Code plugin
2. The generated plugin is TypeScript/JS — the correct runtime language for Claude Code plugins
3. Generator calls engine crates directly (not daemon HTTP) to produce the output
4. Generated hooks, skills, commands are in Claude Code's expected format
5. The connector source (the generator) is Rust; the generated output is TypeScript/JS
6. This matches the architecture: Rust below, TypeScript only at the surface (Claude Code runtime is a surface)

## Scope

Multi-session project for each component. Both work correctly today but violate the language boundary. CLI is higher priority since it's the primary user-facing access method.
