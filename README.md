![License](https://img.shields.io/badge/license-BSL%201.1-blue)
![Status](https://img.shields.io/badge/status-pre--release-orange)
![TypeScript](https://img.shields.io/badge/TypeScript-3178C6?logo=typescript&logoColor=white)

![OrqaStudio](https://github.com/orqastudio/orqastudio-brand/blob/main/assets/banners/banner-1680x240.png?raw=1)

![OrqaStudio](https://github.com/orqastudio/orqastudio-brand/blob/main/assets/banners/banner-1680x240.png?raw=1)

# @orqastudio/svelte-components

> **Pre-release** — APIs, component contracts, and design tokens may change without notice until v1.0.0.

Svelte 5 component library for OrqaStudio apps and plugins. Ships the design system, UI primitives, and pattern abstractions that make OrqaStudio visually consistent.

---

## Purpose

This library is the single source of truth for all visual components in the OrqaStudio ecosystem. The app frontend is a thin shell — all UI primitives, shared components, layout patterns, and design tokens come from this package.

Plugins use the same components to ensure visual consistency without reimplementing the design system.

## Architecture

### Two Layers

| Layer | Path | Dependency | Use case |
|---|---|---|---|
| **Pure** | `@orqastudio/svelte-components/pure` | None (props only) | Any consumer — app, plugins, Storybook |
| **Connected** | `@orqastudio/svelte-components/connected` | `@orqastudio/sdk` stores | App and plugins that need live state |

### Component Design

Every component follows a consistent pattern:

- **Simple mode** — pass props for the common case. Types enforce minimum requirements.
- **Custom mode** — pass snippets for bespoke layouts. Types prevent mixing modes.
- **Parts exported** — internal sub-components (e.g. `CardHeader`, `DialogFooter`) available for edge cases.

```svelte
<!-- Simple mode -->
<Card title="Graph Health" description="Connection metrics">
  <p>Content here</p>
</Card>

<!-- Custom mode -->
<Card>
  {#snippet header()}
    <CardHeader>...</CardHeader>
  {/snippet}
  <p>Custom layout</p>
</Card>
```

## Installation

```bash
npm install @orqastudio/svelte-components
```

Peer dependencies: `svelte >=5.0.0`, `@orqastudio/sdk >=0.1.0`, `@orqastudio/types >=0.1.0`

## Usage

```svelte
<script>
  import { Button, Card, Status, Icon, Sparkline } from "@orqastudio/svelte-components/pure";
</script>

<Card title="Pipeline">
  <Status status="active" mode="badge" />
  <Icon name="target" size="md" />
  <Sparkline values={[3, 7, 2, 9, 4]} strokeColor="#22c55e" />
</Card>
```

## Exports

| Path | What you get |
|---|---|
| `@orqastudio/svelte-components` | Everything |
| `@orqastudio/svelte-components/pure` | Pure components (no SDK dependency) |
| `@orqastudio/svelte-components/connected` | Store-connected components |
| `@orqastudio/svelte-components/testing` | Mock stores + fixtures for development |
| `@orqastudio/svelte-components/tokens` | Design token CSS (OKLch variables) |

## Component Catalogue

### Primitives
Button, Badge, Input, Textarea, Separator, ScrollArea

### Composed (simple props or custom snippets)
Tooltip, Popover, Collapsible, Card, Dialog, AlertDialog, DropdownMenu, Tabs, Resizable

### Status & Icons
Status (with `resolveStatus`, `statusColor`, `statusColorClass` utilities), Icon (with `resolveIcon`, `DEFAULT_ICON_MAP`)

### Data Visualisation
Sparkline (with `sparklinePath`, `trendPercent`, `formatTrend` utilities), MetricCell

### Shared Components
EmptyState, ErrorDisplay, LoadingSpinner, SearchInput, SelectMenu, MetadataRow, SmallBadge, PipelineStages, ThinkingBlock, ConfirmDialog, Breadcrumb

### Pattern Abstractions
FormCard, ListCard, DashboardCard, ProgressBar, ViewContainer, Toolbar, NavItem

### Connected (store-aware)
StatusIndicator, ArtifactListItem, ArtifactLink, ToastContainer, ErrorToast, AppShell, ActivityBar, NavSubPanel, StatusBar, ConnectedToolbar

## Development

```bash
npm install
npm run storybook     # Launch Storybook on :6006
npm run build         # Build to dist/ via svelte-package
npm run check         # Type-check with svelte-check
```

## Design Principles

- **Consistency over flexibility** — semantic props (`variant`, `size`, `mode`) control appearance. Raw `class` is limited to layout containers.
- **Composed by default** — components ship a sensible default layout. Parts are available for edge cases, not the primary API.
- **The library owns display dependencies** — Tailwind, Lucide, bits-ui, paneforge are encapsulated. Consumers never import them directly.
- **Plugins extend via registration** — icons, statuses, and themes are extensible through registries, not source modification.

## License

[BSL 1.1](LICENSE) — converts to Apache 2.0 on 2030-03-16
