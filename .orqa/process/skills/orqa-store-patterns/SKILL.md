---
id: SKILL-016
title: Orqa Store Patterns
description: |
  OrqaStudio Svelte 5 rune store patterns: class-based stores with $state/$derived,
  store-to-component data flow, and reactive state management.
  Use when: Creating or modifying Svelte stores, wiring stores to components,
  managing loading/error/empty states, or debugging reactivity issues.
status: active
created: 2026-03-01
updated: 2026-03-10
layer: project
category: domain
file-patterns:
  - "ui/src/lib/stores/**"
version: 1.0.0
user-invocable: true
relationships:
  - target: PILLAR-001
    type: grounded
    rationale: Class-based rune stores with explicit loading/error/empty states make frontend data flow predictable and inspectable
  - target: TASK-009
    type: grounded
  - target: TASK-026
    type: grounded
  - target: TASK-075
    type: grounded
  - target: TASK-076
    type: grounded
  - target: TASK-082
    type: grounded
  - target: TASK-083
    type: grounded
  - target: TASK-190
    type: grounded
  - target: TASK-421
    type: grounded
  - target: TASK-422
    type: grounded
  - target: TASK-471
    type: grounded
  - target: TASK-473
    type: grounded
---


OrqaStudio uses Svelte 5 rune-based stores exclusively. All stores live in `ui/src/lib/stores/*.svelte.ts` files and use class-based patterns with `$state` and `$derived`.

## Patterns

### 1. Class-Based Store (Primary Pattern)

Orqa stores use a class instantiated as a module-level singleton:

```typescript
// ui/src/lib/stores/navigation.svelte.ts
export type ActivityView = "chat" | "project" | "docs" | "research" | "plans" | "agents" | "rules" | "lessons" | "skills" | "hooks" | "settings" | "configure";

class NavigationStore {
    activeActivity = $state<ActivityView>("chat");
    explorerView = $state<ExplorerView>("artifact-list");
    selectedArtifactPath = $state<string | null>(null);
    navPanelCollapsed = $state(false);

    get showNavPanel(): boolean {
        if (this.navPanelCollapsed) return false;
        return ACTIVITIES_WITH_NAV_PANEL.includes(this.activeActivity);
    }

    get isArtifactActivity(): boolean {
        return ARTIFACT_ACTIVITIES.includes(this.activeActivity);
    }

    setActivity(view: ActivityView) {
        this.activeActivity = view;
        this.selectedArtifactPath = null;
    }
}

export const navigationStore = new NavigationStore();
```

**Key rules:**
- One class per store file
- `$state()` for reactive properties
- Getter properties for computed values (replaces `$derived` at class level)
- Action methods for mutations
- Module-level singleton export

### 2. Store with Backend Data (invoke Pattern)

Stores that fetch from the Rust backend MUST handle three states: loading, loaded, error.

```typescript
// ui/src/lib/stores/conversation.svelte.ts
class ConversationStore {
    messages = $state<Message[]>([]);
    isLoading = $state(false);
    error = $state<string | null>(null);

    async loadMessages(sessionId: number): Promise<void> {
        this.isLoading = true;
        this.error = null;
        try {
            this.messages = await invoke<Message[]>("message_list", { sessionId });
        } catch (err) {
            this.error = err instanceof Error ? err.message : String(err);
        } finally {
            this.isLoading = false;
        }
    }
}

export const conversationStore = new ConversationStore();
```

**The loading/error/data lifecycle is mandatory.** Every async operation follows: set loading → clear error → try/catch → finally clear loading.

### 3. Store with Streaming State

The conversation store manages streaming with additional reactive state:

```typescript
class ConversationStore {
    messages = $state<Message[]>([]);
    streamingContent = $state("");
    streamingThinking = $state("");
    isStreaming = $state(false);
    activeToolCalls = $state<SvelteMap<string, ToolCallState>>(new SvelteMap());
    pendingApproval = $state<PendingApproval | null>(null);
    contextEntries = $state<ContextEntry[]>([]);

    async sendMessage(sessionId: number, content: string): Promise<void> {
        this.streamingContent = "";
        this.activeToolCalls = new SvelteMap();
        this.isStreaming = true;

        const channel = createStreamChannel((event: StreamEvent) => {
            this.handleStreamEvent(event);
        });

        await invoke("stream_send_message", {
            sessionId, content, model: this.selectedModel, onEvent: channel,
        });
    }

    private handleStreamEvent(event: StreamEvent) {
        switch (event.type) {
            case "text_delta":
                this.streamingContent += event.data.content;
                break;
            case "turn_complete":
                this.isStreaming = false;
                break;
            // ... handle each event type
        }
    }
}
```

