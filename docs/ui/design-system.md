# Design System

**Date:** 2026-03-02 | **Status:** Phase 0d specification

The authoritative design system specification for Forge's UI. Consolidates decisions from [design-tokens research](/research/design-tokens), [branding research](/research/branding), [frontend research](/research/frontend), and [brand identity guidelines](/ui/brand-identity) into an implementation-ready reference.

---

## Design Principles

1. **Professional authority** — Forge governs product development. Decisive, not playful.
2. **Technical competence** — Information-dense, developer-familiar patterns (monospace code, keyboard shortcuts, dark mode).
3. **Quiet confidence** — Forge recedes when project themes are applied. The project brand dominates, not Forge's.
4. **Clarity over decoration** — Every pixel serves information delivery.

---

## Color System

### Base Palette: Zinc

Forge uses shadcn-svelte's zinc neutral scale. Zinc's cool undertone reads as "technical" without the blue conflict of slate. It is the shadcn-svelte default — zero customization of the neutral scale.

### shadcn-svelte CSS Variables (Complete Set)

These are the component library's contract. Every shadcn-svelte component references these variables. The naming convention is `--{semantic-role}` for backgrounds and `--{semantic-role}-foreground` for text on those backgrounds.

#### Light Mode (`:root`)

```css
:root {
  --radius: 0.625rem;

  /* Zinc neutrals (shadcn-svelte defaults, unchanged) */
  --background: oklch(1 0 0);
  --foreground: oklch(0.145 0 0);
  --card: oklch(1 0 0);
  --card-foreground: oklch(0.145 0 0);
  --popover: oklch(1 0 0);
  --popover-foreground: oklch(0.145 0 0);
  --secondary: oklch(0.97 0 0);
  --secondary-foreground: oklch(0.205 0 0);
  --muted: oklch(0.97 0 0);
  --muted-foreground: oklch(0.556 0 0);
  --accent: oklch(0.97 0 0);
  --accent-foreground: oklch(0.205 0 0);
  --destructive: oklch(0.577 0.245 27.325);
  --border: oklch(0.922 0 0);
  --input: oklch(0.922 0 0);

  /* Forge signature indigo-violet accent */
  --primary: oklch(0.55 0.18 280);
  --primary-foreground: oklch(0.985 0 0);
  --ring: oklch(0.55 0.18 280);

  /* Sidebar */
  --sidebar: oklch(0.985 0 0);
  --sidebar-foreground: oklch(0.145 0 0);
  --sidebar-primary: oklch(0.55 0.18 280);
  --sidebar-primary-foreground: oklch(0.985 0 0);
  --sidebar-accent: oklch(0.97 0.005 280);
  --sidebar-accent-foreground: oklch(0.205 0 0);
  --sidebar-border: oklch(0.922 0 0);
  --sidebar-ring: oklch(0.55 0.18 280);

  /* Charts — indigo-anchored palette */
  --chart-1: oklch(0.55 0.18 280);
  --chart-2: oklch(0.6 0.118 184.704);
  --chart-3: oklch(0.398 0.07 227.392);
  --chart-4: oklch(0.828 0.189 84.429);
  --chart-5: oklch(0.769 0.188 70.08);
}
```

#### Dark Mode (`.dark`)

```css
.dark {
  --background: oklch(0.145 0 0);
  --foreground: oklch(0.985 0 0);
  --card: oklch(0.145 0 0);
  --card-foreground: oklch(0.985 0 0);
  --popover: oklch(0.145 0 0);
  --popover-foreground: oklch(0.985 0 0);
  --secondary: oklch(0.269 0 0);
  --secondary-foreground: oklch(0.985 0 0);
  --muted: oklch(0.269 0 0);
  --muted-foreground: oklch(0.708 0 0);
  --accent: oklch(0.269 0 0);
  --accent-foreground: oklch(0.985 0 0);
  --destructive: oklch(0.577 0.245 27.325);
  --border: oklch(0.269 0 0);
  --input: oklch(0.269 0 0);

  /* Indigo lightens in dark mode for adequate contrast */
  --primary: oklch(0.70 0.18 280);
  --primary-foreground: oklch(0.15 0.02 280);
  --ring: oklch(0.50 0.15 280);

  --sidebar: oklch(0.205 0 0);
  --sidebar-foreground: oklch(0.985 0 0);
  --sidebar-primary: oklch(0.70 0.18 280);
  --sidebar-primary-foreground: oklch(0.985 0 0);
  --sidebar-accent: oklch(0.269 0.01 280);
  --sidebar-accent-foreground: oklch(0.985 0 0);
  --sidebar-border: oklch(0.269 0 0);
  --sidebar-ring: oklch(0.50 0.15 280);

  --chart-1: oklch(0.70 0.18 280);
  --chart-2: oklch(0.696 0.17 162.48);
  --chart-3: oklch(0.769 0.188 70.08);
  --chart-4: oklch(0.627 0.265 303.9);
  --chart-5: oklch(0.645 0.246 16.439);
}
```

