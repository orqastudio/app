---
id: "RULE-5c9a1b74"
type: rule
title: "Self-Hosted Development"
description: "Methodology for developing software while running it. You are editing the app you are running inside — this creates unique constraints around restarts, protocol changes, and state preservation."
status: "active"
created: "2026-03-22"
updated: "2026-03-22"
enforcement:
  - mechanism: behavioral
    message: "When developing software while running it, treat every restart as a session boundary; preserve session state before any restart"
  - mechanism: hook
    type: SessionStart
    action: inject
    description: "Self-hosted context injected into system prompt when the project self-hosted flag is active"
---
When developing software while running it, you are editing the app you are running inside. The codebase IS the running instance. This creates unique constraints that do not apply to normal development.

## The Core Constraint

**Changes to the backend require a restart. In-app context: that restart ends your session.**

This is the fundamental difference between self-hosted development and normal development. A developer editing an external project can restart freely. A developer editing the project they are running inside must treat every restart as a session boundary.

## Context Distinction (CRITICAL)

The same constraints apply differently depending on where the agent is running:

| Situation | Restart Impact | Session State Required |
|-----------|---------------|----------------------|
| Running inside the app being edited | Restart ends the session | YES — save before restart |
| Running from external CLI / tooling | Restart just restarts the app | NO — session survives |

Always determine your context before acting on restart-sensitive changes. When in doubt, treat yourself as external (the safer assumption).

## What Requires a Restart

- Backend / compiled code changes — must recompile, cannot hot-reload
- Communication protocol changes — the bridge is live; changing it mid-session risks the connection
- Configuration changes that affect startup — only applied at next launch

## What Can Hot-Reload

- Frontend / UI changes (if a hot-reload mechanism is active) — appear immediately
- Static assets, styles, markup — usually safe to change live

## Hot-Reload Risks (In-App Context)

- Editing UI components while a response is streaming can crash the active window
- Avoid editing components that are actively rendering output mid-stream
- When in doubt, wait for the current operation to complete before saving frontend changes

## Restart Protocol (In-App Context)

Before any restart that will end your session:

1. **Save session state** — record tasks completed, in-progress work, what to resume next
2. **Commit all changes** — nothing should be lost when the session ends
3. **Offer the restart explicitly** — describe what will happen and confirm before proceeding
4. The next session picks up from the saved state

## Communication Protocol Changes (In-App Context)

The communication layer between the agent runtime and the backend is live while you are editing it. Changing the protocol format mid-session is dangerous.

- Warn before modifying any communication protocol files
- Rebuild the communication layer after changes
- Restart the app before continuing work that depends on the protocol

## Preview Tooling

Self-hosted projects cannot preview themselves. A running app cannot render an alternate version of itself inside its own window. Disable any preview features for self-hosted projects.

## FORBIDDEN

- Restarting without saving session state (in-app context)
- Changing communication protocol files without warning the user (in-app context)
- Editing actively-rendering UI components mid-stream (in-app context)
- Treating a restart as safe without first determining your context