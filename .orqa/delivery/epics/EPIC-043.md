---
id: EPIC-043
title: Dogfood Readiness Verification
description: "First round of user acceptance testing. The user exercises the app end-to-end, testing conversation flow, artifact browsing, governance visibility, settings, and streaming behaviour. Findings are captured as tasks within this epic."
status: completed
priority: P1
created: 2026-03-09
updated: 2026-03-09
horizon: null
scoring: null
relationships:
  - target: MS-001
    type: delivers
    rationale: Epic belongs to this milestone
  - target: TASK-037
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-038
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-039
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-040
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-041
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-042
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-043
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-044
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-045
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-046
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-333
    type: delivered-by
    rationale: Epic contains this task
  - target: PILLAR-001
    type: grounded-by
---
## Implementation Design

### Approach

The user performs hands-on testing of OrqaStudio in dogfood mode. Each issue, bug, or
improvement identified during UAT is captured as a task (TASK-NNN) referencing this epic.
Tasks are then prioritised and implemented following the structure-before-work rule.

### Test Areas

| Area | What to Test |
|------|-------------|
| **Conversation** | Send messages, streaming, tool calls, tool approval, auto-naming |
| **Artifact browsing** | Navigate artifacts, view frontmatter, tree structure, empty states |
| **Settings** | Project settings, model selection, provider status, custom prompt |
| **Governance** | Scanner results, process violations, lesson display |
| **Streaming** | Long responses, partial messages, error recovery, thinking output |
| **Session management** | Create/switch/resume sessions, session persistence |
| **Layout** | Three-zone layout, panel resizing, status bar, toolbar |
| **Search** | Regex search, semantic search (if indexed), code_research |

### Task Creation Protocol

For each UAT finding:
1. Create `TASK-NNN.md` with `epic: [EPIC-043](EPIC-043)`
2. Set `scope` to the files/components affected
3. Set `acceptance` to describe the expected behaviour
4. Classify: `bug` (broken), `ux` (works but poor experience), `missing` (not implemented)

### UAT Round 1 Findings

#### Bugs

| # | Finding | Type | Area |
|---|---------|------|------|
| F1 | Documentation section not visible in UI at all | bug | Artifact browsing |
| F2 | Main nav icon → sidebar and artifact viewer are empty | bug | Layout / Navigation |
| F3 | Crash/freeze after navigating many artifacts (possible memory leak) — may be tied to large artifacts like git-workflow or general accumulation | bug | Stability |

#### Data Quality

| # | Finding | Type | Area |
|---|---------|------|------|
| F4 | Research audits missing description fields | data | Governance |
| F5 | Research grouped by MVP milestone — inconsistent, should either group all by milestone or none (use `milestone:` YAML field instead) | data | Governance |
| F6 | Lessons have code-reference titles (not human-readable) and some lack descriptions | data | Governance |
| F7 | Rules don't all have descriptions | data | Governance |
| F8 | [MS-000](MS-000) (Foundation & Scaffold) has unstructured phase content — should be restructured like [MS-001](MS-001) with phases backfilled as epics and tasks | data | Planning |

#### UX Improvements

| # | Finding | Type | Area |
|---|---------|------|------|
| F9 | Null YAML values displayed in read views (e.g., `promoted-to: null`, `deadline: Invalid Date`) — general rule: hide null/empty fields | ux | Artifact display |
| F10 | Milestone `gate` question should be displayed last, not mixed with other fields | ux | Artifact display |
| F11 | Epic P1/P2/P3 tags not explained in UI — unclear what they mean | ux | Planning |
| F12 | Task list inconsistent — sometimes title+description, sometimes just title, sometimes shows raw TASK-ID despite having a title | ux | Planning |
| F13 | Status badges are generic chips except for "active" — need clearer visual hierarchy per status meaning | ux | All planning/governance |
| F14 | Artifact list bars need status indicator (dot, icon, or colour) and wider layout | ux | All list views |
| F15 | Breadcrumbs in research section duplicate path: Planning > Research > .orqa > Planning > Research > Audits > File — base should be a home icon linking to dashboard | ux | Navigation |
| F16 | Loading spinner in main viewer is too small — animation is nice but lost at current size | ux | Layout |
| F17 | Status/metadata indicators in list items should be a reusable component across planning AND governance sections (systematic, not per-section) | ux | Component design |

#### Hooks & Platform

| # | Finding | Type | Area |
|---|---------|------|------|
| F18 | Hooks section shows "no hooks yet" — but Claude hooks exist in `.claude/settings.json`. Unclear whether they should appear here. Ties into hooks system research. | missing | Governance |

#### Future Ideas (not tasks — capture as IDEA artifacts)

| # | Finding |
|---|---------|
| I1 | Artifact platform audit — which artifacts are built into the platform vs external/manual |
| I2 | Hooks system research — own hooks layer on top of Claude hooks, Claude hookify plugin as inspiration for CLI compatibility |

### Systemic Themes

The findings cluster around a few systemic issues rather than isolated bugs:

1. **Null/empty value handling** — F9, F10, F4, F6, F7 all stem from the same root: the artifact renderer displays every YAML field regardless of whether it has a value. A single "skip null/empty fields" rule in the renderer fixes most of these.

