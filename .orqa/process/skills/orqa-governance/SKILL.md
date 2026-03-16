---
id: SKILL-011
title: Orqa Governance Patterns
description: |
  OrqaStudio governance patterns: artifact types, scanning pipeline, lesson promotion,
  rule enforcement, frontmatter schemas, and .orqa/ directory structure.
  Use when: Working with governance artifacts (docs, research, lessons, rules),
  modifying scanning or enforcement, or maintaining the .orqa/ directory.
status: active
created: 2026-03-01
updated: 2026-03-10
layer: project
category: domain
file-patterns:
  - ".orqa/**"
version: 2.0.0
user-invocable: true
relationships:
  - target: PILLAR-001
    type: grounded
    rationale: Artifact types, frontmatter schemas, and scanning pipelines make governance decisions browsable and enforceable in the app
  - target: TASK-004
    type: grounded
  - target: TASK-005
    type: grounded
  - target: TASK-006
    type: grounded
  - target: TASK-010
    type: grounded
  - target: TASK-011
    type: grounded
  - target: TASK-012
    type: grounded
  - target: TASK-019
    type: grounded
  - target: TASK-021
    type: grounded
  - target: TASK-030
    type: grounded
  - target: TASK-032
    type: grounded
  - target: TASK-033
    type: grounded
  - target: TASK-034
    type: grounded
  - target: TASK-035
    type: grounded
  - target: TASK-036
    type: grounded
  - target: TASK-040
    type: grounded
  - target: TASK-044
    type: grounded
  - target: TASK-046
    type: grounded
  - target: TASK-047
    type: grounded
  - target: TASK-048
    type: grounded
  - target: TASK-049
    type: grounded
  - target: TASK-050
    type: grounded
  - target: TASK-051
    type: grounded
  - target: TASK-052
    type: grounded
  - target: TASK-053
    type: grounded
  - target: TASK-054
    type: grounded
  - target: TASK-055
    type: grounded
  - target: TASK-056
    type: grounded
  - target: TASK-057
    type: grounded
  - target: TASK-058
    type: grounded
  - target: TASK-059
    type: grounded
  - target: TASK-060
    type: grounded
  - target: TASK-061
    type: grounded
  - target: TASK-062
    type: grounded
  - target: TASK-063
    type: grounded
  - target: TASK-064
    type: grounded
  - target: TASK-065
    type: grounded
  - target: TASK-066
    type: grounded
  - target: TASK-067
    type: grounded
  - target: TASK-068
    type: grounded
  - target: TASK-070
    type: grounded
  - target: TASK-071
    type: grounded
  - target: TASK-072
    type: grounded
  - target: TASK-080
    type: grounded
  - target: TASK-081
    type: grounded
  - target: TASK-164
    type: grounded
  - target: TASK-171
    type: grounded
  - target: TASK-172
    type: grounded
  - target: TASK-173
    type: grounded
  - target: TASK-174
    type: grounded
  - target: TASK-178
    type: grounded
  - target: TASK-179
    type: grounded
  - target: TASK-183
    type: grounded
  - target: TASK-184
    type: grounded
  - target: TASK-186
    type: grounded
  - target: TASK-211
    type: grounded
  - target: TASK-212
    type: grounded
  - target: TASK-213
    type: grounded
  - target: TASK-214
    type: grounded
  - target: TASK-216
    type: grounded
  - target: TASK-220
    type: grounded
  - target: TASK-221
    type: grounded
  - target: TASK-222
    type: grounded
  - target: TASK-406
    type: grounded
  - target: TASK-409
    type: grounded
  - target: TASK-411
    type: grounded
  - target: TASK-412
    type: grounded
  - target: TASK-413
    type: grounded
  - target: TASK-414
    type: grounded
  - target: TASK-460
    type: grounded
  - target: TASK-461
    type: grounded
  - target: TASK-462
    type: grounded
  - target: TASK-463
    type: grounded
  - target: TASK-464
    type: grounded
  - target: TASK-465
    type: grounded
  - target: TASK-466
    type: grounded
  - target: TASK-467
    type: grounded
  - target: TASK-474
    type: grounded
  - target: TASK-475
    type: grounded
  - target: TASK-477
    type: grounded
  - target: AGENT-008
    type: grounded
---


OrqaStudio's governance layer manages documentation, research, lessons, rules, agents, skills, and hooks as browsable, scannable artifacts. Understanding this system is critical for anyone working on the governance features.

## .orqa/ Directory Structure

```
.orqa/
  project.json              # Project config (name, artifacts array, default model, etc.)
  icon.svg                  # Project icon
  documentation/            # Documentation (tree structure with subdirectories)
    architecture/           #   Architecture docs
    development/            #   Development guides
    process/                #   Process docs
    product/                #   Product docs
    ui/                     #   UI specs
  planning/                 # Planning artifacts
    pillars/                #   PILLAR-NNN.md (guiding principles)
    ideas/                  #   IDEA-NNN.md
    research/               #   Research documents (investigations, designs, spikes)
    milestones/             #   MS-NNN.md
    epics/                  #   EPIC-NNN.md (contain implementation design in body)
    tasks/                  #   TASK-NNN.md
  governance/               # Governance artifacts
    lessons/                #   IMPL-NNN.md
    decisions/              #   AD-NNN.md
    rules/                  #   RULE-NNN.md
    hooks/                  #   Hook scripts
  team/                     # Team artifacts
    agents/                 #   Agent definitions
    skills/                 #   Skill directories (each has SKILL.md)
```

