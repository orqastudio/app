---
id: DOC-016
title: Svelte Component Tree
description: Component tree structure and hierarchy of the Svelte 5 frontend application.
created: "2026-03-02"
updated: "2026-03-04"
---

**Date:** 2026-03-02 | **Updated:** 2026-03-04 | **Status:** Aligned with Phase 1 implementation | **References:** [Frontend Research](RES-004), [Design Tokens Research](RES-003), [Information Architecture](DOC-040)

Defines the complete Svelte component architecture for OrqaStudio™: directory layout, route structure, component hierarchy, store design, command mapping, data flow, streaming integration, and theme integration.

---

## Directory Structure

```
ui/
├── app.html                          # Tauri entry HTML
├── app.css                           # Global styles (Tailwind directives, CSS variables)
├── app.d.ts                          # Global TypeScript declarations
├── routes/
│   ├── +layout.svelte                # Root layout — ThemeProvider, global keybindings
│   └── +page.svelte                  # Single page — AppLayout container
├── lib/
│   ├── components/
│   │   ├── ui/                       # shadcn-svelte primitives (auto-generated)
│   │   │   ├── button/
│   │   │   ├── input/
│   │   │   ├── textarea/
│   │   │   ├── label/
│   │   │   ├── checkbox/
│   │   │   ├── select/
│   │   │   ├── separator/
│   │   │   ├── resizable/            # PaneForge wrapper
│   │   │   ├── scroll-area/
│   │   │   ├── tabs/
│   │   │   ├── collapsible/
│   │   │   ├── sidebar/
│   │   │   ├── badge/
│   │   │   ├── card/
│   │   │   ├── table/
│   │   │   ├── tooltip/
│   │   │   ├── dialog/
│   │   │   ├── popover/
│   │   │   ├── sheet/
│   │   │   ├── command/
│   │   │   ├── dropdown-menu/
│   │   │   ├── alert/
│   │   │   └── sonner/
│   │   ├── layout/                   # Application shell components
│   │   │   ├── AppLayout.svelte
│   │   │   ├── ActivityBar.svelte
│   │   │   ├── ActivityBarItem.svelte
│   │   │   ├── NavSubPanel.svelte
│   │   │   ├── Toolbar.svelte
│   │   │   ├── StatusBar.svelte
│   │   │   └── WelcomeScreen.svelte
│   │   ├── conversation/             # Conversation domain
│   │   │   ├── ConversationView.svelte
│   │   │   ├── SessionHeader.svelte
│   │   │   ├── MessageBubble.svelte
│   │   │   ├── UserMessage.svelte
│   │   │   ├── AssistantMessage.svelte
│   │   │   ├── SystemMessage.svelte
│   │   │   ├── MessageInput.svelte
│   │   │   └── StreamingIndicator.svelte
│   │   ├── shared/                   # Shared reusable components
│   │   │   ├── EmptyState.svelte
│   │   │   ├── ErrorDisplay.svelte
│   │   │   ├── LoadingSpinner.svelte
│   │   │   ├── MetadataRow.svelte
│   │   │   ├── SearchInput.svelte
│   │   │   ├── SelectMenu.svelte
│   │   │   └── SmallBadge.svelte
│   │   ├── tool/                     # Tool call domain
│   │   │   └── ToolCallCard.svelte
│   │   ├── content/                  # Content rendering domain
│   │   │   ├── MarkdownRenderer.svelte
│   │   │   └── CodeBlock.svelte
│   │   ├── artifact/                 # Artifact domain
│   │   │   ├── ArtifactLanding.svelte     # Landing page for artifact categories
│   │   │   ├── ArtifactViewer.svelte
│   │   │   ├── AgentViewer.svelte         # Specialized viewer for agent definitions
│   │   │   ├── HookViewer.svelte          # Specialized viewer for hooks
│   │   │   ├── SkillViewer.svelte         # Specialized viewer for skills
│   │   │   └── Breadcrumb.svelte
│   │   ├── navigation/               # Navigation domain
│   │   │   ├── DocTreeNav.svelte
│   │   │   ├── ArtifactListNav.svelte
│   │   │   └── SettingsCategoryNav.svelte  # Settings category navigation
│   │   ├── dashboard/                # Dashboard domain
│   │   │   └── ProjectDashboard.svelte
│   │   └── settings/                 # Settings domain
│   │       ├── SettingsView.svelte
│   │       ├── ProjectSetupWizard.svelte       # First-run project configuration wizard
│   │       ├── ProjectGeneralSettings.svelte   # Project name, description, icon
│   │       ├── ProjectGovernanceSettings.svelte # Governance artifact management
│   │       └── ProjectScanningSettings.svelte  # Codebase scan configuration
│   ├── stores/                       # Svelte 5 $state stores
│   │   ├── session.svelte.ts         # Active session, session list
│   │   ├── navigation.svelte.ts      # Activity bar selection, nav sub-panel state
│   │   ├── conversation.svelte.ts    # Messages, streaming accumulation
│   │   ├── artifact.svelte.ts        # Selected artifact, artifact index
│   │   ├── project.svelte.ts         # Project metadata, governance counts
│   │   └── settings.svelte.ts        # App and project settings
│   ├── commands/                     # Tauri invoke() wrappers
│   │   ├── session.ts                # Session CRUD commands
│   │   ├── conversation.ts           # Send message, stream setup
│   │   ├── artifact.ts               # Artifact CRUD commands
│   │   ├── project.ts                # Project scan, metadata
│   │   ├── sidecar.ts                # Sidecar lifecycle commands
│   │   └── settings.ts               # Settings read/write
│   ├── types/                        # TypeScript type definitions
│   │   ├── session.ts
│   │   ├── message.ts
│   │   ├── artifact.ts
│   │   ├── provider-event.ts         # ProviderEvent enum mirror
│   │   └── project.ts
│   └── utils/                        # Pure utility functions
│       ├── keybindings.ts            # Keyboard shortcut registration
│       ├── markdown.ts               # Markdown processing helpers
│       └── format.ts                 # Date, token count formatting
└── static/
    └── favicon.png
```

