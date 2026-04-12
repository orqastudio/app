---
id: "EPIC-3a3e5aea"
type: epic
title: "Schema-driven LSP enforcement for artifact intelligence"
description: "Implement full schema-driven artifact intelligence in the orqa LSP: real-time validation, autocomplete, hover, go-to-definition, code actions, broken link detection, reverse relationship checks, and plugin schema ingestion. All powered by plugin schemas — nothing hardcoded."
status: captured
priority: "P1"
created: 2026-03-24T00:00:00.000Z
updated: 2026-03-24T00:00:00.000Z
scoring:
  impact: 5
  urgency: 5
  complexity: 4
  dependencies: 2
relationships:
  - target: "MS-b1ac0a20"
    type: "fulfils"
    rationale: "LSP enforcement is needed for dogfooding"
  - target: "PILLAR-c9e0a695"
    type: "grounded"
    rationale: "LSP enforcement makes governance artifacts visible and manageable in the editor — mechanical enforcement of structural rules"
  - target: "PD-a44384d1"
    type: "implements"
    rationale: "CLI as single interface — LSP runs as a CLI protocol mode, powered by the validation daemon"
---

# Schema-Driven LSP Enforcement for Artifact Intelligence

## Context

The orqa LSP server (`libs/lsp-server/`) currently provides basic diagnostics: frontmatter structure checks (missing delimiters, duplicate keys, ID format) and graph-level checks delegated to the validation daemon (`libs/validation/`). It does **not** provide the rich editor intelligence expected from a modern language server — no autocomplete, no hover, no go-to-definition, no code actions.

This epic transforms the LSP from a diagnostic-only server into a full artifact intelligence engine. Every capability is schema-driven: plugin schemas (`orqa-plugin.json`) define artifact types, valid statuses, relationship vocabulary, and frontmatter schemas. The LSP reads these at startup and re-reads when plugins change. Nothing is hardcoded.

## Why P1

This is the foundation for the LSP-first enforcement model. Once the LSP provides real-time validation, autocomplete, hover, and code actions, most behavioral rules and hooks can be demoted or removed — the editor catches violations before they reach the commit stage. Without this, governance enforcement remains dependent on AI behavioral rules (fragile) and pre-commit hooks (late feedback).

Serves [PILLAR-c9e0a695](PILLAR-c9e0a695) gate question: "Does the system mechanically enforce its own structural rules?"

## Current State

| Capability | Status |
| ----------- | -------- |
| Frontmatter structure checks | Done — `validation.rs` |
| JSON Schema validation | Done — delegates to `orqa_validation::checks::schema` |
| Broken link detection (local) | Done — `check_relationship_targets` |
| Broken link detection (daemon) | Done — daemon `/validate` returns BrokenLink checks |
| Missing inverse detection | Done — daemon `/validate` returns MissingInverse checks |
| Plugin schema ingestion | Partial — daemon loads plugin manifests, LSP does not pass types to `validate_file` |
| Autocomplete | Not started |
| Hover | Not started |
| Go to definition | Not started |
| Code actions | Not started |

## Implementation Design

### Architecture

The LSP server (`libs/lsp-server/`) delegates heavy graph work to the validation daemon (`libs/validation/`) via HTTP. The pattern:

- **Text-level checks** (fast, buffer-local): run in the LSP process on every keystroke
- **Graph-level checks** (need full graph): delegated to daemon via `POST /validate`
- **Navigation features** (autocomplete, hover, go-to-def): need the daemon's graph index

The daemon already holds `ArtifactGraph` + `PluginContributions` in memory. New daemon endpoints expose the data the LSP needs for navigation features.

### Plugin Schema Flow

```text
Plugin orqa-plugin.json
  → daemon scan_plugin_manifests() [startup + POST /reload]
  → daemon DaemonState.plugin_contributions
  → daemon exposes artifact_types via new GET /schema endpoint
  → LSP fetches on initialize + on plugin manifest save
  → LSP passes artifact_types to validate_file()
```

### New Daemon Endpoints

| Endpoint | Purpose |
| ---------- | --------- |
| `GET /schema` | Return all artifact type definitions (for LSP to use in validation + autocomplete) |
| `GET /artifacts` | Return all artifact IDs with title, type, status, file path (for autocomplete + hover) |
| `GET /artifact/:id` | Return full metadata for a single artifact (for hover) |

### Capability Registration

The LSP `initialize` response must declare capabilities:

```rust
ServerCapabilities {
    text_document_sync: Full,
    completion_provider: Some(CompletionOptions { trigger_characters: ["-", ":"], .. }),
    hover_provider: Some(true),
    definition_provider: Some(true),
    code_action_provider: Some(CodeActionOptions { .. }),
}
```

## Tasks

| Task | Title | Status |
| ------ | ------- | -------- |
| [TASK-47225043](TASK-47225043) | Plugin schema ingestion in LSP | ready |
| [TASK-c9d9baab](TASK-c9d9baab) | Real-time schema validation diagnostics | ready |
| [TASK-ee73dc63](TASK-ee73dc63) | Broken link detection with line-level positioning | ready |
| [TASK-84df448e](TASK-84df448e) | Reverse relationship checks with code actions | ready |
| [TASK-d423f4f7](TASK-d423f4f7) | Autocomplete for artifact IDs and statuses | ready |
| [TASK-d0e94e95](TASK-d0e94e95) | Hover provider for artifact references | ready |
| [TASK-2d104e1f](TASK-2d104e1f) | Go to definition for artifact IDs | ready |
| [TASK-75b100dd](TASK-75b100dd) | Code actions for quick fixes | ready |
| [TASK-c733e628](TASK-c733e628) | Reconcile EPIC-3a3e5aea | ready |

## Out of Scope

- VS Code extension packaging (separate epic)
- Semantic token highlighting for frontmatter (nice-to-have, not enforcement)
- Rename refactoring across artifacts (complex, separate epic)
