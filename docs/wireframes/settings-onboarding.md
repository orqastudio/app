---
title: "Wireframe: Settings & Onboarding"
description: "Wireframe specification for settings pages and the onboarding flow."
category: wireframes
tags: []
created: 2026-03-02
updated: 2026-03-04
---

**Date:** 2026-03-02 | **Informed by:** [Onboarding Research](/research/onboarding), [MVP Spec F-001, F-001b, F-009](/product/mvp-specification)

Settings appears in the Explorer Panel when the user clicks the Settings icon in the Activity Bar (bottom) or presses `Ctrl+,`. Onboarding flows are full-window overlays shown on first run or when no project is loaded.

---

## 1. Settings in Explorer Panel (All Sections)

```plantuml
@startsalt
{
  {/ <b>Settings</b> }
  ---
  {SI
    {
      <b>Provider</b>
      ---
      Claude Code CLI Path
      { [/usr/local/bin/claude           ] | [Browse...] | [Auto-detect] }
      ---
      Sidecar Status: | <&circle-check> <color:green>Running</color>
      Connection Health: | <&signal> Connected
      ---
      Subscription: | <b>Max</b> (active)
      ---
      [Restart Sidecar]
    }
    ---
    {
      <b>Project</b>
      ---
      Project Root
      { [~/code/orqa-studio                    ] | [Change...] }
      ---
      Detected Stack
      { Languages: Rust, TypeScript, Svelte }
      { Frameworks: Tauri v2, shadcn-svelte }
      { Build: Cargo, Vite }
      ---
      File Watcher: | <&circle-check> Active (watching 1,247 files)
      ---
      [Rescan Project]
    }
    ---
    {
      <b>Appearance</b>
      ---
      Theme | ^System (default)^
      Font Size | ^14px^
      ---
      Panel Default Widths
      Nav Sub-Panel | [200] px
      Explorer | [360] px
      ---
      [X] Enable per-project theming
    }
    ---
    {
      <b>Keyboard Shortcuts</b>
      ---
      {#
        Shortcut | Action
        Ctrl+K | Global search
        Ctrl+N | New session
        Ctrl+B | Toggle Nav Sub-Panel
        Ctrl+, | Open settings
        Ctrl+Enter | Send message
        Ctrl+Shift+N | New project
        Ctrl+O | Open project
        Ctrl+/ | Toggle shortcuts card
        Esc | Close overlay / cancel
      }
    }
    ---
    .
  }
}
@endsalt
```

### Settings Section Behavior

| Section | Notes |
|---------|-------|
| **Provider** | CLI path can be typed, browsed via native file dialog, or auto-detected. Auto-detect searches PATH and common install locations. Sidecar status updates in real time. |
| **Project** | Project settings are stored in `.orqa/project.json` (file-based, source of truth). When no settings file exists, shows a setup wizard with scan + save flow. When settings file exists, shows an editable form (name, description, model, excluded paths, detected stack, governance counts). Rescan re-detects stack and governance artifacts. Changes save on blur — no manual save button. |
| **Appearance** | Theme dropdown offers Light, Dark, System. Font size dropdown ranges 12px--20px. Panel widths accept numeric input with min/max validation. |
| **Keyboard Shortcuts** | Read-only reference card. All shortcuts are global (not remappable in MVP). |

---

## 2. First-Run Welcome Screen (No Project, No API Key)

Full-window overlay. No panels -- just the centered welcome content.

```plantuml
@startsalt
{+
  {/ <b>OrqaStudio</b> | . | . | . | . | . | . | . | . }
  {+
    {
      . | . | . | . | . | .
      ---
      .
      .
      { . | . | <b><size:24>OrqaStudio</size></b> | . | . }
      { . | . | <i>AI-powered product management</i> | . | . }
      .
      ---
      { . | . | Welcome to OrqaStudio. Let's get you set up. | . | . }
      .
      { . | . | To start, OrqaStudio needs access to Claude Code | . | . }
      { . | . | CLI so it can run AI sessions on your behalf. | . | . }
      .
      ---
      .
      { . | . | [Set Up Claude Code CLI ->] | . | . }
      .
      ---
      .
      .
    }
  }
  { . | . | . | . | . | . | . | . | . }
}
@endsalt
```

### Welcome Screen Behavior

