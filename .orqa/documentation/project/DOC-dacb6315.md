---
id: DOC-dacb6315
type: doc
title: First-Run Setup Wizard
category: architecture
description: Design of the first-run setup wizard that guides users through initial project configuration.
created: 2026-03-04
updated: 2026-03-04
sort: 14
relationships: []
---


**Date:** 2026-03-04
**Phase:** 2a
**Status:** Design

## Overview

A version-gated setup wizard that runs on first launch. The wizard detects what is already configured, skips completed steps, and guides the user through missing ones. When future builds add new requirements, incrementing a version constant re-triggers the wizard for the new steps only.

The wizard is a full-screen overlay that blocks interaction with the main app until all steps are complete. This ensures the sidecar and embedding model are ready before the user tries to start a conversation.

## Version Gate Design

The setup wizard uses a version-gated approach to avoid re-running completed steps while supporting future expansion.

**Mechanism:**

- A global setting `setup_version` is stored in the Tauri `app` scope (via `tauri-plugin-store`)
- An app constant `CURRENT_SETUP_VERSION` is defined in the Rust backend (starts at `1`)
- On app launch, the backend compares `stored_version` vs `CURRENT_SETUP_VERSION`
- If `stored_version < CURRENT_SETUP_VERSION` (or does not exist), the wizard runs
- Each setup version defines which steps are required
- When all steps for the current version complete, `setup_version` is set to `CURRENT_SETUP_VERSION`

**Future expansion:**

To add new setup requirements in a future build (e.g., "configure MCP servers"), increment `CURRENT_SETUP_VERSION` to `2` and add the new steps to the version 2 step list. Users who already completed version 1 will only see the new steps.

## Setup Steps (v1)

### Step 1: Agent SDK CLI

Detect whether the Agent SDK CLI is installed and accessible.

- **Detection:** Run the configured CLI binary with `--version` via `std::process::Command`
- **If found:** Display the version string (e.g., "Claude Code v1.2.3"), auto-advance after 1 second
- **If not found:** Display install instructions with links to the provider's official install page. Show a "Check Again" button that re-runs detection
- **Stored data:** `agent_cli_version` in app settings

### Step 2: Provider Authentication

Verify the user is authenticated with the active AI provider.

- **Detection:** Run the CLI authentication status check via subprocess, or attempt a sidecar health check
- **If authenticated:** Display subscription info (e.g., "Subscription active"), auto-advance
- **If not authenticated:** Display a "Log in" button that launches the provider authentication flow in a terminal window. Show a "Check Again" button
- **Stored data:** `provider_auth_status`, `provider_subscription_type` in app settings

### Step 3: Start Sidecar

Start the Agent SDK sidecar process. This step is automatic once CLI and auth are confirmed.

- **Action:** Call `ensure_sidecar_running` (existing command) with the StartupTracker for progress reporting
- **Display:** Spinner with "Starting sidecar..." message, then "Connected" with a health check confirmation
- **Error state:** If sidecar fails to start, display error message with "Retry" button and troubleshooting tips
- **No stored data:** Sidecar is ephemeral (restarted each app launch)

### Step 4: Embedding Model

Ensure the ONNX embedding model is downloaded and ready for semantic search.

- **Detection:** Check for model files at `app_data_dir/models/bge-small-en-v1.5/`
- **If present:** Display "Model ready", auto-advance
- **If not present:** Display "Downloading embedding model..." with a progress bar. Uses the model download infrastructure built into the app scaffold.
- **Stored data:** `embedding_model_status` in app settings

### Step 5: Setup Complete

Confirmation screen.

- **Action:** Set `setup_version = CURRENT_SETUP_VERSION` in the store
- **Display:** "You're all set!" with a summary of what was configured
- **Transition:** "Get Started" button dismisses the wizard and shows the main app (Welcome/project open view)

## Backend Changes

| File | Change |
|------|--------|
| `backend/src-tauri/src/domain/setup.rs` | New — setup domain types |
| `backend/src-tauri/src/commands/setup_commands.rs` | New — setup Tauri commands |
| `backend/src-tauri/src/domain/mod.rs` | Add `pub mod setup;` |
| `backend/src-tauri/src/commands/mod.rs` | Add `pub mod setup_commands;` |
| `backend/src-tauri/src/lib.rs` | Register setup commands in the Tauri app builder; add `CURRENT_SETUP_VERSION` constant; check setup_version on launch |

## Key Types

```rust
/// Overall setup status returned to the frontend
#[derive(Debug, Clone, Serialize)]
pub struct SetupStatus {
    pub setup_complete: bool,
    pub current_version: u32,
    pub stored_version: u32,
    pub steps: Vec<SetupStepStatus>,
}

/// Status of an individual setup step
#[derive(Debug, Clone, Serialize)]
pub struct SetupStepStatus {
    pub id: String,
    pub label: String,
    pub status: StepStatus,
    pub detail: Option<String>,
}

/// Step lifecycle states
#[derive(Debug, Clone, Serialize)]
pub enum StepStatus {
    Pending,
    Checking,
    Complete,
    Error,
    ActionRequired,
}

/// Information about the Agent SDK CLI installation
#[derive(Debug, Clone, Serialize)]
pub struct AgentCliInfo {
    pub installed: bool,
    pub version: Option<String>,
    pub path: Option<String>,
    pub authenticated: bool,
    pub subscription_type: Option<String>,
}
```

