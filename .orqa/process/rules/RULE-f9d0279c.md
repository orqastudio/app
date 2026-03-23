---
id: "RULE-f9d0279c"
title: "Automated Knowledge Injection"
description: "When agents touch specific code areas, relevant domain knowledge is auto-injected. Enforcement entries map file paths to knowledge artifact names."
status: "active"
created: "2026-03-11"
updated: "2026-03-12"
enforcement:
  - mechanism: behavioral
    message: "When agents touch specific code areas, relevant domain knowledge must be auto-injected; knowledge injection mappings must be kept up to date"
  - mechanism: hook
    type: PostToolUse
    event: file
    action: inject
    pattern: "backend/src-tauri/src/domain/**"
    knowledge: "orqa-domain-services, orqa-error-composition"
  - mechanism: hook
    type: PostToolUse
    event: file
    action: inject
    pattern: "backend/src-tauri/src/commands/**"
    knowledge: "orqa-ipc-patterns, orqa-error-composition"
  - mechanism: hook
    type: PostToolUse
    event: file
    action: inject
    pattern: "backend/src-tauri/src/repo/**"
    knowledge: "orqa-repository-pattern"
  - mechanism: hook
    type: PostToolUse
    event: file
    action: inject
    pattern: "sidecar/src/**"
    knowledge: "orqa-streaming"
  - mechanism: hook
    type: PostToolUse
    event: file
    action: inject
    pattern: "ui/src/lib/components/**"
    knowledge: "component-extraction, svelte5-best-practices"
  - mechanism: hook
    type: PostToolUse
    event: file
    action: inject
    pattern: "ui/src/lib/stores/**"
    knowledge: "orqa-store-patterns, orqa-store-orchestration"
  - mechanism: hook
    type: PostToolUse
    event: file
    action: inject
    pattern: ".orqa/**"
    knowledge: "orqa-governance, orqa-documentation"
relationships:
  - target: "AD-ea4a5979"
    type: "enforces"
    rationale: "Auto-generated inverse of practiced-by relationship from AD-ea4a5979"
  - target: "AD-f9034c99"
    type: "enforces"
  - target: "DOC-e0042602"
    type: "documented-by"
---
When agents write to specific code areas, the enforcement engine automatically injects
relevant domain knowledge as system context. This implements Layer 2 (Knowledge Injection)
of the structured thinking enforcement system.

## How It Works

Enforcement entries with `action: inject` and a `knowledge` array are evaluated on every
Write/Edit tool call. When a file path matches, the specified knowledge artifacts are read from
`.orqa/process/knowledge/<name>/KNOW.md` and returned as `systemMessage` to inject into the
agent's context.

## Path-to-Knowledge Map

| File Path Pattern | Injected Knowledge | Why |
|------------------|-----------------|-----|
| `backend/src-tauri/src/domain/**` | `orqa-domain-services`, `orqa-error-composition` | Domain logic needs service anatomy and error flow |
| `backend/src-tauri/src/commands/**` | `orqa-ipc-patterns`, `orqa-error-composition` | IPC boundary needs contract discipline |
| `backend/src-tauri/src/repo/**` | `orqa-repository-pattern` | Data access has specific patterns |
| `sidecar/src/**` | `orqa-streaming` | Sidecar protocol is fragile |
| `ui/src/lib/components/**` | `component-extraction`, `svelte5-best-practices` | Components need purity discipline |
| `ui/src/lib/stores/**` | `orqa-store-patterns`, `orqa-store-orchestration` | Reactive state needs specific patterns |
| `.orqa/**` | `orqa-governance`, `orqa-documentation` | Artifacts need structural consistency |

## Deduplication

Knowledge artifacts are injected once per session. If an agent writes to `backend/src-tauri/src/domain/foo.rs`
and then `backend/src-tauri/src/domain/bar.rs`, the domain knowledge is only injected on the first
write. The enforcement engine tracks injected knowledge per session and skips re-injection.

## Adding New Injection Mappings

To add a new path-to-knowledge mapping:

1. Add an enforcement entry to this rule's frontmatter
2. Set `event: file`, `action: inject`
3. Set `paths` to the glob patterns
4. Set `knowledge` to the knowledge artifact directory names
5. Set `message` to a brief description

Ensure the referenced knowledge artifacts exist in `.orqa/process/knowledge/`.

## FORBIDDEN

- Injection entries that block tool calls (inject is always non-blocking)
- Injection entries without a `knowledge` field
- Referencing knowledge artifacts that don't exist in `.orqa/process/knowledge/`

## Related Rules

- [RULE-deab6ea7](RULE-deab6ea7) (knowledge-enforcement) — knowledge loading model and tier system
- [RULE-b49142be](RULE-b49142be) (coding-standards) — the standards that injected knowledge helps enforce
- [RULE-7f416d7d](RULE-7f416d7d) (tooling-ecosystem) — linter delegation complements knowledge injection
