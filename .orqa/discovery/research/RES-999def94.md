---
id: RES-999def94
type: discovery-research
title: Repository architecture trade-offs — monorepo vs multi-repo with Forgejo hosting
description: Investigate monorepo vs multi-repo trade-offs for OrqaStudio's dev environment, with Forgejo as the target hosting platform and production packaging as a key constraint.
status: completed
created: 2026-03-23
updated: 2026-03-23
category: infrastructure
relationships:
  - target: PD-ee2910b1
    type: informs
    rationale: May produce a formal AD on repo structure
  - target: PD-9ab3e0a4
    type: informs
    rationale: Research led to the Universal Plugin Capability Model decision
  - target: EPIC-8b01ee51
    type: guides
    rationale: Research identified plugin framework as prerequisite work
  - target: EPIC-2f720d43
    type: guides
    rationale: Research guided the Git Infrastructure epic
---

## Research Questions

### Q1: Monorepo vs Multi-Repo — What Are the Trade-Offs for OrqaStudio Specifically?

Consider 30 submodules across 7 categories (app, libs, plugins, connectors, integrations, registries, templates). Each has independent versioning needs and different release cadences.

Dimensions to evaluate:

- Development experience (DX) for humans and AI agents
- Version management and dependency resolution
- CI/CD pipeline complexity
- Plugin isolation and independent release
- Contributor onboarding and partial checkout
- Licence boundary enforcement (BSL-1.1 vs GPLv3+ plugins)

### Q2: How Does Each Structure Affect Production Packaging?

OrqaStudio ships as:

- A Tauri desktop app (Rust + Svelte bundle)
- Plugins distributed independently
- Libraries published to npm (and potentially crates.io)
- CLI distributed as a standalone tool

For each repo structure, how does the build/release/distribute pipeline work?

### Q3: How Does Forgejo Change the Equation?

With self-hosted Forgejo:

- Can branch protection be centralised?
- Does josh proxy make monorepo viable without losing per-component repos?
- What does the GitHub mirror story look like?
- How does Forgejo CI (Forgejo Actions) compare to GitHub Actions?

### Q4: Is There a Hybrid Path?

Could OrqaStudio use:

- Monorepo for core (app + libs) where tight coupling exists
- Separate repos for plugins (independent lifecycle)
- Forgejo as the primary host with GitHub as a mirror

### Q5: What Does `orqa git` Need to Do Regardless of Structure?

Common tooling needs that apply regardless of repo architecture:

- Unified status/diff across the working set
- Coordinated version bumps
- Cross-component commit linking
- Branch protection enforcement

## Methodology

1. Map the current dependency graph (which components depend on which)
2. Identify tight coupling vs loose coupling boundaries
3. Evaluate each repo structure against production packaging constraints
4. Research Forgejo + josh proxy feasibility
5. Produce a recommendation with trade-off matrix

## Findings

### F1: Current Architecture Has Two Critical Structural Problems

1. **No Cargo workspace** — The 4 Rust crates (`validation`, `search`, `mcp-server`, `lsp-server`) each compile independently. No shared build cache. The app references them via fragile relative `path = "../../../libs/search"` paths.

2. **npm link chain** — All 8 TS packages use `npm link` for development resolution, with a hardcoded `LIB_ORDER` in `install.ts`. This is fragile: breaks on Node version changes, clobbered by `npm install`, doesn't survive `node_modules` deletions.

Both problems are solved by workspaces (Cargo workspace for Rust, npm/pnpm workspaces for TS) — and workspaces work best in a monorepo or hybrid setup.

### F2: Clear Coupling Boundary Exists

**Tightly coupled (15 components, change together):**

- App (frontend + backend)
- All 11 libraries
- `plugins/typescript` (misclassified — actually infrastructure consumed by `cli` and `sdk`)
- `connectors/claude-code`
- `tools/debug`

**Loosely coupled (15 components, independent lifecycle):**