2. **Status UX system** — F13, F14, F17 are all the same problem: status is shown but not meaningfully differentiated. Needs a reusable `StatusIndicator` component with semantic colours/icons that works in both list items and detail views, across planning and governance.

3. **List item metadata** — F12, F14, F17 overlap: list items need consistent structure (title, optional description, status indicator) regardless of artifact type. One shared `ArtifactListItem` component pattern.

4. **Navigation/breadcrumbs** — F2, F15 relate to navigation structure. Breadcrumbs need a home-rooted path that doesn't duplicate the section hierarchy.

5. **Data backfill** — F5, F6, F7, F8 are governance data quality issues fixable without code changes.

6. **Memory/stability** — F3 needs investigation before other work proceeds.

### UAT Round 2 Findings (post-fix verification)

#### Remaining Bugs

| # | Finding | Type | Area |
|---|---------|------|------|
| F19 | Tasks still showing [TASK-002](TASK-002) etc in nav instead of title; not all have descriptions | bug | Artifact display |
| F21 | Status dots/indicators not showing on artifact list items (DocNode lacks status from backend) | bug | Artifact display |
| F28 | Hooks still not displayed in governance section | bug | Governance |

#### Data & Classification

| # | Finding | Type | Area |
|---|---------|------|------|
| F22 | Some research documents are misclassified (should be epics, e.g., "UX polish sprint") | data | Governance |
| F24 | Ideas need status audit — some are implemented/part-implemented but not linked to decisions/epics | data | Planning |
| F29 | Research has mix of subfolder and flat organisation — should flatten, use YAML fields for relationships | data | Governance |

#### Architecture / Governance Model

| # | Finding | Type | Area |
|---|---------|------|------|
| F20 | Epic titles contain process words (UAT, Phase) instead of describing outcomes | ux | Governance |
| F23 | Artifact type definitions unclear — no clear enforcement of what each type is FOR | missing | Governance |
| F25 | Agent scope needs categorisation (software-engineering vs governance vs general) | missing | Three-layer model |
| F26 | Skills need categorisation (project-type-specific vs universal platform) | missing | Three-layer model |
| F27 | Rules and hooks need canon vs project classification | missing | Three-layer model |

### Systemic Themes (Round 2)

7. **Three-layer governance classification** — F25, F26, F27 are the same issue across agents, skills, rules, and hooks. Everything needs canon/project/plugin categorisation. This is the foundational architecture that enables multi-project support.

8. **Artifact type clarity & enforcement** — F22, F23, F29 stem from unclear definitions of what each artifact type is for. Need formal type definitions with usage rules and automated enforcement.

9. **Backend data for list items** — F19, F21 are the same root cause: `DocNode` from the Rust scanner doesn't carry `status` or reliable `title` from frontmatter to the frontend. The `ArtifactListItem` component is ready but has no data to display.

10. **Idea-to-decision trail** — F24 is a data audit: trace implemented ideas to their decisions and epics.

11. **Hooks visibility** — F28 needs backend investigation: the scanner may not handle hook files correctly, or hooks need a different display model than markdown artifacts.

### UAT Round 3 Findings (post-classification verification)

#### Bugs

| # | Finding | Type | Area |
|---|---------|------|------|
| F30 | Git workflow rule triggers crash (third occurrence — increasingly specific to that artifact) | bug | Stability |
| F31 | Most rules don't show status dots in artifact list (rules likely missing `status` field in frontmatter) | bug | Artifact display |
| F32 | Tasks still showing inconsistent descriptions and raw TASK-XXX in artifact list panel | bug | Artifact display |
| F33 | Dashboard showing stale "3 of 7" counts | bug | Dashboard |

### Systemic Themes (Round 3)

12. **Rule frontmatter completeness** — F31 is because rules have `layer` and `id` fields but many lack a `status` field. The scanner extracts status from frontmatter — no status field means no dot.

13. **Task label fallback** — F32 is a recurring issue (was F19). DocNode now carries status, but the label/title extraction may still fall through to raw filename for some tasks. Need to verify `extract_basic_frontmatter` handles all task files correctly.

14. **Crash on specific artifact** — F30 is the third report involving git-workflow.md. May be file size, content parsing, or markdown rendering issue specific to that document.

### UAT Round 4 Findings (post-Round-3-fix verification)

#### Display & Rendering

| # | Finding | Type | Area |
|---|---------|------|------|
| F34 | Agent Governance research doc metadata not displaying in viewer (title/description visible in list but not in detail view) | bug | Artifact display |
| F35 | Research docs have inconsistent formats — some have IDs, some don't, varying field sets | data | Schema enforcement |
| F40 | Hooks viewer shows .sh files as markdown — need file-type detection to switch between code viewer and markdown/YAML parser | ux | Artifact display |

#### Schema & Data Quality

