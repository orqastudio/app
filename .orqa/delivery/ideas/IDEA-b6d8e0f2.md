---
id: IDEA-b6d8e0f2
title: "Port allocation standardisation and CLI process ownership"
type: idea
status: promoted
description: "Standardise all service ports above 10000, fix daemon port mismatch, move process lifecycle management to CLI, extract search engine from MCP server, and demote dev controller to debug-only."
pillars:
  - PILLAR-569581e0
created: 2026-03-24
updated: 2026-03-24
relationships:
  - target: PERSONA-cda6edd6
    type: benefits
    rationale: "Serves the primary developer persona"
  - target: PILLAR-569581e0
    type: grounded
    rationale: "Standardised port allocation and CLI process ownership make infrastructure configuration visible and structured"
  - target: EPIC-a4c7e9b1
    type: realises
    rationale: "Promoted to epic for implementation"
---

## Context

OrqaStudio runs multiple services (daemon, MCP server, search engine, Vite dev server, Tauri app) with inconsistent port allocation. The daemon port is mismatched between CLI (3002) and MCP server (9258). The dev controller (`dev.mjs`) manages process lifecycle but should be demoted to debug-only tooling, with the CLI owning all process management.

Session on 2026-03-24 identified:
- All ports should move above 10000 to avoid conflicts with common development tools
- CLI should own process lifecycle (daemon, search, MCP)
- Fix daemon port mismatch (CLI starts on 3002, MCP expects 9258)
- Extract search engine from MCP server into separate process
- Dev controller becomes debug-only

## Research Questions

1. What ports are currently used across all config files?
2. What port ranges are safe (above 10000, avoiding registered IANA ports)?
3. How should CLI manage process lifecycle (start/stop/status for each service)?
4. What is the cleanest extraction boundary for the search engine from the MCP server?

## Expected Outcome

Epic with tasks covering port remapping, daemon fix, CLI process lifecycle, search extraction, and dev controller demotion.
