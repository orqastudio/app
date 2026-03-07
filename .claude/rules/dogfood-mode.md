---
scope: system
---

# Dogfood Mode (CONDITIONAL — only when `dogfood: true`)

This rule applies ONLY when `.orqa/project.json` contains `"dogfood": true`. For non-dogfood projects, ignore this rule entirely.

## What Dogfooding Means

You are editing the app you are running inside. The OrqaStudio codebase IS the running OrqaStudio instance. This creates unique constraints that don't apply to normal projects.

## Enhanced Caution Rules

### Dev Server

- `make dev` uses `--no-watch` so editing `.rs` files does NOT auto-restart the app and kill the active session
- **NEVER use `make dev-watch`** — it causes the app to restart on every Rust file save, destroying the session
- After Rust backend changes, the user must manually restart `make dev`

### Restart Protocol

After making Rust backend changes:

1. Write session state to `tmp/session-state.md` (tasks completed, in-progress work, what to resume)
2. Tell the user the backend needs a manual restart
3. Do NOT continue with work that depends on the Rust changes until the user confirms restart

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

- `development-commands.md` — `make dev` and `make restart` commands
- `coding-standards.md` — general coding standards apply regardless of dogfood mode
