# File Audit: Root Files and Configuration

Scope: repo root, `.claude/`, `.forgejo/`, `.state/`

---

## 1. Root-Level Files

| File | Type | Size | Lines | Description |
|------|------|------|-------|-------------|
| `VERSION` | Version | 10 B | 1 | Current version: `0.1.4-dev` |
| `VERSIONS` | Version history | 371 B | 10 | BSL-1.1 release history template. No stable releases yet; pre-release versions do not start the change-date clock. Format: `version | release-date | change-date` |
| `.gitignore` | Config | 117 B | 13 | Ignores: `node_modules/`, `dist/`, `build/`, `target/`, `models/`, `.svelte-kit/`, `.claude/`, `.state/`, swap files, `.DS_Store`, DuckDB files |
| `.mcp.json` | Config (MCP) | 114 B | 10 | MCP server config. Single server `orqastudio` running `orqa mcp` |
| `.lsp.json` | Config (LSP) | 651 B | 38 | LSP server config. Four servers: `rust` (rust-analyzer), `svelte` (svelteserver), `typescript` (typescript-language-server), `orqastudio` (orqa lsp for .md files) |
| `Makefile` | Build | 524 B | 16 | Bootstrap-only Makefile. Single target `install` runs `scripts/install.sh`. Comments point to `orqa` CLI for all other commands |
| `package.json` | Config (npm) | 627 B | 30 | npm workspace root `orqastudio-monorepo` (private). 25 workspaces across `libs/`, `plugins/`, `connectors/`, `integrations/`, `app`. Single devDependency: `yaml ^2.8.2` |
| `package-lock.json` | Lockfile (npm) | 232 KB | 9,117 | npm dependency lockfile |
| `Cargo.toml` | Config (Rust) | 598 B | 25 | Rust workspace root. 4 members: `libs/validation`, `libs/search`, `libs/mcp-server`, `libs/lsp-server`, `app/src-tauri`. Resolver v2. Workspace-wide clippy lints (pedantic with select allows) |
| `Cargo.lock` | Lockfile (Rust) | 197 KB | 8,168 | Rust dependency lockfile |
| `LICENSE` | Legal | 3,752 B | 80 | BSL-1.1 (Business Source License). Licensor: Bobbi Byrne-Graham. Change License: Apache 2.0. Commercial use restricted (no competing products/services). Internal, non-commercial, plugin dev, and evaluation use permitted |
| `README.md` | Documentation | 2,936 B | 76 | Project README. Describes OrqaStudio as "AI-assisted clarity engine for structured thinking and adaptive action". Covers prerequisites (Git, Docker, Node 22+, Rust), setup (`make install`), daily CLI commands, repo structure |
| `CONTRIBUTING.md` | Documentation | 1,095 B | 25 | Contribution guide. States not accepting contributions yet (pre-release). Primary dev on Forgejo (self-hosted), GitHub is read-only mirror. Links to discussions and future community-plugins repo |
| `ARCHITECTURE.md` | Documentation | 33 KB | 566 | Comprehensive architecture reference. Describes intended architecture as benchmark for auditing. Covers design principles (P1-P4+), plugin-composed everything, one-context-window-per-task, generated prompts, declarative over imperative. Sources: RES-d6e8ab11, AD-1ef9f57c |
| `validation_stderr.txt` | Log/output | 19 KB | 570 | JSON-formatted validation output from the daemon's validation engine. Contains `BrokenLink` errors for artifact references that don't resolve (e.g., KNOW-* references). Not auto-fixable entries |

### Root-Level Directories (not inventoried here)

| Directory | Description |
|-----------|-------------|
| `app/` | Tauri desktop app (Rust backend + Svelte frontend) |
| `libs/` | Shared libraries (TypeScript + Rust) |
| `plugins/` | First-party plugins |
| `connectors/` | AI tool connectors (Claude Code) |
| `integrations/` | SDK integrations (Claude Agent SDK sidecar) |
| `templates/` | Plugin scaffold templates |
| `infrastructure/` | Docker configs for local git server + sync bridge |
| `tools/` | Development utilities |
| `scripts/` | Build/install scripts |
| `models/` | ML models (gitignored) |
| `.orqa/` | Governance artifacts |
| `file-audit/` | This audit output |
| `tmp/` | Temporary files (legacy; new state goes to `.state/`) |
| `node_modules/` | npm dependencies (gitignored) |
| `target/` | Rust build output (gitignored) |

---

## 2. `.claude/` Directory

Configuration for Claude Code AI assistant tooling. This directory is gitignored (listed in `.gitignore`).

### Root Files

| File | Type | Size | Lines | Description |
|------|------|------|-------|-------------|
| `CLAUDE.md` | Config (Claude) | 6,718 B | 133 | Project instructions for Claude Code. Defines: autonomous execution rules, three-layer agent taxonomy, team discipline (TeamCreate/TaskCreate/Agent/TeamDelete lifecycle), hub-spoke orchestration, role-based tool constraints, completion gate protocol, key design decisions, git workflow, session protocol, drift prevention rules |
| `settings.json` | Config (Claude) | 127 B | 7 | Claude Code settings. Sets `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE=70` and `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1`. Empty hooks object |
| `settings.local.json` | Config (Claude) | 221 B | 11 | Local (non-shared) Claude Code settings. Permissions: allows `git add:*`, `git commit:*`, `cd:*`, plus two specific `cp`/`rm` commands for plugin rename |