## IPC Commands

| Command | Input | Output | Description |
|---------|-------|--------|-------------|
| `get_setup_status` | — | `SetupStatus` | Check overall setup state |
| `check_agent_cli` | — | `AgentCliInfo` | Detect CLI installation |
| `check_provider_auth` | — | `AgentCliInfo` | Verify authentication |
| `check_embedding_model` | — | `SetupStepStatus` | Check model download status |
| `complete_setup` | — | `()` | Mark setup as complete, set version |

## Frontend Changes

| File | Change |
|------|--------|
| `ui/src/lib/types/setup.ts` | New — TypeScript interfaces matching Rust types |
| `ui/src/lib/stores/setup.svelte.ts` | New — setup store with step state, detection results, actions |
| `ui/src/lib/components/setup/SetupWizard.svelte` | New — full-screen overlay container, step navigation |
| `ui/src/lib/components/setup/AgentCliStep.svelte` | New — CLI detection UI |
| `ui/src/lib/components/setup/ProviderAuthStep.svelte` | New — auth detection + login flow |
| `ui/src/lib/components/setup/SidecarStep.svelte` | New — sidecar startup with spinner |
| `ui/src/lib/components/setup/EmbeddingModelStep.svelte` | New — model download with progress bar |
| `ui/src/lib/components/setup/SetupComplete.svelte` | New — completion confirmation |
| `ui/src/routes/+layout.svelte` (or AppLayout) | Mount SetupWizard when `setup_complete === false` |

## Component State Table

| Component | States | What the user sees |
|-----------|--------|-------------------|
| SetupWizard | `loading`, `active`, `complete` | Loading: spinner. Active: current step. Complete: dismissed (main app visible). |
| AgentCliStep | `checking`, `found`, `not_found`, `error` | Checking: spinner. Found: version + auto-advance. Not found: install instructions + "Check Again". Error: error message + retry. |
| ProviderAuthStep | `checking`, `authenticated`, `not_authenticated`, `error` | Checking: spinner. Authenticated: subscription info + auto-advance. Not authenticated: "Log in" button. Error: error message + retry. |
| SidecarStep | `starting`, `connected`, `error` | Starting: spinner. Connected: green status. Error: error message + "Retry" + troubleshooting. |
| EmbeddingModelStep | `checking`, `downloading`, `complete`, `error` | Checking: spinner. Downloading: progress bar. Complete: auto-advance. Error: retry option. |
| SetupComplete | `ready` | Summary + "Get Started" button. |

## Settings Integration

After setup completes, the Settings > Provider section exposes the detected configuration:

- **Agent SDK CLI:** Version string, installation path
- **Authentication:** Auth status, subscription type, "Re-authenticate" button (re-runs the provider login flow)
- **Sidecar:** Health status (connected/disconnected), "Restart" button
- **Embedding Model:** Status (ready/downloading/missing), model version

This reuses the data collected during setup. The Settings view calls the same `check_agent_cli` and `check_provider_auth` commands to refresh status on demand.

## User Journeys

**First-time install (everything missing):**
1. User launches OrqaStudio™ for the first time
2. Wizard appears: Step 1 shows "Agent SDK CLI not found" with install link
3. User installs CLI, clicks "Check Again" — detected, auto-advances
4. Step 2: "Not authenticated" — user clicks "Log in", authenticates via the provider's flow
5. Step 3: Sidecar starts automatically
6. Step 4: Embedding model downloads with progress bar
7. Step 5: "You're all set!" — user clicks "Get Started"

**Returning user (everything configured):**
1. User launches OrqaStudio
2. Backend checks `setup_version` — matches `CURRENT_SETUP_VERSION`
3. Wizard does not appear, main app loads immediately

**Upgrade scenario (new version adds steps):**
1. User updates OrqaStudio to a version with `CURRENT_SETUP_VERSION = 2`
2. On launch, stored version (1) < current version (2)
3. Wizard appears showing only the new steps (previously completed steps show as "Complete" and are skipped or shown briefly)
4. After new steps complete, `setup_version` updated to 2

## Verification Criteria

1. **Fresh install:** Wizard appears, all steps walkable end-to-end
2. **Already configured:** Steps auto-detect and auto-complete, wizard finishes quickly
3. **All steps done:** Wizard does not re-appear on subsequent launches
4. **Version increment:** Wizard re-triggers for new steps only
5. **Error recovery:** Each step handles errors gracefully with retry options
6. **Settings reflect setup:** Provider section shows correct CLI version, auth status

## Related Documents

- Roadmap
- Streaming Pipeline
- IPC Commands
- Settings & Onboarding Wireframes
