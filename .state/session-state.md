# Session State — 2026-04-04

## What was done this session

### 1. Process Manager Library (`cli/src/lib/process-manager.ts`)

- Dependency-aware graph reads from package.json + Cargo.toml (no hardcoded lists)
- Tier-based parallel builds via Kahn's algorithm
- Service lifecycle with health polling and crash recovery (exponential backoff)
- File watch coordinator with 500ms debounce and cascading rebuilds
- Refactored dev.ts from 1373 → 548 lines
- `orqa dev graph` subcommand
- Fixed `shell: isWindows()` for Windows .cmd wrappers
- Fixed `taskkill /T /F` for fast process kills
- Fixed `findPidsByNames()` batched discovery
- Added Storybook launch in startServices()
- Added pre-build libs in cmdDev() before devtools Vite starts

### 2. Unified Storage (`engine/storage/`)

- Consolidated 4 SQLite DBs into one `.state/orqa.db`
- 15 files, 9 repos
- `Frozen<T>` immutability wrapper for storage boundary
- App, daemon, devtools all wired to use engine/storage
- Session database with lifecycle, batch writer, 30-day retention

### 3. FP Audit and Fixes

- 23 domain reports at `.state/findings/fp-audit/`
- All CRITICAL items PASS across entire codebase
- Push-loops converted to iterator chains across all Rust engine crates
- `readonly` + `assertNever` added system-wide in TypeScript
- `deepFreeze()` on all IPC invoke results
- `DeepReadonly<T>` utility type

### 4. UI Component Library

- Created 19 new ORQA primitives: Table, Typography, Layout, FormGroup, Link, Kbd, Prose, VisuallyHidden, Checkbox, Switch, RadioGroup, Chat family
- Typography stack with semantic variants
- Storybook stories for all components
- RULE-55092f35 governance rule for component story enforcement
- Swept 100+ component files replacing raw HTML/Tailwind with ORQA primitives
- Removed `class` prop from Card family and typography components

### 5. Composability enforcement — zero raw HTML/Tailwind in apps

Four commits across four waves completing the class-prop removal and
component-composability enforcement across app, devtools, and plugins.

**Wave 1 (`5ff89bb3d`)** — Component library consumes its own primitives:
- HStack extended with `onclick`, `onkeydown`, `role`, `tabindex`,
  `aria-selected`, `aria-label`, `full`, `style` props for interactive
  row patterns without raw divs
- Stack `full` prop added
- Pure components (ChatBubble/Container/Input, ListCard, NavItem,
  PipelineStages, ThinkingBlock) now use HStack/Stack/Center/Text/Caption
- App components fixed: TraceabilityPanel, ContextEntry, StreamingIndicator,
  SystemMessage, AboutDialog

**Wave 2 (`043bb6464`)** — Library prop and primitive extensions:
- `Box` primitive (new) — general container with padding/position/inset/
  zIndex/overflow/flex/border/rounded/background/width/minWidth
- `Stack`/`HStack` typed: padding/paddingX/paddingY/paddingTop/paddingBottom,
  height ("full"|"screen"), overflow, minHeight, flex, borderTop/Bottom,
  marginTop, role, tabindex, aria-label
- `Center` typed: gap, flex, padding
- `Text` 6 new variants: body-strong, body-strong-muted, caption-strong,
  caption-mono, caption-tabular, overline-muted
- `Text`/`Caption` `lineClamp` prop (1-4), truncate-wins precedence
- `Caption` restricted variant prop (caption|caption-strong|caption-mono|
  caption-tabular)
- `Code` `block` prop renders <pre>
- `Button` `variant="card"` (full-width ghost card button)
- `Badge` `variant="success"` + `capitalize` axis
- `Dot` (new) — inline circular indicator (size, color)
- `CountBadge` (new) — small circular count indicator
- `VerticalText` (new) — writing-mode: vertical-rl wrapper for Kanban

**Wave 3 round 1 (`b2429143c`)** — 68 files:
- Consumer sweep: devtools (11 files), plugins/software-kanban (6 files),
  app dashboard/settings/tool/conversation
- 9 fix-overlay-table Button regressions restored: ToolApprovalDialog,
  ToolCallCard, ContextEntry, DecisionQueueWidget, GraphHealthWidget,
  MilestoneContextCard, RoadmapView, PluginInstallDialog, ProviderSwitcher
- Library passthrough: Badge, Card, CardFooter, Checkbox, Input, Label
  accept `Omit<HTMLAttributes<T>, "class" | "style">` — allows legitimate
  aria-*, data-*, events, id, title, autofocus while BLOCKING class and
  style escape hatches

