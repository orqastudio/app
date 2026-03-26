---
id: "AD-d01b9e0a"
type: "decision"
title: "Security Model"
description: "Tauri three-layer security model with filesystem scoping, sensitive path denials, and OS keychain for API key storage."
status: completed
created: 2026-03-02T00:00:00.000Z
updated: 2026-03-02T00:00:00.000Z
relationships:
  - target: "EPIC-05ae2ce7"
    type: "drives"
---
## Decision

Tauri's three-layer security model (permissions → scopes → capabilities) configured with `$HOME/**` base file system scope, sensitive path denials (`.ssh`, `.gnupg`), pre-declared shell commands with argument validators, and `tauri-plugin-keyring` for API key storage in the OS keychain.

## Rationale

Tauri's capability system compiles permissions into the binary at build time. Deny always takes precedence over allow. The keyring plugin stores secrets in macOS Keychain, Windows Credential Manager, or Linux Secret Service — never in plaintext files or the app store.

## Consequences

Capabilities defined in `backend/src-tauri/capabilities/default.json`. Runtime scope expansion via `app_handle.fs_scope().allow_directory()` for user-selected project directories. `tauri-plugin-persisted-scope` remembers permissions across restarts. Shell commands (git, sh) must be pre-declared with regex argument validators.