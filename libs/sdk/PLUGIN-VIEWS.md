# Plugin Views — Developer Guide

Plugin views are custom UI panels that render inside the OrqaStudio app. They are declared in your plugin manifest, built as standalone ESM bundles, and loaded at runtime into a sandboxed container.

---

## Overview

The app loads plugin views dynamically at runtime using `PluginViewContainer`. Each view is a pre-built JavaScript bundle that ships with your plugin. The bundle is loaded from the plugin's install path and mounted into a dedicated container element.

Plugin views communicate with the rest of the app through two mechanisms:

- **Shared modules** — `window.__orqa` gives views access to the same store instances and component library the app uses.
- **Store API** — `getStores()` from `@orqastudio/sdk` returns live reactive stores for reading and writing app state.

---

## window.__orqa — Shared Module Map

The app calls `exposeSharedModules()` once at startup, placing shared libraries on `window.__orqa`. Plugin bundles mark these as externals so they are not bundled separately — at runtime their imports resolve from this global.

| Key | Module | What It Provides |
|-----|--------|-----------------|
| `sdk` | `@orqastudio/sdk` | Stores, IPC, logger, router, types |
| `components` | `@orqastudio/svelte-components/pure` | UI primitives with no store dependency |
| `componentsConnected` | `@orqastudio/svelte-components/connected` | UI components that read from stores |
| `svelte` | `svelte` | Svelte internals (`mount`, `unmount`, runes) |

This ensures that a plugin view accesses the same store instances as the host app, regardless of how many copies of the SDK exist in memory.

---

## Store Access

Plugin views access stores through `getStores()`:

```ts
import { getStores } from "@orqastudio/sdk";

const { artifactStore, navigationStore, toast } = getStores();
```

Stores live on `globalThis.__orqa_stores`. The app initializes this at startup; plugin views always read the same instances.

Available stores:

| Store | Type | What It Provides |
|-------|------|-----------------|
| `artifactGraphSDK` | `ArtifactGraphSDK` | Graph queries and mutations |
| `sessionStore` | `SessionStore` | Active session state |
| `projectStore` | `ProjectStore` | Project metadata and settings |
| `artifactStore` | `ArtifactStore` | Artifact CRUD and queries |
| `conversationStore` | `ConversationStore` | Active conversation and tool calls |
| `enforcementStore` | `EnforcementStore` | Enforcement run results |
| `lessonStore` | `LessonStore` | Lesson entries |
| `setupStore` | `SetupStore` | App setup and onboarding state |
| `settingsStore` | `SettingsStore` | User and app settings |
| `errorStore` | `ErrorStoreImpl` | Error surfacing |
| `navigationStore` | `NavigationStore` | Active view, navigation actions |
| `toastStore` | `ToastStore` | Toast notifications |
| `pluginRegistry` | `PluginRegistry` | Installed plugin metadata |
| `pluginStore` | `PluginStore` | Plugin page state |
| `toast` | convenience object | `toast.success()`, `toast.error()`, etc. |

---

## View Mount Contract

`PluginViewContainer` loads your bundle from `dist/views/{viewKey}.js` and mounts it into an empty `<div>`. Your bundle must export one of:

### Option 1 — `mount` function (preferred)

```ts
export function mount(container: HTMLElement): (() => void) | void {
    // Mount your view into `container`
    // Return a cleanup function, or nothing
    return () => {
        // Tear down your view
    };
}
```

The container is a full-height, full-width `<div>` with `overflow: auto`. The cleanup function is called when the view is unmounted.

### Option 2 — Default Svelte 5 component

```ts
import MyView from "./MyView.svelte";
export default MyView;
```

The app will call Svelte's `mount()` and `unmount()` on your component. Use this only for simple Svelte components. For views that manage their own lifecycle, use the `mount` function.

---

## Output Path

Plugin views must be built to:

```
dist/views/{viewKey}.js
```

The `viewKey` must match the key declared in your manifest's `provides.views` array.

---

## Manifest Declaration

Declare views in `orqa-plugin.json`:

```json
{
  "name": "@yourorg/plugin-name",
  "provides": {
    "views": [
      {
        "key": "my-view",
        "label": "My View",
        "icon": "layout-dashboard"
      }
    ]
  },
  "defaultNavigation": [
    {
      "key": "my-view",
      "type": "plugin",
      "icon": "layout-dashboard",
      "pluginSource": "@yourorg/plugin-name"
    }
  ]
}
```

The `defaultNavigation` entry adds your view to the app's activity bar when the plugin is installed.

---

## Vite Config

Configure Vite to build each view as an ESM library bundle with shared modules marked as externals:

```ts
// vite.config.ts
import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";

export default defineConfig({
    plugins: [svelte()],
    build: {
        lib: {
            entry: "src/views/my-view.ts",
            formats: ["es"],
            fileName: () => "my-view.js",
        },
        outDir: "dist/views",
        rollupOptions: {
            external: [
                "@orqastudio/sdk",
                "@orqastudio/svelte-components/pure",
                "@orqastudio/svelte-components/connected",
                "svelte",
                /^svelte\//,
            ],
            output: {
                globals: {
                    "@orqastudio/sdk": "window.__orqa.sdk",
                    "@orqastudio/svelte-components/pure": "window.__orqa.components",
                    "@orqastudio/svelte-components/connected": "window.__orqa.componentsConnected",
                    svelte: "window.__orqa.svelte",
                },
            },
        },
    },
});
```

For plugins with multiple views, create one Vite config per view (or use a build script that runs Vite once per entry).

---

## package.json

```json
{
  "name": "@yourorg/plugin-name",
  "version": "0.1.0-dev",
  "type": "module",
  "scripts": {
    "build": "vite build",
    "dev": "vite build --watch"
  },
  "peerDependencies": {
    "@orqastudio/sdk": "dev",
    "@orqastudio/types": "dev",
    "svelte": "^5.0.0"
  },
  "devDependencies": {
    "@sveltejs/vite-plugin-svelte": "^5.0.0",
    "svelte": "^5.0.0",
    "typescript": "^5.7.0",
    "vite": "^6.0.0"
  }
}
```

---

## Example Plugin View

A minimal Svelte 5 component using the shared component library:

```svelte
<!-- src/views/MyView.svelte -->
<script lang="ts">
    import { Icon } from "@orqastudio/svelte-components/pure";
    import { getStores } from "@orqastudio/sdk";

    const { projectStore } = getStores();
    const projectName = $derived(projectStore.project?.name ?? "—");
</script>

<div class="p-6">
    <h2 class="text-lg font-semibold flex items-center gap-2">
        <Icon name="layout-dashboard" size="md" />
        {projectName}
    </h2>
    <p class="mt-2 text-sm text-muted-foreground">
        Replace this with your plugin's view content.
    </p>
</div>
```

Entry point that exports the component as default:

```ts
// src/views/my-view.ts
export { default } from "./MyView.svelte";
```

---

## Error Handling

`PluginViewContainer` catches load and mount errors and displays them in the UI. Your `mount` function should throw a descriptive error if it cannot initialize, rather than silently failing.

---

## Plugin Templates

Use `orqa plugin new` to scaffold a new plugin from a template:

| Template | Description |
|----------|-------------|
| `frontend` | Views and components only — no backend code |
| `sidecar` | AI provider sidecar with streaming and tool execution |
| `cli-tool` | One-shot CLI tools registered as plugin commands |
| `full` | Complete plugin with views, sidecar, CLI tools, and content |