### `.claude/agents/` -- Agent Role Definitions

6 agent definition files. Each defines a universal role with YAML frontmatter (name, description) followed by role-specific instructions, tool constraints, and behavioral rules.

| File | Role | Size | Lines | Description |
|------|------|------|-------|-------------|
| `implementer.md` | Implementer | 4,148 B | 103 | Implements code changes. Can edit files and run shell. Does not self-certify |
| `reviewer.md` | Reviewer | 4,065 B | 101 | Reviews code/artifacts for quality and compliance. Read-only, produces PASS/FAIL verdicts |
| `researcher.md` | Researcher | 3,755 B | 94 | Investigates questions, gathers information. Read-only codebase access, produces findings |
| `writer.md` | Writer | 3,595 B | 90 | Creates/edits documentation. No source code writing or shell access |
| `governance-steward.md` | Governance Steward | 3,926 B | 98 | Maintains `.orqa/` governance artifacts. Ensures graph integrity |
| `planner.md` | Planner | 3,632 B | 91 | Designs approaches, maps dependencies. Read-only, does not implement |

---

## 3. `.forgejo/` Directory

CI/CD workflow definitions for self-hosted Forgejo (Gitea-compatible) instance.

### `.forgejo/workflows/`

| File | Type | Size | Lines | Description |
|------|------|------|-------|-------------|
| `check.yml` | CI workflow | 1,238 B | 52 | Runs on PR and push to main. Steps: checkout, setup Node 22, install Rust (stable + clippy + rustfmt), npm install, cargo fetch, build TypeScript packages in dependency order, rust format check, clippy (deny warnings), rust tests. Container: `ghcr.io/catthehacker/ubuntu:act-22.04` |
| `publish.yml` | CI workflow | 647 B | 33 | Runs on version tags (`v*`). Steps: checkout, setup Node 22 with GitHub npm registry, install Rust, npm install, cargo fetch, then runs `orqa install publish`. Uses `GITHUB_TOKEN` for npm auth |

---

## 4. `.state/` Directory

Operational state and session data. This directory is gitignored. Contains runtime state, logs, metrics, and agent team findings.

### Root-Level `.state/` Files

| File | Type | Size | Lines | Description |
|------|------|------|-------|-------------|
| `session-state.md` | Session state | 1,510 B | 35 | Current session state. Records EPIC-2451d1a9 as COMPLETE (31/31 tasks, 29 ACs PASS). Documents plugin architecture table, key fixes, final commit c4ef8e6b |
| `daemon.pid` | PID file | 6 B | 1 | Stores PID of the validation daemon process: `23432` |
| `dev-controller.json` | Runtime state | 110 B | 8 | Dev controller state. Shows PID 40552, state `app-crashed`, with search (19408), MCP (47284), and LSP (38952) subprocess PIDs. App is null (crashed) |
| `dev-controller.log` | Log | 440 KB | 2,088 | Dev controller log output. Runtime logs from the development environment controller |
| `hook-metrics.json` | Metrics | 6,510 B | 18 | JSONL-formatted hook execution metrics. Records bash-safety, validate-artifact, and prompt-injector hook events with timestamps, durations, outcomes (allowed/blocked/injected/invalid) |
| `orchestrator-preamble.md` | Generated prompt | 148 KB | 2,985 | Generated orchestrator preamble from the prompt pipeline. Classified as "review" mode. Shows 30 sections included, 15 trimmed, 36,499 tokens generated against 2,500 budget |
| `precommit-violations.jsonl` | Violation log | 13 KB | 20 | JSONL log of pre-commit hook violations. Records eslint and validation engine failures with timestamps and affected file lists |
| `business-logic-duplication-audit.md` | Audit report | 21 KB | 389 | Full-codebase audit of business logic duplication (dated 2026-03-24). Covers libs/, app/, backend/, sidecar/, connectors/, plugins/, tools/. Enforces "one source of truth per concern" principle |

### `.state/scripts/`

| File | Type | Size | Lines | Description |
|------|------|------|-------|-------------|
| `fix-filename-mismatches.js` | Migration script | 4,451 B | ~100 | Node.js script to fix filename/ID mismatches in `.orqa/`. Renames files to match frontmatter IDs and cleans up manifest.json references |

### `.state/team/` -- Agent Team Findings

Contains findings files from agent team executions. **36 team directories** with **214 total files** plus 1 root-level file.

Root file:
| File | Type | Size | Description |
|------|------|------|-------------|
| `restructure-findings.md` | Findings | 3,932 B | Restructure analysis findings |

#### Team Directories Summary

