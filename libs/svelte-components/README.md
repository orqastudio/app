# @orqastudio/svelte-components

Svelte 5 component library for OrqaStudio and its plugins.

---

## Entry Points

The package exports three entry points:

| Entry Point | Import Path | Use Case |
|-------------|-------------|----------|
| Pure components | `@orqastudio/svelte-components/pure` | UI primitives — no store dependency. Safe in any context. |
| Connected components | `@orqastudio/svelte-components/connected` | Components that read from OrqaStudio stores. Require stores to be initialized. |
| Root (re-exports both) | `@orqastudio/svelte-components` | Convenience for app code. Not suitable for plugin views. |

Plugin views should import from `/pure` or `/connected` directly. Marking these paths as externals in your Vite config ensures the app's shared instances are used at runtime.

---

## Pure Components

No store dependency. Safe to use in plugins, tests, and standalone Svelte apps.

Import from `@orqastudio/svelte-components/pure`.

### Primitives

| Component | Description |
|-----------|-------------|
| `Button` | Standard button with variant and size props |
| `Badge` | Small status label |
| `Input` | Text input |
| `Textarea` | Multi-line text input |
| `Separator` | Horizontal or vertical divider |
| `ScrollArea` | Custom scrollable container |

### Overlays and Dialogs

| Component | Description |
|-----------|-------------|
| `Tooltip` | Hover tooltip with content slot |
| `Popover` | Click-triggered floating panel |
| `Dialog` | Modal dialog with header, body, footer |
| `AlertDialog` | Confirmation dialog with action and cancel |
| `ConfirmDialog` | Simplified confirm/cancel dialog |
| `DropdownMenu` | Click-triggered menu with items, groups, separators |

### Layout

| Component | Description |
|-----------|-------------|
| `Card` | Surface container with header, content, footer |
| `Tabs` | Tabbed content panels |
| `Collapsible` | Expandable section |
| `Resizable` | Draggable resizable pane group |
| `ScrollArea` | Custom scrollable container |

### Navigation

| Component | Description |
|-----------|-------------|
| `Breadcrumb` | Path breadcrumb trail |
| `NavItem` | Single navigation item |
| `Toolbar` | Horizontal action bar with items |

### Status and Icons

| Component | Description |
|-----------|-------------|
| `Status` | Status badge, dot, or inline label |
| `Icon` | Lucide icon by name |

### Data Display

| Component | Description |
|-----------|-------------|
| `Sparkline` | Inline trend line chart |
| `MetricCell` | Single metric value with optional trend |
| `PipelineStages` | Visual pipeline stage diagram |
| `ThinkingBlock` | Collapsible agent reasoning display |
| `MetadataRow` | Label-value metadata pair |
| `SmallBadge` | Compact badge for tags and counts |

### Feedback

| Component | Description |
|-----------|-------------|
| `EmptyState` | Empty content placeholder |
| `ErrorDisplay` | Error message display |
| `LoadingSpinner` | Animated loading indicator |

### Inputs

| Component | Description |
|-----------|-------------|
| `SearchInput` | Search field with icon and clear button |
| `SelectMenu` | Dropdown select |

### Pattern Abstractions

| Component | Description |
|-----------|-------------|
| `FormCard` | Card with a form layout |
| `ListCard` | Card with a list layout |
| `DashboardCard` | Card sized for dashboard grids |
| `ProgressBar` | Horizontal progress indicator |
| `ViewContainer` | Full-height view wrapper with header and body |

---

## Connected Components

Require `getStores()` to return initialized stores. Use in app code or plugin views that have confirmed store initialization.

Import from `@orqastudio/svelte-components/connected`.

| Component | Description |
|-----------|-------------|
| `AppShell` | Root app layout shell |
| `ActivityBar` | Left-side icon navigation bar |
| `NavSubPanel` | Expandable sub-navigation panel |
| `StatusBar` | Bottom status bar |
| `ConnectedToolbar` | Toolbar connected to app actions |
| `ArtifactListItem` | List item for a single artifact |
| `ArtifactLink` | Inline link to an artifact by ID — navigates on click |
| `StatusIndicator` | Status badge resolved from project config |
| `ToastContainer` | Renders active toast notifications |
| `ErrorToast` | Error-specific toast display |

### Store dependency

Connected components call `getStores()` from `@orqastudio/sdk` at mount time. This reads from `globalThis.__orqa_stores`, which the app initializes at startup. Plugin views using connected components must ensure the app has called `initializeStores()` before mounting — this is always true when a view is loaded by `PluginViewContainer`.

---

## Usage in Plugin Views

Mark both entry points as externals in your Vite config. The app exposes them via `window.__orqa`:

```ts
rollupOptions: {
    external: [
        "@orqastudio/svelte-components/pure",
        "@orqastudio/svelte-components/connected",
    ],
    output: {
        globals: {
            "@orqastudio/svelte-components/pure": "window.__orqa.components",
            "@orqastudio/svelte-components/connected": "window.__orqa.componentsConnected",
        },
    },
},
```

Then import normally in your view:

```svelte
<script lang="ts">
    import { Icon, Button, Card } from "@orqastudio/svelte-components/pure";
    import { ArtifactLink } from "@orqastudio/svelte-components/connected";
</script>
```

See `libs/sdk/PLUGIN-VIEWS.md` for the full plugin view development guide.

---

## Utilities

| Export | Description |
|--------|-------------|
| `cn(...classes)` | Merges Tailwind class names (clsx + tailwind-merge) |
| `createMockStores()` | Creates mock store instances for testing |
| `getStores()` | Re-exported from `@orqastudio/sdk` for convenience |

---

## Peer Dependencies

```json
{
  "@orqastudio/sdk": "dev",
  "@orqastudio/types": "dev",
  "svelte": ">=5.0.0"
}
```

---

## Development

```bash
npm run build        # Build the library
npm run dev          # Watch mode
npm run check        # Type check
npm run storybook    # Launch Storybook on port 6006
```

Components are documented with Storybook. Each component has a `.stories.ts` file alongside it.
