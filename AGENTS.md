# Forge

Desktop application that automates product management and the agentic implementation cycle. Designed for Product Managers and Tech Leads who define process governance, delegate implementation to AI agents, and verify results — with a visual process layer where governance artifacts (agents, skills, rules, learning loops, documentation) live alongside the conversation as interactive, editable documents.

## Tech Stack

| Layer | Technology | Purpose |
|-------|------------|---------|
| **Desktop Shell** | Tauri v2 (Rust backend) | Native app, IPC, file system access, process spawning |
| **Frontend** | Svelte 5 (runes) / TypeScript 5.x | Reactive UI with `$state`, `$derived`, `$effect` |
| **UI Components** | shadcn-svelte / Tailwind CSS / bits-ui | Accessible component library |
| **AI Integration** | Claude API + Agent SDK | Streaming responses, tool-use loops, agent orchestration |
| **Database** | SQLite (rusqlite or sqlx) | Session history, metrics, project config |
| **Testing** | cargo test / Vitest / Playwright | Full coverage from day one |
| **Quality** | clippy (pedantic) / rustfmt / ESLint / svelte-check | Zero errors policy |

## Feature Governance (MANDATORY)

Every feature request, task, and implementation must pass the Two-Pillar Test before work begins.

**Pillar 1: Self-Learning Loop** — Does this feature help the system improve over time? Does it capture lessons, track metrics, feed retrospectives back into governance, or accumulate knowledge across sessions?

**Pillar 2: Process Governance** — Does this feature enforce or surface standards, rules, and workflows? Does it make governance tangible, visible, and manageable?

### Feature Rejection Criteria

Reject any feature proposal that:

- Serves neither pillar
- Adds complexity without improving learning or governance
- Is a generic developer tool feature with no connection to managed agentic development
- Cannot explain how it makes the system smarter (Pillar 1) or governance more visible/enforceable (Pillar 2)

### Questions Every Agent Should Ask

Before implementing any feature:

1. Which pillar does this serve? (If neither, raise it before proceeding)
2. Does this help the system learn from past sessions? (Pillar 1)
3. Does this make governance more visible or enforceable? (Pillar 2)
4. Does the implementation stay within the IPC boundary? (Tauri commands only)
5. Does this use Result types for error handling? (No unwrap in production)

### Foundational Principles Are Immutable

The Two-Pillar framework, the Tauri v2 + Svelte 5 + Rust + SQLite technology stack, the IPC boundary design, and the governance rules are **foundational principles**. They can ONLY be changed with explicit user direction and approval. No agent may modify, weaken, or work around these principles autonomously.

## End-to-End Completeness (MANDATORY)

Every feature requires all 4 layers in the same commit:

1. **Rust command** — `#[tauri::command]` handler in `src-tauri/src/commands/`
2. **IPC types** — Serializable Rust structs + matching TypeScript interfaces
3. **Svelte component** — UI that calls the command via `invoke()`
4. **Store binding** — Reactive store in `.svelte.ts` that manages state

A feature is not done until the full chain works: user action → Svelte → IPC → Rust → SQLite/API → response → store update → UI re-render.

### Questioning Misaligned Instructions

If a user instruction conflicts with a foundational principle: flag the conflict, ask for clarification, document changes if confirmed, never silently comply.

## Quick Start

```bash
# Prerequisites
# - Node.js 18+
# - Rust (via rustup) with stable toolchain
# - Tauri CLI: cargo install tauri-cli
# - System dependencies for Tauri: https://v2.tauri.app/start/prerequisites/

# 1. Install frontend dependencies
npm install

# 2. Development mode (starts both Rust backend and Svelte frontend)
cargo tauri dev

# 3. Production build
cargo tauri build

# 4. Run tests
cargo test --manifest-path src-tauri/Cargo.toml
npm run test
```

## Project Structure