| # | Finding | Type | Area |
|---|---------|------|------|
| F36 | [TASK-002](TASK-002), [TASK-035](TASK-035), [TASK-038](TASK-038) still showing raw IDs as title in list despite having titles in file | bug | Artifact display |
| F37 | Most tasks missing descriptions in list view | data | Schema enforcement |
| F38 | All artifacts of every type MUST have title + description fields — need schema enforcement | missing | Schema enforcement |
| F39 | Status dots need 100% field coverage per type — every artifact must have the field that drives its status dot | data | Schema enforcement |

#### Governance Model

| # | Finding | Type | Area |
|---|---------|------|------|
| F41 | Rule status should be active/inactive only — historical-artifacts.md rule references surpassed which doesn't apply to rules | data | Governance |
| F42 | Agents/orchestrator instructions don't check rule status before enforcement — inactive rules could still be enforced | missing | Governance |
| F43 | Rules should NOT use surpassed status — if a rule is superseded, mark inactive and add a leading comment noting the reason | data | Governance |
| F44 | Lessons should have promotion-based status indicators: unpromoted (neutral), recurring/threshold (amber), promoted (green) | ux | Governance |

### Systemic Themes (Round 4)

15. **Schema enforcement gap** — F35, F37, F38, F39 are the same root issue: no formal schema defines required fields per artifact type, so data quality is inconsistent. Need a schema rule + enforcement mechanism.

16. **Rule lifecycle vocabulary** — F41, F43 clarify that rules use only active/inactive. Surpassed is for research/tasks/plans. Rules that are superseded become inactive with a comment.

17. **Rule-aware enforcement** — F42 reveals that agents enforce all rules regardless of status. Need agents to check `status: active` before applying a rule.

18. **Lesson promotion indicators** — F44 is a UX enhancement: lessons should visually indicate their promotion state (unpromoted → recurring → promoted to rule/skill/standard).

19. **File-type-aware rendering** — F40 is the hooks viewer needing to detect `.sh` vs `.md` and route to the appropriate renderer.

20. **Persistent task display bug** — F36 is the third occurrence of specific tasks showing raw IDs. Likely a caching issue or a title extraction edge case in those specific files.

### Round 4 Fixes Applied

| # | Finding | Fix |
|---|---------|-----|
| F34 | Research doc metadata not displaying | Fixed CRLF line ending issue in frontmatter.ts — Windows files had \\r breaking regex $ anchors |
| F35 | Research doc formats inconsistent | Updated artifact-framework.md schema definitions |
| F36 | [TASK-002](TASK-002)/035/038 showing raw IDs | Fixed invalid YAML quoting (embedded double quotes caused serde_yaml parse failure) |
| F37 | Most tasks missing descriptions | Audit: all 50 tasks already have descriptions — no action needed |
| F38 | Schema enforcement needed | Updated artifact-framework.md with required fields for lessons and rules |
| F39 | Status dots need coverage | Lessons now have status field (Round 4); rules got status in Round 3 |
| F40 | Hooks rendered as markdown | Added file-type detection in ArtifactViewer — .sh routes to HookViewer |
| F41 | Rule status should be active/inactive | Updated historical-artifacts.md with rule status vocabulary |
| F42 | Agents don't check rule status | Added Rule Status Awareness section to skill-enforcement.md |
| F43 | Rules should not use surpassed | Fixed — rules use active/inactive; surpassed is for research/tasks only |
| F44 | Lessons need promotion indicators | Added status to all 14 lessons; recurring/inactive in StatusIndicator |

### UAT Round 5 Findings (title audit + navigation)

#### UX Improvements

| # | Finding | Type | Area |
|---|---------|------|------|
| F45 | Cross-links within artifacts navigate to the parent type, not the specific item referenced | bug | Navigation |
| F46 | Many artifact titles too long for sidebar — over 50 characters, some over 70 | ux | All artifact types |

#### Data Quality

| # | Finding | Type | Area |
|---|---------|------|------|
| F47 | [TASK-031](TASK-031) missing description field in frontmatter | data | Tasks |

### Round 5 Fixes Applied

| # | Finding | Fix |
|---|---------|-----|
| F46 | Long artifact titles | Shortened 23 titles across lessons, tasks, research, decisions, epics, and ideas. Added title guidelines to artifact-framework.md |
| F47 | [TASK-031](TASK-031) missing description | Added description to frontmatter |

### Systemic Themes (Round 5)

21. **Title discipline** — F46 revealed no guidance existed for title length. Added "Title and Description Guidelines" section to artifact-framework.md with 40-char target, 50-char limit, and human-descriptive style rules.

22. **Cross-link deep navigation** — F45 is an existing bug in ArtifactNav.svelte. The `$effect` that auto-selects artifacts from cross-links bails out for tree-structured types (`isTree === true`) and uses fragile label matching. Needs investigation as a separate task.

### Completion Criteria

- [ ] User has exercised all test areas
- [x] Round 1 findings captured as tasks and fixed
- [x] Round 2 findings captured as tasks and fixed
- [x] Round 2 architectural concerns [EPIC-044](EPIC-044) implemented
- [x] Round 3 findings investigated and fixed
- [x] Round 4 findings investigated and fixed
- [ ] Non-critical findings prioritised for future sprints

## Context

This epic addresses a need identified during project development.

## Tasks

Task breakdown to be defined.
