---
id: RULE-af5771e3
type: rule
title: No Stubs or Placeholders
description: No hardcoded fake data, TODO functions, or scaffolded implementations in production code.
status: active
created: 2026-03-07
updated: 2026-03-12
enforcement:
  - mechanism: behavioral
    message: "No hardcoded fake data, TODO functions, or scaffolded implementations in production code; all implementations must be real"
  - mechanism: hook
    type: PostToolUse
    event: file
    action: check
    pattern: "src/**/*"
relationships:
  - target: DOC-9814ec3c
    type: documented-by
  - target: PD-af88bb69
    type: enforces
---
## What Counts as a Stub

- Hardcoded return values pretending to be real data (e.g., `status: "connected"`, `latency: 42`)
- Default arrays/objects with fake data that should come from a real source
- Functions that log "TODO" or do nothing
- Test functions that always return success without actually testing anything
- "No-op" event handlers that log instead of performing the action
- Async functions with TODO comments in their implementation bodies
- Functions that return default values instead of doing real work
- Any function that claims to persist data but only modifies in-memory state
- API handler functions that return hardcoded data instead of computing real results
- Client calls wrapped in try/catch that silently return fake fallback data on error

## Verification Before Commit

For EVERY new feature endpoint or UI component:

1. Does the UI call a real backend endpoint or command? If not, it's a stub
2. Does the backend function perform real work and return real data? If it returns hardcoded defaults, it's a stub
3. Does the data displayed come from the backend? If it uses hardcoded defaults as the primary source, it's a stub
4. Does error handling show real errors? If it always returns success, it's a stub

## When Backend Doesn't Exist Yet

If a backend function doesn't exist yet, you MUST either:

- Create the backend function FIRST, then wire the frontend
- Show an explicit "Not configured" / "Not available" state in the UI
- NEVER show fake success data to make it look like it works

## Automated Enforcement

A stub scanner should be part of the CI/quality checks. It scans all production source code for:

- Mock/placeholder/TODO/FIXME/HACK comments
- Hardcoded data standing in for real backend responses
- Scaffolded implementations that don't do real work

**If the scanner finds stubs, the build fails.** Legitimate exceptions (e.g., known incomplete features tracked in `.orqa/implementation/tasks/`) can be added to an allowlist with a documented reason.

## Code Search Integration

Use `search_regex` for a function name to instantly verify it exists in both the backend and the frontend call sites.

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
```text

**The "What Is NOT Done" section is NON-NEGOTIABLE.** Omitting it is treated as a review FAIL. An empty section with justification ("Nothing — all deliverables are fully implemented") is acceptable. A missing section is not.

**Why this exists:** Agents naturally emphasize what they accomplished and downplay gaps. This section forces explicit acknowledgment of limitations. The user reads this section FIRST to understand the true state of work.

## Related Rules

- end-to-end-completeness — the full chain that must exist
- [RULE-0be7765e](RULE-0be7765e) (error-ownership) — if the function doesn't exist, create it
- [RULE-0d29fc91](RULE-0d29fc91) (code-search-usage) — tools for verifying implementations exist