### 4. SvelteMap for Keyed Collections

Use `SvelteMap` from `svelte/reactivity` for reactive key-value collections:

```typescript
import { SvelteMap } from "svelte/reactivity";

activeToolCalls = $state<SvelteMap<string, ToolCallState>>(new SvelteMap());

// Setting a value triggers reactivity
this.activeToolCalls.set(toolCallId, newState);

// Clearing
this.activeToolCalls = new SvelteMap();
```

### 5. Store-to-Component Data Flow

Components import stores and read reactive properties directly:

```svelte
<script lang="ts">
  import { conversationStore } from "$lib/stores/conversation.svelte";
  import { navigationStore } from "$lib/stores/navigation.svelte";
</script>

{#if conversationStore.isLoading}
  <LoadingSpinner />
{:else if conversationStore.error}
  <ErrorDisplay message={conversationStore.error} />
{:else if conversationStore.messages.length === 0}
  <EmptyState title="No messages" />
{:else}
  {#each conversationStore.messages as message}
    <MessageBubble {message} />
  {/each}
{/if}
```

**Components read from stores. They never write to stores directly — they call store action methods.**

### 6. $effect for Side Effects in Components

Use `$effect()` in components for data loading triggered by store state changes:

```svelte
<script lang="ts">
  import { sessionStore } from "$lib/stores/session.svelte";
  import { conversationStore } from "$lib/stores/conversation.svelte";

  $effect(() => {
    const session = sessionStore.activeSession;
    if (session) {
      conversationStore.loadMessages(session.id);
    }
  });
</script>
```

### 7. $derived for Computed Values in Components

```svelte
<script lang="ts">
  let { items } = $props<{ items: Item[] }>();

  let filteredItems = $derived(items.filter(i => i.active));
  let itemCount = $derived(filteredItems.length);
</script>
```

## Anti-Patterns

### Direct state mutation from components
```svelte
<!-- WRONG: components should not write store state directly -->
<button onclick={() => conversationStore.messages = []}>Clear</button>

<!-- CORRECT: call a store method -->
<button onclick={() => conversationStore.clear()}>Clear</button>
```

### Missing error handling in stores
```typescript
// WRONG: no error state, no loading state
async load() {
    this.items = await invoke<Item[]>("list_items");
}

// CORRECT: full lifecycle
async load() {
    this.isLoading = true;
    this.error = null;
    try {
        this.items = await invoke<Item[]>("list_items");
    } catch (err) {
        this.error = String(err);
    } finally {
        this.isLoading = false;
    }
}
```

### Svelte 4 patterns (FORBIDDEN)
```typescript
// WRONG: Svelte 4 store
import { writable } from 'svelte/store';
export const items = writable<Item[]>([]);

// WRONG: export let for props
export let title: string;

// WRONG: $: reactive statement
$: filteredItems = items.filter(i => i.active);
```

### invoke() in display components
```svelte
<!-- FORBIDDEN: only stores and page-level code call invoke() -->
<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  // This belongs in a store, not a component
</script>
```

## Key Files

| File | Purpose |
|------|---------|
| `ui/src/lib/stores/conversation.svelte.ts` | Conversation/streaming state (most complex store) |
| `ui/src/lib/stores/navigation.svelte.ts` | Activity view, nav panel, artifact selection |
| `ui/src/lib/stores/session.svelte.ts` | Session lifecycle (create, list, select, delete) |
| `ui/src/lib/stores/artifact.svelte.ts` | Document/research/plan tree scanning and content |
| `ui/src/lib/stores/project.svelte.ts` | Project settings and status |
| `ui/src/lib/ipc/invoke.ts` | Typed invoke wrapper used by all stores |
