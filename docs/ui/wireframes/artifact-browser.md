# Wireframe: Artifact Browser

**Date:** 2026-03-02 | **Informed by:** [Information Architecture](/product/information-architecture), [Frontend Research](/research/frontend), [MVP Spec F-007, F-008](/product/mvp-specification)

The artifact browser surfaces governance artifacts (Docs, Agents, Rules, Skills, Hooks) in the detail panel: as a category list (browser) and as a rendered/editable view (viewer). Both views live in the detail panel so the conversation remains visible in the primary panel — the core workflow is collaborating with Claude *on* artifacts. Artifacts are Markdown files with YAML frontmatter stored in `.forge/` directories.

---

## 1. Detail Panel: Artifact Browser (Docs Tab — Default)

The default view when the detail panel shows the artifact browser. Docs is the default tab because documentation is the most frequently touched artifact category during active development. Agents, Rules, and other governance artifacts are primarily modified during retrospectives.

```plantuml
@startsalt
{
  {/ <b>Docs</b> | Agents | Rules | Skills | Hooks }
  ---
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
    {
      <&document> <b>mvp-specification</b>
      Feature specs and acceptance criteria
    }
    ---
    {
      <&document> <b>roadmap</b>
      Phase plan and feature backlog
    }
    ---
    .
    .
  }
  ---
  [+ New Doc]
}
@endsalt
```

### List Item Anatomy

| Element | Description |
|---------|-------------|
| **Icon** | Category-specific: `<&person>` agents, `<&shield>` rules, `<&bolt>` skills, `<&loop-circular>` hooks, `<&document>` docs |
| **Name** | Artifact filename without extension (e.g., `backend-engineer`) |
| **Description** | First line of the `description` frontmatter field, truncated to 2 lines |
| **Status indicator** | Colored dot from frontmatter `status`: green = `active`, yellow = `draft`, gray = `archived` |

### Interactions

