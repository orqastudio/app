---
id: IDEA-c7a3f1e2
type: discovery-idea
title: "Evaluate agent framework alternatives for multi-agent orchestration"
description: "Evaluate external frameworks (Optio, NanoPM) against OrqaStudio's multi-agent governance model"
status: captured
priority: medium
created: "2026-03-29"
tags:
  - agent-framework
  - orchestration
  - evaluation
relationships:
  - type: benefits
    target: PERSONA-c4afd86b
---

## Context

Evaluate external agent/PM frameworks against OrqaStudio's P6 hub-spoke orchestration model and multi-agent governance architecture.

## Candidates

### Optio

https://github.com/jonwiggins/optio

Agent framework — evaluate for multi-agent coordination patterns.

### NanoPM

https://github.com/nmrtn/nanopm

Lightweight project management framework — evaluate for governance artifact management and workflow orchestration patterns.

### ProofShot

https://github.com/AmElmo/proofshot

Visual proof/screenshot tool — evaluate for use as part of the app framework review process (UI verification, visual regression).

## Evaluation Criteria

- How does it compare to Claude Code agent teams for hub-spoke coordination?
- Does it support ephemeral task-scoped workers (P2)?
- Can it enforce role-based tool constraints?
- Does it support structured summary handoff between agents?
- How does it handle persistent orchestrator + ephemeral workers pattern?
- What patterns can be adopted without replacing the current architecture?

## Relationship to Current Architecture

OrqaStudio currently uses CC agent teams (TeamCreate/Agent/SendMessage) for multi-agent work. This maps to the P6 hub-spoke model. These frameworks may offer patterns or ideas that improve the current approach.
