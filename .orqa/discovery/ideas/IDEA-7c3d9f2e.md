---
id: IDEA-7c3d9f2e
type: idea
title: OrqaStudio Cloud — Forgejo-based git hosting as a plugin
description: A minimal OrqaStudio-branded git hosting platform based on Forgejo, shipped as a Docker container and deployable as a plugin for local or cloud use. Provides project/organisation/user oversight with the same governance model that runs locally.
status: captured
created: 2026-03-21
updated: 2026-03-21
relationships:
  - target: PILLAR-569581e0
    type: grounded
    rationale: Clarity Through Structure — centralised governance visibility across projects
  - target: PILLAR-94b281db
    type: grounded
    rationale: Purpose Through Continuity — projects maintain coherence whether local or hosted
  - target: PERSONA-cda6edd6
    type: benefits
    rationale: Alex (Lead) — organisation-level oversight across all projects
  - target: PERSONA-a4b15450
    type: benefits
    rationale: Jordan (Independent) — can run locally without cloud dependency
---

## The Idea

A minimal OrqaStudio-branded git hosting platform based on Forgejo (GPLv3+, community-governed), shipped as a Docker container. Deployable two ways:

1. **As a plugin** — local Docker container for developers who want self-hosted git with OrqaStudio governance built in
2. **As a cloud service** — hosted multi-tenant instance for organisations wanting centralised project oversight

### Architecture

```
OrqaStudio App (local)
  └── orqa-cloud plugin
       └── Forgejo container (git server)
       └── OrqaStudio management API (project/org/user management)
       └── Governance dashboard (health metrics across all projects)
```

The .orqa/ directory is just files in a repo — works against Forgejo the same as GitHub. All governance artifacts, MCP tools, validation, and the learning loop operate identically regardless of git backend.

### What Forgejo Provides (base)

- Git hosting (repos, branches, PRs, issues)
- User/organisation management
- API for automation
- Docker-native, lightweight Go binary
- GPLv3+ — can't be closed-sourced, aligns with democratisation mission

### What OrqaStudio Adds (management layer)

- Governance health dashboard across all hosted projects
- Cross-project graph metrics (pillar traceability, orphan rates, relationship completeness)
- Organisation-level rules and knowledge that cascade to all projects
- Plugin marketplace served from the cloud instance
- Cross-project learning aggregation (opt-in, per IDEA-4f8e2a1c)
- AI-driven semantic merge for version upgrades

### Deployment Options

| Mode | Who | How |
|------|-----|-----|
| **Local plugin** | Solo developer, small team | `orqa plugin install @orqastudio/cloud` → Docker container starts locally |
| **Self-hosted cloud** | Organisation | Deploy Docker image to own infrastructure |
| **Managed cloud** | Organisation (SaaS) | OrqaStudio-hosted instance |

### josh — Virtual Monorepo Proxy (worth investigating)

