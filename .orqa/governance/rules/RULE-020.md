---
id: RULE-020
title: No Stubs or Placeholders
description: No hardcoded fake data, TODO functions, or scaffolded implementations in production code.
status: active
created: "2026-03-07"
updated: "2026-03-07"
layer: core
scope: [AGENT-002, AGENT-006]
enforcement:
  - event: file
    pattern: "TODO|FIXME|HACK|XXX"
    paths: ["src-tauri/src/**/*.rs", "ui/**/*.ts", "ui/**/*.svelte", "sidecar/src/**/*.ts"]
    action: warn
    message: "TODO/FIXME/HACK comments are forbidden in production code (RULE-020). Track in TODO.md instead."
---
## What Counts as a Stub

- Hardcoded return values pretending to be real data (e.g., `status: "connected"`, `latency: 42`)
- Default arrays/objects with fake data that should come from the backend (e.g., hardcoded scanner results)
- Functions that log "TODO" or do nothing (e.g., `save_session()` that only prints a message)
- `test_connection()` that always returns `Ok(())` without actually testing anything
- "No-op" event handlers that `console.log` instead of performing the action
- Async functions with TODO comments in their implementation bodies
- Rust functions that return `Ok(Default::default())` instead of doing real work
- Any function that claims to persist data but only modifies local/in-memory state without writing to SQLite or disk
- `#[tauri::command]` functions that return hardcoded data instead of computing real results
- TypeScript `invoke()` calls wrapped in try/catch that silently return fake fallback data on error

## Verification Before Commit

For EVERY new UI component or Tauri command:

1. Does the Svelte component call a real Tauri command via `invoke()`? If not, it's a stub
2. Does the Rust command perform real work and return real data? If it returns hardcoded defaults, it's a stub
3. Does the data displayed come from the Rust backend? If it uses hardcoded defaults as the primary source, it's a stub
4. Does error handling show real errors? If it always returns success, it's a stub

## When Backend Doesn't Exist Yet

If a Rust command doesn't exist, you MUST either:

- Create the Rust command FIRST, then wire the frontend
- Show an explicit "Not configured" / "Not available" state in the UI
- NEVER show fake success data to make it look like it works

## Automated Enforcement

A stub scanner should be part of the CI/quality checks. It scans all production source code for:

- Mock/placeholder/TODO/FIXME/HACK comments
- Hardcoded data standing in for real backend responses
- Scaffolded implementations that don't do real work

**If the scanner finds stubs, the build fails.** Legitimate exceptions (e.g., known incomplete features tracked in `.orqa/planning/tasks/`) can be added to an allowlist with a documented reason.

## ChunkHound Integration

Use `search_regex` for the command name (e.g. `"get_hardware_info"`) to instantly verify a Tauri command exists in both the Rust backend and the frontend invoke calls.

## Agent Completion Reports (MANDATORY)

Every agent completing implementation work MUST include these sections in its final output:

### Required Output Structure

```text
## What Was Done
[List of specific deliverables with file paths]

## What Is NOT Done
[Explicit list of anything incomplete, scaffolded, or not yet wired]
[If everything is genuinely complete, write: "Nothing — all deliverables are fully implemented and wired end-to-end."]

## Evidence
[Actual command output proving the work is real — not "tests pass" but the actual test output]
[Actual invocation results showing real responses — not "command works" but the response data]

## Smoke Test
[What happens if the user tries to USE this feature right now?]
[Did you actually try it? What was the result?]
```

**The "What Is NOT Done" section is NON-NEGOTIABLE.** Omitting it is treated as a review FAIL. An empty section with justification ("Nothing — all deliverables are fully implemented") is acceptable. A missing section is not.

**Why this exists:** Agents naturally emphasize what they accomplished and downplay gaps. This section forces explicit acknowledgment of limitations. The user reads this section FIRST to understand the true state of work.

## Related Rules

- [RULE-010](RULE-010) (end-to-end-completeness) — the full chain that must exist
- [RULE-012](RULE-012) (error-ownership) — if the command doesn't exist, create it
- [RULE-005](RULE-005) (chunkhound-usage) — tools for verifying implementations exist
