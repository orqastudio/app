---
id: SKILL-015
title: Orqa Store Orchestration
description: |
  How multiple Svelte 5 rune stores coordinate in OrqaStudio without tight coupling.
  Covers store independence, component-level orchestration via $derived and $effect,
  circular dependency prevention, cross-store derived state, and initialization order.
  Use when: Wiring multiple stores together, adding cross-store coordination,
  debugging state synchronization issues, or planning new multi-store features.
status: active
created: "2026-03-01"
updated: "2026-03-10"
layer: project
version: 1.0.0
user-invocable: true
---


OrqaStudio has 10 singleton stores, each owning a single domain. Stores never import each other (with one documented exception). All cross-store coordination happens in components — primarily layout components that have visibility into multiple domains.

## Store Inventory

| Store | Domain | Imports Other Stores? |
|-------|--------|----------------------|
| `projectStore` | Active project, settings, scanning | No |
| `settingsStore` | App preferences, theme, sidecar status | No |
| `sessionStore` | Session CRUD, active session | No |
| `conversationStore` | Messages, streaming, tool calls | Yes (see below) |
| `navigationStore` | Activity view, artifact selection | No |
| `artifactStore` | Governance artifacts, doc/research/plan trees | No |
| `governanceStore` | Governance scanning and analysis | No |
| `enforcementStore` | Enforcement rules and violations | No |
| `lessonStore` | Lessons learned | No |
| `setupStore` | First-run setup wizard | No |

## Pattern 1: Store Independence

Each store owns exactly one domain. It manages its own loading/error/data states, calls `invoke()` for its own backend commands, and exposes reactive `$state` properties and action methods.

```typescript
// session.svelte.ts — only imports its own types and invoke
class SessionStore {
    sessions = $state<SessionSummary[]>([]);
    activeSession = $state<Session | null>(null);
    isLoading = $state(false);
    error = $state<string | null>(null);

    async selectSession(sessionId: number): Promise<void> { /* ... */ }
    async createSession(projectId: number): Promise<Session> { /* ... */ }
}
```

## Pattern 2: Component-Level Orchestration via $effect

When store A's state change should trigger an action on store B, the coordination lives in a **component**, not in either store.

```svelte
<!-- AppLayout.svelte -->
<script lang="ts">
  import { navigationStore } from "$lib/stores/navigation.svelte";
  import { artifactStore } from "$lib/stores/artifact.svelte";

  // Load doc tree when switching to docs activity
  $effect(() => {
    if (navigationStore.activeActivity === "docs" && artifactStore.docTree.length === 0) {
      artifactStore.loadDocTree();
    }
  });
</script>
```

`navigationStore` knows nothing about `artifactStore`. `artifactStore` knows nothing about `navigationStore`. AppLayout wires them together.

```svelte
<!-- ConversationView.svelte -->
<script lang="ts">
  const session = $derived(sessionStore.activeSession);

  $effect(() => {
    if (session) { conversationStore.loadMessages(session.id); }
    else { conversationStore.clear(); }
  });
</script>
```

Session selection triggers message loading, but the two stores have no knowledge of each other.

## Pattern 3: Derived Cross-Store State

When the template needs a value computed from multiple stores, use `$derived` at the component level:

```svelte
<script lang="ts">
  const hasProject = $derived(projectStore.hasProject);
  const needsSetup = $derived(projectStore.settingsLoaded && !projectStore.hasSettings);
  const hideChatPanel = $derived(navigationStore.activeActivity === "settings");
</script>
```

These combine state from multiple stores without any store importing another.

## Pattern 4: Initialization Order

AppLayout controls startup via `onMount`:

```
settingsStore.initialize()
  -> setupStore.checkSetupStatus()
    -> projectStore.loadActiveProject()
      -> [via $effect] governanceStore.scan()
      -> [via $effect] artifactStore.loadDocTree() (when navigated)
      -> [via ConversationView] sessionStore.loadSessions()
        -> [via $effect] conversationStore.loadMessages()
```

Each step is gated by the previous one completing. `$effect` blocks naturally fire only when their reactive conditions are met.

## Pattern 5: Scoped Orchestration

No single component coordinates all 10 stores. Each component coordinates the subset it needs.

**AppLayout coordinates:**
- `projectStore` + `navigationStore` (switch to project dashboard)
- `navigationStore` + `artifactStore` (load trees on activity change)
- `projectStore` + `governanceStore` (auto-scan on project load)

**ConversationView coordinates:**
- `sessionStore` + `conversationStore` (load messages on session change)
- `projectStore` + `sessionStore` (restore last session)

## Pattern 6: Event-Driven Cross-Store Updates

Sometimes a real-time event needs to update another store. The receiving store exposes a public method:

```typescript
// conversationStore handles stream event and delegates to sessionStore:
case "session_title_updated":
    sessionStore.handleTitleUpdate(event.data.session_id, event.data.title);
    break;

// sessionStore — public method for external callers:
handleTitleUpdate(sessionId: number, title: string): void {
    if (this.activeSession && this.activeSession.id === sessionId) {
        this.activeSession = { ...this.activeSession, title };
    }
}
```

## Known Coupling: conversationStore -> sessionStore

`conversationStore` directly imports `sessionStore` to handle the `session_title_updated` stream event. This is the **only store-to-store import** in the codebase.

**Why it exists:** Stream events arrive via a Tauri Channel callback — there is no component in scope to mediate.

**Alternatives (not yet implemented):** Emit a custom event from conversationStore and let ConversationView handle it, or use a lightweight event bus.

When adding similar cross-store event handling, prefer the component-mediated pattern.

## Anti-Patterns

### Store importing another store for data reads

```typescript
// WRONG: artifactStore reading from navigationStore
class ArtifactStore {
    get currentTree() {
        if (navigationStore.activeActivity === "docs") return this.docTree;
        return this.researchTree;
    }
}
```

**Fix:** Compute in the component that has both stores in scope.

### Bidirectional store imports

```typescript
// FORBIDDEN: creates a circular dependency
// session.svelte.ts imports conversationStore
// conversation.svelte.ts imports sessionStore
```

If two stores need to react to each other, coordination MUST live in a component's `$effect` blocks.

### All coordination in one god component

Split by scope. AppLayout handles layout concerns. ConversationView handles conversation concerns.

## Adding New Cross-Store Coordination

1. **Identify the scope** — layout concern or feature concern?
2. **Add a `$effect` in the appropriate component** — reads store A, calls action on store B
3. **Do NOT add an import of store B in store A** — if no component context is available, document the coupling
4. **Test via component integration test** that mounts the mediating component

## Related Skills

- **orqa-store-patterns** — single-store patterns (class structure, loading lifecycle)
- **orqa-composability** — the overarching philosophy
- **svelte5-best-practices** — runes syntax and reactivity rules
- **orqa-ipc-patterns** — the invoke/Channel layer stores call
