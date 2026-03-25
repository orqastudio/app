---
id: IDEA-f4b0eeba
type: idea
title: orqa git — unified git operations across submodules
status: captured
description: CLI commands that orchestrate git operations across the dev environment's 25+ submodules, ensuring consistent state, correct execution order, and linked commits.
created: 2026-03-22
relationships:
  - target: PILLAR-c9e0a695
    type: grounded
    rationale: Structured git workflow across submodules
  - target: PERSONA-c4afd86b
    type: benefits
    rationale: Lead coordinates multi-repo development
  - target: EPIC-2f720d43
    type: realises
    rationale: Bundled into the Git Infrastructure epic (Phase 5 — Developer Tooling)
---

# orqa git — Unified Git Operations Across Submodules

## Problem

The dev environment has 25+ submodules. Every coordinated change (version bumps,
artifact migrations, dependency updates) requires:

1. Committing inside each affected submodule individually
2. Pushing each submodule
3. Staging the updated submodule pointers in the parent repo
4. Committing the parent repo
5. Pushing the parent repo

This is error-prone, order-dependent, and tedious. Agents frequently end up in the
wrong working directory, forget to push a submodule, or commit the parent repo before
all submodule pushes are complete.

## What `orqa git` Brings

### Core Principle: Always Resolve to Dev Root

Every `orqa git` command first resolves `ORQA_ROOT` (same as all orqa commands), then
executes from there. This means an agent or human running `orqa git status` from inside
`plugins/svelte/knowledge/` gets the same result as running it from the repo root.
No more "which directory am I in?" failures.

### Commands

```
orqa git status          Show git status of dev repo + all submodules with changes
orqa git diff            Show diffs across all dirty submodules
orqa git commit -m "X"  Commit in all dirty submodules, then commit parent with pointer updates
orqa git push            Push all submodules that are ahead, then push parent
orqa git pull            Pull parent, then update all submodules recursively
orqa git sync            Full pull + push cycle (resolve divergence)
orqa git stash           Stash across all dirty submodules + parent
orqa git stash pop       Pop stashes in reverse order
```

### Execution Order (Critical)

**Commit order** (inside-out):
1. Identify all submodules with staged/unstaged changes
2. Commit each submodule (alphabetical within dependency tier)
3. Stage all updated submodule pointers in parent
4. Commit parent repo with a summary message linking submodule commits

**Push order** (inside-out):
1. Push all submodules that are ahead of their remote
2. Push parent repo (which references the just-pushed submodule SHAs)

**Pull order** (outside-in):
1. Pull parent repo (gets new submodule pointer SHAs)
2. `git submodule update --init --recursive` (checks out the new SHAs)

### Commit Linking Convention

When `orqa git commit` commits the parent repo, the commit message includes a
trailer block listing all submodule commits made in that batch:

```
Artifact migration: core → owning plugins

Submodule-Commits:
  plugins/agile-governance: a9b3f5c Migrate core artifacts
  plugins/systems-thinking: 3d561be Migrate core artifacts
  connectors/claude-code: fa8ae7f Migrate connector setup knowledge

Co-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>
```

This creates a traceable link: from any parent commit you can see exactly which
submodule commits were part of that batch, and from any submodule commit you can
find the parent commit that recorded its pointer.

### Flags

```
--dry-run         Show what would happen without executing
--submodule=X     Target specific submodule(s) only
--exclude=X       Exclude specific submodule(s)
--parallel        Run independent operations in parallel (default for push)
--sequential      Force sequential execution (default for commit)
--message, -m     Commit message (applied to parent; submodules get auto-messages)
```

## Benefits

1. **No directory confusion** — always resolves to dev root regardless of cwd
2. **Correct ordering** — commits/pushes happen inside-out, pulls happen outside-in
3. **Atomic batches** — parent commit links to all submodule commits in the batch
4. **Agent-safe** — agents can run `orqa git commit -m "..."` from anywhere and get correct behavior
5. **Visibility** — `orqa git status` shows the full picture in one command
6. **Parallelism where safe** — pushes and status checks run in parallel; commits run sequentially

## Potential Downsides / Risks

1. **Hides git complexity** — developers who don't understand submodules may be surprised when things
   break in ways that `orqa git` doesn't handle (e.g., detached HEAD in a submodule, merge conflicts
   in submodule pointers). Mitigation: always show raw git output, never swallow errors.

2. **Commit granularity loss** — if `orqa git commit` auto-commits all dirty submodules with one
   message, you lose the ability to write per-submodule commit messages. Mitigation: `--submodule=X`
   flag for targeted commits, and allow passing per-submodule messages via a manifest file.

3. **Divergent branches** — the commands assume all submodules track `main`. Feature branches
   in individual submodules would need special handling. Mitigation: detect non-main branches and
   warn, but don't block.

4. **Push failures mid-batch** — if one submodule push fails (auth, hook rejection), the parent
   commit already references the unpushed SHA. Mitigation: push all submodules first, only then
   commit+push parent. If any submodule push fails, abort before touching the parent.

5. **Performance** — 25+ submodules means 25+ git operations per command. Some may be slow.
   Mitigation: parallel execution where safe, skip clean submodules, cache status.

6. **Interference with existing workflow** — `make` targets and raw git commands still work.
   If someone uses `orqa git commit` and someone else uses raw `git commit` in a submodule,
   the parent pointer won't update. Mitigation: this is a coordination tool, not a gate. Document
   that `orqa git` is the recommended path but raw git still works.

