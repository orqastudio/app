# Wireframe: Core Layout

**Date:** 2026-03-02 | **Informed by:** [Information Architecture](/product/information-architecture), [Frontend Research](/research/frontend), [Design System](/ui/design-system)

The main window structure showing all zones: Activity Bar, Nav Sub-Panel, Explorer Panel, and Chat Panel, plus Toolbar and Status Bar. This replaces the previous four-zone layout with a three-zone + nav sub-panel design (AD-019).

---

## Default State (All Zones Open)

```plantuml
@startsalt
{+
  {/ <b>Orqa Studio</b> | . | . | . | . | [Ctrl+K Search...] | . | . }
  {+
    {
      <&grid>
      ---
      <&document>
      ---
      <&person>
      ---
      <&shield>
      ---
      <&bolt>
      ---
      <&fork>
      ---
      .
      ---
      .
      ---
      .
      ---
      <&cog>
    } | {
      <b>Product</b>
      ---
      {SI
        . Vision
        . Governance
        . Personas
        . Journeys
      }
      ---
      <b>Architecture</b>
      ---
      {SI
        . Decisions
        . IPC Commands
      }
      ---
      .
    } | {
      [Filter docs...            ]
      ---
      {SI
        {
          <&document> <b>architecture-decisions</b>
          Formal architecture decisions log
        }
        ---
        {
          <&document> <b>coding-standards</b>
          Rust and TypeScript conventions
        }
        ---
        {
          <&document> <b>glossary</b>
          Domain model and terminology
        }
        ---
        .
        .
      }
      ---
      [+ New Doc]
    } | {
      { ^Fix auth bug   v^ | . | ^Auto (recommended)^ | 1.2k tokens }
      ---
      {SI
        {
          <&person> <b>You</b> | . | 14:32
          ---
          The login endpoint returns 401 even
          with valid credentials. Can you look
          at src/auth/handler.rs?
        }
        ---
        {
          <&chat> <b>Claude</b> | . | 14:32
          ---
          I'll look at the auth handler. Let me
          read the file first.
          ---
          {+
            {/ <b>Read</b> src/auth/handler.rs | ^Completed^ }
            .
          }
          ---
          The issue is on line 42...
        }
        ---
        .
      }
      ---
      {+ [Type a message...] | [Send] }
    }
  }
  { <&signal> Connected | . | Claude Code v1.2 | . | . | . | Sidecar: running }
}
@endsalt
```

### Zone Dimensions

| Zone | Default | Min | Max | Collapsible |
|------|---------|-----|-----|-------------|
| Activity Bar | 48px (fixed) | 48px | 48px | No |
| Nav Sub-Panel | 200px | 160px | 280px | Yes (collapse to 0px) |
| Explorer Panel | Flex (fills remaining) | 280px | — | No |
| Chat Panel | Flex (fills remaining) | 360px | — | No |
| Toolbar | Full width | — | — | No |
| Status Bar | Full width | — | — | No |

Nav Sub-Panel, Explorer, and Chat share remaining horizontal space after the Activity Bar (48px). When the Nav Sub-Panel is collapsed, its space redistributes to Explorer and Chat.

### Zone Relationship to PaneForge

The Activity Bar sits **outside** PaneForge as a fixed-width CSS flex element (48px). PaneForge manages the three resizable zones: Nav Sub-Panel | Explorer Panel | Chat Panel.

---

## Nav Sub-Panel Collapsed State

When the Nav Sub-Panel is collapsed via `Ctrl+B`, its space redistributes to the Explorer and Chat panels.

```plantuml
@startsalt
{+
  {/ <b>Orqa Studio</b> | . | . | . | . | . | [Ctrl+K Search...] | . | . }
  {+
    {
      <&grid>
      ---
      <&document>
      ---
      <&person>
      ---
      <&shield>
      ---
      <&bolt>
      ---
      <&fork>
      ---
      .
      ---
      .
      ---
      .
      ---
      <&cog>
    } | {
      [Filter docs...                     ]
      ---
      {SI
        {
          <&document> <b>architecture-decisions</b>
          Formal architecture decisions log
        }
        ---
        {
          <&document> <b>coding-standards</b>
          Rust and TypeScript conventions
        }
        ---
        {
          <&document> <b>glossary</b>
          Domain model and terminology
        }
        ---
        {
          <&document> <b>mvp-specification</b>
          Feature specs and acceptance criteria
        }
        ---
        .
        .
      }
      ---
      [+ New Doc]
    } | {
      { ^Fix auth bug   v^ | . | . | . | ^Auto (recommended)^ | 1.2k tokens }
      ---
      {SI
        {
          <&person> <b>You</b> | . | . | . | 14:32
          ---
          The login endpoint returns 401 even with valid
          credentials. Can you look at src/auth/handler.rs?
        }
        ---
        {
          <&chat> <b>Claude</b> | . | . | . | 14:32
          ---
          I'll look at the auth handler. Let me read the
          file first.
          ---
          {+
            {/ <b>Read</b> src/auth/handler.rs | . | ^Completed^ }
            .
          }
          ---
          The issue is on line 42. The password hash
          comparison uses == instead of constant-time...
        }
        ---
        .
      }
      ---
      {+ [Type a message... (Enter to send)] | . | [Send] }
    }
  }
  { <&signal> Connected | . | Claude Code v1.2 | . | . | . | . | . | Sidecar: running }
}
@endsalt
```

---

## Activity Bar: Agents Selected

When the user clicks the Agents icon in the Activity Bar, the Explorer Panel switches to show the Agents artifact list.