## Artifact Traceability Chain

```
Task (TASK-NNN) → Epic (EPIC-NNN) → Milestone (MS-NNN)
                       ↑
                  research-refs → Research (RES-NNN)

Lesson (IMPL-NNN) --grounded-by--> Rule (RULE-NNN)
Rule (RULE-NNN)   --observes-->    Lesson (IMPL-NNN)
```

- **Tasks** always have `epic:` field referencing an existing EPIC-NNN
- **Epics** always have `milestone:` field referencing an existing MS-NNN
- **Epics** may have `research-refs:` array linking to RES-NNN documents
- **Research** documents are investigations, design explorations, and spikes
- **All governance artifacts** have a `relationships` array with bidirectional pipeline connections
- **There is NO "Plan" artifact type** — epics contain implementation design in their body

### FORBIDDEN

- `plan:` field on any artifact — this field is deprecated and removed
- Creating files in `.orqa/delivery/plans/` — this directory no longer exists
- Tasks without an `epic:` field
- Epics without a `milestone:` field

## Config-Driven Artifact Scanning

The `artifacts` array in `project.json` is the SINGLE SOURCE OF TRUTH for what gets scanned and displayed. The scanner does NOT guess — it reads config and scans exactly those paths.

### Config Schema

```jsonc
"artifacts": [
  // Direct type — scans a directory (flat or tree)
  { "key": "docs", "label": "Documentation", "icon": "file-text", "path": ".orqa/documentation" },
  // Group — renders as expandable group, each child scanned independently
  { "key": "planning", "label": "Planning", "icon": "target",
    "children": [
      { "key": "ideas", "label": "Ideas", "path": ".orqa/delivery/ideas" },
      { "key": "research", "label": "Research", "path": ".orqa/delivery/research" },
      { "key": "epics", "label": "Epics", "path": ".orqa/delivery/epics" }
    ]
  }
]
```

### Scanning Behavior (File Explorer Pattern)

The scanner recursively walks directories like a file explorer:

1. **Flat directories** — Scans `.md` files directly (milestones, epics, etc.)
2. **Tree directories** — Recurses into subdirectories, creating `DocNode` entries with `children` for folders
3. **Frontmatter extraction** — Every `.md` file gets YAML frontmatter parsed for `title` (→ label) and `description`
4. **Label priority**: frontmatter `title` > `humanize_name(filename)` > raw filename
5. **Artifact IDs** (all-caps like `[EPIC-001](EPIC-001)`, `[AD-015](AD-015)`) are preserved as-is, not humanized
6. **README.md** is navigation metadata, skipped as browsable artifact at all levels
7. **Hidden entries** (`.` or `_` prefix) skipped
8. **Empty directories** omitted from tree

### Critical Rule: Config Paths Must Match Disk

Every `path` in the config must resolve to an actual directory. Moving files on disk requires updating the config. See `.orqa/process/rules/[RULE-003](RULE-003).md`.

## Directory README Format

Every artifact directory and group directory has a `README.md` that provides navigation metadata for the UI. READMEs are NOT browsable artifacts — they are skipped by the scanner.

### Group README (parent directories)

```yaml
---
role: group
label: "Planning"
description: "Strategic planning artifacts."
icon: "clipboard-list"
sort: 2
---

Body text describing what this group contains.
```

### Artifact README (leaf directories)

```yaml
---
role: artifacts
label: "Epics"
description: "Trackable work units that group related tasks together."
icon: "layers"
sort: 2
---

# Epics

Description, lifecycle diagram, key concepts, and Related section.
```

### Fields

| Field | Required | Values | Purpose |
|-------|----------|--------|---------|
| `role` | Yes | `group` or `artifacts` | Group = parent with children, artifacts = scannable leaf directory |
| `label` | Yes | string | Display name in nav sidebar |
| `description` | Yes | string | Tooltip/subtitle in nav |
| `icon` | Yes | string | Lucide icon name (e.g., `layers`, `target`, `compass`) |
| `sort` | Yes | integer | Display order within parent (0 = first) |

### Body Structure (artifact READMEs)

1. **Heading** matching the label
2. **One-paragraph description** of what this artifact type is
3. **Lifecycle** section with status flow diagram
4. **Key concepts** — what makes a good artifact of this type, gates, relationships
5. **Related** section linking to connected artifact types

### When to Create/Update a README

- **New artifact directory**: Create a README before adding any artifacts
- **New artifact type registered in project.json**: Create matching README
- **Renaming or moving a directory**: Update the README's label and description
- **Changing the artifact's lifecycle or schema**: Update the README to match

## Artifact Frontmatter Schemas

All governance artifacts use YAML frontmatter parsed by a generic function.

### Epic Frontmatter (key artifact)