| Action | Result |
|--------|--------|
| Click artifact row | Opens artifact viewer in the detail panel (replaces the browser list). Conversation stays visible in primary panel. |
| Click "Filter agents..." | Focuses input; filters list by name and description substring match |
| Click "+ New Agent" | Creates new artifact from category template, opens in editor mode |
| `Ctrl+\` | Collapses detail panel |

---

## 2. Detail Panel: Rules Tab with Path Scopes

The Rules tab shows rule artifacts with their applicable path scopes, helping users understand which rules apply where.

```plantuml
@startsalt
{
  {/ Docs | Agents | <b>Rules</b> | Skills | Hooks }
  ---
  [Filter rules...           ]
  ---
  {SI
    {
      <&shield> <b>code-style</b> | . | <&media-record> active
      Enforce consistent Rust and TS
      formatting conventions
      <i><&folder> **/*</i>
    }
    ---
    {
      <&shield> <b>error-handling</b> | . | <&media-record> active
      Use Result types, no unwrap()
      in production code
      <i><&folder> src-tauri/**/*.rs</i>
    }
    ---
    {
      <&shield> <b>component-patterns</b> | . | <&media-record> active
      Svelte 5 runes, $props(),
      snippet-based composition
      <i><&folder> src/lib/**/*.svelte</i>
    }
    ---
    {
      <&shield> <b>test-coverage</b> | . | <&circle-x> draft
      Minimum 80% coverage for new
      modules
      <i><&folder> **/tests/**</i>
    }
    ---
    {
      <&shield> <b>security-review</b> | . | <&media-record> active
      Auth and crypto require
      two-person review
      <i><&folder> src-tauri/src/auth/**</i>
    }
    ---
    .
    .
  }
  ---
  [+ New Rule]
}
@endsalt
```

### Path Scope Display

Rules include a `globs` field in frontmatter that controls where the rule applies. The path scope is shown in italics below the description as a folder icon followed by the glob pattern. Rules with `**/*` (global scope) may optionally hide the path indicator to reduce noise.

---

## 3. Detail Panel: Artifact Viewer (Rendered)

When clicking an artifact in the browser, the detail panel switches from the list to the artifact viewer. The conversation remains visible in the primary panel so the user can discuss the artifact with Claude. YAML frontmatter is displayed as structured metadata above the rendered Markdown body.

```plantuml
@startsalt
{+
  {
    { <&home> Project / Agents / <b>backend-engineer</b> | . | . | . | [Edit] }
  }
  ---
  {SI
    {+
      {
        <b>backend-engineer</b>
        ---
        {
          Status: <&media-record> active | . | Model: ^claude-4-opus^
        }
        {
          Trigger: manual | . | Version: 1.3
        }
      }
    }
    ---
    ---
    <b>System Prompt</b>
    .
    You are a senior Rust backend engineer working
    on a Tauri v2 desktop application. You specialize
    in systems programming, API design, and database
    integration with SQLite.
    .
    <b>Guidelines</b>
    .
    - Follow the project's error handling rules:
      use Result<T, ForgeError> for all fallible
      operations
    - Prefer async functions using tokio
    - Write integration tests for all new endpoints
    - Use sqlx for database queries with
      compile-time verification
    .
    <b>Context Files</b>
    .
    - src-tauri/src/lib.rs
    - src-tauri/src/database/mod.rs
    - src-tauri/src/api/mod.rs
    .
    <b>Example Interactions</b>
    .
    When asked to create a new API endpoint, always:
    1. Define the request/response types
    2. Add the Tauri command handler
    3. Register in the command list
    4. Write a test
    .
    .
  }
}
@endsalt
```

### Metadata Card Layout

| Element | Source | Display |
|---------|--------|---------|
| **Title** | Frontmatter `name` or filename | Large heading at top |
| **Status badge** | Frontmatter `status` | Colored dot + label |
| **Model** | Frontmatter `model` | Dropdown display (read-only in view mode) |
| **Trigger** | Frontmatter `trigger` | Text badge |
| **Version** | Frontmatter `version` | Text label |

Metadata fields vary by category. The viewer dynamically renders whatever frontmatter keys are present. The card uses a two-column layout for compactness.

### Interactions

| Action | Result |
|--------|--------|
| Click "Edit" button | Switches to source editing mode (State 4) |
| Click breadcrumb "Agents" | Returns to detail panel Agents tab |
| Click breadcrumb "Project" | Returns to project overview |
| Links in rendered Markdown | Open in default browser (external) or navigate (internal) |

---

## 4. Detail Panel: Artifact Editor (Source)

The source editing mode replaces the rendered view in the detail panel with a CodeMirror 6 editor. YAML frontmatter and Markdown syntax highlighting are provided. An unsaved changes indicator appears when the buffer differs from disk.

```plantuml
@startsalt
{+
  {
    { <&home> Project / Agents / <b>backend-engineer</b> | . | <&circle-check> Unsaved changes | [Save (Ctrl+S)] | [Cancel] }
  }
  ---
  {SI
    {+
      <code>
      ""---""
      name: backend-engineer
      status: active
      model: claude-4-opus
      trigger: manual
      version: 1.3
      ""---""
      .
      # System Prompt
      .
      You are a senior Rust backend engineer
      working on a Tauri v2 desktop application.
      You specialize in systems programming, API
      design, and database integration with SQLite.
      .
      # Guidelines
      .
      ""- Follow the project's error handling rules:""
      ""  use Result<T, ForgeError> for all""
      ""  fallible operations""
      ""- Prefer async functions using tokio""
      ""- Write integration tests for all new""
      ""  endpoints""
      ""- Use sqlx for database queries with""
      ""  compile-time verification""
      .
      # Context Files
      .
      ""- src-tauri/src/lib.rs""
      ""- src-tauri/src/database/mod.rs""
      ""- src-tauri/src/api/mod.rs""
      .
      .
    }
  }
}
@endsalt
```

### Editor Features

| Feature | Implementation |
|---------|---------------|
| **Syntax highlighting** | CodeMirror 6 with `@codemirror/lang-markdown` and `@codemirror/lang-yaml` (frontmatter region) |
| **Unsaved indicator** | Yellow dot + "Unsaved changes" text appears when buffer differs from last saved state |
| **Save** | `Ctrl+S` writes to disk via Tauri filesystem API. Indicator clears on success. |
| **Cancel** | Discards changes, reverts buffer to last saved state, returns to rendered view |
| **Line numbers** | Shown in left gutter |
| **Word wrap** | Enabled by default for Markdown content |
| **Bracket matching** | Auto-close for `()`, `[]`, `{}`, backticks |

### Editor Behavior

| Action | Result |
|--------|--------|
| `Ctrl+S` | Save file, show brief "Saved" toast, remain in editor |
| `Escape` or Cancel click | If unsaved changes: confirm dialog. If clean: return to rendered view. |
| Navigate away with unsaved changes | Confirm dialog: "Discard unsaved changes?" with Save / Discard / Cancel |
| `Ctrl+Z` / `Ctrl+Y` | Undo / Redo within editor session |

---

## 5. Detail Panel: Empty State

Shown when a category tab has no artifacts yet. Provides guidance and a clear call to action.

```plantuml
@startsalt
{
  {/ Docs | Agents | Rules | <b>Skills</b> | Hooks }
  ---
  [Filter skills...          ]
  ---
  {SI
    .
    .
    .
    {
      <&box>
      .
      <b>No skills yet</b>
      .
      Skills define reusable capabilities
      that agents can invoke during
      sessions. Create your first skill
      to get started.
    }
    .
    .
    .
  }
  ---
  [+ New Skill]
}
@endsalt
```

### Empty State Content by Category

| Category | Heading | Description |
|----------|---------|-------------|
| **Agents** | No agents yet | Agents define AI personas with specialized knowledge and behavior. Create your first agent to customize how Claude works on your project. |
| **Rules** | No rules yet | Rules enforce coding standards and project conventions. They are automatically applied based on file path globs. |
| **Skills** | No skills yet | Skills define reusable capabilities that agents can invoke during sessions. Create your first skill to get started. |
| **Hooks** | No hooks yet | Hooks run automated actions before or after AI operations, such as linting or testing. |
| **Docs** | No docs yet | Docs are reference documents that provide context to agents during sessions. Add documentation to improve AI responses. |

---

## Keyboard Navigation

| Shortcut | Context | Action |
|----------|---------|--------|
| `Up` / `Down` | Detail panel list | Navigate between artifacts |
| `Enter` | Detail panel list | Open selected artifact in detail panel viewer |
| `Ctrl+N` | Detail panel | Create new artifact in current category |
| `Ctrl+F` | Detail panel | Focus filter input |
| `Ctrl+E` | Artifact viewer | Toggle to editor mode |
| `Ctrl+S` | Artifact editor | Save changes |
| `Escape` | Artifact editor | Cancel editing (with confirm if unsaved) |
| `Tab` | Category tabs | Move between category tabs |

---

## Responsive Behavior

| Condition | Behavior |
|-----------|----------|
| Detail panel at minimum width (280px) | Description text truncated to 1 line; path scopes hidden. Artifact viewer/editor uses full available width. |
| Detail panel wider than 400px | Full 2-line descriptions; path scopes shown. Artifact viewer has room for metadata card 2-column layout. |
| Detail panel at maximum width (480px) | Rendered Markdown limited to panel width for readability. Editor has comfortable editing width. |

---

## File Watching

Artifacts are stored as files on disk (`.forge/agents/*.md`, `.forge/rules/*.md`, etc.). The detail panel list uses Tauri's `fs.watch` API to live-reload when files change externally (e.g., via git pull or direct editing). A brief fade animation indicates a refresh.
