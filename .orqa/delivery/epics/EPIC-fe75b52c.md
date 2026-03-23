---
id: "EPIC-fe75b52c"
type: epic
title: "AI-Mediated Artifact Editing"
description: "Build an AI-mediated editing experience where creates, edits, and deletes are proposals sent to the AI conversation panel. The AI analyses cross-reference implications and suggests cascading updates to keep artifacts in sync."
status: "captured"
priority: "P1"
created: "2026-03-07"
updated: "2026-03-11"
horizon: "next"
scoring:
  impact: 5
  urgency: 3
  complexity: 5
  dependencies: 4
relationships:
  - target: "DOC-01ddd8aa"
    type: "documented-by"
  - target: MS-654badde
    type: fulfils
    rationale: "Epic fulfils this milestone"
---
## Why P1

Can't manage governance in-app without editing. Currently requires switching to a text editor. But raw CRUD is insufficient — artifact mutations have cross-referencing implications that users can't track manually. AI-mediated editing turns every change into an informed decision.

## Context

Originally scoped as a CodeMirror-based editing UI, this epic was briefly absorbed into [EPIC-6787bb93](EPIC-6787bb93). It is now restored as a standalone epic with a fundamentally different approach: artifact mutations (create, edit, delete) are **proposals** sent to the AI conversation panel, not direct file operations.

When a user wants to create, modify, or delete an artifact, the action is formatted as a structured prompt and sent to the AI. The AI analyses the artifact graph to understand cross-reference implications — what depends on this artifact, what references it, whether changing its status would affect epic completion, milestone progress, or task dependencies — and responds with a proposal that includes the requested change plus any suggested cascading updates.

This approach surpasses traditional editing tools because the AI acts as a governance-aware assistant that understands the implications of every change.

## Design Principles

> Artifact mutations are proposals, not commands. The AI evaluates implications before changes are applied. — User direction, 2026-03-11

> The conversation panel is the editing interface. Structured prompts trigger AI analysis and proposal generation. — User direction, 2026-03-11

## Implementation Design

Research and detailed design to be conducted. Key areas to investigate:

- Structured prompt format for create/edit/delete proposals
- AI system prompt context: artifact graph summary, schema constraints, cross-references
- Proposal display: how the AI presents suggested changes (diff view, change list, affected artifacts)
- Approval flow: user reviews and approves/rejects the proposal
- Execution: approved proposals applied via existing backend CRUD commands
- Cascading updates: AI identifies and proposes updates to related artifacts

## Tasks

Task breakdown to be created during planning.