| Element | Notes |
|---------|-------|
| **Anvil mark** | Rendered above the title in the actual UI; Salt cannot depict graphical logos. |
| **Set Up Claude Code CLI** | Primary action button, advances to the CLI setup step (State 3). |
| **No wizard steps** | There is no numbered step indicator. Progression is linear but presented as self-contained screens, consistent with conversation-first progressive disclosure. |

---

## 3. API Key / CLI Setup Step

Still a full-window overlay. The user provides the Claude Code CLI path.

```plantuml
@startsalt
{+
  {/ <b>OrqaStudio</b> | . | . | . | . | . | . | . | . }
  {+
    {
      . | . | . | . | . | .
      ---
      .
      { . | <b><size:18>Connect Claude Code CLI</size></b> | . | . }
      .
      { . | OrqaStudio uses the Claude Code CLI as its AI engine. | . | . }
      { . | Enter the path or let OrqaStudio find it automatically. | . | . }
      .
      ---
      { . | CLI Path | . | . }
      { . | [                                ] | [Browse...] | . }
      .
      { . | [Auto-detect] | . | . }
      .
      ---
      { . | <b>Status:</b> | . | . }
      { . | <&circle-x> <color:red>Not configured</color> | . | . }
      .
      ---
      .
      { . | [<- Back] | . | [Continue ->] | . }
      .
    }
  }
  { . | . | . | . | . | . | . | . | . }
}
@endsalt
```

### CLI Setup -- After Successful Detection

```plantuml
@startsalt
{+
  {/ <b>OrqaStudio</b> | . | . | . | . | . | . | . | . }
  {+
    {
      . | . | . | . | . | .
      ---
      .
      { . | <b><size:18>Connect Claude Code CLI</size></b> | . | . }
      .
      { . | OrqaStudio uses the Claude Code CLI as its AI engine. | . | . }
      { . | Enter the path or let OrqaStudio find it automatically. | . | . }
      .
      ---
      { . | CLI Path | . | . }
      { . | [/usr/local/bin/claude           ] | [Browse...] | . }
      .
      { . | [Auto-detect] | . | . }
      .
      ---
      { . | <b>Status:</b> | . | . }
      { . | <&circle-check> <color:green>Found Claude Code v1.2 -- Max subscription active</color> | . | . }
      .
      ---
      .
      { . | [<- Back] | . | [Continue ->] | . }
      .
    }
  }
  { . | . | . | . | . | . | . | . | . }
}
@endsalt
```

### CLI Setup States

| State | Status Indicator | Continue Button |
|-------|-----------------|-----------------|
| Not configured | <span style="color:red">Red X</span> -- "Not configured" | Disabled |
| Detecting... | Spinner -- "Searching..." | Disabled |
| Found | <span style="color:green">Green check</span> -- version and subscription info | Enabled |
| Error | <span style="color:red">Red X</span> -- "Not found at path" or "CLI returned error" | Disabled |

---

## 4. Project Open with Scan Results

After CLI setup, the user selects a project folder. This screen shows the scan results and invites the user into the main workspace.

```plantuml
@startsalt
{+
  {/ <b>OrqaStudio</b> | . | . | . | . | . | . | . | . }
  {+
    {
      . | . | . | . | . | .
      ---
      .
      { . | <b><size:18>Project Scanned</size></b> | . | . }
      .
      { . | <b>orqa-studio</b> -- ~/code/orqa-studio | . | . }
      .
      ---
      { . |
        {#
          Category | Detected
          Languages | Rust, TypeScript, Svelte
          Frameworks | Tauri v2, shadcn-svelte
          Build Tools | Cargo, Vite
          VCS | Git (clean)
          Files | 1,247 tracked
        }
      | . | . }
      .
      ---
      { . | Governance artifacts (.claude/ directory): | . | . }
      { . | <&circle-check> 3 Agents, 5 Rules, 2 Skills, 1 Hook | . | . }
      .
      ---
      .
      { . | [<- Change Project] | . | [Open Workspace ->] | . }
      .
    }
  }
  { . | . | . | . | . | . | . | . | . }
}
@endsalt
```

### Project Selection -- Preceding Step

Before the scan results appear, the user sees a project selection screen.

