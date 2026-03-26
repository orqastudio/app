---
id: KNOW-1f4aba8f
type: knowledge
title: Three-Layer Enforcement Model
description: |
summary: "|. OrqaStudio enforces governance through **three layers**, each handling different kinds of violations. Understanding which layer owns a check prevents duplication, ensures the right response time, and enables safe demotion of behavioral rules as mechanical enforcement improves."
  How OrqaStudio enforces governance through three layers: LSP real-time diagnostics,
  behavioral rules for judgement calls, and pre-commit hard gates. What each layer
  handles, when rules can be demoted from behavioral to mechanical enforcement. Use
  when: designing enforcement for a new rule, deciding where a check belongs, or
  demoting behavioral rules after LSP covers them.
status: active
created: 2026-03-24
updated: 2026-03-24
category: methodology
version: 1.0.0
user-invocable: false
relationships:
  - target: DOC-1f4aba8f
    type: synchronised-with
    rationale: "User-facing documentation pair for this agent-facing knowledge artifact"
  - target: KNOW-a16b7bc7
    type: complements
    rationale: "Stability tracking handles the lifecycle after a rule is demoted from this model"
---

## Purpose

OrqaStudio enforces governance through **three layers**, each handling different kinds of violations. Understanding which layer owns a check prevents duplication, ensures the right response time, and enables safe demotion of behavioral rules as mechanical enforcement improves.

---

## The Three Layers

```
Layer 1: LSP (real-time)          ← Fastest. Red squiggles as you type.
Layer 2: Behavioral rules         ← Judgement. Agent prompt injection.
Layer 3: Pre-commit (hard gate)   ← Final. Blocks bad commits.
```

### Layer 1: LSP Real-Time Diagnostics

| What It Catches | How | Response |
|----------------|-----|----------|
| Invalid status values | Schema enum validation | Red squiggle + suggested fix |
| Wrong relationship types | Plugin schema lookup | Red squiggle + valid options |
| Missing required fields | JSON Schema required check | Warning squiggle |
| Broken artifact references | Graph lookup | Error squiggle |
| Type mismatches | JSON Schema type check | Error squiggle |

**Characteristics:** Mechanical, deterministic, instant feedback. No judgement required. The LSP validates what the schema defines — nothing more, nothing less.

### Layer 2: Behavioral Rules (Prompt Injection)

| What It Catches | How | Response |
|----------------|-----|----------|
| Documentation-before-code violations | Orchestrator delegation discipline | Agent refuses to proceed |
| Delegation boundary violations | Role constraint injection | Agent stays in lane |
| Scope creep (pillar misalignment) | Pillar gate questions in context | Agent flags to user |
| Process sequencing violations | Workflow rules in context | Agent follows the process |
| Honest reporting lapses | Output structure requirements | Agent includes required sections |

**Characteristics:** Requires judgement. Cannot be reduced to a schema check. The rule must be in the agent's context for the agent to follow it. Enforced via prompt injection at delegation time.

### Layer 3: Pre-Commit Hard Gate

| What It Catches | How | Response |
|----------------|-----|----------|
| All Layer 1 checks (redundant safety net) | Shared validation engine | Commit blocked |
| Lint failures (Rust clippy, ESLint) | Linter execution | Commit blocked |
| Type check failures (svelte-check, tsc) | Compiler execution | Commit blocked |
| Test failures | Test runner execution | Commit blocked |
| Artifact schema violations | Schema validation | Commit blocked |

**Characteristics:** Hard gate. Nothing passes without passing all checks. The pre-commit hook calls `orqa check` which runs the shared validation engine plus language-specific linters and tests.

---

## Rule Demotion Model

When a behavioral rule (Layer 2) becomes mechanically enforceable (Layer 1 or 3), it can be **demoted**:

```
Behavioral rule active
        │
  LSP/validator now catches it?
        │
        YES → Demote rule to inactive
               │
               Stability tracking begins (KNOW-a16b7bc7)
               │
               N clean sessions → Safe to delete
```

### What CAN Be Demoted

- Rules about valid statuses → LSP schema validation covers them
- Rules about valid relationship types → LSP schema validation covers them
- Rules about required fields → LSP schema validation covers them
- Rules about artifact ID format → Pre-commit schema validation covers them

### What CANNOT Be Demoted

- Rules requiring judgement (pillar alignment, documentation-first, honest reporting)
- Rules about process sequencing (structure-before-work, enforcement-before-code)
- Rules about delegation boundaries (who may do what)
- Rules about communication style (never ask to stop, terse responses)

---

## Agent Actions

| Situation | Action |
|-----------|--------|
| Designing enforcement for a new rule | Determine which layer: mechanical check → LSP/pre-commit; judgement → behavioral |
| LSP now covers a behavioral rule | Demote the rule per KNOW-a16b7bc7 lifecycle |
| Pre-commit fails on commit | Fix the violation. Read the error. Never `--no-verify`. |
| LSP shows a diagnostic | Fix immediately before proceeding. |
| Behavioral rule violation discovered in review | Log as a lesson. If recurring, strengthen enforcement. |

---

## FORBIDDEN

- Duplicating a mechanical check as a behavioral rule when LSP already catches it
- Demoting a judgement-based rule to inactive (judgement rules cannot be mechanically enforced)
- Bypassing pre-commit checks for any reason
- Ignoring LSP diagnostics ("I'll fix it later")
