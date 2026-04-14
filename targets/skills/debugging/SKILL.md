# Thinking Mode: Debugging

You are now in Debugging Mode. Something is broken or behaving unexpectedly. Your goal is diagnosis: reproduce the failure, isolate the layer where it occurs, and identify the root cause.

**Debugging ends when the root cause is known — it does not include the fix.** An agent that jumps from "something is broken" to "let me change code" without isolating the cause often fixes the wrong thing or fixes a symptom while leaving the root cause intact.

## Workflow

1. **Reproduce** — establish a reliable way to trigger the failure. If it cannot be reproduced, it cannot be debugged.
2. **Isolate** — narrow the failing layer. For OrqaStudio: is it in the Svelte component, the store, the IPC call, or the Rust command?
3. **Identify** — find the specific line, function, or data state causing the failure.
4. **Classify** — is this a code bug (→ Implementation Mode) or a governance gap (→ Learning Loop Mode)?

## What You Have Access To

- `search_regex` — find the exact function where the failure occurs
- `search_semantic` — find similar patterns that may be relevant
- Shell access for running tests and reproducing failures
- Error logs, stack traces, and diagnostic output

## Quality Criteria

- The failure is reproduced reliably before any diagnosis is attempted
- The failing layer is isolated (frontend / store / IPC / backend / database)
- The root cause is identified at the specific line/function/state level
- The diagnosis is classified: code bug vs governance gap
- The diagnosis does NOT include a fix — only the root cause and its classification

## What Happens Next

Debugging routes to other modes once diagnosis is complete:
- **Code bug** → **Implementation Mode** to fix it
- **Governance gap** (missing rule, unenforced standard) → **Learning Loop Mode** to capture it
- **Root cause unclear** → **Research Mode** for deeper investigation

## Governance

- `diagnostic-methodology` knowledge artifact defines the full reproduction and isolation protocol
- RULE-005: use semantic search to find relevant code before reading files manually
- RULE-009: enforcement gaps discovered during development are immediately CRITICAL (dogfood mode)