---

## Route Structure

OrqaStudio is a desktop application running on SvelteKit's static adapter. There is a single route; all navigation is state-driven through Svelte stores. No client-side routing or URL changes occur.

```
routes/
├── +layout.svelte        # Root layout
│   Responsibilities:
│   - Wraps app in ThemeProvider (mode-watcher)
│   - Registers global keyboard shortcuts ($effect)
│   - Initializes sidecar connection on mount
│   - Provides Sonner toast container
│
└── +page.svelte           # Single application page (container)
    Responsibilities:
    - Reads navigation stores to determine visible views
    - Calls invoke() to load initial data (active session, project metadata)
    - Passes data as props to AppLayout
    - Subscribes to Tauri Channel<T> for streaming events
    - This is the PRIMARY data-fetching boundary (AD-006)
```

**Why a single route:** SvelteKit is used as the build toolchain (Vite, HMR, adapter-static), not for its routing capabilities. OrqaStudio's navigation model (Activity Bar + panel switching) does not map to URL paths. A single `+page.svelte` acts as the application container and data-fetching boundary.

---

## Component Hierarchy

### Full Tree

Indentation shows parent-child nesting. Components prefixed with `ui:` are shadcn-svelte primitives imported from `$lib/components/ui/`. All others are custom components from `$lib/components/`.

