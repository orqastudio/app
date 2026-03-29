---
id: KNOW-1f4aba8f
type: knowledge
status: active
title: "Three-Layer Enforcement Model"
description: "Three enforcement layers: LSP real-time diagnostics (Layer 1), behavioral rules via prompt injection (Layer 2), and pre-commit hard gate (Layer 3) — with the rule demotion model"
tier: always
created: 2026-03-29
roles: [orchestrator, implementer, reviewer, governance-steward]
paths: [engine/enforcement/, .orqa/learning/rules/]
tags: [enforcement, lsp, pre-commit, behavioral-rules, demotion]
relationships:
  - type: synchronised-with
    target: DOC-1f4aba8f
---

# Three-Layer Enforcement Model

## Overview

OrqaStudio enforces governance through three layers. Each targets a different class of violation. The layers work together: mechanical checks catch deterministic errors instantly, behavioral rules guide agent judgement, and pre-commit gates ensure nothing slips through.

## Layer 1: LSP Real-Time Diagnostics

Fastest layer. Validates artifact files in real-time in the editor.

**What it catches:**

| Violation | Response |
| ----------- | --------- |
| Invalid status value | Red squiggle + valid values |
| Wrong relationship type | Red squiggle + valid type list |
| Missing required field | Warning squiggle |
| Broken artifact reference | Error squiggle |
| Type mismatch in frontmatter | Error squiggle |

**Characteristics:** Mechanical and deterministic. Driven by plugin schemas — no hardcoded rules. Zero-latency feedback.

## Layer 2: Behavioral Rules (Prompt Injection)

The judgement layer. Some constraints cannot be reduced to a schema check — they require understanding context, intent, and trade-offs. These are enforced by injecting rules into agent context at delegation time.

**What it covers:**

| Constraint | Enforcement |
| ----------- | ------------ |
| Documentation-before-code | Orchestrator includes rule in delegation prompt |
| Delegation boundary crossing | Role constraints in agent system prompt |
| Pillar misalignment | Pillar gate questions in context |
| Process sequencing | Workflow rules in context |
| Incomplete reporting | Output structure requirements in context |

**Characteristics:** Requires judgement. Context-dependent. Cannot be replaced by mechanical checks.

## Layer 3: Pre-Commit Hard Gate

Final safety net. Nothing enters the repository without passing all checks.

| Check | Tool |
| ------- | ------ |
| Artifact schema validation | Shared validation engine (same as LSP) |
| Rust lint violations | `cargo clippy` with pedantic warnings |
| TypeScript/Svelte type errors | `svelte-check`, ESLint |
| Test failures | `cargo test`, Vitest |
| Formatting violations | `cargo fmt --check` |

**Characteristics:** Hard gate — commit blocked on failure. Redundant with LSP for schema checks (defense in depth).

## How the Layers Work Together

- **Layer 1 catches most mechanical errors** before they are ever committed
- **Layer 2 catches process violations** that require judgement
- **Layer 3 catches anything that escaped** Layers 1 and 2

## Rule Demotion Model

When a behavioral rule (Layer 2) becomes mechanically enforceable, it can be **demoted** to inactive. This prevents duplication — a mechanical check is always stronger than a behavioral reminder.

**Demotion flow:**

1. Add mechanical enforcement (Layer 1 or 3) that covers the rule's constraint
2. Set rule to `status: inactive`, add demotion metadata: `demoted_date`, `demoted_reason`, `replaced_by`, `stability_threshold` (default 10), `stability_count` (auto-managed)
3. Stability tracking begins: each clean session increments counter; any violation resets it
4. When counter reaches threshold: rule surfaces as safe to delete

**What CAN be demoted:** Valid status values, valid relationship types, required fields, ID format, file naming conventions — anything the schema and validation engine can check.

**What CANNOT be demoted:** Pillar alignment, documentation-before-code, honest reporting, delegation boundaries, process sequencing — these require judgement and will always need behavioral enforcement.
