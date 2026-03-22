---
id: DOC-a9a6ef19
title: "Thinking Mode: Debugging"
description: "Something is broken or behaving unexpectedly — reproduce, isolate, identify root cause, then route to implementation or learning loop."
category: platform
status: active
created: 2026-03-21
updated: 2026-03-21
relationships:
  - target: KNOW-323c2803
    type: synchronised-with
---

## What This Mode Is

Debugging Mode is active when something is broken or behaving unexpectedly. The agent's goal is diagnosis: reproduce the failure, isolate the layer where it occurs, and identify the root cause. Debugging Mode ends when the root cause is known — it does not include the fix.

This separation is important. An agent that jumps from "something is broken" to "let me change code" without first isolating the cause often fixes the wrong thing, or fixes a symptom while leaving the root cause intact. Diagnosis first, then route to the appropriate mode.

---

## When It Activates

The orchestrator routes here when the user reports a concrete failure — not a question about the system, but a specific thing that is not working.

Typical signals:
- "this is broken"
- "why is X showing wrong numbers"
- "it's not working"
- "there's an error when I do Y"
- "the dashboard doesn't match the data"
- "something changed and now it fails"
- "the command returns unexpected results"

---

## What the Agent Needs

Debugging uses the diagnostic methodology (`diagnostic-methodology` knowledge):

1. **Reproduce** — establish a reliable way to trigger the failure. If it cannot be reproduced, it cannot be debugged.
2. **Isolate** — narrow the failing layer. For OrqaStudio: is it in the Svelte component, the store, the IPC call, or the Rust command?
3. **Identify** — find the specific line, function, or data state causing the failure.
4. **Classify** — is this a code bug (→ Implementation Mode) or a governance gap (→ Learning Loop Mode)?

Search tools are essential during isolation: `search_regex` to find the exact function, `search_semantic` to find similar patterns that may be relevant.

---

## How It Connects to the Thinking Framework

Debugging Mode routes to other modes once diagnosis is complete:

- **Root cause is a code bug** → **Implementation Mode** to fix it
- **Root cause is a governance gap** (missing rule, unenforced standard) → **Learning Loop Mode** to capture it
- **Root cause is unclear** → **Research Mode** for deeper investigation before attempting a fix

Debugging never directly produces code changes — it produces a diagnosis that the appropriate downstream mode acts on.

---

## Governance

- `diagnostic-methodology` knowledge artifact defines the full reproduction and isolation protocol
- RULE-005 (search over grep): use semantic search to find relevant code before reading files manually
- Findings from debugging that reveal systemic gaps are CRITICAL in dogfood mode (RULE-009)
