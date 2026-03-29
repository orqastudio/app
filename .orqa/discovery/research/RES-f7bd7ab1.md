---
id: "RES-f7bd7ab1"
type: discovery-research
title: "Inter-Artifact Linking Design"
description: "Design a robust ID-to-path resolution system for artifact cross-references, replacing the fragile label-prefix matching with backend-resolved paths and consistent YAML link conventions."
status: completed
created: "2026-03-10"
updated: "2026-03-10"
relationships:
  - target: "EPIC-d45b4dfd"
    type: "guides"
    rationale: "Research findings informed the design of Artifact Graph SDK and Structural Integrity"
---

## Problem

Artifact cross-references (e.g., an epic linking to its milestone, a task linking to its epic, a decision referencing another decision) are inconsistent and partially broken. The current system uses a fragile frontend-only resolution mechanism that fails for tree-structured directories and depends on label string matching rather than real ID-to-path resolution.

## Current State

### What Works

Flat artifact types with standard ID prefixes navigate correctly:

| Prefix | Target | Status |
| -------- | -------- | -------- |
| MS | `.orqa/implementation/milestones/` | Working |
| EPIC | `.orqa/implementation/epics/` | Working |
| TASK | `.orqa/implementation/tasks/` | Working |
| IDEA | `.orqa/implementation/ideas/` | Working |
| AD | `.orqa/process/decisions/` | Working |
| IMPL | `.orqa/process/lessons/` | Working |

### What's Broken

1. **Tree-structured directories** — `ArtifactNav` has an `if (isTree) return;` guard that prevents auto-selection for tree types (documentation, hooks, skills). Cross-links to documentation pages are impossible.

2. **Missing prefixes** — Several artifact types have no entry in `ARTIFACT_PREFIX_MAP`:
   - `RES-NNN` (research)
   - `PILLAR-NNN` (pillars)
   - `RULE-NNN` (rules)
   - Agent and skill files (no ID prefix pattern)

3. **Fragile label matching** — The auto-select uses `n.label.startsWith(pendingId)` against the display label (which comes from frontmatter `title`, not the filename). If the title doesn't start with the ID, the match fails.

4. **No backend resolution** — All resolution happens in the frontend via the NavTree. There is no Rust command to resolve an artifact ID to a file path. This means:
   - Resolution only works after the NavTree has loaded
   - Broken links are silent (no error, no feedback)
   - Deep-linking from outside the app is impossible

5. **No markdown cross-linking** — Artifact IDs mentioned in document bodies (e.g., "See [PD-0dfa4d52](PD-0dfa4d52)") are plain text, not interactive links.

## Resolution Mechanism Analysis

### Current Flow (Frontend-Only)

```text
ArtifactLink click
  → navigationStore.navigateToArtifact(id)
    → Extract prefix from ID (e.g., "EPIC" from "EPIC-797972a7")
    → Look up ARTIFACT_PREFIX_MAP[prefix]
    → Set activeGroup + activeSubCategory
    → Set pendingArtifactId = id
      → ArtifactNav $effect detects pending ID
        → if (isTree) return;  // ← BREAKS HERE for tree types
        → rawNodes.find(n => n.label.startsWith(pendingId))  // ← FRAGILE
        → navigationStore.openArtifact(match.path)
```

### Problems with Frontend-Only Resolution

| Problem | Impact |
| --------- | -------- |
| No ID field in NavTree nodes | Must match on label, not on the actual artifact ID |
| Tree types excluded | Documentation, rules (if nested), hooks are unreachable |
| Prefix map is hardcoded | Adding new artifact types requires code changes |
| Silent failures | Unknown prefix → `return;` with no feedback |
| Label mismatch | If frontmatter `title` ≠ `"[EPIC-797972a7](EPIC-797972a7) - Something"`, match fails |

## Proposed Design

### Principle: Backend Resolves, Frontend Navigates

The backend owns the mapping from artifact ID to file path. The frontend asks the backend where an artifact lives, then navigates to it. This separates concerns: the backend knows the file system, the frontend knows the UI state.

### 1. Backend: Artifact ID Index

Add an in-memory index built during `artifact_scan_tree` that maps every artifact ID to its file path:

```rust
// Domain type
pub struct ArtifactIndex {
    id_to_path: HashMap<String, String>,  // "EPIC-797972a7" → ".orqa/implementation/epics/EPIC-797972a7.md"
}

// Built during scan by extracting frontmatter `id` from every .md file
// Stored in AppState alongside the NavTree
```

**New Tauri command:**

```rust
#[tauri::command]
pub fn resolve_artifact_id(
    id: String,
    state: State<'_, AppState>,
) -> Result<Option<String>, OrqaError> {
    // Returns the relative path for the given artifact ID, or None if not found
}
```

This is cheap — the index is already built during the scan. The command is a HashMap lookup.

### 2. YAML Link Convention

All inter-artifact references in YAML frontmatter use artifact IDs (not paths):

```yaml
# Links to artifacts with known ID patterns
milestone: MS-b1ac0a20
epic: EPIC-489c0a47
depends-on:
  - TASK-8cd409ae
  - TASK-d402df01
pillars:
  - PILLAR-c9e0a695
research-refs:
  - RES-63cda7a7
supersedes: PD-0dfa4d52
promoted-to: RULE-8cb4bd04

# Links to files without ID patterns (documentation pages, etc.)
docs-required:
  - .orqa/documentation/reference/scanner-dashboard.md
docs-produced:
  - .orqa/documentation/development/streaming.md
```

**Convention:**