**Wave 3 round 2 (`4d81b3786`)** — 50 files:
- Remaining app folders: artifact, layout, navigation, graph, lessons,
  governance, enforcement, content, setup, plugin
- 3 more regressions restored: FullGraphView, LessonViewer, ViolationsPanel
- Breadcrumb.svelte now uses ORQA Breadcrumb primitive
- ClaudeCliStep uses Link primitive
- DynamicArtifactTable uses Table primitives

## Compilation status

- **app**: 0 errors, 1 pre-existing PipelineStepper a11y warning
- **devtools**: 0 errors, 0 warnings
- **plugins/software-kanban**: 0 errors, 0 warnings
- **cargo check --workspace**: clean (unchanged this session)
- **orqa enforce --stories**: pending (library has new component dirs —
  should re-run to verify story coverage)

## Legitimate raw-HTML exceptions (kept, documented)

- `<div bind:this>` for Cytoscape canvas: FullGraphView, RelationshipGraphView
- `<div bind:this>` for Mermaid: MermaidDiagram
- `<div bind:this>` for plugin host: PluginViewContainer
- `<div bind:this>` for virtualised viewport: LogTable, LogRow (devtools)
- `<a href>` in MarkdownLink.svelte (internal link renderer)
- `<img>` for logos: StatusBar, AboutDialog, ProjectGeneralSettings
- `<input type="color">` in ProjectArtifactLinksSettings
- JIT Tailwind category color spans in plugin registry (dynamic per-plugin)
- `<div draggable>` in HorizonBoard (HTML5 drag API)

## Follow-ups identified during sweep

### Library gaps (LOW/MEDIUM priority primitives)

- **Image/Avatar** primitive — replace raw `<img>` in StatusBar, AboutDialog,
  ProjectGeneralSettings
- **ColorInput** primitive — replace `<input type="color">` in
  ProjectArtifactLinksSettings
- **StepIndicator** primitive — replace step dots in SetupWizard (currently
  uses Dot + HStack manually)
- **Sparkline block variant** — HealthTrendWidget/ImprovementTrendsWidget
  use raw `<svg>`; existing Sparkline primitive may need extensions
- **Timeline/Stepper** primitive — PipelineStepper has pixel-sized connector
  lines between stage circles that no current primitive expresses cleanly

### Library prop extensions (small additions)

- **Box `backgroundOpacity` prop** — patterns like `bg-warning/10`
  (TraceabilityPanel, SidecarStatusCard error messages)
- **Box `width` arbitrary values** — w-60, w-[7rem] label columns
  (ArtifactMasterDetail sidebar, FrontmatterHeader labels)
- **HStack `height` numeric presets** — h-10, h-8 breadcrumb bars
  (ArtifactViewer breadcrumb row)
- **Text `leading-relaxed` variant** — description paragraphs
  (FrontmatterHeader, ArtifactViewer)
- **Stack/HStack `borderLeft` prop** — `border-l-2` wrappers
  (ToolCallCard, ToolCallGroup, ToolCallSummary)
- **Box corner-specific rounding** (`roundedTl`, `roundedTr`) — chat bubble
  shapes (MessageBubble)
- **Caption `overline-muted` variant** — currently only on Text; Caption
  should support it too for consistent caption family
- **Grid `items-baseline` + `justify-self`** — label/value grid patterns
  (RelationshipsList, ReferencesPanel)
- **CardRoot `selected` prop** — replace data-selected passthrough pattern
  in ProcessCard with typed state prop

### Behavioral/architectural

- **Wrapper span `display: contents` pattern** — devtools uses these for
  scoped-CSS state targeting. Long-term replace with typed state props
  on the underlying ORQA components (e.g. `selected`, `active`).
- **Inline style escape hatch** — a few legitimate dynamic styles remain
  (margin-left indentation in TraceabilityPanel, dynamic drag positions
  in HorizonBoard). These need an explicit `style` prop or dynamic CSS
  variable pattern.

## Next priorities

1. Run `orqa enforce --stories` to verify the new library primitives
   (Box, Dot, CountBadge, VerticalText) have story files
2. Fix the 1 pre-existing PipelineStepper a11y warning
3. Test full dev environment: `orqa dev` end-to-end (apps + devtools
   start cleanly, no runtime errors from the refactor)
4. Decide: address the "legitimate exceptions" list above, or treat as
   acceptable trade-offs for now and move on
5. Library gap primitives (Image, ColorInput, StepIndicator) as
   opportunistic follow-ups when next touching those consumer files