```plantuml
@startsalt
{+
  {/ <b>OrqaStudio</b> | . | . | . | . | . | . | . | . }
  {+
    {
      . | . | . | . | . | .
      ---
      .
      { . | <b><size:18>Open a Project</size></b> | . | . }
      .
      { . | Select a folder to manage with OrqaStudio. | . | . }
      .
      ---
      { . | [Open Folder...] | . | . }
      .
      ---
      { . | <b>Recent Projects</b> | . | . }
      { . | <i>(No recent projects)</i> | . | . }
      .
      ---
      .
      { . | [<- Back] | . | . }
      .
    }
  }
  { . | . | . | . | . | . | . | . | . }
}
@endsalt
```

### Project Selection -- With Recent Projects (Return Visit)

```plantuml
@startsalt
{+
  {/ <b>OrqaStudio</b> | . | . | . | . | . | . | . | . }
  {+
    {
      . | . | . | . | . | .
      ---
      .
      { . | <b><size:18>Open a Project</size></b> | . | . }
      .
      { . | Select a folder to manage with OrqaStudio. | . | . }
      .
      ---
      { . | [Open Folder...] | . | . }
      .
      ---
      { . | <b>Recent Projects</b> | . | . }
      {SI
        {
          <&folder> <b>orqa-studio</b>
          ~/code/orqa-studio -- Rust, TypeScript, Svelte
        }
        ---
        {
          <&folder> <b>acme-api</b>
          ~/code/acme-api -- Go, PostgreSQL
        }
        ---
        {
          <&folder> <b>marketing-site</b>
          ~/code/marketing-site -- TypeScript, Next.js
        }
      }
      .
      ---
      .
      { . | [<- Back] | . | . }
      .
    }
  }
  { . | . | . | . | . | . | . | . | . }
}
@endsalt
```

---

## 5a. New Project -- Discovery Prompt

When the user opens a folder that has no `.claude/` directory (or creates a new project), OrqaStudio™ offers to start a project discovery conversation instead of just scaffolding generic files.

```plantuml
@startsalt
{+
  {/ <b>OrqaStudio</b> | . | . | . | . | . | . | . | . }
  {+
    {
      . | . | . | . | . | .
      ---
      .
      { . | <b><size:18>Project Scanned</size></b> | . | . }
      .
      { . | <b>new-saas-app</b> -- ~/code/new-saas-app | . | . }
      .
      ---
      { . |
        {#
          Category | Detected
          Languages | TypeScript, Python
          Frameworks | React, FastAPI
          Build Tools | npm, pip
          VCS | Git (3 uncommitted)
          Files | 312 tracked
        }
      | . | . }
      .
      ---
      { . | <&warning> No governance artifacts found. | . | . }
      .
      { . | OrqaStudio can help you set up your project through | . | . }
      { . | a quick conversation about what you're building, | . | . }
      { . | your tech stack, and your conventions. | . | . }
      .
      ---
      .
      { . | [Skip -- Use Defaults] | . | [Start Project Discovery ->] | . }
      .
    }
  }
  { . | . | . | . | . | . | . | . | . }
}
@endsalt
```

### Discovery Prompt -- Empty Project Variant

For a truly new project with no code detected, the scan table is replaced with a brief message.

```plantuml
@startsalt
{+
  {/ <b>OrqaStudio</b> | . | . | . | . | . | . | . | . }
  {+
    {
      . | . | . | . | . | .
      ---
      .
      { . | <b><size:18>New Project Created</size></b> | . | . }
      .
      { . | <b>my-app</b> -- ~/code/my-app | . | . }
      .
      ---
      { . | New project, no code detected. | . | . }
      { . | <&warning> No governance artifacts found. | . | . }
      .
      ---
      { . | OrqaStudio can help you set up your project through | . | . }
      { . | a quick conversation about what you're building, | . | . }
      { . | your tech stack, and your conventions. | . | . }
      .
      ---
      .
      { . | [Skip -- Use Defaults] | . | [Start Project Discovery ->] | . }
      .
    }
  }
  { . | . | . | . | . | . | . | . | . }
}
@endsalt
```

### Discovery Prompt Behavior

| Element | Notes |
|---------|-------|
| **Scan results table** | Shown for existing-code projects. Hidden for truly empty new projects. |
| **Start Project Discovery** | Primary action. Opens the main workspace with a discovery conversation session already in progress. |
| **Skip -- Use Defaults** | Secondary action. Creates a minimal CLAUDE.md (project name + date), empty governance directories, and opens the workspace with a blank conversation. |
| **No file list** | Unlike the old scaffold prompt, no specific files are listed — the discovery conversation determines what gets created. |

