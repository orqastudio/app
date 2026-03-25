---
id: TASK-c3ce6480
type: task
title: "Create documentation placement guide in core-framework plugin"
description: "Create a documentation page explaining where docs live, the docs-knowledge pairing rule, and how to decide which location is correct for a new document. This is production-app documentation that ships with the core-framework plugin."
status: ready
created: 2026-03-24
updated: 2026-03-24
acceptance:
  - A documentation page exists in plugins/core/ explaining the three documentation locations
  - The page explains the docs-knowledge pairing rule (docs = user-facing, knowledge = agent-facing)
  - The page includes a decision flowchart or table for choosing the correct location
  - The page covers core-framework (production features), project (.orqa/ — development), and plugin-specific locations
  - A matching knowledge artifact exists in plugins/core/knowledge/ for agent context injection
relationships:
  - target: EPIC-9e3d320b
    type: delivers
    rationale: "Establishes the documentation placement architecture that all other docs tasks follow"
  - target: IMPL-08d70280
    type: addresses
    rationale: "Implements the lesson about docs-knowledge pairing and placement architecture"
---

## What

Create a guide that explains the documentation placement architecture for OrqaStudio. This is a recurring source of confusion: agents and contributors don't know whether a new doc belongs in the core-framework plugin, the project's `.orqa/` directory, or a domain-specific plugin.

## Required Content

### Documentation Page (user-facing)

1. **Three Documentation Locations**

   | Location | What Belongs | Examples |
   |----------|-------------|---------|
   | Core-framework plugin (`plugins/core/`) | Production app docs — features, ports, runtime config | Port allocation, service architecture, app configuration |
   | Project (`.orqa/`) | Development docs — architecture decisions, process, governance, coding standards | Coding standards, workflow guide, artifact framework |
   | Plugin-specific (`plugins/<name>/`) | Plugin domain docs | Svelte patterns (in svelte plugin), Tauri patterns (in tauri plugin) |

2. **The Pairing Rule** — Documentation and knowledge ALWAYS come in pairs:
   - Documentation (`docs/`) = user-facing. Written for humans.
   - Knowledge (`knowledge/`) = agent-facing. Structured for injection into agent context.
   - Creating one without the other is incomplete work.

3. **Decision Flowchart** — "Where does my new doc go?"
   - Does it describe a feature that ships with the production app? → `plugins/core/`
   - Does it describe how the team develops the app? → `.orqa/`
   - Does it describe patterns specific to a technology plugin? → `plugins/<name>/`

### Knowledge Artifact (agent-facing)

A matching `KNOW-NNN.md` in `plugins/core/knowledge/` that agents can load when creating new documentation, ensuring they place it correctly and create both the doc and knowledge pair.

## Verification

1. The documentation page renders correctly and is discoverable via search
2. The knowledge artifact is loadable by agents
3. Both artifacts cross-reference each other
