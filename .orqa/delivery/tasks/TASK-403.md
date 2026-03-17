---
id: TASK-403
title: Fix navigation and layout bugs
description: "Fix 6 navigation/layout bugs: missing sidebar icons, duplicate key error in ReferencesPanel, settings secondary nav not rendering, and config modal double-rendering."
status: completed
priority: P1
scoring:
  impact: 4
  urgency: 5
  complexity: 2
  dependencies: 3
created: 2026-03-13
updated: 2026-03-13
assignee: null
acceptance:
  - "Process, Delivery, Docs, Milestones, and Verification all show correct icons in sidebar nav"
  - No Svelte duplicate key error on any artifact page — ReferencesPanel keys include direction prefix
  - Navigating to Project Settings hides the artifact secondary nav and shows the settings category nav
  - Opening app configuration modal does NOT replace content viewer behind the modal
relationships:
  - target: EPIC-063
    type: delivers
    rationale: Theme A — navigation and layout bug fixes from UAT
  - target: SKILL-030
    type: informed-by
  - target: SKILL-042
    type: informed-by
  - target: SKILL-030
    type: informed-by
  - target: SKILL-042
    type: informed-by
---

## Scope

### Missing Nav Icons (Findings #10, #22)
- **Root cause**: `ActivityBar.svelte` icon resolution fails for group-level entries
- **Files**: `ui/src/lib/components/layout/ActivityBar.svelte` (lines 69-96), `GroupSubPanel.svelte` (lines 64-83)
- **Fix**: Ensure navTree group icons from README frontmatter propagate to ActivityBar

### Duplicate Key Error (Finding #16)
- **Root cause**: `ReferencesPanel.svelte` uses `ref.source_id + ref.field` / `ref.target_id + ref.field` — collides when same artifact appears in both directions
- **File**: `ui/src/lib/components/artifact/ReferencesPanel.svelte` (lines 51, 62)
- **Fix**: Prefix keys with `"in_"` / `"out_"` to ensure uniqueness

### Settings Nav Bug (Findings #31, #32)
- **Root cause**: `NavSubPanel.svelte` conditionally renders `SettingsCategoryNav` but condition may not trigger
- **Files**: `ui/src/lib/components/layout/NavSubPanel.svelte` (lines 28-41), `AppLayout.svelte`
- **Fix**: Ensure `activeActivity === "settings"` correctly triggers settings-specific secondary nav

### Config Modal Double-Render (Finding #33)
- **Root cause**: Both `SettingsDialog.svelte` modal AND `AppLayout.svelte` render `SettingsView` simultaneously
- **Files**: `ui/src/lib/components/layout/SettingsDialog.svelte`, `AppLayout.svelte` (lines 162-169)
- **Fix**: Remove duplicate rendering — modal should be the only place settings content appears when modal is open