```
+layout.svelte
└── ThemeProvider (mode-watcher)
    ├── Sonner (toast container)
    └── +page.svelte [CONTAINER — fetches data, subscribes to streams]
        └── AppLayout
            ├── Toolbar
            │   ├── ui:Command (global search, Ctrl+K)
            │   └── ui:Button (new session)
            │
            ├── [Activity Bar — fixed 48px, outside PaneForge]
            │   └── ActivityBar
            │       ├── ActivityBarItem (Project Dashboard — top)
            │       ├── ActivityBarItem (Docs — default active)
            │       ├── ActivityBarItem (Agents)
            │       ├── ActivityBarItem (Rules)
            │       ├── ActivityBarItem (Skills)
            │       ├── ActivityBarItem (Hooks — lifecycle hooks + hookify enforcement rules)
            │       ├── ui:Separator
            │       ├── ActivityBarItem (Scanners — Phase 3+)
            │       ├── ActivityBarItem (Metrics — Phase 5)
            │       ├── ActivityBarItem (Learning — Phase 5)
            │       ├── ui:Separator
            │       └── ActivityBarItem (Settings — bottom-aligned)
            │
            ├── ui:Resizable (three-zone container)
            │   │
            │   ├── [Explorer Pane — always visible, artifact-centric]
            │   │   ├── [when activeActivity is artifact category]
            │   │   │   ├── [when explorerView === "artifact-list"]
            │   │   │   │   └── ArtifactLanding
            │   │   │   │       ├── EmptyState (when no artifacts)
            │   │   │   │       └── [artifact category landing content]
            │   │   │   │
            │   │   │   └── [when explorerView === "artifact-viewer"]
            │   │   │       └── ArtifactViewer / AgentViewer / HookViewer / SkillViewer
            │   │   │           ├── Breadcrumb
            │   │   │           ├── MarkdownRenderer (rendered view)
            │   │   │           └── MetadataRow (metadata display)
            │   │   │
            │   │   ├── [when activeActivity === "settings"]
            │   │   │   └── SettingsView
            │   │   │       ├── ProjectSetupWizard (first-run)
            │   │   │       ├── ProjectGeneralSettings
            │   │   │       ├── ProjectGovernanceSettings
            │   │   │       └── ProjectScanningSettings
            │   │   │
            │   │   ├── [when activeActivity === "scanners"]
            │   │   │   └── ScannerDashboard (Phase 3+)
            │   │   │
            │   │   ├── [when activeActivity === "metrics"]
            │   │   │   └── MetricsDashboard (Phase 5)
            │   │   │
            │   │   └── [when activeActivity === "project-dashboard"]
            │   │       └── ProjectDashboard
            │   │           ├── ui:Card (project metadata)
            │   │           ├── ui:Badge (stack tags)
            │   │           └── [governance summary + quick links]
            │   │
            │   ├── [Nav Sub-Panel — collapsible via Ctrl+B]
            │   │   └── NavSubPanel
            │   │       ├── [when activeActivity === "docs"]
            │   │       │   └── DocTreeNav
            │   │       │       ├── ui:Collapsible (section groups)
            │   │       │       └── ui:ScrollArea
            │   │       │
            │   │       ├── [when activeActivity is artifact category (agents/rules/skills/hooks)]
            │   │       │   └── ArtifactListNav
            │   │       │       ├── SearchInput (search filter)
            │   │       │       └── ui:ScrollArea
            │   │       │
            │   │       ├── [when activeActivity === "settings"]
            │   │       │   └── SettingsCategoryNav
            │   │       │
            │   │       └── [when activeActivity is dashboard — hidden]
            │   │
            │   └── [Chat Pane — always conversation]
            │       ├── [when activeSession exists]
            │       │   └── ConversationView
            │       │       ├── SessionHeader
            │       │       │   ├── ui:Input (editable title)
            │       │       │   ├── ui:Select (model selector: Auto | Opus | Sonnet | Haiku)
            │       │       │   └── ui:Badge (token usage)
            │       │       ├── ui:ScrollArea (message stream)
            │       │       │   ├── UserMessage (repeated)
            │       │       │   │   └── MessageBubble
            │       │       │   │       ├── MarkdownRenderer
            │       │       │   │       │   ├── CodeBlock
            │       │       │   │       │   │   └── ui:Button (copy)
            │       │       │   │       │   └── [custom renderers]
            │       │       │   │       └── ui:Tooltip (timestamp)
            │       │       │   ├── AssistantMessage (repeated)
            │       │       │   │   └── MessageBubble
            │       │       │   │       ├── MarkdownRenderer
            │       │       │   │       └── ToolCallCard (inline, repeated)
            │       │       │   ├── SystemMessage (repeated)
            │       │       │   └── StreamingIndicator (when streaming)
            │       │       └── MessageInput
            │       │           ├── ui:Textarea
            │       │           └── ui:Button (send)
            │       │
            │       └── [when no activeSession]
            │           └── WelcomeScreen
            │               ├── ui:Card (branding, guidance)
            │               └── ui:Button (start conversation)
            │
            └── StatusBar
                └── ui:Badge (connection status, sidecar state, version)
```

