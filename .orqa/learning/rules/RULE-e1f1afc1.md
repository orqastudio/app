---
id: "RULE-e1f1afc1"
type: rule
title: "Automated Knowledge Injection"
description: "When agents touch specific code areas, relevant domain knowledge is auto-injected. Enforcement entries map file paths to knowledge artifact names."
status: active
enforcement_type: mechanical
created: "2026-03-11"
updated: "2026-03-28"
enforcement:

  - mechanism: behavioral

    message: "When agents touch specific code areas, relevant domain knowledge must be auto-injected; knowledge injection mappings must be kept up to date"

  - mechanism: hook

    type: PostToolUse
    event: file
    action: inject
    pattern: "app/src-tauri/src/domain/**"
    knowledge: "orqa-domain-services, orqa-error-composition"

  - mechanism: hook

    type: PostToolUse
    event: file
    action: inject
    pattern: "app/src-tauri/src/commands/**"
    knowledge: "orqa-ipc-patterns, orqa-error-composition"

  - mechanism: hook

    type: PostToolUse
    event: file
    action: inject
    pattern: "app/src-tauri/src/repo/**"
    knowledge: "orqa-repository-pattern"

  - mechanism: hook

    type: PostToolUse
    event: file
    action: inject
    pattern: "app/src/lib/components/**"
    knowledge: "component-extraction, svelte5-best-practices"

  - mechanism: hook

    type: PostToolUse
    event: file
    action: inject
    pattern: ".orqa/**"
    knowledge: "orqa-governance, orqa-documentation"
relationships:

  - target: "PD-e8a0f910"

    type: "enforces"
    rationale: "Auto-generated inverse of practiced-by relationship from PD-e8a0f910"

  - target: "PD-c6c2d9fb"

    type: "enforces"

  - target: "DOC-e6fb92b0"

    type: "documents"
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
| --- | --- | --- |
| `app/src-tauri/src/domain/**` | `orqa-domain-services`, `orqa-error-composition` | Domain logic needs service anatomy and error flow |
| `app/src-tauri/src/commands/**` | `orqa-ipc-patterns`, `orqa-error-composition` | IPC boundary needs contract discipline |
| `app/src-tauri/src/repo/**` | `orqa-repository-pattern` | Data access has specific patterns |
| `app/src/lib/components/**` | `component-extraction`, `svelte5-best-practices` | Components need purity discipline |
| `.orqa/**` | `orqa-governance`, `orqa-documentation` | Artifacts need structural consistency |

## Deduplication

Knowledge artifacts are injected once per session. If an agent writes to `app/src-tauri/src/domain/foo.rs`
and then `app/src-tauri/src/domain/bar.rs`, the domain knowledge is only injected on the first
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

- [RULE-dd5b69e6](RULE-dd5b69e6) (knowledge-enforcement) — knowledge loading model and tier system
- [RULE-9814ec3c](RULE-9814ec3c) (coding-standards) — the standards that injected knowledge helps enforce
- [RULE-42d17086](RULE-42d17086) (tooling-ecosystem) — linter delegation complements knowledge injection
