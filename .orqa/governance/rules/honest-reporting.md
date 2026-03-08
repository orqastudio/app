---
id: honest-reporting
title: "Honest Reporting"
description: "Report status accurately. Partial work reported as complete is worse than reporting it as incomplete."
scope: system
---


## The Core Rule

**Reporting partial work as complete is WORSE than reporting it as incomplete.**

An agent that says "Phase 2 complete — all Tauri commands fully implemented" when half the commands return placeholder data has actively harmed the user's ability to make decisions. The user now believes they have a capability they don't have.

An agent that says "Phase 2 complete with one exception: the scanner runner command is scaffolded — it parses output format but doesn't actually spawn the scanner process yet" has given the user accurate information to act on.

**Err on the side of declaring things NOT done.** The cost of a false "incomplete" is a follow-up question. The cost of a false "complete" is wasted time, broken trust, and downstream failures.

## The "Would It Work" Test

Before marking ANY deliverable as complete, ask:

> "If the user tried to USE this feature right now — not inspect the code, not run tests,
> but actually USE it as a real user would — what would happen?"

- If it works as documented -> it's done
- If it shows empty/default/mock data -> it's NOT done (it's scaffolding)
- If it errors -> it's NOT done
- If it partially works -> describe exactly what works and what doesn't

## Status Categories

Use these precise terms — not vague language like "mostly done" or "almost complete":

| Term | Meaning | Example |
|------|---------|---------|
| **Done** | Works end-to-end as documented | "Session persistence: messages save to SQLite, reload on app restart" |
| **Partially done** | Some paths work, others don't | "File tools: Read and Glob work, Write doesn't handle binary files" |
| **Scaffolded** | Code exists but doesn't do real work | "Scanner runner exists but delegates to a TODO function" |
| **Not started** | No implementation exists | "No metrics dashboard component" |

**FORBIDDEN terms:**

- "Mostly complete" — what's the incomplete part?
- "Should work" — did you test it?
- "Will be wired up later" — then it's not done now
- "Works in isolation" — does it work in the system?
- "Passes type checking" — does it actually DO anything?

## Agent Output Requirements

See `.orqa/rules/no-stubs.md` > "Agent Completion Reports" for the mandatory output structure.

## Lesson Documentation Status

Completion reports from review agents (`code-reviewer`, `qa-tester`, `ux-reviewer`) MUST include lesson documentation status:

- Were any new IMPL entries added to `.orqa/lessons/`?
- Were any existing IMPL entries updated with recurrence increments?
- Was `.orqa/lessons/` checked for known patterns before reporting findings?

Review agents that skip lesson documentation are in violation of this rule AND `.orqa/rules/lessons-learned.md`.

## Related Rules

- `no-stubs.md` — what counts as a stub + mandatory output structure
- `no-deferred-deliverables.md` — deferring scoped deliverables is dishonest reporting
- `plan-mode-compliance.md` — verification gate protocol + evidence requirements
- `error-ownership.md` — all errors are your responsibility
- `lessons-learned.md` — learning loop enforcement for review agents
