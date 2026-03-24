---
id: TASK-b7c8d9e0
title: "Create canonical port allocation reference doc"
type: task
description: "Create a knowledge artifact AND a matching documentation page in the core-framework plugin (plugins/core/) as the single source of truth for all OrqaStudio service port assignments. Docs and knowledge come in pairs: docs = user-facing, knowledge = agent-facing. This is framework-level infrastructure, not project-specific."
status: ready
created: 2026-03-24
updated: 2026-03-24
acceptance:
  - A new KNOW-NNN.md exists in plugins/core/knowledge/ with the complete port allocation table (agent-facing)
  - A matching documentation page exists in plugins/core/docs/ or plugins/core/documentation/ (user-facing) — docs and knowledge come in pairs
  - The knowledge artifact includes all services, current ports, new ports, config locations, and ORQA_PORT_BASE configuration
  - The documentation page includes a human-readable explanation of port allocation, how to configure, and how to add new services
  - Both artifacts include the architecture constraints (separate processes, kill-on-conflict, PID files, health checks)
  - DOC-46ffe1d2 (Dev Controller) is updated to reference the new port doc instead of hardcoding port 3001
  - DOC-65eb8303 (Dev Environment Setup) references the port doc for service configuration
  - Both artifacts are discoverable via search and linked from related docs
relationships:
  - target: EPIC-a4c7e9b1
    type: delivers
    rationale: "Creates the canonical port reference that all other tasks and docs reference"
  - target: TASK-71a2d3e4
    type: depends-on
    rationale: "Port allocation must be finalized before documenting it"
---

## What

The port allocation table in the epic body is an implementation plan. Developers need a permanent reference document they can look up when configuring services, debugging port conflicts, or adding new services. This doc becomes the single source of truth.

## Location

Two paired artifacts in the core-framework plugin:

1. **Knowledge** (`plugins/core/knowledge/KNOW-<id>.md`) — agent-facing. Injected into agent context when working on service configuration. Structured for machine consumption.
2. **Documentation** (`plugins/core/docs/<name>.md` or `plugins/core/documentation/<name>.md`) — user-facing. Human-readable explanation of port allocation, configuration, and procedures.

## Required Content

1. **Port Allocation Table** — complete inventory from the epic, including:
   - Service name
   - Port number
   - Transport (HTTP, TCP, stdio, random)
   - Config locations (all files that reference this port)
   - Notes (e.g., "keep as-is", "TCP debug only")

2. **ORQA_PORT_BASE** — how the configurable base works, default value, how to override for multiple instances

3. **Architecture Constraints** — the 5 NON-NEGOTIABLE rules from the epic:
   - Separate processes
   - Kill-on-conflict
   - PID file lifecycle
   - Health check endpoints
   - Configurable port base

4. **Adding a New Service** — procedure for allocating a port to a new service (pick next available offset, update this doc, update config)

5. **Cross-references** — links to DOC-46ffe1d2 (dev controller), DOC-357f8d7c (commands), DOC-65eb8303 (dev setup)

## Verification

1. `search_semantic` for "port allocation" or "service ports" finds this doc
2. DOC-46ffe1d2 references this doc for port 10401 instead of hardcoding 3001
3. The doc matches the actual port constants in code after TASK-71a2d3e4 is complete