### Component Relationships Summary

| Parent | Children | Relationship |
|--------|----------|-------------|
| `+page.svelte` | `AppLayout` | Single child, passes all data as props |
| `AppLayout` | `Toolbar`, `ActivityBar`, `ui:Resizable`, `StatusBar` | Shell composition (Activity Bar outside PaneForge) |
| `ActivityBar` | `ActivityBarItem` (repeated) | Icon rail with active state |
| `NavSubPanel` | `DocTreeNav`, `ArtifactListNav` | Per-category navigation container |
| `ConversationView` | `SessionHeader`, `ui:ScrollArea`, `MessageInput` | Vertical stack |
| `SessionHeader` | `ui:Input`, `ui:Select`, `ui:Badge` | Session context + model selection |
| `AssistantMessage` | `MessageBubble`, `ToolCallCard` | Message with inline tool calls |
| `ToolCallCard` | (self-contained) | Tool call display |
| `ArtifactViewer` | `Breadcrumb`, `MarkdownRenderer` | View/edit toggle |
| `ArtifactLanding` | (self-contained) | Landing page for artifact categories |
| `SettingsView` | `ProjectSetupWizard`, `ProjectGeneralSettings`, `ProjectGovernanceSettings`, `ProjectScanningSettings` | Settings sections |

---

## Store Architecture

All stores use Svelte 5 runes exclusively ([AD-004](AD-004)). Store files use the `.svelte.ts` extension to enable runes outside of components.

### session.svelte.ts

```typescript
// Active session and session list
class SessionStore {
  sessions = $state<Session[]>([]);
  activeSessionId = $state<string | null>(null);
  sessionDropdownOpen = $state(false);
  sessionSearchFilter = $state("");

  activeSession = $derived(
    this.sessions.find(s => s.id === this.activeSessionId) ?? null
  );

  setActive(id: string) { this.activeSessionId = id; }
  updateList(sessions: Session[]) { this.sessions = sessions; }
}

export const sessionStore = new SessionStore();
```

### navigation.svelte.ts

```typescript
// UI navigation state — what is visible in each zone
type ActivityBarItem = "project-dashboard" | "docs" | "agents" | "rules" | "skills" | "hooks"
  | "scanners" | "metrics" | "learning" | "settings";

type ExplorerView = "artifact-list" | "artifact-viewer" | "artifact-editor"
  | "project-dashboard" | "scanner-dashboard" | "metrics-dashboard" | "learning-loop" | "settings";

class NavigationStore {
  activeActivity = $state<ActivityBarItem>("docs");
  explorerView = $state<ExplorerView>("artifact-list");
  selectedArtifact = $state<string | null>(null);
  navPanelCollapsed = $state(false);

  switchActivity(item: ActivityBarItem) {
    this.activeActivity = item;
    // Map activity bar items to explorer views
    if (["docs", "agents", "rules", "skills", "hooks"].includes(item)) {
      this.explorerView = "artifact-list";
    } else if (item === "project-dashboard") {
      this.explorerView = "project-dashboard";
    } else if (item === "scanners") {
      this.explorerView = "scanner-dashboard";
    } else if (item === "metrics") {
      this.explorerView = "metrics-dashboard";
    } else if (item === "learning") {
      this.explorerView = "learning-loop";
    } else if (item === "settings") {
      this.explorerView = "settings";
    }
    this.selectedArtifact = null;
  }

  openArtifact(path: string) {
    this.selectedArtifact = path;
    this.explorerView = "artifact-viewer";
  }

  toggleNavPanel() {
    this.navPanelCollapsed = !this.navPanelCollapsed;
  }
}

export const navigationStore = new NavigationStore();
```

### conversation.svelte.ts

