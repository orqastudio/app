---
id: EPIC-f2b9e7d3
type: epic
name: "Git Infrastructure: Forgejo + Monorepo"
status: captured
description: Consolidate 30 repos into a monorepo, stand up Forgejo as authoritative git hosting with GitHub as a bidirectional contribution mirror, migrate CI, and build the sync bridge for dual-platform contribution.
priority: P1
relationships:
  - target: EPIC-d4a8c1e5
    type: depends-on
    rationale: Plugin capability model must be solid before restructuring repos around it
  - target: MS-654badde
    type: fulfils
    rationale: Reliable git infrastructure is foundational to dogfooding
  - target: IDEA-09979c9d
    type: addresses
    rationale: orqa git CLI — unified git operations
  - target: IDEA-7c3d9f2e
    type: addresses
    rationale: Forgejo-based git hosting
  - target: IDEA-f3a08e7a
    type: addresses
    rationale: Git integration and workspace awareness
  - target: IDEA-5c25ac99
    type: addresses
    rationale: Git hosting as cloud sync
  - target: AD-c6abc8e6
    type: implements
    rationale: Organisation-mode multi-project architecture — now with concrete repo structure
  - target: AD-b7e3f1a2
    type: depends-on
    rationale: Universal plugin capability model determines how plugins are consumed in the monorepo
---

## Problem

OrqaStudio's development environment consists of 30 git submodules across 28 GitHub repos. This causes:

1. **No atomic commits** — cross-component changes require committing in each submodule individually, then updating the parent. Error-prone and order-dependent.
2. **Fragile dependency resolution** — npm link chains break on Node version changes, npm install, and node_modules deletions. No Cargo workspace means no shared Rust build cache.
3. **No branch protection** — all 28 GitHub repos are public with zero protection on `main`.
4. **Scattered PR/issue management** — 28 repos = 28 issue trackers and PR queues.
5. **No bidirectional contribution** — no single platform where both maintainers and external contributors can interact.

## Target State

- **One monorepo** containing all 30 current submodule components
- **Forgejo** as the authoritative git hosting (self-hosted, Docker)
- **GitHub** as a bidirectional contribution mirror (contributors use either platform)
- **Cargo workspace + npm workspaces** for dependency resolution
- **Forgejo Actions** for CI/CD
- **Sync bridge** for bidirectional PR/issue sync between Forgejo and GitHub
- **`orqa git`** CLI for the new workflow

## Dependencies

**EPIC-d4a8c1e5 (Plugin Framework: Universal Capability Model) must be completed first.** The monorepo structure depends on:
- How plugins declare and consume content (copy vs extends)
- How plugin config is resolved (workspace paths vs installed paths)
- How integrations participate in the lifecycle
- How templates track schema changes

## Phases

### Phase 1: Monorepo Consolidation

Merge all 30 repos into a single monorepo with full git history preserved.

- **Merge repos using `git-filter-repo`** — each repo → its own subdirectory, full history preserved
- **Set up npm workspaces** — root `package.json` with workspace declarations, replacing the entire npm link chain
- **Set up Cargo workspace** — root `Cargo.toml` with all Rust crates as members, shared build cache
- **Update all internal paths** — Cargo `path =` deps, TypeScript imports, plugin content paths
- **Update `orqa install`** — simplify: no more `LIB_ORDER` npm link chain, workspace resolution handles it
- **Update `orqa version sync`** — workspace-aware version management
- **License per directory** — `LICENSE` files at each component root for mixed BSL-1.1 / Apache-2.0
- **Verify full pipeline** — `make install && make check && make build` from clean clone
- **Archive original GitHub repos** — mark as archived with README pointing to monorepo

**Exit criteria:** `git clone <monorepo> && make install && make check && make build` succeeds.

### Phase 2: Forgejo Instance

Stand up self-hosted Forgejo and migrate the monorepo.

- **Docker Compose setup** — Forgejo + Caddy (auto-TLS reverse proxy)
- **Domain + DNS** — `git.orqastudio.dev` or similar
- **Authentication** — GitHub OAuth2 provider (contributors log in with GitHub account)
- **Push monorepo to Forgejo** — Forgejo becomes authoritative source
- **Configure push mirror** — Forgejo → GitHub (automatic on every push)
- **Branch protection** — protect `main` (require PRs, require reviews)
- **Organisation structure** — `orqastudio` org on Forgejo

