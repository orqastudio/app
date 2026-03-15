# OrqaStudio Architecture Working Document

*Created 2026-03-15 — for holistic discussion across sessions*

---

## The Foundation

Everything is a node on a graph. Nodes connect through typed relationships. The graph IS the thinking made visible. That's the product.

## Core Types (Universal, Always Exist)

These represent universal concepts in structured thinking. Every project has them regardless of domain.

| Type | Purpose |
|---|---|
| Pillars | What we believe — guiding principles |
| Vision | Where we're going |
| Personas | Who we serve |
| Grounding | What anchors agent behavior |
| Ideas | Entry point for any change — to the project, process, or principles |
| Decisions | Resolutions that direct work and authorise enforcement |
| Rules | Standards we enforce |
| Lessons | What we've learned |
| Skills | Knowledge we teach |
| Agents | Roles that do work |

**Ideas** are the universal entry point. Where an idea goes depends on what it is — it might evolve into delivery work, or drive a decision that governs a rule. The idea is always preserved as the record of original thinking.

**Decisions** are the bridge. They appear in multiple views because they serve multiple roles.

Labels and icons on core types are configurable per project for contextual clarity. A consulting project might call "Rules" → "Standards" and "Lessons" → "Insights." The canonical type key stays the same.

## Project Types (Configurable)

Everything beyond the core is project-specific:

**Discovery types** — ways of informing ideas and decisions. Software: research, wireframes. Consulting: client discovery, stakeholder maps. Research: literature review, experiments.

**Delivery types** — ways of acting on ideas and decisions. Software: milestones, epics, tasks. Consulting: phases, workstreams, deliverables. Personal: goals, actions.

Labels, icons, hierarchy depth, and status aliases are all project-configurable. A research project might use a flask icon for experiments and a microscope for observations.

## Relationship Vocabulary

Relationships are the ONLY way artifacts connect. Each has a clear semantic:

| Relationship | Inverse | Meaning | Example |
|---|---|---|---|
| `informs` | `informed-by` | Supplementary context | Research informs an idea |
| `evolves-into` | `evolves-from` | Thinking becomes action | Idea evolves into an epic |
| `drives` | `driven-by` | Decision directs work | Decision drives an epic's design |
| `governs` | `governed-by` | Decision authorises enforcement | Decision governs a rule |
| `delivers` | `delivered-by` | Work contributes to a goal | Task delivers to an epic |
| `enforces` | `enforced-by` | Mechanical enforcement | Rule enforces a standard |
| `grounded` | `grounded-by` | Foundational anchor | Agent grounded by a pillar |
| `observes` | `observed-by` | Learning captured | Lesson observes a pattern |

No standalone frontmatter fields for connections. No `epic: EPIC-045`. Only relationships.

## How Ideas Evolve

An idea doesn't "promote" — it **evolves**. And it can evolve in multiple directions:

1. Idea → `evolves-into` → delivery artifact (epic, task, experiment)
2. Idea → `evolves-into` → decision → `governs` → rule
3. Idea → `evolves-into` → decision → `drives` → delivery artifact
4. Idea → status moves to `completed` (the thinking is done, the output is connected)

Research and other discovery types `inform` ideas and decisions. They're supplementary input, not the driver.

## Sections Are Graph Filters

The nav sections don't categorise artifacts. They filter the graph by relationship patterns. An artifact appears in every section its relationships qualify it for.

| Section | Shows | Label for Decisions |
|---|---|---|
| **Principles** | Pillars, Vision, Personas, Grounding | — |
| **Discovery** | Ideas, Decisions, project discovery types | "Decisions" (all) |
| **Learning** | Rules, Lessons, Skills, Agents, + decisions with `governs` edges | "Governing Decisions" |
| **Delivery** | Project delivery types + decisions with `drives` edges | "Driving Decisions" |

Decisions appear in Discovery (where they're made), Learning (when they govern rules), and Delivery (when they drive work). The contextual label explains WHY the decision appears in each section.

## State Machine

Status is a property of each node. Transitions are graph queries.

**Universal statuses**: captured → exploring → ready → prioritised → active → hold → blocked → review → completed → surpassed → recurring

Projects can define **label aliases** for display (key stays canonical). Icons are configurable per status.

**Auto-rules are graph queries:**
```json
{
  "condition": "all-related-in-status",
  "relationship": "delivers",
  "status": "completed",
  "target": "review"
}
```

"When all nodes connected via `delivers` relationships are in `completed` status → propose transitioning this node to `review`."

The state machine isn't a separate system — it's rules about which transitions are valid given the current graph state.

## Views Are Graph Queries

| View | What it renders |
|---|---|
| **Roadmap** | Graph filtered to delivery types, grouped by hierarchy, status as columns |
| **Dashboard** | Aggregate queries: health, status distribution, attention needed, trends |
| **Scratchpad** | Ideas and discovery types on a spatial canvas — each item IS a graph node |
| **Artifact viewer** | Single node with its relationships |
| **Full graph** | Everything |

## Key Design Principles

1. **The graph is the only data structure.** No standalone fields, no side channels.
2. **Sections are views, not categories.** The graph doesn't have sections. Views filter by relationship patterns.
3. **Core types are universal thinking concepts.** They exist in every project, every domain.
4. **Project types are configurable.** Discovery and delivery types adapt to the domain.
5. **The app enforces structure.** State machine rules, graph integrity, and status transitions are app-level enforcement, not AI rules.
6. **AI knows the system through app-shipped docs.** Uneditable conventions, loaded into agent context.
7. **Project rules are project-specific.** Editable by the project, enforce project standards.

---

## Key Architectural Decisions Made This Session

- **AD-049**: Status represented by icons, colors reserved for artifact types
- **AD-050**: Status transitions are config-driven
- **AD-051**: Three-layer configurability — core types universal, instances project-specific, delivery fully configurable
- **IDEA-105**: Delivery pipeline as a future plugin
- **IDEA-106**: Principles/Discovery/Learning section split
- **IDEA-107**: App-shipped conventions vs project-level rules

## Session Artifacts Created

### Epics Completed
- EPIC-064: Enforcement bootstrapping (15 tasks)
- EPIC-073: UAT round 3 (19+ tasks)
- EPIC-074: Dashboard redesign (5 tasks)
- EPIC-075: Documentation reorganisation (6 tasks)
- EPIC-077: Automated status transitions (5 tasks)
- EPIC-078: Configuration-driven delivery pipeline (5 tasks)

### Epics Created (not started)
- EPIC-076: Graph analysis with Cytoscape.js (6 tasks)

### Ideas Captured
- IDEA-095 through IDEA-107 (13 ideas)

---

## Reconnection Instructions

To restore the full governance system after this holistic discussion:

1. **Restore CLAUDE.md, rules, agents, skills:**

   ```bash
   cp .claude/_backup/CLAUDE.md.bak .claude/CLAUDE.md
   mv .claude/_backup/rules .claude/rules
   mv .claude/_backup/agents .claude/agents
   mv .claude/_backup/skills .claude/skills
   rmdir .claude/_backup
   ```

2. **Or just start a new session:**
   The orqastudio-claude-plugin's `session-start.sh` hook recreates these
   from `.orqa/` source of truth on every session start. The `_backup`
   directory can then be deleted.

3. **Clean up:**

   ```bash
   rm WORKING-DOCUMENT.md  # Delete this file when discussion is complete
   ```