---

## 5b. New Project -- Discovery Conversation

After the user clicks "Start Project Discovery", OrqaStudio opens the main three-zone workspace with a discovery conversation already in progress. This is a regular conversation session — the discovery behavior comes from the system prompt, not special UI.

```plantuml
@startsalt
{
  {/ <b>OrqaStudio</b> | . | . | . | . | . | . | . | . }
  {
    {
      <b>Nav Sub-Panel</b>
      ---
      <i>(empty)</i>
    }
    |
    {
      <b>Explorer</b>
      ---
      <i>No artifacts yet.</i>
      <i>Discovery in progress...</i>
      .
    }
    |
    {
      { ^Project Discovery v^ | . | ^Auto (recommended)^ }
      ---
      {SI
        <b>Claude:</b> Welcome to your new project!
        I'd like to learn about what you're
        building so I can set up governance
        artifacts tailored to your needs.
        .
        <b>What are you building? Who is it</b>
        <b>for? What problem does it solve?</b>
        .
        ---
        <b>You:</b> It's a SaaS platform for small
        teams to manage their design tokens
        and component libraries.
        ---
        <b>Claude:</b> Great -- a design system
        management tool. That helps me
        understand the domain.
        .
        <b>What tech stack are you planning?</b>
        Any preferences for languages,
        frameworks, or infrastructure?
        .
      }
      ---
      { [Type your response...              ] | [Send] }
    }
  }
}
@endsalt
```

### Discovery Conversation Behavior

| Element | Notes |
|---------|-------|
| **Session dropdown** | The discovery session appears as "Project Discovery" in the session dropdown — a regular session, not a special mode. |
| **Conversation** | Claude follows a system prompt that covers product, tech stack, team, conventions, and prior art. The user responds naturally. |
| **Explorer Panel** | Empty during discovery. Populates after the user approves generated artifacts. |
| **Exit anytime** | The user can close the conversation, start a new session, or say "that's enough." The discovery session persists and can be resumed. |
| **No special UI** | This is the standard three-zone layout. The discovery "magic" is entirely in the system prompt — no wizard, no special components. |

---

## 5c. New Project -- Governance Artifacts Confirmation

After discovery (or when Claude has enough context), Claude proposes artifacts. The user reviews them in a confirmation dialog before any files are written.

```plantuml
@startsalt
{+
  {/ <b>OrqaStudio</b> | . | . | . | . | . | . | . | . }
  {+
    {
      . | . | . | . | . | .
      ---
      .
      .
      { . |
        {+
          <b>Confirm: Create Governance Artifacts</b>
          ---
          Based on our conversation, I've prepared
          these governance artifacts for your project:
          ---
          {SI
            [X] .claude/CLAUDE.md (project summary)
            [X] .claude/agents/backend-api.md
            [X] .claude/agents/frontend-ui.md
            [X] .claude/rules/conventions.md
            [X] .claude/rules/testing.md
          }
          ---
          Uncheck any files you don't want created.
          You can edit or delete them at any time.
          ---
          { [Cancel] | . | . | [Create Files] }
        }
      | . | . }
      .
      .
    }
  }
  { . | . | . | . | . | . | . | . | . }
}
@endsalt
```

---

## Settings in Explorer Panel -- Sidecar Error State

When the sidecar process encounters an error, the Provider section surfaces the problem.

```plantuml
@startsalt
{
  {/ <b>Settings</b> }
  ---
  {SI
    {
      <b>Provider</b>
      ---
      Claude Code CLI Path
      { [/usr/local/bin/claude           ] | [Browse...] | [Auto-detect] }
      ---
      Sidecar Status: | <&circle-x> <color:red>Error</color>
      Connection Health: | <&ban> Disconnected
      ---
      {
        <&warning> <color:red>Sidecar process exited with code 1.</color>
        Check that the CLI path is correct and
        your subscription is active.
      }
      ---
      [Restart Sidecar] | [View Logs]
    }
    ---
    .
    .
  }
}
@endsalt
```

---

## Settings in Explorer Panel -- Idle Sidecar State

When no session is active, the sidecar is idle.

