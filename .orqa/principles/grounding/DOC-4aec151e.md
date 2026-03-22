---


id: DOC-4aec151e
type: grounding
status: captured
title: Code Principles — Agent Grounding
description: Concise grounding document for Implementer and Reviewer roles. Injected into agent context at initialization.
created: 2026-03-14
updated: 2026-03-14
relationships: []
---
## What Good Code Means Here

Good code is code that a future agent can read, trust, and extend without fear. That requires three things: explicit structure, honest error paths, and testable units. When code is small, typed, and returns explicit results, it is safe to compose. When it silently swallows errors or grows past 50 lines, it becomes opaque — no one can reason about it, and neither can you.

Standards exist not to impose bureaucracy but to enforce the properties that make code trustworthy at speed.

## The Boundaries That Must Never Be Crossed

**Rust owns the domain.** Business logic lives in `backend/src-tauri/src/domain/`. Svelte is a view layer. If you find yourself putting domain logic in the frontend, stop — you are violating the architectural contract that keeps the system testable and maintainable.

**`invoke()` is the only bridge.** There is no HTTP server. There is no shared memory. There is no side channel. Every frontend-backend communication goes through a `#[tauri::command]` function. Every feature that crosses the boundary needs both a Rust command and a matching TypeScript type. If they don't both exist, the feature doesn't exist.

**`Result<T, E>` everywhere.** A desktop app must never crash. There is no `unwrap()`, no `expect()`, no `panic!()` in production Rust. Every function returns a `Result`. Every error is handled explicitly and communicated to the user. This is non-negotiable — not a preference.

**Components display; pages fetch.** No `invoke()` calls inside `$lib/components/`. Pages and containers handle data loading. Components receive props. This separation makes components testable and reusable. A component that fetches its own data cannot be tested without a running backend.

**Svelte 5 runes, nothing else.** `$state`, `$derived`, `$effect`, `$props`. No `$:` reactive declarations. No `export let`. No `<slot>`. Mixing patterns creates confusion that compounds across sessions and reviewers.

## What Goes Wrong

**Taking shortcuts under task pressure.** The IPC boundary feels like overhead when you just need to pass one value. The Result type feels like ceremony when the function can't really fail. These feelings are wrong. The boundary and the error path exist for the cases you haven't thought of yet.

**Violating boundaries "just this once".** There is no "just this once" in a codebase with multiple agents working across sessions. Every violation becomes the precedent that justifies the next one. Maintain the boundaries even when they feel unnecessary.

**Writing code without understanding the system it lives in.** Before you write a function, understand where it sits in the stack — domain, command, store, component. Read the governing docs. Use `code_research` to understand the existing pattern before adding to it. Code that doesn't fit the system is harder to fix than code that was never written.

**Calling a function you haven't read.** Verify the signature. Check the types. Run the checks. "I assumed it would work" is not a verification strategy.