```plantuml
@startsalt
{+
  {/ <b>Orqa Studio</b> | . | . | . | . | [Ctrl+K Search...] | . | . }
  {+
    {
      <&grid>
      ---
      <&document>
      ---
      **<&person>**
      ---
      <&shield>
      ---
      <&bolt>
      ---
      <&fork>
      ---
      .
      ---
      .
      ---
      .
      ---
      <&cog>
    } | {
      [Filter agents...]
      ---
      {SI
        . backend-engineer
        . frontend-engineer
        . code-reviewer
        . debugger
        . test-engineer
        .
      }
    } | {
      [Filter agents...           ]
      ---
      {SI
        {
          <&person> <b>backend-engineer</b>
          Rust / Tauri v2, IPC commands
        }
        ---
        {
          <&person> <b>frontend-engineer</b>
          Svelte 5 / runes, shadcn-svelte
        }
        ---
        {
          <&person> <b>code-reviewer</b>
          clippy, ESLint, zero-error policy
        }
        ---
        .
        .
      }
      ---
      [+ New Agent]
    } | {
      { ^Fix auth bug   v^ | . | . | ^Auto (recommended)^ | 1.2k tokens }
      ---
      {SI
        {
          <&person> <b>You</b> | . | 14:32
          ---
          The login endpoint returns 401...
        }
        ---
        .
      }
      ---
      {+ [Type a message...] | [Send] }
    }
  }
  { <&signal> Connected | . | Claude Code v1.2 | . | . | . | Sidecar: running }
}
@endsalt
```

---

## Project Dashboard View

When the user clicks the Project Dashboard icon (top of Activity Bar), the Explorer Panel shows the project overview. The Nav Sub-Panel is hidden for this view.

```plantuml
@startsalt
{
  { <b>Project Dashboard</b> }
  ---
  {
    <b>orqa-studio</b>
    ~/code/orqa-studio
    ---
    <b>Stack:</b> Rust, TypeScript, Svelte
    <b>Frameworks:</b> Tauri v2, shadcn-svelte
    <b>Build:</b> Cargo, Vite
  }
  ---
  {
    <b>Governance Artifacts</b>
    ---
    <&document> Docs: 12 | <&document> Agents: 3
    <&document> Rules: 5 | <&document> Skills: 2
    <&document> Hooks: 1
  }
  ---
  {
    <b>Recent Sessions</b>
    ---
    Fix auth bug -- Today, 14:32
    Add user model -- Today, 10:15
    Initial setup -- Yesterday
  }
  ---
  {
    <b>Status</b>
    ---
    <&circle-check> Scanners: 7/9 pass >>
    <&signal> Metrics: 12.4 sessions/day >>
    <&book> Learning: 3 lessons, 1 promoted >>
  }
}
@endsalt
```

---

## Element Descriptions

### Toolbar

| Element | Behavior |
|---------|----------|
| **Project name** ("Orqa Studio") | Click opens project switcher dropdown. Shows current project name. |
| **Search** | `Ctrl+K` focuses. FTS5-powered search across sessions and artifacts. Results appear in Explorer Panel. |

Note: The settings gear is removed from the toolbar. Settings is now accessible via the Activity Bar (bottom icon) or `Ctrl+,`. New sessions are created via `Ctrl+N` or the session dropdown in the Chat Panel header.

### Activity Bar

| Element | Behavior |
|---------|----------|
| **Project Dashboard icon** | Top of Activity Bar. Click switches Explorer Panel to project dashboard. Nav Sub-Panel hidden. `Ctrl+0`. |
| **Artifact category icons** | Docs (default), Agents, Rules, Skills, Hooks. Click switches the Explorer Panel to that category's artifact list. Active icon has a 2px left border indicator + highlighted background. The Hooks icon surfaces both lifecycle hooks (`.claude/hooks/`) and hookify enforcement rules (`.claude/hookify.*.local.md`). |
| **Dashboard icons** | Scanners, Metrics, Learning (Phase 3-5). Click switches the Explorer Panel to the corresponding dashboard. |
| **Settings icon** | Bottom-aligned. Click switches the Explorer Panel to the settings view. `Ctrl+,`. |
| **Tooltips** | Each icon shows a tooltip on hover with the zone name and keyboard shortcut. |
| **Keyboard shortcuts** | `Ctrl+0` for Project Dashboard. `Ctrl+1` through `Ctrl+5` for artifact categories. `Ctrl+,` for settings. |

### Status Bar

| Element | Behavior |
|---------|----------|
| **Connection indicator** | Green dot = connected. Red dot = disconnected. Click to view connection details. |
| **Claude Code version** | Shows the detected CLI version. |
| **Sidecar status** | "running", "idle", "error". Shows current sidecar process state. |

### Resize Handles

PaneForge provides drag handles between the Nav Sub-Panel, Explorer, and Chat panes. Handles are 1px borders with an 8px invisible drag target. Double-click a handle to collapse/expand the Nav Sub-Panel.

---

## Keyboard Navigation

| Shortcut | Action |
|----------|--------|
| `Ctrl+0` | Project Dashboard |
| `Ctrl+1` through `Ctrl+5` | Switch artifact category |
| `Ctrl+B` | Toggle Nav Sub-Panel |
| `Ctrl+K` | Focus global search |
| `Ctrl+N` | New session |
| `Ctrl+,` | Open settings in Explorer Panel |
| `Tab` | Move focus between zones (Activity Bar > Nav Sub-Panel > Explorer > Chat) |