```plantuml
@startsalt
{
  {/ <b>Settings</b> }
  ---
  {SI
    {
      <b>Provider</b>
      ---
      Claude Code CLI Path
      { [/usr/local/bin/claude           ] | [Browse...] | [Auto-detect] }
      ---
      Sidecar Status: | <&clock> Idle
      Connection Health: | <&signal> Connected
      ---
      Subscription: | <b>Max</b> (active)
      ---
      [Restart Sidecar]
    }
    ---
    .
    .
  }
}
@endsalt
```

---

## Onboarding Flow Diagram

The overall onboarding progression, for reference.

```plantuml
@startuml
(*) --> "Welcome Screen"
"Welcome Screen" --> "CLI Setup"

"CLI Setup" --> "Auto-detect / Browse / Manual"
"Auto-detect / Browse / Manual" --> if "CLI Found?" then
  -->[yes] "Project Selection"
else
  -->[no] "Show Error, Retry"
  "Show Error, Retry" --> "Auto-detect / Browse / Manual"
endif

"Project Selection" --> "Open Folder Dialog"
"Open Folder Dialog" --> "Scan Project"
"Scan Project" --> if ".claude/ exists?" then
  -->[yes] "Show Scan Results"
  "Show Scan Results" --> "Open Workspace"
else
  -->[no] "Discovery Prompt"
  "Discovery Prompt" --> if "User starts discovery?" then
    -->[yes] "Discovery Conversation"
    "Discovery Conversation" --> "Propose Artifacts"
    "Propose Artifacts" --> "Confirm & Create Files"
    "Confirm & Create Files" --> "Open Workspace"
  else
    -->[no, skip] "Create Default Scaffold"
    "Create Default Scaffold" --> "Open Workspace"
  endif
endif
"Open Workspace" --> (*)
@enduml
```

---

## Element Descriptions

### Onboarding Screens

| Screen | Purpose | Exit Condition |
|--------|---------|----------------|
| **Welcome** | Brand introduction, single CTA to begin setup. | User clicks "Set Up Claude Code CLI". |
| **CLI Setup** | Configure the Claude Code CLI path. Auto-detect available. | Valid CLI detected and verified. |
| **Project Selection** | Pick a folder. Recent projects shown on return visits. | Folder selected via native dialog or recent list. |
| **Scan Results** | Display detected stack, file count, existing artifacts. For projects with existing `.claude/`. | User clicks "Open Workspace" or "Change Project". |
| **Discovery Prompt** | Offer project discovery conversation for projects without `.claude/`. Explains that OrqaStudio can learn about the project through conversation. | User clicks "Start Project Discovery" or "Skip -- Use Defaults". |
| **Discovery Conversation** | A regular conversation session in the main workspace where Claude asks about product, tech stack, team, and conventions. Not a special UI mode. | User completes discovery or says "that's enough". Claude proposes artifacts. |
| **Governance Confirmation** | Review proposed governance artifacts (from discovery or defaults) before writing to disk. User can uncheck individual files. | User confirms or cancels. |

### Settings Sections

| Section | Persisted To | Scope |
|---------|-------------|-------|
| **Provider** | App-level config (SQLite) | Global |
| **Project** | `.orqa/project.json` (file-based, per-project) | Project |
| **Appearance** | App-level config; per-project override if toggle enabled | Global / Project |
| **Keyboard Shortcuts** | Read-only in MVP | Global |

### State Indicators

| Indicator | Icon | Color | Meaning |
|-----------|------|-------|---------|
| Sidecar running | `<&circle-check>` | Green | Process alive, accepting commands |
| Sidecar idle | `<&clock>` | Neutral | Process alive, no active session |
| Sidecar error | `<&circle-x>` | Red | Process exited or unreachable |
| Connected | `<&signal>` | Green | IPC channel healthy |
| Disconnected | `<&ban>` | Red | IPC channel broken |

---

## Keyboard Navigation

| Shortcut | Action |
|----------|--------|
| `Ctrl+,` | Open settings in Explorer Panel |
| `Ctrl+O` | Open project (shows folder dialog) |
| `Ctrl+Shift+N` | New project (onboarding flow from project selection) |
| `Esc` | Close onboarding overlay / return to workspace |
| `Tab` | Move focus between form fields within settings |
| `Enter` | Activate focused button |