| Field Pattern | Value Format | Resolution |
| -------------- | ------------- | ------------ |
| Fields listing artifact IDs | `[EPIC-797972a7](EPIC-797972a7)`, `[PD-0dfa4d52](PD-0dfa4d52)`, etc. | Resolve via `resolve_artifact_id` |
| Fields listing file paths | `.orqa/documentation/...` | Navigate directly to path |
| Mixed fields | Should not exist — pick one format per field | |

This convention already exists informally. The design formalizes it and ensures the frontend handles both patterns.

### 3. Frontend: ArtifactLink Resolution

Replace the prefix-map approach with backend resolution:

```typescript
// navigationStore
async navigateToArtifact(id: string) {
    // 1. Ask backend to resolve
    const path = await invoke<string | null>("resolve_artifact_id", { id });
    if (!path) {
        // Show "artifact not found" toast
        return;
    }
    // 2. Find the artifact in the NavTree by path (not label)
    this.navigateToPath(path);
}

navigateToPath(path: string) {
    // Walk the NavTree to find which group/subcategory contains this path
    // Set activeGroup, activeSubCategory, and select the artifact
    // Works for BOTH flat and tree types
}
```

**Key change:** Navigation by path instead of by label. The `navigateToPath` function walks the full NavTree (including tree children) to find the node, eliminating the `isTree` guard.

### 4. NavTree Enhancement: Include Artifact IDs

Add an optional `id` field to `DocNode`:

```typescript
interface DocNode {
    label: string;
    path: string | null;
    children: DocNode[] | null;
    id?: string;          // NEW: artifact ID from frontmatter (e.g., "EPIC-797972a7")
    status?: string;
    description?: string;
}
```

The backend already reads frontmatter for title/status/description. Adding `id` is trivial. This enables:

- Frontend can build its own ID→path map from the NavTree (for instant resolution without an IPC call)
- Auto-select by ID becomes: `nodes.find(n => n.id === pendingId)` — exact match, not label prefix

### 5. FrontmatterHeader Link Rendering

The `FrontmatterHeader` component already has `LINK_FIELDS` and renders `ArtifactLink` for each value. The fix is in `ArtifactLink` itself — it should use the new resolution mechanism instead of the prefix map.

For `docs-required` and `docs-produced` (which are file paths, not IDs), the component should render a different link type that navigates directly to the path.

### 6. Markdown Cross-Linking

Artifact IDs in document bodies are auto-linked during markdown rendering. The `MarkdownRenderer` component preprocesses text to wrap patterns like `EPIC-\d+`, `AD-\d+`, etc. in clickable links.

Implementation:

- Regex pass over rendered markdown matching all known artifact ID patterns (EPIC-NNN, TASK-NNN, AD-NNN, MS-NNN, IDEA-NNN, IMPL-NNN, RES-NNN, PILLAR-NNN, RULE-NNN)
- Wrap matches in clickable elements that call `navigateToArtifact`
- Always-on — no configuration needed. All matching patterns are linked.

## Implementation Phases

### Phase 1: Backend Index + Resolution Command

- Add `id` extraction to `scan_recursive_nodes` in `artifact_reader.rs`
- Build `ArtifactIndex` (HashMap) during scan
- Store index in `AppState`
- Add `resolve_artifact_id` Tauri command
- Add `id` field to `DocNode` in both Rust and TypeScript types

### Phase 2: Frontend Navigation by Path

- Add `navigateToPath(path: string)` to NavigationStore
- This method walks the full NavTree (flat AND tree) to find the matching node
- Remove `ARTIFACT_PREFIX_MAP` (replaced by backend resolution)
- Remove `if (isTree) return;` guard in ArtifactNav
- Update `navigateToArtifact` to use `resolve_artifact_id` or local NavTree ID lookup

### Phase 3: Link Rendering Polish

- `ArtifactLink` distinguishes between ID links and path links
- ID links use the resolution mechanism
- Path links navigate directly
- Unknown IDs show a "not found" indicator instead of silently failing
- `FrontmatterHeader` renders `docs-required`/`docs-produced` as path links

### Phase 4: Markdown Cross-Linking

- Auto-detect artifact ID patterns in rendered markdown (EPIC-NNN, TASK-NNN, AD-NNN, etc.)
- Wrap in clickable elements that call `navigateToArtifact`
- Always-on for all known artifact ID patterns

### Phase 5: File Watcher for Index Refresh

- Watch `.orqa/` for file system changes (create, modify, delete, rename)
- Rebuild artifact index on change (debounced)
- Emit NavTree refresh event to frontend so the sidebar stays current
- Replaces manual rescan — the index is always up to date

## Resolved Questions

- **Index refresh**: File watcher — the artifact index rebuilds automatically on `.orqa/` file system changes. No manual rescan needed.
- **Partial matches**: No — exact ID match only. Partial matching adds ambiguity ([EPIC-797972a7](EPIC-797972a7) vs [TASK-855582b4](TASK-855582b4) vs [PD-7121ec20](PD-7121ec20)).
- **Broken links**: Styled as a "broken link" — same chip/box shape as a working link but with a broken-link icon and the app's warning colour token. Visually distinct, not hidden.
- **Path validation**: Yes — `docs-required`/`docs-produced` paths are validated against disk during scan. Broken paths are flagged in the UI.

## Related Artifacts

- [RES-63cda7a7](RES-63cda7a7) — Consistent entity body structure (complementary: body templates may include cross-reference sections)
- [RULE-63cc16ad](RULE-63cc16ad) — Artifact config integrity (config paths must match disk)
- RULE-b03009da — End-to-end completeness (linking touches all layers)
- [PD-48b310f9](PD-48b310f9) — Universal agent model (agents are artifacts too, but without standard ID prefixes)