```typescript
// Message list and streaming state for the active conversation
class ConversationStore {
  messages = $state<Message[]>([]);
  isStreaming = $state(false);
  streamingContent = $state("");
  streamingThinking = $state("");
  /** The model actually being used for the current stream.
   *  Distinct from the session's `model` field (which may be "auto").
   *  Updated when streaming begins and the sidecar reports the resolved model
   *  via the StreamStart event's resolved_model field or a ModelResolved event.
   *  When model is pinned (not auto), this equals the pinned model name. */
  resolvedModel = $state<string | null>(null);

  messageCount = $derived(this.messages.length);

  // Append a completed message
  pushMessage(message: Message) {
    this.messages.push(message);
  }

  // Accumulate streaming text delta (called per Channel<T> event)
  appendStreamDelta(delta: string) {
    this.streamingContent += delta;
  }

  appendThinkingDelta(delta: string) {
    this.streamingThinking += delta;
  }

  // Finalize streaming into a committed message
  finalizeStream(message: Message) {
    this.isStreaming = false;
    this.streamingContent = "";
    this.streamingThinking = "";
    this.messages.push(message);
  }

  // Set the resolved model (called when sidecar reports model_resolved or stream_start with resolved_model)
  setResolvedModel(model: string) {
    this.resolvedModel = model;
  }

  // Replace entire message list (session switch)
  loadMessages(messages: Message[]) {
    this.messages = messages;
    this.isStreaming = false;
    this.streamingContent = "";
    this.resolvedModel = null;
  }
}

export const conversationStore = new ConversationStore();
```

### artifact.svelte.ts

```typescript
// Selected artifact and category-indexed artifact list
class ArtifactStore {
  selectedPath = $state<string | null>(null);
  // hooks category includes both lifecycle hooks (.orqa/hooks/) and hookify
  // enforcement rules (.orqa/hookify/), distinguished by hook_kind
  artifacts = $state<Record<ArtifactCategory, ArtifactMeta[]>>({
    agents: [], rules: [], skills: [], hooks: [], docs: []
  });
  isEditing = $state(false);

  selectedArtifact = $derived(
    this.selectedPath
      ? Object.values(this.artifacts).flat().find(a => a.path === this.selectedPath) ?? null
      : null
  );

  select(path: string) { this.selectedPath = path; }
  toggleEdit() { this.isEditing = !this.isEditing; }
  updateIndex(category: ArtifactCategory, items: ArtifactMeta[]) {
    this.artifacts[category] = items;
  }
}

export const artifactStore = new ArtifactStore();
```

### project.svelte.ts

```typescript
// Project metadata and governance summary
class ProjectStore {
  name = $state("");
  rootPath = $state("");
  detectedStack = $state<string[]>([]);
  governanceCounts = $state<Record<ArtifactCategory, number>>({
    agents: 0, rules: 0, skills: 0, hooks: 0, docs: 0
  });

  hasGovernance = $derived(
    Object.values(this.governanceCounts).some(c => c > 0)
  );

  load(project: ProjectMeta) {
    this.name = project.name;
    this.rootPath = project.rootPath;
    this.detectedStack = project.detectedStack;
    this.governanceCounts = project.governanceCounts;
  }
}

export const projectStore = new ProjectStore();
```

### settings.svelte.ts

```typescript
// App and project settings
class SettingsStore {
  // Settings state managed here
  // Replaces the spec'd sidecar.svelte.ts and theme.svelte.ts
}

export const settingsStore = new SettingsStore();
```

> **Phase 0e stores not implemented:** `sidecar.svelte.ts` and `theme.svelte.ts` do not exist as separate files. Their concerns are handled by `settings.svelte.ts` and inline component state respectively.

---

## Component-to-Command Mapping

Per [AD-006](AD-006), only containers and pages call `invoke()`. Display components receive data via `$props()`.

### Data-Fetching Boundary: `+page.svelte`

`+page.svelte` is the primary container. It calls Tauri commands and distributes results to stores, which components read via props or direct store access.

