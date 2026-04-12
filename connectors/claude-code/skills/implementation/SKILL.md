# Thinking Mode: Implementation

You are now in Implementation Mode. The user wants something built, added, fixed, created, or refactored. The result is always a concrete deliverable: code, an artifact, a configuration change, or a migration.

**No stubs, no placeholders, no deferred deliverables.** The agent does real work.

## Workflow

1. **Understand the task** — read the task artifact and its acceptance criteria. If there's a plan document, read that too.
2. **Load domain knowledge** — the orchestrator injects knowledge appropriate to the implementation domain (see table below)
3. **Implement** — write the code, create the artifact, make the configuration change
4. **Verify completeness** — check the four-layer completeness rule for IPC features
5. **Self-check against acceptance criteria** — every criterion must be met before declaring done

## Domain Knowledge Injection

| Domain | Injected Knowledge |
|--------|-------------------|
| Svelte/frontend | `svelte5-best-practices`, `orqa-frontend-best-practices` |
| Rust/backend | `rust-async-patterns`, `orqa-backend-best-practices` |
| IPC boundary | `orqa-ipc-patterns`, `orqa-error-composition` |
| Stores | `orqa-store-patterns`, `orqa-store-orchestration` |

## Quality Criteria

- Every acceptance criterion from the task artifact is met
- Four-layer completeness (RULE-010) for IPC features: Rust command + IPC types + Svelte component + store binding, all committed together
- No stubs or placeholder implementations (RULE-020)
- Coding standards followed (RULE-006)
- All new code has appropriate error handling

## What Happens Next

Implementation never self-certifies. After the implementer finishes, the orchestrator routes to **Review Mode** for a separate reviewer to verify quality. A FAIL verdict routes back here for fixes.

## Governance

- RULE-006: coding standards apply to all implementation work
- RULE-010: end-to-end completeness for IPC boundary features
- RULE-020: no stubs — real implementations only
