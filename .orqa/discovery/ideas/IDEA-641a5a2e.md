---
id: IDEA-641a5a2e
type: idea
title: "Artifact discussions — threaded conversations on artifacts with AI participation"
description: "Multi-user discussion threads attached to artifacts. Team members discuss decisions, review plans, debate approaches — with AI as a participant that can summarise, challenge, and surface relevant context."
status: captured
created: 2026-03-19
updated: 2026-03-19
research-needed:
  - Where do discussions live? As part of the artifact frontmatter? Separate files? SQLite?
  - If file-based, do discussions sync via git like everything else?
  - How does AI participate — as a named participant, or as an annotation layer?
  - Can AI summarise a long discussion thread into a decision artifact?
  - Can AI flag when a discussion contradicts an existing decision or rule?
  - Can AI surface relevant research or lessons when a discussion topic overlaps with existing knowledge?
  - How do discussions interact with the status model — does an unresolved discussion block status transitions?
  - How do discussions map to GitHub Discussions/PR reviews if the GitHub plugin is active?
  - Should discussions be a core platform feature or a plugin?
  - How do notifications work — how does a user know someone replied?
  - What's the minimum viable version — just comments on artifacts, or full threaded discussions?
relationships:
  - target: PILLAR-2acd86c1
    type: grounded
    rationale: Discussions are the collaborative learning loop — team members reflect together and AI surfaces patterns
  - target: PERSONA-c4afd86b
    type: benefits
---

# Artifact Discussions

Every artifact in the graph is a potential discussion point. A decision might need debate. A plan might need review. A lesson might need context from the person who experienced it.

## The Model

A discussion is a thread of messages attached to an artifact. Each message has:
- **Author** — a human user or the AI
- **Timestamp**
- **Content** — markdown
- **Type** — comment, question, approval, objection, summary

## AI as Participant

AI doesn't just observe discussions — it participates:

- **Summarise** — condense a long thread into key points and open questions
- **Challenge** — flag when a proposal contradicts an existing decision or rule
- **Connect** — surface related research, lessons, or ideas from the graph
- **Draft** — propose artifact content based on what's been discussed
- **Remind** — "this discussion has been open for 5 days with no resolution"

AI contributions are clearly labelled as AI-generated and can be dismissed.

## Where Discussions Live

Two options:

1. **File-based** (alongside artifacts) — discussions are markdown files in a `discussions/` subdirectory. Sync via git. Full history. But file-per-message creates noise.

2. **SQLite** (like conversations) — discussions stored in the database. Faster, richer querying, real-time. But don't sync via git without additional infrastructure.

The hybrid might be: discussions live in SQLite for speed, but are periodically serialised to markdown for git-based sync and backup. The serialised form is a single file per artifact's discussion thread.

## Interaction with Status Model

An unresolved objection on an artifact could block its status transition. For example:
- A task can't move from `review` to `completed` if there's an unresolved objection
- A decision can't move from `exploring` to `active` if there are open questions

This makes discussions a governance mechanism, not just communication.