| Command | Tauri `invoke()` | Store Updated | Triggered By |
|---------|------------------|---------------|-------------|
| Load session list | `invoke("list_sessions", { projectId })` | `sessionStore.sessions` | App mount, project switch |
| Load session messages | `invoke("get_session_messages", { sessionId })` | `conversationStore.messages` | Session selection |
| Send message | `invoke("send_message", { sessionId, content, onEvent })` | `conversationStore` (via Channel) | MessageInput submit |
| Create session | `invoke("create_session", { projectId })` | `sessionStore` | Toolbar new session button |
| Load project metadata | `invoke("get_project_metadata", { path })` | `projectStore` | App mount, project switch |
| List artifacts | `invoke("list_artifacts", { projectId, category })` | `artifactStore.artifacts` | Artifact browser tab change |
| Get artifact content | `invoke("get_artifact", { path })` | `artifactStore` | ArtifactListNav item click |
| Save artifact | `invoke("save_artifact", { path, content })` | `artifactStore` | ArtifactViewer save |
| Start sidecar | `invoke("start_sidecar")` | `settingsStore` | App mount |
| Get settings | `invoke("get_settings")` | Settings props | Settings view open |
| Save settings | `invoke("save_settings", { settings })` | — | Settings form save |
| Update session title | `invoke("update_session_title", { sessionId, title })` | `sessionStore` | SessionHeader title edit |
| Search | `invoke("search", { query })` | Search results (local) | Command palette submit |
| Scan project | `invoke("scan_project", { path })` | `projectStore` | ProjectSettings rescan |

### Container vs. Display Component Rules

```
CONTAINERS (may call invoke):          DISPLAY COMPONENTS (props only):
─────────────────────────────          ─────────────────────────────────
+page.svelte                           AppLayout
+layout.svelte                         ActivityBar / ActivityBarItem
                                       NavSubPanel
                                       Toolbar / StatusBar
                                       ConversationView / SessionHeader
                                       MessageBubble / UserMessage / AssistantMessage / SystemMessage
                                       MessageInput (dispatches events up, does NOT invoke)
                                       ToolCallCard
                                       MarkdownRenderer / CodeBlock
                                       ArtifactLanding / ArtifactViewer
                                       AgentViewer / HookViewer / SkillViewer
                                       Breadcrumb
                                       DocTreeNav / ArtifactListNav / SettingsCategoryNav
                                       ProjectDashboard
                                       SettingsView / ProjectSetupWizard
                                       ProjectGeneralSettings / ProjectGovernanceSettings
                                       ProjectScanningSettings
                                       EmptyState / ErrorDisplay / LoadingSpinner
                                       MetadataRow / SearchInput / SelectMenu / SmallBadge
                                       WelcomeScreen / StreamingIndicator
```

Display components communicate user intent upward via callback props (e.g., `onSend`, `onSelectSession`, `onSaveArtifact`). The container (`+page.svelte`) receives these callbacks and calls the appropriate `invoke()` command.

---

## Data Flow

### Request/Response Flow (invoke)

```
User Action
  │
  ▼
Display Component ──callback prop──▶ +page.svelte (container)
                                        │
                                        ▼
                                    invoke("command", args)
                                        │
                                        ▼ (IPC)
                                    Rust #[tauri::command]
                                        │
                                        ▼
                                    Domain logic (src-tauri/src/domain/)
                                        │
                                        ▼
                                    Result<T, E>
                                        │
                                        ▼ (IPC return)
                                    +page.svelte
                                        │
                                        ▼
                                    Store update ($state mutation)
                                        │
                                        ▼ (Svelte 5 fine-grained reactivity)
                                    Component re-renders (only affected DOM nodes)
```

### Event Flow (emit/listen — low-frequency)

Used for file system changes, sidecar lifecycle, and app-level notifications.

```
Rust Backend
  │
  ├── app_handle.emit("sidecar:status", payload)
  ├── app_handle.emit("fs:artifact-changed", payload)
  └── app_handle.emit("project:scan-complete", payload)
        │
        ▼
    +layout.svelte / +page.svelte
        │  listen("sidecar:status", handler)
        │  listen("fs:artifact-changed", handler)
        ▼
    Store update
        │
        ▼
    Component re-renders
```

### Streaming Flow (Channel<T> — high-frequency)

Used for Claude token streaming. See next section for detail.