```yaml
---
id: EPIC-NNN
layer: project
title: "Epic Title"
status: draft | ready | in-progress | review | done
milestone: MS-NNN
priority: P1 | P2 | P3
research-refs:          # Optional — links to research documents
  - research-doc-name
scoring:
  pillar: 0-5
  impact: 1-5
  dependency: 1-3
  effort: 1-5
  score: computed
docs-required: []       # Docs that must exist before implementation
docs-produced: []       # Docs this work creates/updates
depends-on: []
blocks: []
description: >
  What this epic delivers.
tags: []
---

## Implementation Design

[The epic body contains the implementation design that previously lived
in a separate plan document. Data model, IPC contracts, component
breakdown, and approach all go here.]
```

### Task Frontmatter

```yaml
---
id: TASK-NNN
title: "Task Title"
status: todo | in-progress | done
epic: EPIC-NNN          # REQUIRED — always references an epic
created: YYYY-MM-DD
updated: YYYY-MM-DD
assignee: agent-name
skills: [skill1, skill2]
  - file/paths
acceptance:
  - criteria
tags: []
---
```

### Research Frontmatter

```yaml
---
id: RES-NNN
title: "Research Title"
description: "Brief description"
status: draft | complete | surpassed
created: YYYY-MM-DD
updated: YYYY-MM-DD
surpassed-by: RES-NNN          # Set when status: surpassed
---
```

### Rule Frontmatter

```yaml
---
id: RULE-NNN
title: "Rule Title"
description: "What this rule enforces"
status: active
created: YYYY-MM-DD
updated: YYYY-MM-DD
layer: core | project
relationships:
  - type: grounded
    target: PILLAR-NNN
    rationale: "Why this rule serves this pillar"
  - type: observes
    target: IMPL-NNN
    rationale: "Lesson that prompted this rule"
---
```

### Lesson Frontmatter

```yaml
---
id: IMPL-NNN
title: "Lesson Title"
description: "Brief description of the lesson"
status: active
created: YYYY-MM-DD
updated: YYYY-MM-DD
maturity: observation | understanding
recurrence: 0
relationships:
  - type: grounded
    target: AD-NNN
    rationale: "Decision this lesson informs"
---
```

## Artifact Status Workflows

### Epic: `draft → ready → in-progress → review → done`
### Task: `todo → in-progress → done`
### Research: `draft → complete → surpassed`
### Decision: `proposed → accepted → superseded` (or `→ deprecated`)
### Idea: `captured → exploring → shaped → promoted` (or `→ archived`)
### Milestone: `planning → active → complete`

## Historical Artifact Preservation

- **Documentation** (`.orqa/documentation/`) — DELETE when outdated, replace with current
- **Research, tasks** — PRESERVE, mark `status: surpassed` with `surpassed-by` reference
- **Never delete** research or task files — they are historical records

## Lesson Pipeline

```
Lesson documented (.orqa/process/lessons/IMPL-NNN.md)
    → Recurrence tracked (frontmatter count field incremented)
    → Promoted at threshold (recurrence >= 2)
    → Becomes rule or coding standard addition
    → Enforcement verified
```

## Pillar Alignment

Active pillars are defined in `.orqa/process/pillars/PILLAR-NNN.md`. Every governance artifact and feature must serve at least one active pillar. To evaluate alignment, read each pillar's `gate` questions and check if the work can answer "yes" to at least one question from at least one pillar.

Pillars are equal in importance — when they conflict, flag the conflict to the user and ask for direction.

Features that serve no active pillar are out of scope.

## Key Files

| File | Purpose |
|------|---------|
| `.orqa/project.json` | Project configuration (includes `artifacts` array) |
| `.orqa/process/pillars/` | Product pillars (PILLAR-NNN.md) — guiding principles |
| `.orqa/process/lessons/` | Implementation lessons (IMPL-NNN.md) |
| `.orqa/process/decisions/` | Architecture decisions (AD-NNN.md) |
| `.orqa/process/rules/` | Governance rules |
| `.orqa/process/hooks/` | Hook scripts |
| `.orqa/delivery/ideas/` | Ideas (IDEA-NNN.md) |
| `.orqa/delivery/research/` | Research documents (investigations, designs, spikes) |
| `.orqa/delivery/milestones/` | Milestones (MS-NNN.md) |
| `.orqa/delivery/epics/` | Epics (EPIC-NNN.md) — contain implementation design |
| `.orqa/delivery/tasks/` | Tasks (TASK-NNN.md) — always reference an epic |
| `.orqa/process/agents/` | Agent definitions |
| `.orqa/process/skills/` | Skill definitions |
| `.orqa/documentation/` | Documentation tree (subdirs: architecture, product, etc.) |
| `backend/src-tauri/src/domain/artifact.rs` | Frontmatter parsing, artifact types |
| `backend/src-tauri/src/domain/artifact_reader.rs` | Config-driven recursive scanner |
| `backend/src-tauri/src/commands/artifact_commands.rs` | Tree scan and read commands |
| `backend/src-tauri/src/domain/project_settings.rs` | Project settings + ArtifactEntry config types |
