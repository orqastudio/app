---
id: "IDEA-c4f10c20"
title: "OrqaStudio package ecosystem for plugin portability"
description: "Extract shared code into scoped npm packages hosted on GitHub Packages so that plugins can import types, validation, SDK, and UI components without depending on the full app."
status: "completed"
created: "2026-03-14"
updated: "2026-03-14"
horizon: "active"
research-needed:
  - "SDK extraction scope — what from ui/src/lib/sdk/ is portable vs app-specific?"
  - "Component library extraction — which shadcn wrappers and shared components should be in the package?"
  - "Monorepo vs multi-repo — should packages live in separate repos or a monorepo with workspaces?"
  - "Versioning strategy — how do package versions relate to app versions?"
relationships:
  - target: "EPIC-a210c825"
    type: "realises"
    rationale: "Auto-generated inverse of informs relationship from EPIC-a210c825"
  - target: "PILLAR-569581e0"
    type: "grounded"
  - target: "PERSONA-015e8c2c"
    type: "benefits"
---
## Motivation

OrqaStudio plugins need to import shared code — types, validation, the artifact graph SDK, and eventually UI components. Currently everything lives inside the monolith. Extracting into scoped packages enables:

- Plugin developers import `@orqastudio/types` for artifact shapes
- Plugins run `@orqastudio/integrity-validator` on their own artifacts
- Plugins use `@orqastudio/sdk` to query the artifact graph
- Plugins use `@orqastudio/components` for consistent UI
- All repos share `@orqastudio/eslint-config` for code standards

## Package Architecture

| Package | Source | Status | Priority |
|---------|--------|--------|----------|
| `@orqastudio/types` | Extract from integrity-validator + sdk types | Not started | P1 — foundation for everything |
| `@orqastudio/integrity-validator` | `orqastudio-integrity-validator` repo | Scaffolded, needs tests + lint | P1 — already created |
| `@orqastudio/eslint-config` | Extract from orqa-studio eslint.config.js + tsconfig | Not started | P1 — needed before tests |
| `@orqastudio/sdk` | Extract from `ui/src/lib/sdk/artifact-graph.svelte.ts` | Not started | P2 — SDK needs to be framework-agnostic |
| `@orqastudio/components` | Extract from `ui/src/lib/components/shared/` | Not started | P3 — Svelte-specific, needs design |
| `@orqastudio/create-plugin` | New scaffolding CLI | Not started | P3 — after SDK and components exist |

## Sketch

### Dependency graph

```
@orqastudio/types
  ↑
@orqastudio/integrity-validator
@orqastudio/sdk
@orqastudio/eslint-config (no type dep)
  ↑
@orqastudio/components (depends on sdk + types)
  ↑
@orqastudio/create-plugin (scaffolds with all of the above)
```

### SDK extraction considerations

The current SDK (`artifact-graph.svelte.ts`) is tightly coupled to:
- Svelte 5 runes (`$state`, `$derived`)
- Tauri `invoke()` calls

For portability, the SDK needs two layers:
1. **Core** (framework-agnostic): graph building, querying, traversal — pure TypeScript
2. **Svelte adapter**: reactive wrapper using runes, calls invoke()

Plugins in the app context use the Svelte adapter. Plugins in CLI/Node context use the core.

### Component library considerations

Shared components (`EmptyState`, `LoadingSpinner`, `StatusIndicator`, etc.) are Svelte 5 + shadcn-svelte. The package would be:
- Svelte 5 only (no framework-agnostic option needed — plugins render inside the app)
- Depends on the app's tailwind theme tokens
- Published as source (not compiled) so Svelte can process them
