---
id: RULE-009
slug: dogfood-mode
layer: project
title: "Dogfood Mode"
description: "Project-level rule for when an app is editing itself. Provides agent context awareness, operational caution, and system prompt injection. Active when dogfood: true in project.json."
status: active
tags: [project-level, dogfood, operational, context-awareness]
---

# Dogfood Mode (CONDITIONAL — only when `dogfood: true`)

This rule applies ONLY when `.orqa/project.json` contains `"dogfood": true`. For non-dogfood projects, ignore this rule entirely. This is a **project-level** rule, not a universal rule.

## What Dogfooding Means

You are editing the app you are running inside. The codebase IS the running instance. This creates unique constraints that don't apply to normal projects.

## Agent Context Awareness

When the dogfood flag is set, the project's system prompt injection tells orchestrating agents their operational context. Agents must always know:

| Question | Answer in Dogfood Mode |
|----------|----------------------|
| **What am I?** | An orchestrating agent coordinating implementation work |
| **Where am I?** | CLI (Claude Code) or App (OrqaStudio) — determines which tools and governance are available |
| **Am I dogfooding?** | Yes — my changes affect my own runtime environment |
| **What does that mean practically?** | Extra caution on restarts, protocol changes, and structural modifications. The systems-thinking rule applies with heightened urgency because I am modifying the system I am operating within. |

**This is NOT recursive reasoning about reasoning.** The agent needs one clear signal: "you are editing the app you are running inside." The project system prompt provides that signal. This rule provides the operational specifics.

### System Prompt Injection

When `dogfood: true`, the app's system prompt builder should include context like:

> You are working on a project that is dogfooding — the app you are building IS the app you are running inside. Changes to the backend require a restart. Changes to the sidecar protocol affect your active connection. Frontend changes apply via HMR but can crash mid-stream. Apply the systems-thinking rule with awareness that you are part of the system you are modifying.

This injection is what transitions an agent from "building an app" to "building the app I'm running in." The CLI achieves this via the dogfood-mode rule (`.claude/rules/` symlinks to `.orqa/team/rules/`) being loaded into context. The app achieves it via the system prompt injection.

## Enhanced Caution Rules

### Dev Server

- `make dev` uses `--no-watch` so editing `.rs` files does NOT auto-restart the app and kill the active session
- **NEVER use `make dev-watch`** — it causes the app to restart on every Rust file save, destroying the session
- After Rust backend changes, the orchestrator manages the restart (see Restart Protocol)

### Restart Protocol

After making Rust backend changes, the orchestrator manages the full restart lifecycle:

1. Write session state to `tmp/session-state.md` (tasks completed, in-progress work, what to resume)
2. Commit all changes (so nothing is lost when the app closes)
3. **Offer to restart**: "Backend changes need a restart. Shall I run `make restart`?"
4. If the user approves, run `make restart` as a single atomic command — this stops all processes, rebuilds, and relaunches in one step
5. **NEVER break restart into multiple commands** — the app closes when processes are killed, so multi-step sequences fail halfway. `make restart` handles the entire lifecycle atomically.
6. The session will end when the app restarts. The next session picks up from `tmp/session-state.md`.

**The orchestrator owns the dev lifecycle.** Do not tell the user to run development commands — offer to run them and execute on approval.

### Sidecar Self-Edit Warnings

The sidecar (`sidecar/src/`) is the communication bridge between the Agent SDK and the Rust backend. You are communicating THROUGH it while potentially editing it.

- Before modifying `sidecar/src/protocol.ts`, `sidecar/src/provider.ts`, or `sidecar/src/index.ts`: warn the user that this may affect the active connection
- After sidecar changes: the sidecar must be rebuilt (`cd sidecar && bun run build`) and the app restarted
- Never change the NDJSON protocol format mid-session without a restart

### Frontend Hot Reload

- Vite HMR handles frontend changes live — Svelte/TypeScript/CSS changes appear immediately
- BUT editing components mid-stream (while a response is streaming) can crash the window
- Avoid editing conversation-related components (`ConversationView`, `StreamingIndicator`, `MessageInput`) while a conversation is active

### Preview Tooling

- Dogfood projects cannot preview themselves (you can't render yourself inside yourself)
- When preview tooling is added in the future, it should be disabled for dogfood projects

## Detection

Check `.orqa/project.json` for `"dogfood": true` at task start. In the app context, the system prompt includes dogfood context when the flag is set.

## Related Rules

- RULE-028 (systems-thinking) — universal rule that applies to all projects; dogfood mode heightens its urgency
- RULE-007 (development-commands) — `make dev` and `make restart` commands
- RULE-006 (coding-standards) — general coding standards apply regardless of dogfood mode
