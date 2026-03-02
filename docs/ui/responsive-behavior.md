# Responsive Behavior

**Date:** 2026-03-02 | **Informed by:** [Information Architecture](/product/information-architecture), [Frontend Research](/research/frontend)

How Forge's layout adapts to different window sizes. Forge is a desktop application — there is no mobile viewport — but windows can be resized from ultrawide monitors down to fairly small sizes.

---

## Breakpoint Model

Forge does not use CSS media query breakpoints in the traditional web sense. Instead, it uses **panel collapse points** — thresholds at which panels auto-collapse to maintain minimum usable widths.

### Minimum Panel Widths

| Panel | Min Width | Below Min |
|-------|-----------|-----------|
| Sidebar | 180px | Collapses to 0px |
| Primary | 400px | Never collapses — always visible |
| Detail | 280px | Collapses to 0px |

### Window Width Ranges

| Window Width | Layout |
|-------------|--------|
| **> 1200px** | All three panels open. Comfortable working space. |
| **900-1200px** | Sidebar + Primary open. Detail collapsed (opens on demand). |
| **720-900px** | Primary only. Both sidebar and detail collapsed. |
| **< 720px** | Minimum viable: Primary panel fills window. Sidebar/detail accessible via overlay (Sheet). |

### Collapse Priority

When the window shrinks, panels collapse in this order:
1. **Detail panel** collapses first (least critical for primary workflow)
2. **Sidebar** collapses second (session list is secondary to active conversation)
3. **Primary panel** never collapses (always the active workspace)

When the window grows, panels restore in reverse order.

---

## Panel Adaptation

### Sidebar Adaptations

| Width | Behavior |
|-------|----------|
| 240px+ (normal) | Full session list with titles, dates, preview text |
| 180-240px (narrow) | Truncated titles, dates only, no preview |
| < 180px | Collapsed. Content accessible via `Ctrl+B` toggle. |

### Primary Panel Adaptations

| Available Width | Behavior |
|----------------|----------|
| 600px+ | Full message width. Code blocks show without horizontal scroll. |
| 400-600px | Narrower messages. Code blocks may scroll horizontally. |
| 400px | Minimum. Message input, send button, basic conversation. |

### Detail Panel Adaptations

| Width | Behavior |
|-------|----------|
| 360px+ (normal) | Full artifact browser, settings, dashboards |
| 280-360px (narrow) | Compact list items, abbreviated descriptions |
| < 280px | Collapsed. Content accessible via `Ctrl+\` toggle or Sheet overlay. |

---

## Overlay Mode (Narrow Windows)

When the window is too narrow for side panels (< 720px), sidebar and detail content become Sheet overlays:

- **Sidebar content** slides in from the left as a Sheet
- **Detail content** slides in from the right as a Sheet
- Sheets overlay the primary panel with a backdrop
- Click outside or press `Escape` to dismiss

This preserves all functionality in narrow windows without requiring a completely different layout.

---

## Toolbar Adaptations

| Width | Behavior |
|-------|----------|
| **> 900px** | Full toolbar: project name, search bar, new session button, settings icon |
| **600-900px** | Search bar collapses to icon. `Ctrl+K` still works. |
| **< 600px** | Project name truncated. Search and new session as icons only. |

---

## Conversation Input Adaptations

| Width | Behavior |
|-------|----------|
| **> 500px** | Multi-line input with visible Send button on right |
| **< 500px** | Input spans full width. Send button overlays bottom-right of input |

---

## Status Bar Adaptations

| Width | Behavior |
|-------|----------|
| **> 800px** | Full: connection indicator + CLI version + sidecar status |
| **500-800px** | Connection indicator + sidecar status. Version hidden. |
| **< 500px** | Connection indicator only (colored dot). |

---

## PaneForge Configuration

```svelte
<PaneGroup direction="horizontal">
  <Pane
    defaultSize={20}
    minSize={15}
    collapsible={true}
    collapsedSize={0}
  >
    <!-- Sidebar -->
  </Pane>
  <PaneResizeHandle />
  <Pane
    defaultSize={50}
    minSize={30}
  >
    <!-- Primary -->
  </Pane>
  <PaneResizeHandle />
  <Pane
    defaultSize={30}
    minSize={20}
    collapsible={true}
    collapsedSize={0}
  >
    <!-- Detail -->
  </Pane>
</PaneGroup>
```

PaneForge sizes are percentages, not pixels. The actual pixel widths depend on the window size. Min sizes ensure panels don't shrink below their minimum pixel widths — PaneForge handles this automatically via `minSize` as a percentage, but we also need to handle collapse triggers when the calculated pixel width drops below the minimum.

---

## Window State Persistence

All layout state is persisted via `tauri-plugin-window-state`:

| State | Persisted |
|-------|-----------|
| Window size (width, height) | Yes |
| Window position (x, y) | Yes |
| Panel widths (percentage) | Yes |
| Sidebar collapsed | Yes |
| Detail collapsed | Yes |
| Maximized state | Yes |

On app restart, the window restores to its previous size, position, and panel configuration.

---

## Testing Matrix

Minimum set of window sizes to validate responsive behavior:

| Size | Name | Expected Layout |
|------|------|----------------|
| 1920x1080 | Full HD | All panels, comfortable |
| 1440x900 | Laptop | All panels, slightly tighter |
| 1280x720 | Small laptop | Sidebar + Primary; detail collapsed |
| 1024x768 | Compact | Sidebar + Primary; detail collapsed |
| 800x600 | Minimum recommended | Primary only; sidebar/detail as overlays |
| 720x480 | Minimum viable | Primary only; everything overlay |