| Team Directory | Files | Date Range | Purpose |
|----------------|-------|------------|---------|
| `ac-completion/` | 3 | Mar 26 | Knowledge structure, tmp fix, type migration findings |
| `ad-fixes/` | 1 | Mar 25 | Architecture decision fixes |
| `agent-lifecycle/` | 2 | Mar 25 | Agent lifecycle management tasks |
| `audit/` | 7 | Mar 25 | Epic-level audit reports (7 epics audited) |
| `audit-fixes/` | 4 | Mar 25 | Audit remediation tasks |
| `audits/` | 3 | Mar 25 | Codebase audit tasks |
| `cleanup/` | 1 | Mar 25 | Cleanup task findings |
| `content-migration/` | 6 | Mar 25 | Content migration tasks + 3 JS scripts (assign-unowned.js, tier-classification.js, update-plugin-manifests.js) |
| `dedup-epic/` | 7 | Mar 25 | Epic deduplication tasks |
| `enforcement/` | 6 | Mar 25 | Enforcement rule tasks (daemon rules, rule engine, session start, field check, tool matcher, contributions) |
| `epic-rewrite/` | 2 | Mar 25 | Epic rewrite tasks |
| `final-gate/` | 1 | Mar 26 | Final gate verification |
| `fixes-and-audits/` | 3 | Mar 25 | Combined fix and audit tasks |
| `fix-sot/` | 0 | Mar 25 | Empty directory (source-of-truth fixes) |
| `graph-foundation/` | 12 | Mar 25 | Graph foundation work: ID migration scripts, audit data (328 KB JSON), migration report (79 KB JSON), task findings |
| `graph-foundation-p2/` | 3 | Mar 25 | Graph foundation phase 2 tasks |
| `graph-perf/` | 1 | Mar 25 | Graph performance research (12 KB) |
| `graph-relationships-fix/` | 4 | Mar 25 | Graph relationship fix tasks |
| `human-gates/` | 1 | Mar 25 | Human gate implementation task |
| `phase1-decomposition/` | 26 | Mar 25-26 | Phase 1 decomposition: 23 task findings + phase4 principles + p1 fix + migration task |
| `port-epic/` | 15 | Mar 25 | Port epic work: SOT audit + 14 task/config/doc findings |
| `port-fix/` | 5 | Mar 25 | Port fix tasks |
| `post-migration-fixes/` | 12 | Mar 25 | Post-migration fix tasks (tasks 1-12, skipping some numbers) |
| `prompt-pipeline/` | 3 | Mar 25 | Prompt pipeline tasks |
| `remaining-fixes/` | 4 | Mar 25 | Remaining fix tasks |
| `remediation/` | 25 | Mar 25 | Major remediation: 24 task findings + enforcement research |
| `research/` | 1 | Mar 25 | Token efficiency research |
| `session-priorities/` | 15 | Mar 25 | Session priority tasks (tasks 1-17, some combined) |
| `sot-fixes/` | 24 | Mar 25 | Source-of-truth fixes: CLI simplification, architecture docs, epic updates, LSP, precommit, validation restructure, and 18 numbered tasks |
| `statusbar-fix/` | 2 | Mar 25 | Status bar fix tasks |
| `task-creation/` | 3 | Mar 25 | Phase 1/2/3/4/5 task creation findings |
| `team-design-research/` | 8 | Mar 25 | Team design research tasks. Largest team by content: 7 research tasks (27-53 KB each) + summary task |
| `validator-fix/` | 2 | Mar 26 | Validator fixes: scanner fix, YAML fix |
| `workflow-engine/` | 4 | Mar 25 | Workflow engine tasks |
| `zero-warnings/` | 2 | Mar 25 | Zero-warning cleanup tasks |

---

## 5. Workspace Layout (from package.json)

The npm workspace defines 25 packages:

| Category | Packages |
|----------|----------|
| **libs/** | `types`, `logger`, `brand`, `cli`, `sdk`, `svelte-components`, `graph-visualiser` |
| **plugins/** | `agile-workflow`, `cli`, `coding-standards`, `core`, `githooks`, `rust`, `software-kanban`, `svelte`, `systems-thinking`, `tauri`, `typescript` |
| **connectors/** | `claude-code` |
| **integrations/** | `claude-agent-sdk` |
| **app** | `app` (root) |

The Rust workspace defines 5 crates:

| Crate | Path |
|-------|------|
| `validation` | `libs/validation` |
| `search` | `libs/search` |
| `mcp-server` | `libs/mcp-server` |
| `lsp-server` | `libs/lsp-server` |
| `src-tauri` | `app/src-tauri` |

---

## 6. Summary Statistics

| Area | Count |
|------|-------|
| Root-level files | 15 |
| Root-level directories | 16 |
| `.claude/` files | 9 (3 config + 6 agent defs) |
| `.forgejo/` files | 2 CI workflows |
| `.state/` root files | 8 |
| `.state/scripts/` files | 1 |
| `.state/team/` directories | 36 |
| `.state/team/` total files | 214 |
| npm workspaces | 25 |
| Rust workspace crates | 5 |
