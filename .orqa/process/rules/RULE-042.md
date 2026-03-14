---
id: RULE-042
title: Automated Skill Injection
description: When agents touch specific code areas, relevant domain skills are auto-injected. Enforcement entries map file paths to skill names.
status: active
created: 2026-03-11
updated: 2026-03-12
layer: project
relationships:
  - type: observed-by
    target: IMPL-054
    rationale: IMPL-054 documents a case where the orchestrator bypassed this system in favor of raw platform hooks
  - target: PILLAR-001
    type: grounded
    rationale: Skill injection automates knowledge structure loading at the right moments
  - target: RULE-026
    type: informs
    rationale: Automated injection implements the skill loading model by triggering Tier 2 skills based on file paths
  - target: RULE-006
    type: informs
    rationale: Injected skills help agents comply with coding standards specific to the code area they are editing
  - target: RULE-043
    type: informs
    rationale: Skill injection complements linter delegation — skills provide knowledge while linters enforce patterns
  - type: informed-by
    target: RULE-043
    rationale: Linter delegation defines which enforcement belongs in tooling vs skill injection, preventing duplication
  - target: TASK-412
    type: enforced-by
    rationale: Auto-generated inverse of enforced-by relationship from TASK-412
  - target: AD-045
    type: practiced-by
    rationale: Auto-generated inverse of practiced-by relationship from AD-045
  - target: AD-048
    type: enforced-by
    rationale: Auto-generated inverse of enforced-by relationship from AD-048
  - target: RES-056
    type: observed-by
    rationale: Auto-generated inverse of observed-by relationship from RES-056
  - type: scoped-to
    target: AGENT-003
    rationale: Migrated from scope field
enforcement:
  - event: file
    paths:
      - backend/src-tauri/src/domain/**
    action: inject
    skills:
      - orqa-domain-services
      - orqa-error-composition
    message: Injecting domain service and error composition patterns.
  - event: file
    paths:
      - backend/src-tauri/src/commands/**
    action: inject
    skills:
      - orqa-ipc-patterns
      - orqa-error-composition
    message: Injecting IPC patterns and error composition.
  - event: file
    paths:
      - backend/src-tauri/src/repo/**
    action: inject
    skills:
      - orqa-repository-pattern
    message: Injecting repository pattern.
  - event: file
    paths:
      - sidecar/src/**
    action: inject
    skills:
      - orqa-streaming
    message: Injecting streaming patterns.
  - event: file
    paths:
      - ui/src/lib/components/**
    action: inject
    skills:
      - component-extraction
      - svelte5-best-practices
    message: Injecting component extraction and Svelte 5 patterns.
  - event: file
    paths:
      - ui/src/lib/stores/**
    action: inject
    skills:
      - orqa-store-patterns
      - orqa-store-orchestration
    message: Injecting store patterns and orchestration.
  - event: file
    paths:
      - .orqa/**
    action: inject
    skills:
      - orqa-governance
      - orqa-documentation
    message: Injecting governance and documentation patterns.
---
When agents write to specific code areas, the enforcement engine automatically injects
relevant domain skills as system context. This implements Layer 2 (Knowledge Injection)
of the structured thinking enforcement system.

## How It Works

Enforcement entries with `action: inject` and a `skills` array are evaluated on every
Write/Edit tool call. When a file path matches, the specified skills are read from
`.orqa/process/skills/<name>/SKILL.md` and returned as `systemMessage` to inject into the
agent's context.

## Path-to-Skill Map

| File Path Pattern | Injected Skills | Why |
|------------------|-----------------|-----|
| `backend/src-tauri/src/domain/**` | `orqa-domain-services`, `orqa-error-composition` | Domain logic needs service anatomy and error flow |
| `backend/src-tauri/src/commands/**` | `orqa-ipc-patterns`, `orqa-error-composition` | IPC boundary needs contract discipline |
| `backend/src-tauri/src/repo/**` | `orqa-repository-pattern` | Data access has specific patterns |
| `sidecar/src/**` | `orqa-streaming` | Sidecar protocol is fragile |
| `ui/src/lib/components/**` | `component-extraction`, `svelte5-best-practices` | Components need purity discipline |
| `ui/src/lib/stores/**` | `orqa-store-patterns`, `orqa-store-orchestration` | Reactive state needs specific patterns |
| `.orqa/**` | `orqa-governance`, `orqa-documentation` | Artifacts need structural consistency |

## Deduplication

Skills are injected once per session. If an agent writes to `backend/src-tauri/src/domain/foo.rs`
and then `backend/src-tauri/src/domain/bar.rs`, the domain skills are only injected on the first
write. The enforcement engine tracks injected skills per session and skips re-injection.

## Adding New Injection Mappings

To add a new path-to-skill mapping:

1. Add an enforcement entry to this rule's frontmatter
2. Set `event: file`, `action: inject`
3. Set `paths` to the glob patterns
4. Set `skills` to the skill directory names
5. Set `message` to a brief description

Ensure the referenced skills exist in `.orqa/process/skills/`.

## FORBIDDEN

- Injection entries that block tool calls (inject is always non-blocking)
- Injection entries without a `skills` field
- Referencing skills that don't exist in `.orqa/process/skills/`

## Related Rules

- [RULE-026](RULE-026) (skill-enforcement) — skill loading model and tier system
- [RULE-006](RULE-006) (coding-standards) — the standards that injected skills help enforce
- [RULE-043](RULE-043) (tooling-ecosystem) — linter delegation complements skill injection
