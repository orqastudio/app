---
id: AD-d01b9e0a
type: decision
title: Security Model
description: "Tauri three-layer security model with filesystem scoping, sensitive path denials, and OS keychain for API key storage."
status: completed
created: 2026-03-02
updated: 2026-03-02
relationships:
  - target: RES-7b24ff49
    type: informed-by
    rationale: RES-7b24ff49 documented Tauri v2's capability-based security model with compiled-in permissions, deny-precedence scoping, and persisted-scope plugin
  - target: EPIC-05ae2ce7
    type: drives
  - target: DOC-52b00632
    type: documented-by
  - target: DOC-ec909ab0
    type: documented-by
---
## Decision

Tauri's three-layer security model (permissions → scopes → capabilities) configured with `$HOME/**` base file system scope, sensitive path denials (`.ssh`, `.gnupg`), pre-declared shell commands with argument validators, and `tauri-plugin-keyring` for API key storage in the OS keychain.

## Rationale

Tauri's capability system compiles permissions into the binary at build time. Deny always takes precedence over allow. The keyring plugin stores secrets in macOS Keychain, Windows Credential Manager, or Linux Secret Service — never in plaintext files or the app store.

## Consequences

Capabilities defined in `backend/src-tauri/capabilities/default.json`. Runtime scope expansion via `app_handle.fs_scope().allow_directory()` for user-selected project directories. `tauri-plugin-persisted-scope` remembers permissions across restarts. Shell commands (git, sh) must be pre-declared with regex argument validators.