# Phase 0 Research

Tech stack research and decision documentation. Each research document captures findings, options evaluated, and the rationale for the chosen approach. Once a research topic results in a firm decision, it is promoted to an Architecture Decision in `docs/architecture/decisions.md`.

## Research Areas

| Document | Status | Topic | ADs Produced |
|----------|--------|-------|-------------|
| [Claude Integration](/research/claude-integration) | Complete | Agent SDK, Claude Max, tool strategy, streaming | AD-007, AD-008, AD-009, AD-010, AD-017 |
| [Tauri v2](/research/tauri-v2) | Complete | Capabilities, IPC design, security, plugins | AD-011, AD-012 |
| [Frontend](/research/frontend) | Complete | Markdown editing, conversation UI, panels, charts | AD-013 |
| [Persistence](/research/persistence) | Complete | SQLite schema, file/DB boundary, session model | AD-014 |
| [Onboarding](/research/onboarding) | Complete | Codebase scanning, governance format, progressive disclosure | AD-015, AD-016 |
| [Wireframing](/research/wireframing) | Complete | Wireframing tool selection (PlantUML Salt + D2) | — |
| [Design Tokens](/research/design-tokens) | Complete | Token format, runtime theming, extraction pipeline, per-project persistence | — |
| [Branding](/research/branding) | Complete | Forge brand identity, design system as governance artifact, brand-aware code generation, conflict resolution | — |

## Research-to-Decision Pipeline

```
Research → Findings → Decision → AD-NNN → Implementation
```

1. Research captures options, trade-offs, and evidence
2. When enough evidence exists, a recommendation is made
3. User approves the recommendation
4. An Architecture Decision (AD) is recorded in `docs/architecture/decisions.md`
5. The research document is updated to link to the AD

## Frontmatter Convention

All research documents use YAML frontmatter to enable future migration into Forge's research artifact system (a future phase). This frontmatter is parseable by `yaml-front-matter` / `comrak` and maps directly to the planned research artifact schema.

```yaml
---
type: research                              # Artifact type (always "research")
status: complete                            # open | in-progress | complete
date: 2026-03-02                            # Date created or last major update
category: claude-integration                # Kebab-case identifier (matches filename)
description: >                              # One-line summary for listing/search
  How Forge integrates with Claude's API.
questions:                                  # Structured list of research questions
  - id: Q1                                  # Stable identifier
    title: Claude Agent SDK vs Raw API      # Human-readable title
    status: resolved                        # open | in-progress | resolved
    verdict: >                              # One-line summary of the decision
      Agent SDK as primary integration.
open_questions:                             # Unresolved items (optional)
  - id: OQ1
    title: Java Runtime Dependency
    status: open                            # Always "open"
    description: >                          # What still needs to be determined
      Validate GraalVM native-image vs bundled JRE.
produces_decisions: [AD-007, AD-008]        # ADs promoted from this research
informs_phases: [0b, 0e, 1]                 # Roadmap phases this feeds into
informs_features: [F-002, F-003]            # MVP features this informs
---
```

### Field Reference

| Field | Required | Type | Description |
|-------|----------|------|-------------|
| `type` | Yes | string | Always `research` |
| `status` | Yes | enum | `open`, `in-progress`, `complete` |
| `date` | Yes | date | ISO date of creation or last major update |
| `category` | Yes | string | Kebab-case identifier matching the filename |
| `description` | Yes | string | One-line summary for listings and search |
| `questions` | Yes | array | Structured research questions with id, title, status, verdict |
| `open_questions` | No | array | Unresolved items requiring further investigation |
| `produces_decisions` | Yes | array | AD identifiers promoted from this research |
| `informs_phases` | Yes | array | Roadmap phase identifiers this research feeds into |
| `informs_features` | Yes | array | MVP feature identifiers (F-NNN) this research informs |

### Why This Convention

These markdown files will be migrated into Forge's research artifact system in a future phase. The YAML frontmatter provides:

1. **Machine-parseable metadata** — Forge can index these files into SQLite using the same `yaml-front-matter` + `comrak` pipeline used for `.claude/` governance artifacts
2. **Question-level tracking** — Each question has its own status, enabling partial-completion visibility
3. **Traceability** — `produces_decisions`, `informs_phases`, and `informs_features` create the upstream/downstream links needed for the decision traceability graph
4. **Search and filtering** — `category`, `status`, and `type` fields support the discovery dashboard
5. **Open question surfacing** — Unresolved items are explicitly tracked and can be surfaced in the discovery dashboard