### Forge Brand Extension Variables

These extend the shadcn-svelte base set for Forge-specific needs (contextual spark colors, status states, logo theming). See [Brand Identity Guidelines](/ui/brand-identity) for full context.

```css
:root {
  /* Contextual spark / accent colors */
  --forge-ember: oklch(0.64 0.27 25);     /* Action / execution */
  --forge-amber: oklch(0.80 0.17 75);     /* Learning loop */
  --forge-arc: oklch(0.78 0.12 220);      /* Governance / rules */

  /* Status colors */
  --forge-success: oklch(0.72 0.19 150);  /* Pass / success */
  --forge-warning: oklch(0.79 0.17 75);   /* Warning */
  --forge-info: oklch(0.76 0.13 230);     /* Informational */
  /* --destructive covers error (shadcn base) */

  /* Logo theming */
  --forge-logo-body: oklch(0.145 0 0);
  --forge-logo-spark: var(--forge-ember);
}

.dark {
  --forge-ember: oklch(0.64 0.27 25);
  --forge-amber: oklch(0.80 0.17 75);
  --forge-arc: oklch(0.78 0.12 220);
  --forge-success: oklch(0.72 0.19 150);
  --forge-warning: oklch(0.79 0.17 75);
  --forge-info: oklch(0.76 0.13 230);
  --forge-logo-body: oklch(0.92 0.003 260);
  --forge-logo-spark: var(--forge-ember);
}
```

### Layout Tokens

```css
:root {
  /* Layout tokens */
  --forge-activity-bar-width: 48px;
  --forge-activity-bar-icon-size: 24px;
  --forge-activity-bar-hit-target: 48px;
  --forge-activity-bar-indicator: 2px;
  --forge-sessions-panel-default: 240px;
  --forge-sessions-panel-min: 180px;
  --forge-sessions-panel-max: 320px;
}
```

### Color Usage Rules

| Do | Don't |
|----|-------|
| `bg-primary text-primary-foreground` | `bg-blue-600 text-white` |
| `border-border` | `border-gray-200` |
| `text-muted-foreground` | `text-[#71717a]` |
| `bg-destructive` for error states | `bg-red-500` |
| Use semantic classes exclusively | Use raw Tailwind palette colors for brand elements |
| Use `--forge-success` / `--forge-warning` / `--forge-info` for status | Invent ad-hoc status colors |

Dark mode is handled entirely by CSS variable redefinition under `.dark`. Components never need `dark:` Tailwind variants for colors.

---

## Typography

### Font Stacks

```css
:root {
  --font-sans: Inter, Roboto, 'Helvetica Neue', 'Arial Nova', 'Nimbus Sans',
    Arial, sans-serif;
  --font-mono: 'JetBrains Mono', 'Cascadia Code', 'Fira Code', Menlo, Monaco,
    'Courier New', monospace;
}
```

**System fonts, no bundled fonts.** A Tauri desktop app should feel native. Inter is the preferred face (common on developer machines) but the stack degrades gracefully.

### Type Scale

| Role | Class | Weight |
|------|-------|--------|
| Page heading | `text-2xl` | `font-semibold` |
| Section heading | `text-xl` | `font-semibold` |
| Subsection heading | `text-lg` | `font-medium` |
| Body text | `text-sm` (14px) | `font-normal` |
| Small / caption | `text-xs` | `font-normal` |
| Code / technical | `font-mono text-sm` | `font-normal` |

Body text defaults to `text-sm` (14px). This is the standard for information-dense desktop tools (VS Code, Linear, GitHub).

### Typography Rules

- Headings use `font-semibold` or `font-medium`, never `font-black`
- Code blocks and technical content use `font-mono`
- No heading levels deeper than `h4` in the UI — flatten the hierarchy
- Use `Inter var` optical sizing for large headings if the variable font is available: `font-variation-settings: 'opsz' 28`

---

## Spacing & Layout

### Spacing Scale

Tailwind's default 4px increment scale. Commonly used values:

| Token | Value | Typical Usage |
|-------|-------|---------------|
| `gap-1` / `p-1` | 4px | Icon-to-text gap, tight inline spacing |
| `gap-2` / `p-2` | 8px | Related items within a group |
| `gap-3` / `p-3` | 12px | Compact card padding |
| `gap-4` / `p-4` | 16px | Default section gap, standard padding |
| `gap-6` / `p-6` | 24px | Card content padding, section spacing |
| `gap-8` / `p-8` | 32px | Major section separation |

### Border Radius

`--radius: 0.625rem` (10px) — the shadcn-svelte default. All components derive from this:

