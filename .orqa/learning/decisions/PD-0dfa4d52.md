---
id: "PD-0dfa4d52"
type: principle-decision
title: "Governance Artifact Format"
description: "Governance artifacts use native Claude Code format (markdown + YAML frontmatter). OrqaStudio-specific metadata lives only in SQLite. Superseded by PD-4ea9a290 which moves source of truth from .claude/ to .orqa/."
status: archived
created: 2026-03-02T00:00:00.000Z
updated: 2026-03-08T00:00:00.000Z
relationships:
  - target: "PD-4ea9a290"
    type: "evolves-into"
    rationale: "PD-4ea9a290 moves source of truth from .claude/ to .orqa/, superseding the .claude/-native format"
  - target: "EPIC-05ae2ce7"
    type: "drives"
---

## Decision

OrqaStudio reads and writes governance artifacts as native Claude Code artifacts in the exact `.claude/` format (markdown with YAML frontmatter for agents/skills, pure markdown for rules, JSON for settings). All `.claude/` files created by OrqaStudio work identically in Claude Code CLI — there is no OrqaStudio-specific format or metadata embedded in the files. OrqaStudio-specific metadata (compliance status, usage counts, parsed timestamps) lives only in SQLite — files are never modified to add OrqaStudio metadata.

## Rationale

Full compatibility with Claude Code CLI. Users can switch between OrqaStudio and CLI seamlessly. Markdown is the natural format for AI instructions. Parsing via `yaml-front-matter` (frontmatter extraction) + `comrak` (markdown body parsing/rendering, used by crates.io and docs.rs).

## Consequences

Files are always authoritative — if a file changes on disk, the SQLite cache is updated to match. No OrqaStudio-specific annotations in `.claude/` files. The DB stores enriched metadata (word count, heading structure, tool lists, compliance status) that does not exist in the files.