```text
forge/
├── AGENTS.md                     # Cross-agent project instructions (this file)
├── .claude/                      # Claude Code configuration
│   ├── CLAUDE.md                 # Orchestrator config (imports AGENTS.md)
│   ├── settings.json             # Hooks and settings
│   ├── agents/                   # 15 specialized subagent definitions
│   ├── hooks/                    # Event hooks (UserPromptSubmit, Stop)
│   ├── skills/                   # Skill definitions
│   └── rules/                    # 20 enforcement rules
│
├── src-tauri/                    # Rust backend (Tauri v2)
│   ├── src/
│   │   ├── main.rs               # Entry point
│   │   ├── lib.rs                # Library root
│   │   ├── commands/             # Tauri IPC command handlers
│   │   ├── domain/               # Domain models (sessions, artifacts, governance)
│   │   ├── persistence/          # SQLite repositories
│   │   ├── claude/               # Claude API/Agent SDK integration
│   │   └── tools/                # File system tools (Read, Write, Edit, Glob, Grep)
│   ├── Cargo.toml
│   └── tauri.conf.json
│
├── src/                          # Svelte 5 frontend
│   ├── routes/                   # File-based routing
│   ├── lib/
│   │   ├── components/           # UI components
│   │   │   ├── ui/               # shadcn-svelte primitives
│   │   │   ├── shared/           # Shared components (EmptyState, LoadingSpinner, etc.)
│   │   │   ├── conversation/     # Chat/conversation UI
│   │   │   ├── artifacts/        # Process artifact panels
│   │   │   └── dashboard/        # Scanner/metrics dashboards
│   │   ├── stores/               # Svelte stores (.svelte.ts)
│   │   ├── types/                # TypeScript type definitions
│   │   └── api/                  # Tauri IPC client wrappers
│   └── app.html
│
├── docs/                         # Docsify documentation
│   ├── product/                  # Vision, pillars, governance, roadmap
│   ├── architecture/             # Decisions, IPC design, module structure
│   ├── ui/                       # UX specs, component library
│   ├── development/              # Coding standards, setup, lessons
│   ├── research/                 # Tech stack decisions (Phase 0)
│   └── process/                  # Governance framework
│
├── tests/                        # E2E tests (Playwright)
└── TODO.md                       # Current priorities
```

## Architecture Overview

Forge uses a **thick backend** architecture. Rust owns the domain model — sessions, artifacts, governance state, and agent orchestration. Svelte is a view layer that renders what Rust tells it.

```text
┌─────────────────────────────────────────────────────────┐
│                     Svelte 5 Frontend                    │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌─────────┐ │
│  │Conversat.│  │ Artifact │  │ Scanner  │  │ Metrics │ │
│  │  Panel   │  │  Panel   │  │Dashboard │  │  Panel  │ │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬────┘ │
│       │              │              │              │      │
│  ┌────┴──────────────┴──────────────┴──────────────┴────┐ │
│  │              Svelte Stores (.svelte.ts)               │ │
│  └──────────────────────┬───────────────────────────────┘ │
└─────────────────────────┼─────────────────────────────────┘
                          │ Tauri IPC (invoke / listen)
┌─────────────────────────┼─────────────────────────────────┐
│                     Rust Backend                          │
│  ┌──────────────────────┴───────────────────────────────┐ │
│  │              Tauri Commands (IPC handlers)            │ │
│  └───────┬──────────────┬──────────────┬────────────────┘ │
│          │              │              │                   │
│  ┌───────▼──────┐ ┌─────▼──────┐ ┌────▼───────┐          │
│  │ Domain Logic │ │  Claude    │ │   File     │          │
│  │ (Sessions,   │ │  API/SDK   │ │   System   │          │
│  │  Artifacts,  │ │ (Streaming,│ │  Tools     │          │
│  │  Governance) │ │  Tool Use) │ │            │          │
│  └───────┬──────┘ └────────────┘ └────────────┘          │
│          │                                                │
│  ┌───────▼──────┐                                         │
│  │   SQLite     │                                         │
│  │ (Persistence)│                                         │
│  └──────────────┘                                         │
└───────────────────────────────────────────────────────────┘
```

## Development

### Rust Backend (`src-tauri/`)

```bash
cargo build                                          # Build
cargo test                                           # Run tests
cargo clippy --all-targets -- -D warnings            # Lint (pedantic)
cargo fmt --check                                    # Format check
```

### Frontend (`src/`)

```bash
npm install                # Install dependencies
npm run dev                # Dev server (use cargo tauri dev instead)
npm run build              # Production build
npm run check              # svelte-check + TypeScript
npm run lint               # ESLint
npm run test               # Vitest unit tests
npm run test:e2e           # Playwright E2E tests
```

### Tauri