| Component | Radius |
|-----------|--------|
| Button, Input | `rounded-md` (calc from `--radius`) |
| Card, Dialog | `rounded-lg` |
| Badge, Tag | `rounded-full` for pills, `rounded-md` for labels |
| Avatar | `rounded-full` |

### Layout Conventions

| Pattern | Implementation |
|---------|---------------|
| Card padding | `p-6` |
| Section gaps | `space-y-4` or `gap-4` |
| Page margins | `px-4 md:px-6` |
| Sidebar width | 240-280px (collapsible) |
| Panel minimum width | 320px |
| Panel gap (splitter) | 1px border, 8px drag handle |

---

## Dark Mode

### Strategy: Class-based via `mode-watcher`

- System preference detection on first run
- Manual light / dark / system toggle
- Persistent preference in `localStorage`
- `.dark` class applied to `<html>` — CSS variables redefine under `.dark`
- **Default:** system preference. **Fallback** (if system preference unavailable): dark

### Implementation

```svelte
<!-- src/routes/+layout.svelte -->
<script lang="ts">
  import '../app.css';
  import { ModeWatcher } from 'mode-watcher';
  let { children } = $props();
</script>

<ModeWatcher />
{@render children?.()}
```

Toggle:

```svelte
<script lang="ts">
  import { toggleMode } from 'mode-watcher';
  import Sun from 'lucide-svelte/icons/sun';
  import Moon from 'lucide-svelte/icons/moon';
</script>

<button onclick={toggleMode}>
  <Sun class="dark:hidden" />
  <Moon class="hidden dark:block" />
</button>
```

### Tauri Note

On Linux, Tauri's WebView does not reliably propagate `prefers-color-scheme`. The class-based approach (`.dark` on `<html>`) avoids this entirely — no dependency on the media query.

---

## Component Library: shadcn-svelte

### Import Convention

```svelte
<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { Card, CardHeader, CardTitle, CardContent } from "$lib/components/ui/card";
</script>
```

### Core Components (Phase 1)

| Category | Components |
|----------|-----------|
| **Actions** | Button, Toggle, DropdownMenu |
| **Data display** | Badge, Card, Table, Separator |
| **Feedback** | Alert, Tooltip, Sonner (toast) |
| **Forms** | Input, Textarea, Select, Checkbox, Label |
| **Layout** | Resizable (PaneForge), ScrollArea, Tabs, Collapsible |
| **Overlay** | Dialog, Popover, Sheet, Command |
| **Navigation** | Sidebar |

### Custom Components (Phase 1)

These are not provided by shadcn-svelte and must be built:

| Component | Purpose | Builds On |
|-----------|---------|-----------|
| `ConversationView` | Message stream with streaming token display | ScrollArea, Card |
| `MessageBubble` | Single message (user, assistant, system) | Card |
| `ToolCallCard` | Collapsible tool call with input/output | Collapsible, Card, Badge |
| `CodeBlock` | Syntax-highlighted code with copy button | `svelte-highlight` |
| `MarkdownRenderer` | Rendered markdown content | `@humanspeak/svelte-markdown` |
| `MarkdownEditor` | Source-level markdown editor | `svelte-codemirror-editor` (CodeMirror 6) |
| `ArtifactBrowser` | Browse/filter governance artifacts | Table, Badge, Input |
| `SessionDropdown` | Session switcher dropdown with search | DropdownMenu, Input |
| `ThemeToggle` | Light/dark/system mode switch | `mode-watcher`, Button |

### Component Usage Rules

- Use shadcn-svelte components for all standard UI patterns — do not reimplement
- Use `variant` prop for visual variants (`default`, `destructive`, `outline`, `secondary`, `ghost`, `link`)
- Compose complex UI from shadcn primitives rather than building monoliths
- Follow shadcn-svelte patterns for forms (controlled inputs, validation with `formsnap` + `sveltekit-superforms`)

---

## Per-Project Theming

Forge adapts its UI to match the active project's design tokens. This makes Forge feel like a native companion to each project.

### Theme Resolution Chain

```
User Override  >  Extracted Project Token  >  Forge Default Theme
```

### How It Works

1. **Extraction** — On project open, Rust backend scans for design tokens in CSS `:root`, `tailwind.config.*`, and component library conventions
2. **Normalization** — All colors converted to OKLCH via `csscolorparser`
3. **Storage** — JSON token object stored in SQLite (`project_themes` + `project_theme_overrides` tables)
4. **Application** — Frontend applies tokens via `document.documentElement.style.setProperty()`
5. **Dark overrides** — Project's dark mode tokens injected as a `<style>` element scoped to `.dark`
6. **Reset** — On project switch, all inline overrides removed, new project's theme applied

### Runtime Application

