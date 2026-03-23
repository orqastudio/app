---
id: KNOW-a4b5c6d7
type: knowledge
title: Project Type Detection
description: |
  Methodology for detecting project type, language stack, frameworks, build
  tools, and existing governance from file system signals. Produces a structured
  project profile used to drive setup and skill-selection decisions.
  Use when: Onboarding a new project, selecting appropriate rules and skills,
  or detecting project characteristics for governance initialisation.
status: active
created: 2026-03-22
updated: 2026-03-22
category: methodology
version: 1.0.0
onboarding: true
user-invocable: true
---

Methodology for inferring a project's characteristics from its file structure and configuration files. The output is a structured project profile that drives skill selection, rule loading, and setup decisions. This process is always read-only — no project files are modified during detection.

## Detection Categories

### 1. Languages

| Signal | Language |
|--------|----------|
| `*.rs`, `Cargo.toml` | Rust |
| `*.ts`, `*.tsx`, `tsconfig.json` | TypeScript |
| `*.js`, `*.jsx`, `package.json` | JavaScript |
| `*.py`, `pyproject.toml`, `setup.py` | Python |
| `*.go`, `go.mod` | Go |
| `*.java`, `pom.xml`, `build.gradle` | Java |
| `*.cs`, `*.csproj` | C# |
| `*.swift`, `Package.swift` | Swift |

### 2. Frameworks

| Signal | Framework |
|--------|-----------|
| `svelte.config.js`, `*.svelte` | Svelte |
| `next.config.*`, `app/layout.tsx` | Next.js |
| `nuxt.config.*` | Nuxt |
| `angular.json` | Angular |
| `tauri.conf.json` | Tauri |
| `electron-builder.*`, `electron/` | Electron |
| `Cargo.toml` with `actix-web`, `axum`, or `rocket` dependency | Rust web framework |
| `django/`, `manage.py` | Django |
| `Gemfile` with `rails` | Ruby on Rails |

### 3. Build Tools

| Signal | Tool |
|--------|------|
| `Makefile` | Make |
| `package.json` with `scripts` | npm / yarn / bun scripts |
| `Cargo.toml` | Cargo |
| `Dockerfile`, `docker-compose.yml` | Docker |
| `.github/workflows/` | GitHub Actions |
| `Jenkinsfile` | Jenkins |

### 4. Existing Governance and Tooling

| Signal | What It Means |
|--------|---------------|
| `AGENTS.md` | Cross-agent instructions exist |
| `CONVENTIONS.md`, `CONTRIBUTING.md` | Project conventions documented |
| `.editorconfig` | Editor configuration exists |
| `.pre-commit-config.yaml` | Pre-commit hooks exist |
| Governance directory (e.g., `.orqa/`, `.rules/`) | Structured governance already initialised |
| AI tool configuration files | Another AI assistant is already configured |

When existing governance is detected, migration methodology applies before adding new governance layers. See `governance-migration-methodology`.

### 5. Project Type Signals

| Signals | Likely Type |
|---------|-------------|
| Backend source directory + frontend source directory + both `Cargo.toml` and `package.json` | Desktop app or full-stack app |
| `package.json` + framework config + no backend | Frontend web app |
| `Cargo.toml` + no frontend | Rust library or service |
| `package.json` + `server/` or `api/` directory | Full-stack web app |
| No code files, mostly `.md` | Documentation or knowledge project |
| Mixed languages, no clear structure | Monorepo or multi-project |

## Project Profile Output

Detection produces a structured profile:

```yaml
project:
  name: "my-project"
  type: "desktop-app"          # web-app | library | service | documentation | monorepo
  languages: [rust, typescript]
  frameworks: [tauri, svelte]
  build_tools: [make, cargo, npm]
  existing_governance:
    structured: false          # true if a governance directory was found
    ai_tools: []               # list of detected AI tool configurations
    conventions_documented: true
    pre_commit: true
  detected_patterns:
    has_tests: true
    has_ci: true
    has_linting: true
  recommendations:
    skills_to_load: []         # populated by the setup skill based on profile
    migration_needed: false    # true if existing governance was found
```

## Detection Procedure

1. Scan the root directory for configuration files
2. Scan the first two levels of subdirectories for language and framework signals
3. Check for existing governance configurations
4. Classify the project type based on combined signals
5. Generate the project profile with confidence levels
6. Recommend appropriate skills and flag any migration needs

## Confidence Levels

Report a confidence level for the project type classification:

| Level | Basis |
|-------|-------|
| **High** | Multiple independent signals confirm the same type |
| **Medium** | Some signals present, minor ambiguity |
| **Low** | Single signal, or signals conflict |

When signals conflict, report all possibilities rather than guessing. Do not pick one silently.

## Critical Rules

- NEVER modify any project files during detection — this is strictly read-only
- NEVER assume a project type from a single signal — require multiple confirming signals for high confidence
- Always check for existing governance before recommending fresh initialisation
- If signals conflict, surface the conflict rather than resolving it arbitrarily
- Report "no governance found" and "governance found but unrecognised format" as distinct states