```
Claude API (SSE)
  │
  ▼
TypeScript Sidecar (Agent SDK)
  │  Translates to ProviderEvent NDJSON
  ▼  stdout
Rust Backend
  │  Parses NDJSON, deserializes to ProviderEvent
  │  Sends via Channel<T>
  ▼
+page.svelte (Channel callback)
  │
  ▼
conversationStore.$state mutation
  │  streamingContent += delta
  ▼
ConversationView / AssistantMessage re-render
  (fine-grained: only the streaming text node updates)
```

---

## Streaming Integration

### Channel Setup

When the user sends a message, `+page.svelte` creates a Tauri `Channel<ProviderEvent>` and passes it to the `send_message` command. The Rust backend writes streaming events into the channel as they arrive from the sidecar.

```typescript
// In +page.svelte — the data-fetching container
import { Channel } from "@tauri-apps/api/core";

async function handleSendMessage(content: string) {
  conversationStore.isStreaming = true;
  conversationStore.streamingContent = "";

  const channel = new Channel<ProviderEvent>();

  channel.onmessage = (event: ProviderEvent) => {
    switch (event.type) {
      case "stream_start":
        // When auto model selection is active, resolved_model reports the actual model chosen
        if (event.resolvedModel) {
          conversationStore.setResolvedModel(event.resolvedModel);
        }
        break;
      case "model_resolved":
        // Explicit model resolution event (auto model selection)
        conversationStore.setResolvedModel(event.resolvedModel);
        break;
      case "text_delta":
        conversationStore.appendStreamDelta(event.delta);
        break;
      case "thinking_delta":
        conversationStore.appendThinkingDelta(event.delta);
        break;
      case "tool_use_start":
        // Add pending tool call to message
        conversationStore.pushToolCall(event.toolCall);
        break;
      case "tool_result":
        // Update tool call with result
        conversationStore.updateToolCall(event.toolCallId, event.result);
        break;
      case "message_complete":
        conversationStore.finalizeStream(event.message);
        break;
      case "error":
        conversationStore.isStreaming = false;
        // Surface error via toast
        break;
    }
  };

  await invoke("send_message", {
    sessionId: sessionStore.activeSessionId,
    content,
    onEvent: channel,
  });
}
```

### ProviderEvent Types

The `ProviderEvent` discriminated union mirrors the Rust enum ([AD-009](AD-009), [AD-017](AD-017)):

```typescript
// $lib/types/provider-event.ts
type ProviderEvent =
  | { type: "text_delta"; delta: string }
  | { type: "thinking_delta"; delta: string }
  | { type: "tool_use_start"; toolCall: ToolCallStart }
  | { type: "tool_use_delta"; toolCallId: string; delta: string }
  | { type: "tool_result"; toolCallId: string; result: ToolResult }
  | { type: "message_complete"; message: Message }
  | { type: "stream_start"; messageId: string }
  | { type: "stream_end" }
  | { type: "error"; code: string; message: string };
```

### Reactivity Path

```
Channel<T>.onmessage
  │
  ▼
conversationStore.streamingContent += delta    ← $state mutation
  │
  ▼ (Svelte 5 fine-grained reactivity)
AssistantMessage reads conversationStore.streamingContent
  │
  ▼
MarkdownRenderer receives updated text
  │
  ▼
Only the text node in the DOM updates (no full re-render)
```

Svelte 5's fine-grained reactivity ensures that appending a token to `streamingContent` only updates the specific text node in the DOM, not the entire message list. This is critical for smooth streaming at 30-100ms per token.

### Auto-Scroll Behavior

```typescript
// ConversationView.svelte — display component
let { messages, isStreaming, streamingContent } = $props();
let scrollContainer: HTMLElement;
let userScrolledUp = $state(false);

// Auto-scroll to bottom during streaming unless user scrolled up
$effect(() => {
  if (isStreaming && !userScrolledUp) {
    streamingContent; // track dependency
    scrollContainer?.scrollTo({ top: scrollContainer.scrollHeight });
  }
});
```

---

## Theme Integration

### Architecture

Theme management uses three layers:

1. **mode-watcher** — manages light/dark/system mode at the document level
2. **shadcn-svelte CSS variables** — define the color palette per mode in `app.css`
3. **Project design tokens** — optional per-project color overrides loaded from `.orqa/settings.json`

