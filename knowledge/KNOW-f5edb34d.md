---
id: KNOW-f5edb34d
type: knowledge
title: Diagnostic Methodology
description: |
  Root cause analysis methodology: capture, reproduce, isolate, fix, verify.
  Covers debug process, common issue categories, root cause classification,
  and the discipline of fixing causes not symptoms.
  Use when: Debugging failures, diagnosing unexpected behavior, tracing
  data flow through multi-layer systems, or investigating production issues.
status: active
created: 2026-03-01
updated: 2026-03-10
category: methodology
version: 1.0.0
user-invocable: true
relationships:
  - target: DOC-a1b2c3d4
    type: synchronised-with
---


Systematic debugging methodology for finding and fixing root causes. This skill teaches the *process* of diagnosis — the domain-specific knowledge (e.g., which layers exist, what tools are available) comes from the project's architecture skills.

## Debug Process (Strict Sequence)

Follow this sequence. Do not skip steps.

### 1. Capture

- Gather the exact error message, stack trace, or unexpected behavior description
- Identify the affected layer or boundary in the system
- Note reproduction conditions: when does it happen, how consistently

### 2. Reproduce

- Attempt to reproduce with the minimal set of conditions
- Write a failing test if possible — this becomes the regression test
- If the issue is intermittent, identify the timing or state conditions

### 3. Isolate

- Narrow down to the specific function, module, or boundary at fault
- Use code search to find all callers and callees of the suspect code
- Check version history to see when the code last changed
- Verify assumptions: is the data what you expect at each boundary?

### 4. Fix

- Apply the minimal change that addresses the root cause
- Do not fix symptoms — fix causes
- If the fix is complex, explain the chain of causation
- Check if the same pattern exists elsewhere (search for similar code)

### 5. Verify

- Run the full test suite (or the relevant subset)
- Confirm the original reproduction case no longer fails
- Check for regressions in adjacent functionality

## Root Cause Classification

After diagnosis, classify the root cause:

| Classification | Description | Fix Pattern |
|---------------|-------------|-------------|
| **Logic Error** | Code does the wrong thing | Algorithm/logic correction |
| **Type Error** | Wrong type at a boundary | Type correction at the source |
| **State Error** | Stale, missing, or conflicting state | State management fix |
| **Integration Error** | Two components disagree on protocol | Interface/contract fix |
| **Data Error** | Bad data in storage or input | Validation or migration |
| **Race Condition** | Timing-dependent failure | Synchronization or ordering fix |
| **Configuration Error** | Wrong settings, missing env | Config correction |

## Anti-Patterns

| Anti-Pattern | Why It's Wrong | What To Do Instead |
|-------------|----------------|-------------------|
| Guess and patch | Wastes time, may introduce new bugs | Follow the 5-step process |
| Suppress the error | Hides the real problem | Find and fix the cause |
| "It works on my machine" | Not a diagnosis | Identify the environmental difference |
| Fix the test instead of the code | Breaks the safety net | The test is probably right |
| Add a retry loop | Hides intermittent failures | Find the race condition |

## Critical Rules

- NEVER apply a fix without understanding the root cause
- NEVER suppress errors to "fix" them (no empty catch blocks, no silent defaults hiding failures)
- Always check if the same pattern exists elsewhere
- Document the root cause and fix, even for simple bugs
- If you cannot reproduce the issue, say so explicitly — do not guess at fixes
- Check the project's lessons/known-issues before reporting a finding as novel
