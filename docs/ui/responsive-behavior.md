# Responsive Behavior

**Date:** 2026-03-02 | **Informed by:** [Information Architecture](/product/information-architecture), [Frontend Research](/research/frontend)

How Forge's layout adapts to different window sizes. Forge is a desktop application — there is no mobile viewport — but windows can be resized from ultrawide monitors down to fairly small sizes.

---

## Breakpoint Model

Forge does not use CSS media query breakpoints in the traditional web sense. Instead, it uses **panel collapse points** — thresholds at which panels auto-collapse to maintain minimum usable widths.

### Minimum Panel Widths

| Zone | Min Width | Below Min |
|------|-----------|-----------|
| Activity Bar | 48px | Never collapses (fixed) |
| Explorer Panel | 280px | Never collapses (focal point) |
| Sessions Panel | 180px | Collapses to 0px |
| Chat Panel | 360px | Never collapses — always visible |

### Window Width Ranges

| Window Width | Layout |
|-------------|--------|
| **> 1200px** | All zones open. Comfortable working space. |
| **900-1200px** | Sessions Panel auto-collapsed. Activity Bar + Explorer + Chat visible. |
| **720-900px** | Sessions Panel as overlay Sheet. Activity Bar + Explorer + Chat visible. |
| **< 720px** | Activity Bar as floating toggle. Chat as overlay Sheet. Explorer fills window. |

### Collapse Priority

When the window shrinks, panels collapse in this order:
1. **Sessions Panel** collapses first (session list is secondary to active content)
2. **Chat Panel** becomes overlay second (conversation accessible via Sheet)
3. **Activity Bar** becomes floating toggle third
4. **Explorer Panel** never collapses (artifact focal point)

When the window grows, panels restore in reverse order.

---

## Panel Adaptation

### Sessions Panel Adaptations

| Width | Behavior |
|-------|----------|
| 240px+ (normal) | Full session list with titles, dates, preview text. Both tabs visible. |
| 180-240px (narrow) | Truncated titles, dates only, no preview. Tabs still accessible. |
| < 180px | Collapsed to 0px. Content accessible via `Ctrl+B` toggle or overlay Sheet. |

### Explorer Panel Adaptations

| Available Width | Behavior |
|----------------|----------|
| 400px+ | Full artifact lists with descriptions. Viewer/editor has comfortable width. |
| 280-400px | Compact artifact list items, abbreviated descriptions. Viewer wraps tighter. |
| 280px | Minimum. Single-column artifact list, basic viewer. |

### Chat Panel Adaptations

| Width | Behavior |
|-------|----------|
| 500px+ | Full message width. Code blocks show without horizontal scroll. |
| 360-500px | Narrower messages. Code blocks may scroll horizontally. |
| 360px | Minimum. Message input, send button, basic conversation. |

---

## Overlay Mode (Narrow Windows)

When the window is too narrow for side panels (< 720px), sidebar and detail content become Sheet overlays:

- **Sidebar content** slides in from the left as a Sheet
- **Detail content** slides in from the right as a Sheet
- Sheets overlay the Explorer Panel with a backdrop
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
<!-- Activity Bar is OUTSIDE PaneForge (CSS flex, fixed 48px) -->
<div class="flex h-full">
  <ActivityBar class="w-12 shrink-0" />
  <PaneGroup direction="horizontal" class="flex-1">
    <Pane
      defaultSize={40}
      minSize={20}
    >
      <!-- Explorer Panel -->
    </Pane>
    <PaneResizeHandle />
    <Pane
      defaultSize={20}
      minSize={12}
      collapsible={true}
      collapsedSize={0}
    >
      <!-- Sessions Panel -->
    </Pane>
    <PaneResizeHandle />
    <Pane
      defaultSize={40}
      minSize={25}
    >
      <!-- Chat Panel -->
    </Pane>
  </PaneGroup>
</div>
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
| Sessions Panel collapsed | Yes |
| Maximized state | Yes |

On app restart, the window restores to its previous size, position, and panel configuration.

---

## Testing Matrix

Minimum set of window sizes to validate responsive behavior:

| Size | Name | Expected Layout |
|------|------|----------------|
| 1920x1080 | Full HD | All zones, comfortable |
| 1440x900 | Laptop | All zones, slightly tighter |
| 1280x720 | Small laptop | Sessions Panel auto-collapsed; Activity Bar + Explorer + Chat |
| 1024x768 | Compact | Sessions Panel auto-collapsed; Activity Bar + Explorer + Chat |
| 800x600 | Minimum recommended | Sessions as overlay; Activity Bar + Explorer + Chat |
| 720x480 | Minimum viable | Activity Bar floating; Chat as overlay; Explorer fills window |