- 10 content-only plugins (markdown/JSON governance artifacts)
- `integrations/claude-agent-sdk`
- 2 registries (JSON manifests)
- `templates`
- `.github-org`

This boundary maps cleanly to a hybrid structure.

### F3: Production Packaging Strongly Favours Core Monorepo

The Tauri desktop app build requires ALL tightly-coupled components:

- Frontend: `app/ui/build` depends on `types`, `sdk`, `svelte-components`, `graph-visualiser`
- Backend: `app/backend` depends on `search`, `validation`, `mcp-server`, `lsp-server`
- CLI: `@orqastudio/cli` depends on `types`, `plugin-typescript`

In the current multi-repo setup, the build pipeline must check out 15+ submodules, run `npm link` in dependency order, then build. A monorepo with workspaces makes the production build self-contained.

Plugin distribution is unaffected — plugins ship as tarballs from their own repos regardless of how the core is structured.

### F4: Forgejo + Josh Is the Most Interesting Long-Term Option

**josh proxy** eliminates the monorepo-vs-multirepo trade-off: one actual repo internally, per-component virtual repos externally. Atomic commits, shared build cache, no submodule orchestration — while preserving independent repos for public consumption on GitHub.

**Risks:**

- josh maturity (active but not widely adopted in production)
- Windows compatibility (dev environment is Windows)
- Requires self-hosted infrastructure (Forgejo + josh)
- GitHub mirror fidelity (git layer works, but issues/PRs/CI don't auto-mirror)

**Recommendation:** josh + Forgejo is the strategic target, but the hybrid monorepo is achievable now without new infrastructure.

### F5: Version Management Works Regardless of Structure

The existing `VERSION` file + `orqa version sync` pattern works for all options. In a monorepo, it becomes simpler (one repo to commit). In multi-repo, `orqa git commit` would handle the cross-repo version bump.

All 30 components currently share a single version (`0.1.4-dev`). This is correct for the tightly-coupled core but unnecessarily constraining for independent plugins. A hybrid structure would allow plugins to version independently.

### Trade-Off Matrix

| Dimension | Multi-Repo (status quo) | Hybrid (core mono + plugin multi) | Forgejo + Josh |
| ----------- | ------------------------ | ----------------------------------- | ---------------- |
| Atomic commits (core) | No | Yes | Yes |
| Dependency resolution | npm link (fragile) | Workspaces (solid) | Workspaces |
| Rust build cache | None | Shared workspace | Shared workspace |
| Plugin independence | Full | Full | Full (virtual repos) |
| CI complexity | N pipelines, no cascade | 1 core + N plugin | 1 pipeline |
| Production build | Fragile link chain | Self-contained | Self-contained |
| Migration effort | None | Medium (15 repos) | High (infra + josh) |
| Infra requirement | GitHub only | GitHub only | Forgejo server + josh proxy |
| External contributor UX | Fork small repo | Fork core or plugin repo | Fork virtual repo |
| License clarity | Per-repo LICENSE | Per-repo + directory-level | Virtual repo LICENSE |

### Recommended Path

**Phase 1 (now): Hybrid monorepo** — Consolidate 15 tightly-coupled components into a single core repo with Cargo workspace + npm workspaces. Keep plugins, registries, and templates as separate repos. This solves the immediate DX problems and makes production builds robust.

**Phase 2 (when ready for cloud): Forgejo** — Stand up a self-hosted Forgejo instance. Centralise branch protection, PR management, and CI. Mirror to GitHub for public collaboration.

**Phase 3 (strategic): Josh proxy evaluation** — If the hybrid monorepo + plugin multi-repo feels cumbersome, evaluate josh to present the monorepo as virtual per-component repos. This eliminates the structural trade-off entirely.

**Regardless of phase: Build `orqa git`** — The CLI tooling for coordinated git operations is needed at every stage. In Phase 1 it handles the remaining multi-repo plugins. In Phases 2-3 it becomes the interface to Forgejo.