```bash
cargo tauri dev            # Dev mode (Rust + Svelte hot reload)
cargo tauri build          # Production build (.msi / .dmg / .AppImage)
```

## Naming Conventions

### Rust (Backend)

| Element | Convention | Example |
|---------|------------|---------|
| File names | `snake_case.rs` | `session_manager.rs` |
| Modules | `snake_case/` | `commands/` |
| Structs/Enums | `PascalCase` | `SessionState` |
| Functions | `snake_case()` | `create_session()` |
| Constants | `SCREAMING_SNAKE_CASE` | `MAX_SESSIONS` |
| Tauri commands | `snake_case` | `#[tauri::command] fn list_sessions()` |

### TypeScript/Svelte (Frontend)

| Element | Convention | Example |
|---------|------------|---------|
| Component files | `PascalCase.svelte` | `ConversationPanel.svelte` |
| Store files | `camelCase.svelte.ts` | `session.svelte.ts` |
| Utility files | `camelCase.ts` | `ipcClient.ts` |
| Variables | `camelCase` with runes | `let count = $state(0)` |
| Types/Interfaces | `PascalCase` | `interface Session {}` |
| Constants | `SCREAMING_SNAKE_CASE` | `const MAX_TOKENS = 4096` |

## Development Standards

From `docs/development/coding-standards.md`:

1. **Functional style** — Pure functions, immutable types, composition
2. **Type safety** — Zero clippy warnings (pedantic), strict TypeScript, no `any`
3. **Error handling** — `thiserror` for Rust errors, `Result<T, E>` everywhere, no `unwrap()` in production
4. **Test-driven** — Tests FIRST, 80%+ coverage
5. **Code quality** — Zero warnings, no TODO comments, no stubs

## Testing

- **Rust unit tests:** `src-tauri/src/**/mod.rs` — `#[cfg(test)] mod tests`
- **Rust integration tests:** `src-tauri/tests/` — Cross-module flows
- **Frontend unit tests:** `src/**/*.test.ts` — Vitest component/store tests
- **Frontend E2E tests:** `tests/` — Playwright browser tests
- **Coverage target:** 80% minimum

## Git Worktree Workflow (MANDATORY)

Every task uses a git worktree. Upon completion: merge to main, delete branch, remove worktree directory, verify builds pass.

```bash
# Create worktree
git worktree add ../forge-<task> -b <agent>/<task>

# After task completion
cd ../forge && git merge <branch>
git branch -d <branch> && git worktree remove ../forge-<task>

# Verify builds
cargo build --manifest-path src-tauri/Cargo.toml && npm run build
```

**No stale worktrees.** After every task batch, verify no orphaned worktrees remain.

## Skills

Skills are domain-specific instruction sets stored in `.claude/skills/` following the open [Agent Skills](https://agentskills.io) standard.

```bash
npx skills find [query]      # Search the ecosystem
npx skills add <source> -y   # Install a skill
npx skills list              # List installed skills
```

| Skill | Purpose |
|-------|---------|
| `svelte` | Svelte 5 components, runes, reactivity |
| `typescript` | Strict TypeScript patterns |
| `tailwind` | Tailwind CSS utilities |
| `chunkhound` | Semantic code search (mandatory) |
| `planning` | Planning methodology |
| `skills-maintenance` | Skill lifecycle management |
| `architecture` | Architectural compliance |

## MCP Tools

### ChunkHound (Code Search)

| Tool | Purpose | When to Use |
|------|---------|-------------|
| `search_regex` | Exact pattern matching | Finding specific symbols, imports, function names |
| `search_semantic` | Meaning-based search | Understanding what code does a thing |
| `code_research` | Architectural analysis with LLM | Answering "how does X work?" questions |

## Resources

| Resource | Location |
|----------|----------|
| **Current Tasks** | `TODO.md` |
| **Cross-Agent Instructions** | `AGENTS.md` (this file) |
| **Claude Config** | `.claude/CLAUDE.md` |
| **Agent Definitions** | `.claude/agents/*.md` |
| **Rules** | `.claude/rules/*.md` |
| **Skills** | `.claude/skills/*/SKILL.md` |
| **Coding Standards** | `docs/development/coding-standards.md` |
| **Architecture Decisions** | `docs/architecture/decisions.md` |
| **UI Specs** | `docs/ui/` |