**Exit criteria:** Monorepo lives on Forgejo. Push mirror updates GitHub automatically. Contributors can authenticate with GitHub credentials.

### Phase 3: CI Migration

Migrate CI/CD from GitHub Actions to Forgejo Actions.

- **Set up Forgejo runner** — Docker runner registered to instance, using `ghcr.io/catthehacker/ubuntu:act-22.04` image
- **Create PR check workflow** — `.forgejo/workflows/check.yml` running `make check` on every PR
- **Migrate publish workflow** — adapt existing GitHub Actions publish workflow
- **CI status on GitHub mirror** — bot posts Forgejo CI results to GitHub PRs as status checks
- **Release workflow** — tag-triggered build + publish

**Exit criteria:** PRs on Forgejo get automated CI checks. CI status visible on GitHub mirror.

### Phase 4: Bidirectional Contribution Bridge

Build custom sync between Forgejo and GitHub for dual-platform contribution.

- **Design sync protocol** — source-of-truth rules, conflict resolution strategy
- **GitHub webhook listener** — catches new PRs, comments, status changes on GitHub mirror
- **Forgejo webhook listener** — catches merges, comments, status changes on Forgejo
- **PR sync: GitHub → Forgejo** — PR on GitHub creates corresponding PR on Forgejo (fetch contributor branch + create PR via Forgejo API)
- **PR sync: Forgejo → GitHub** — PR merged on Forgejo → push mirror updates GitHub → bot closes GitHub PR with merge reference
- **Issue sync** — bidirectional issue creation and status updates
- **Conflict resolution** — same PR modified on both platforms: last-write-wins with notification

**This is custom software.** Built as a standalone webhook service (Node/Bun) deployable alongside Forgejo.

**Exit criteria:** A contributor submits a PR on GitHub → it appears on Forgejo. Maintainer merges on Forgejo → GitHub PR auto-closes. Vice versa works.

### Phase 5: Developer Tooling

Update `orqa` CLI for the new monorepo + Forgejo workflow.

- **`orqa git status`** — monorepo-aware status (which components have changes)
- **`orqa git pr`** — create PR on Forgejo (and optionally GitHub)
- **`orqa repo audit`** — check branch protection, mirror health, sync bridge status
- **`orqa repo protect`** — enforce branch protection rules
- **Update plugin distribution** — plugins publish from monorepo subdirectories
- **Update registry** — point registry entries to monorepo paths/tags
- **Update templates** — scaffold from monorepo structure

**Exit criteria:** Full development workflow operates through `orqa` CLI against Forgejo.

## Tasks

(To be detailed during Phase planning — each phase becomes its own set of TASK-NNN artifacts)

## Risk Register

| Risk | Impact | Mitigation |
|------|--------|-----------|
| Monorepo merge loses history | High | Use `git-filter-repo` (proven), test on throwaway clone first |
| npm workspaces break install | High | Phase 1 includes full pipeline verification before archiving old repos |
| Forgejo Actions incompatible with workflows | Medium | Use compatible runner image; `.forgejo/workflows/` with adapted syntax |
| Sync bridge complexity | High | Start with PR sync only (highest value); add issue sync incrementally |
| GitHub rate limits on webhooks | Low | Batch sync, exponential backoff |
| Contributor confusion during transition | Medium | Keep GitHub repos as archived with README redirect; dual-platform period |
| josh proxy needed later | Low | Monorepo on GitHub is standard (React, Svelte, VS Code pattern); josh is optional enhancement |

## Out of Scope

(Requires user approval to exclude anything)

## Acceptance Criteria

- [ ] Single monorepo with full git history from all 30 original repos
- [ ] Cargo workspace + npm workspaces — no npm link, shared Rust build cache
- [ ] Forgejo self-hosted with auto-TLS, GitHub OAuth
- [ ] Push mirror to GitHub — automatic, no manual sync
- [ ] Branch protection on `main` — PRs required, reviews required
- [ ] Forgejo Actions CI on every PR
- [ ] Bidirectional PR sync between Forgejo and GitHub
- [ ] `orqa git` CLI commands for monorepo workflow
- [ ] Plugin distribution works from monorepo
- [ ] `git clone && make install && make check && make build` succeeds from clean clone