### Integration Flow

```
+layout.svelte
  │
  ├── ModeWatcher (from mode-watcher)
  │   Reads: system/user mode preference
  │   Sets: document class ("light" | "dark")
  │   Writes: CSS variables on :root
  │
  └── $effect: apply project tokens
      Reads: settingsStore (project token overrides)
      Sets: CSS custom properties on :root (overrides shadcn defaults)
```

> **Note:** There is no standalone `ThemeToggle` component or `themeStore` in Phase 1. Theme mode is managed by `mode-watcher` and project design tokens are handled through `settingsStore`.

### CSS Variable Strategy

```css
/* app.css — shadcn-svelte generated, extended with OrqaStudio tokens */
:root {
  /* shadcn-svelte base palette */
  --background: 0 0% 100%;
  --foreground: 222.2 84% 4.9%;
  /* ... full shadcn palette ... */

  /* OrqaStudio semantic tokens */
  --orqa-message-user: var(--primary);
  --orqa-message-assistant: var(--muted);
  --orqa-tool-pending: var(--warning);
  --orqa-tool-complete: var(--success);
  --orqa-tool-error: var(--destructive);
  --orqa-streaming-cursor: var(--primary);
}

.dark {
  --background: 222.2 84% 4.9%;
  --foreground: 210 40% 98%;
  /* ... dark overrides ... */
}
```

### Project Token Override

When project design tokens are enabled via settings, project-specific design tokens are applied as CSS variable overrides on `:root`, taking precedence over the default shadcn-svelte palette. This allows projects to carry branded color schemes.

```typescript
// In +layout.svelte — conceptual pattern
$effect(() => {
  if (settingsStore.useProjectTokens && settingsStore.projectTokens) {
    const root = document.documentElement;
    for (const [key, value] of Object.entries(settingsStore.projectTokens)) {
      root.style.setProperty(`--${key}`, value);
    }
  }
});
```

---

## Conventions Summary

| Convention | Rule | Reference |
|-----------|------|-----------|
| Runes only | `$state`, `$derived`, `$effect`, `$props` everywhere. No `export let`, no `$:`, no `<slot>`. | [AD-004](AD-004) |
| Component purity | `$lib/components/` components are props-only. No `invoke()` inside them. | [AD-006](AD-006) |
| Data fetching | `+page.svelte` and `+layout.svelte` are the only files that call `invoke()`. | [AD-006](AD-006) |
| Callback props | Display components emit user intent via callback props (`onSend`, `onSelect`, etc.). | [AD-006](AD-006) |
| shadcn-svelte imports | `import { Button } from "$lib/components/ui/button"` | [AD-013](AD-013) |
| Activity Bar | 48px fixed icon rail controls Explorer Panel content. Active icon has left border indicator. Hooks icon covers both lifecycle hooks (`.orqa/hooks/`) and hookify enforcement rules (`.orqa/hookify/`). | [AD-019](AD-019) |
| Custom components | Organized by domain: `conversation/`, `tool/`, `content/`, `artifact/`, `navigation/`, `settings/`, `layout/` (includes `ActivityBar.svelte`, `ActivityBarItem.svelte`, `NavSubPanel.svelte`) | — |
| Store files | `.svelte.ts` extension, class-based with `$state` fields, exported singleton | [AD-004](AD-004) |
| Type safety | All `invoke()` calls use typed wrappers from `$lib/commands/` | [AD-002](AD-002) |
| Streaming | `Channel<T>` for high-frequency token streams, `emit/listen` for low-frequency events | [AD-009](AD-009) |
| Snippets over slots | Use `{#snippet}` and `{@render}` instead of `<slot>` for component composition | [AD-004](AD-004) |

---

## Related Documents

- [Component Inventory](DOC-048) — Complete list of all components
- [Information Architecture](DOC-040) — Layout model and navigation
- [Architecture Decisions](DOC-001) — [AD-004](AD-004) (runes), [AD-006](AD-006) (purity), [AD-009](AD-009) (streaming)
- [Design System](DOC-049) — Visual design tokens and typography
- [SQLite Schema](DOC-013) — Database tables backing the stores