The [josh project](https://github.com/josh-project/josh) (Rust, MIT) is a git proxy
that presents sub-paths of a monorepo as independent virtual repositories. Pushes to
virtual repos get mapped back to the monorepo. This means:

- One actual repo, one commit history, atomic commits across submodules for free
- Each submodule still appears as an independent repo to external consumers
- Could sit in front of Forgejo or alongside it

This could solve the "28 separate repos" problem architecturally: internally we work
with one repo (via josh), externally each component mirrors as a separate repo to
GitHub/Forgejo. Submodule orchestration and failure recovery become non-issues because
there is only one repo.

### Why Forgejo (not Gitea/GitLab)

- GPLv3+ licence — guarantees it stays open, aligns with BSL ethical use addendum
- Community-governed (Codeberg e.V.) — not company-controlled
- Lightweight — single Go binary, minimal resources
- Full API — custom management layer on top via REST
- Fork-friendly — can customise the UI to OrqaStudio branding
- Proven at scale (powers Codeberg)

### Licence Compatibility

Forgejo is GPLv3+. OrqaStudio is BSL-1.1. The management layer (API, dashboard, plugin) is a SEPARATE process that communicates with Forgejo via its API — no licence conflict. The Forgejo container is distributed as-is under GPLv3+.

### Cross-Project Learning Role

Orqa Cloud is not just a git host — it is the intermediary learning loop stage that makes cross-project and cross-org learning possible.

**What Cloud provides in the learning architecture:**

- **Aggregation**: the management API receives lessons from all projects associated with an org and identifies patterns that recur across multiple projects
- **Conflict detection**: when lessons from different projects contradict each other, Cloud surfaces the conflict for human review rather than silently choosing a winner
- **Controlled rollout**: org admins review aggregated cross-project rules and explicitly approve what propagates down to all org projects — nothing cascades without approval
- **Global tier bridge**: for orgs that opt in, anonymised patterns flow further up to inform improved default starting artifacts shipped with new versions

**What Cloud does not provide:**

Cloud is the aggregation and rollout mechanism. It is not the data capture mechanism.

- Data capture (observation events, telemetry, hook execution records) requires analytics or observability tooling configured on each individual project
- A project with no analytics tooling produces no data for Cloud to aggregate, even if it is connected to a Cloud instance
- Cloud and analytics tooling are complementary: local analytics without Cloud means learning stays in Tier 1 (project-local); Cloud without local analytics means the aggregation layer has nothing to process

**User control at every tier:**

| Tier | Who controls propagation |
|------|--------------------------|
| Local (Tier 1) | Project team — lessons become rules within the project |
| Org (Tier 2) | Org admin — reviews aggregated patterns, approves cross-project rule rollout |
| Global (Tier 3) | Org admin opts org in; project team can still opt their project out |

**Path to managed service:**

The first-party hosted version of Orqa Cloud opens a direct path to offering Tier 2 and Tier 3 learning as a SaaS offering. Organisations that don't want to operate their own Cloud instance can subscribe to a hosted tier and receive cross-project learning with zero infrastructure overhead. This is a natural extension of the current architecture — the same management API, the same rollout controls, the same consent model — operated by OrqaStudio rather than the customer.

### Registry Hosting

When built locally, Orqa Cloud should host the plugin and project-type registries instead of GitHub. This gives us:

- **Controlled plugin submission flow** — plugins go through a governed review process on our own infrastructure rather than relying on GitHub's PR model
- **Registry independence** — no dependency on a third-party platform for core distribution infrastructure
- **Self-dogfooding** — OrqaStudio's own registries are hosted on OrqaStudio Cloud, proving the platform

### Multi-Origin Sync

Orqa Cloud instances must support syncing to any number of remotes — not just two. The protocol is git itself, and git handles N remotes natively. A local Forgejo instance can sync to:

- A hosted Orqa Cloud instance (org-level)
- GitHub, GitLab, Bitbucket, or any other git remote
- Multiple hosted Cloud instances (e.g. one per department)

Git resolves conflicts between origins using its standard merge/rebase mechanisms. The management layer configures remote lists per-project and handles push mirroring via Forgejo's built-in mirror support.

This enables several critical workflows:

| Scenario | How multi-origin helps |
|----------|----------------------|
| **Team collaboration** | Different team members work against different remotes — some prefer GitHub, some use the local instance, some use both |
| **Domain separation** | Frontend team pushes to GitHub (open source), backend team pushes to internal Forgejo (proprietary governance), ops team pushes to a third remote |
| **Dogfooding** | OrqaStudio itself runs on a local Docker instance for governance AND syncs to GitHub for public collaboration |
| **Disaster recovery** | N remotes means no single point of failure — any origin can reconstruct the full history |
| **Migration path** | Teams can gradually move between providers without a hard cutover — all remotes stay in sync until ready to decommission |
| **Federated orgs** | Multiple organisations each run their own Cloud instance, syncing shared repos between them while keeping org-specific repos private |

The protocol scales because it IS git. Adding a new origin is `git remote add` + a mirror config in the management API. Conflict resolution uses the same merge strategies developers already know.

### Relationship to `orqa git` and Repo Protection

[IDEA-09979c9d](IDEA-09979c9d) (`orqa git`) builds CLI utility commands for coordinating
git operations across the dev environment's 28 submodules. Currently all 28 GitHub repos
are public and unprotected — no branch protection, no PR requirements. Forgejo would
solve this structurally: one instance, one config, all repos protected. `orqa git` remains
the CLI interface regardless of backend (GitHub or Forgejo).

[IDEA-f3a08e7a](IDEA-f3a08e7a) (Git Integration) covers the GitHub-specific protection
plan as a short-term measure until Forgejo is in place.