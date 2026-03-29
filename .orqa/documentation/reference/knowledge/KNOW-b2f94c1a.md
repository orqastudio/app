---
id: KNOW-b2f94c1a
type: knowledge
title: Artifact Principles — Agent Grounding
description: "Concise grounding document for Orchestrator, Writer, Researcher, and Governance Steward roles. Injected into agent context at initialization."
status: active
tier: always
created: 2026-03-29
---

## What Good Artifacts Look Like

Good artifacts are structured, connected, and truthful. Structured means they have valid frontmatter that conforms to their schema — every required field present, every enum value valid. Connected means every relationship has a corresponding inverse on the target artifact. Truthful means the status field reflects reality, not aspiration.

An artifact that claims `status: done` when it is partially complete is worse than an artifact that says it is in progress. The graph is only as useful as the data it contains.

## The Graph Is the Point

Artifacts are not files. They are nodes in a graph. Every `epic` field, every `depends-on`, every `research-refs`, every `relationships` entry is an edge. The graph is what makes artifacts discoverable — when a task is started, the graph is traversed to load its epic, its docs, its skills. If the graph is broken, the context injection fails, and agents work blind.

**Orphaned artifacts are failures.** An epic without a milestone reference is disconnected from the plan. A task without an epic reference cannot be prioritised. A research document not referenced by any epic's `research-refs` is lost knowledge.

**Every relationship has an inverse.** If epic EPIC-797972a7 says it is `informed-by` RES-a5d10705, then RES-a5d10705 must say it `informs` EPIC-797972a7. One-sided relationships indicate a broken edge. The pre-commit hook catches this. Do not bypass it.

## Creating Artifacts With Purpose

Before creating an artifact, ask: what does this connect to? What does it produce? What gates does it satisfy? An artifact created in isolation — no milestone, no research-refs, no docs-required — is planning theater, not planning.

Status transitions are gates, not bookkeeping. A task moving from `todo` to `in-progress` means its dependencies are done. An epic moving from `draft` to `ready` means its `docs-required` items exist. These gates enforce the principle that understanding precedes action. Skipping them destroys that guarantee.

Frontmatter is the contract between the artifact and the system. Fields not in the schema are rejected. Required fields must be present. Enum values must be valid. This is enforced at commit time by schema validation. Writing frontmatter that doesn't conform wastes everyone's time.

## What Goes Wrong

**Creating artifacts without relationships.** An epic created without `research-refs`, `docs-required`, and `docs-produced` is missing its purpose metadata. The system cannot use it to gate work, load context, or verify completion. Populate the relationship fields before marking the artifact ready.

**Treating frontmatter as paperwork.** Every field exists for a reason — to power graph traversal, context injection, or status gate enforcement. When you fill in a field carelessly (a `depends-on` that doesn't match a real task ID, a `evolves-into` that doesn't exist), you corrupt the graph. Future agents load the wrong context or pass the wrong gates.

**Breaking the graph under volume pressure.** When many artifacts need to be created quickly, the temptation is to stub them out and fill in relationships later. Later never comes. Create fewer artifacts correctly rather than many artifacts incompletely. A sparse graph is navigable. A graph with broken edges is not.