```typescript
export function applyProjectTheme(theme: ProjectTheme | null): void {
  const root = document.documentElement;

  if (!theme) {
    // Reset to Forge defaults: remove all inline overrides
    for (const key of Object.keys(FORGE_DEFAULTS)) {
      root.style.removeProperty(`--${key}`);
    }
    return;
  }

  // Apply resolved tokens (user overrides > extracted > defaults)
  for (const [key, defaultValue] of Object.entries(FORGE_DEFAULTS)) {
    const resolved = theme.user_overrides?.[key]
      ?? theme.tokens?.[key]
      ?? null;

    if (resolved) {
      root.style.setProperty(`--${key}`, resolved);
    } else {
      root.style.removeProperty(`--${key}`);
    }
  }
}
```

### Performance

Setting 30 CSS custom properties on `:root`: < 1ms. Subsequent repaint: 2-5ms. Imperceptible.

### What to Avoid

- Do not rebuild stylesheets on theme change — set individual CSS variables
- Do not use Svelte's `--css-prop={value}` syntax for global theming (adds wrapper DOM nodes)
- Do not convert colors at render time — all values pre-converted to OKLCH at extraction
- Batch all property updates in a single synchronous block

---

## Brand-Aware Code Generation

When Claude generates frontend code, Forge injects a compact `<design-system>` context block into the system prompt. This ensures generated code uses semantic token classes rather than raw color values.

### Injection Conditions

The design system context is injected when:
1. Files with frontend extensions are in the conversation (`.svelte`, `.tsx`, `.jsx`, `.css`, etc.)
2. User intent suggests frontend work ("build a component", "create a page", etc.)
3. `.claude/rules/design-system.md` exists and has matching path patterns

### System Prompt Format (~500 tokens)

```xml
<design-system>
## Project Design System

### Tokens (CSS Custom Properties)
- --primary: oklch(0.55 0.18 280) (brand color — CTAs, active states, links)
- --primary-foreground: oklch(0.985 0 0) (text on primary backgrounds)
- --background: oklch(1 0 0) (page background)
- --foreground: oklch(0.145 0 0) (default text)
- --destructive: oklch(0.577 0.245 27.325) (errors, delete actions)
...

### Component Library: shadcn-svelte
- Import from: $lib/components/ui/{component}
- Use component library primitives. Do NOT create custom components
  for functionality the library provides.

### Styling Rules
- ALWAYS use semantic Tailwind classes: bg-primary, text-foreground, border-border
- NEVER use raw palette colors: bg-blue-600, text-gray-900, etc.
- NEVER use hardcoded hex/rgb values
- Dark mode handled by CSS variables — do NOT add dark: variants for colors
</design-system>
```

### Governance Artifact

Auto-generated as `.claude/rules/design-system.md` during codebase scan if design tokens are detected. Path-scoped to frontend files via `paths:` frontmatter. User approval required before writing. See [branding research Q2](/research/branding) for full specification.

---

## Accessibility

### Contrast Requirements

- All text meets WCAG 2.1 AA (4.5:1 for normal text, 3:1 for large text)
- OKLCH lightness check: `L > 0.65` → dark foreground; `L <= 0.65` → light foreground
- Focus rings use `--ring` variable — visible in both light and dark modes
- `--destructive` red passes contrast on both light and dark backgrounds

### Keyboard Navigation

- All interactive elements reachable via Tab
- Focus visible on all interactive elements (`:focus-visible` with `--ring`)
- Escape closes overlays (Dialog, Popover, Sheet, Command)
- shadcn-svelte components have built-in keyboard support via Bits UI

### Motion

- Respect `prefers-reduced-motion` — disable transitions and animations
- Default transitions: 150ms for interactive states, 200ms for panel transitions
- No animations that convey essential information

---

## Iconography

**Library:** `lucide-svelte`

| Usage | Icon Style |
|-------|-----------|
| Navigation | Outline, 20px |
| Inline with text | Outline, 16px, `text-muted-foreground` |
| Status indicators | Filled or outline depending on state |
| Tool call types | Specific icons per tool (File, Terminal, Search, etc.) |

Icons inherit text color. Use semantic color classes: `text-foreground`, `text-muted-foreground`, `text-destructive`.

---

## Reference Links

| Document | What It Covers |
|----------|---------------|
| [Brand Identity Guidelines](/ui/brand-identity) | Dual-mode identity, logo family, contextual spark colors, deliverables checklist |
| [Design Tokens Research](/research/design-tokens) | Token format, runtime application, extraction pipeline, per-project persistence |
| [Branding Research](/research/branding) | Three-layer branding strategy, governance artifact, code generation, hierarchy |
| [Frontend Research](/research/frontend) | Component library selection, markdown rendering, panel layout, charts |
| [Information Architecture](/product/information-architecture) | Three-zone + nav sub-panel layout, navigation model, keyboard shortcuts, state management |
