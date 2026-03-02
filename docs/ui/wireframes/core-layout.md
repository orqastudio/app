# Wireframe: Core Layout

**Date:** 2026-03-02 | **Informed by:** [Information Architecture](/product/information-architecture), [Frontend Research](/research/frontend), [Design System](/ui/design-system)

The main window structure showing all three panes, toolbar, and status bar in their default state.

---

## Default State (All Panels Open)

```plantuml
@startsalt
{+
  {/ <b>Forge</b> | . | . | . | . | [Ctrl+K Search...] | . | [+ New Session] | [Settings] }
  {+
    {
      {/ <b>Sessions</b> | Project }
      {SI
        <b>Session: Fix auth bug</b>
        Today, 14:32 - 8 messages
        ---
        Session: Add user model
        Today, 10:15 - 23 messages
        ---
        Session: Initial setup
        Yesterday - 5 messages
        ---
        Session: Plan API routes
        Mar 1 - 12 messages
        ---
        .
        .
      }
      ---
      [Search sessions...]
    } | {
      { <b>Fix auth bug</b> | . | . | . | ^claude-4-opus^ | 1.2k tokens }
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
          The issue is on line 42. The password
          hash comparison uses == instead of
          constant-time comparison...
        }
        ---
        .
      }
      ---
      {+ [Type a message... (Enter to send, Shift+Enter for newline)] | [Send] }
    } | {
      {/ <b>Docs</b> | Agents | Rules | Skills | Hooks }
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
    }
  }
  { <&signal> Connected | . | Claude Code v1.2 | . | . | . | . | Sidecar: running }
}
@endsalt
```

### Panel Dimensions

| Pane | Default | Min | Max | Collapsible |
|------|---------|-----|-----|-------------|
| Sidebar | 240px | 180px | 320px | Yes (left edge) |
| Primary | Flex (fills remaining) | 400px | — | No |
| Detail | 360px | 280px | 480px | Yes (right edge) |
| Toolbar | Full width | — | — | No |
| Status Bar | Full width | — | — | No |

### Panel Collapse Behavior

When the sidebar collapses, it reduces to a narrow icon strip (48px) showing session/project tab icons. Click any icon to expand.

When the detail panel collapses, the primary panel expands to fill the space. The detail panel can be restored via `Ctrl+\` or clicking the settings gear / artifact link.

---

## Sidebar Collapsed State

```plantuml
@startsalt
{+
  {/ <b>Forge</b> | . | . | . | . | [Ctrl+K Search...] | . | [+ New Session] | [Settings] }
  {+
    {
      <&list>
      ---
      <&folder>
    } | {
      { <b>Fix auth bug</b> | . | . | . | . | ^claude-4-opus^ | 1.2k tokens }
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
          I'll look at the auth handler...
        }
        ---
        .
      }
      ---
      {+ [Type a message...] | [Send] }
    } | {
      {/ <b>Docs</b> | Agents | Rules | Skills | Hooks }
      ---
      {SI
        {
          <&document> <b>architecture-decisions</b>
          Formal architecture decisions log
        }
        ---
        .
      }
      ---
      [+ New Doc]
    }
  }
  { <&signal> Connected | . | . | . | . | . | . | Sidecar: running }
}
@endsalt
```

---

## Detail Panel Collapsed State

```plantuml
@startsalt
{+
  {/ <b>Forge</b> | . | . | . | . | . | [Ctrl+K Search...] | . | [+ New Session] | [Settings] }
  {+
    {
      {/ <b>Sessions</b> | Project }
      {SI
        <b>Session: Fix auth bug</b>
        Today, 14:32 - 8 messages
        ---
        Session: Add user model
        Today, 10:15 - 23 messages
        ---
        Session: Initial setup
        Yesterday - 5 messages
        ---
        .
        .
      }
      ---
      [Search sessions...]
    } | {
      { <b>Fix auth bug</b> | . | . | . | . | . | . | ^claude-4-opus^ | 1.2k tokens }
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
          comparison uses == instead of constant-time
          comparison. This is a security vulnerability.
        }
        ---
        .
      }
      ---
      {+ [Type a message... (Enter to send, Shift+Enter for newline)] | . | [Send] }
    }
  }
  { <&signal> Connected | . | Claude Code v1.2 | . | . | . | . | . | Sidecar: running }
}
@endsalt
```

---

## Sidebar: Project Tab

```plantuml
@startsalt
{
  {/ Sessions | <b>Project</b> }
  ---
  {
    <b>forge</b>
    ~/code/forge
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
    <b>Status</b>
    ---
    <&circle-check> Scanners: 7/9 pass >>
    <&signal> Metrics: 12.4 sessions/day >>
    <&book> Learning: 3 lessons, 1 promoted >>
  }
}
```

---

## Element Descriptions

### Toolbar

| Element | Behavior |
|---------|----------|
| **Project name** ("Forge") | Click opens project switcher dropdown. Shows current project name. |
| **Search** | `Ctrl+K` focuses. FTS5-powered search across sessions and artifacts. Results appear in detail panel. |
| **New Session** | Creates a new conversation session and focuses the input area. `Ctrl+N`. |
| **Settings** | Opens the settings view in the detail panel. |

### Status Bar

| Element | Behavior |
|---------|----------|
| **Connection indicator** | Green dot = connected. Red dot = disconnected. Click to view connection details. |
| **Claude Code version** | Shows the detected CLI version. |
| **Sidecar status** | "running", "idle", "error". Shows current sidecar process state. |

### Resize Handles

PaneForge provides drag handles between panes. Handles are 1px borders with an 8px invisible drag target. Double-click a handle to collapse/expand the adjacent pane.

---

## Keyboard Navigation

| Shortcut | Action |
|----------|--------|
| `Ctrl+B` | Toggle sidebar |
| `Ctrl+\` | Toggle detail panel |
| `Ctrl+K` | Focus global search |
| `Ctrl+N` | New session |
| `Tab` | Move focus between panes (Sidebar > Primary > Detail) |