## Failure Recovery Research (2026-03-22)

No existing tool solves atomic cross-submodule commit with rollback. Key findings:

### Existing Tools Assessed

| Tool | Language | Cross-Repo Atomic | Failure Recovery | Status |
|------|----------|-------------------|-----------------|--------|
| **git2** (Rust crate) | Rust | Build your own | Build your own | Very active, MIT |
| **gix/gitoxide** | Rust | Build your own | Build your own | Very active, MIT |
| **josh** (git proxy) | Rust | Yes (single repo) | N/A | Active, MIT |
| **git-subrepo** | Bash | N/A (inlines) | Single-repo | Active, MIT |
| **meta** | Node.js | No | None | Dead |
| **gita** | Python | No | None | Active, MIT |
| **submod** | Rust | No | Per-operation | Active, MIT |

None provide coordinated rollback across repos. All multi-repo tools are fire-and-forget.

### Recovery Patterns (from distributed systems)

**Pattern D — Stage in Branches (recommended)**:
1. Create a working branch in each submodule
2. Commit to working branches
3. Only merge to main when ALL submodules pass
4. On failure: working branches remain unmerged, no cleanup needed

This leverages git's native branching as a staging area. The `main` branch
only moves forward when the entire batch is verified.

**Pattern F — Incremental Migration (Snellman)**:
Avoid needing atomicity — make each commit backward-compatible and single-repo.
Push shared library first, then push N consumers, then cleanup. Each step
is independently revertible.

**Native git**: `git push --recurse-submodules=on-demand` pushes submodule
changes before the superproject. If a submodule push fails, the superproject
push aborts. Closest thing to built-in orchestration.

### Recommendation

1. Use `git2` Rust crate or `child_process` git shells for operations
2. Implement Pattern D (stage in branches) for coordinated changes
3. Add a saga log recording which submodules committed/pushed for recovery
4. Use `--recurse-submodules=on-demand` for the superproject push
5. For simple day-to-day use, Pattern F (incremental) is often sufficient

### Relationship to Forgejo (IDEA-174fa5c8)

The **josh** project (Rust git proxy, MIT) is architecturally interesting —
it presents sub-paths of a monorepo as independent virtual repos. Pushes to
virtual repos map back to the monorepo. Atomicity is free because there's
one actual repo.

This aligns with the Forgejo idea: a self-hosted git instance could use
josh-style virtual repos to eliminate the multi-repo problem entirely while
still presenting each submodule as an independent repo for public consumption
on GitHub.

## Implementation Approach

1. New command file: `libs/cli/src/commands/git.ts`
2. Uses `getRoot()` to resolve dev root
3. Reads `.gitmodules` to discover all submodules
4. Uses `child_process.execSync` or `spawnSync` for git operations
5. Respects `.orqa/project.json` plugins section for submodule ordering
6. Outputs colored status table (submodule name, branch, ahead/behind, dirty files)
7. `--submodule=X` flag for targeting specific submodule(s)

## Dependency Order (for commits)

Libraries before consumers:
1. `libs/types` (no deps)
2. `libs/validation`, `libs/brand` (depend on types)
3. `libs/sdk`, `libs/cli`, `libs/search`, `libs/mcp-server`, `libs/lsp-server` (depend on types/validation)
4. `libs/svelte-components`, `libs/graph-visualiser` (depend on types/sdk)
5. `plugins/*`, `connectors/*` (depend on libs)
6. `app` (depends on everything)
7. Parent dev repo (records all pointers)

## Relationship to Forgejo Cloud Idea

[IDEA-174fa5c8](IDEA-174fa5c8) (OrqaStudio Cloud — Forgejo-based git hosting) addresses
many of the deeper problems that `orqa git` works around:

| Problem | `orqa git` approach | Forgejo approach |
|---------|-------------------|-----------------|
| Branch protection | Relies on GitHub API (`gh`) to set rules per-repo | Forgejo instance enforces rules centrally |
| PR coordination | `orqa pr create` creates linked PRs across repos | Forgejo manages all repos in one instance — one PR per change |
| Direct push blocking | Must configure 28 separate GitHub repos | One Forgejo config applies to all repos |
| Commit linking | Convention-based trailers in parent commit | Forgejo can natively track cross-repo references |
| Mirror to GitHub | N/A — GitHub is primary | Forgejo mirrors to GitHub automatically |

**If Forgejo is adopted**, the 28-separate-repo problem disappears. Submodules still
exist for code organisation, but all repos live in one Forgejo instance where:
- Branch protection is configured once, not 28 times
- PRs are visible in one dashboard, not scattered across GitHub repos
- Push permissions are managed centrally
- Mirror to GitHub preserves public collaboration

**Pragmatic path**: Build `orqa git` now as utility tooling (it's needed regardless).
When Forgejo lands, it becomes the enforcement layer that `orqa git` coordinates against.
The CLI commands remain the same — only the remote changes.

### Repo Protection Audit (2026-03-22)

All 28 repos in the orqastudio GitHub org are currently:
- **Public**, **unprotected** — no branch protection on `main`
- Anyone with write access can push directly to `main`
- Zero PR requirements, zero review requirements

This needs addressing either via GitHub API (`gh`) as a short-term fix, or via Forgejo
as the medium-term solution. See [IDEA-8cad4236](IDEA-8cad4236) (Git Integration) for
the GitHub-specific protection plan.