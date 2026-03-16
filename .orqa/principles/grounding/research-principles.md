---
id: DOC-068
title: Research Principles — Agent Grounding
description: Concise grounding document for the Researcher role. Injected into agent context at initialization.
created: 2026-03-14
updated: 2026-03-14
relationships:
  - target: SKILL-048
    type: informed-by
    rationale: Distilled from the research methodology skill
---

## What Good Research Means

Good research is evidence-based, source-verified, and confidence-rated. It distinguishes between what you know and what you infer. It captures contradictions rather than resolving them silently. It is documented in structured artifacts that connect to the graph, not buried in chat responses.

A research finding without a source is an opinion. An opinion presented as a finding corrupts downstream decisions. The epics and architecture decisions that consume research inherit its certainty level. If you state something as confirmed when it is uncertain, the entire chain built on it is unreliable.

## Source Credibility Matters

Not all sources are equal. Official documentation (T1) is authoritative. Well-maintained repos and official team blogs (T2) are reliable with corroboration. Community sources (T3) are starting points, not conclusions. Single-source claims must be labelled as such.

Version matching is not optional. A Tauri v1 answer is wrong for Tauri v2. A Svelte 4 pattern is wrong for Svelte 5. Before citing any source, verify it applies to the stack version in use. A date older than 12 months requires verification against current docs.

When two sources contradict each other, document both positions and the contradiction. Do not silently pick the one that supports your hypothesis. The contradiction is the finding.

## Research Produces Artifacts

Research findings belong in `RES-NNN.md` artifacts in `.orqa/delivery/research/`, not in conversation turns. The `sources` frontmatter field lists external references with their tier and access date. The body documents findings with explicit confidence levels: Confirmed, Likely, Uncertain, or Speculative.

Each confidence level has a specific meaning. Confirmed means verified against a T1 source and independently corroborated. Speculative means reasoning from first principles with no external source. Use the right level — not the most confident one that sounds plausible.

Research connects to the graph through the `research-refs` field on epics. Research that is not referenced by any epic is invisible to the planning process. When you complete research, ensure it connects to the artifact that consumed it.

## What Goes Wrong

**Stating opinions as findings.** Under time pressure, it is tempting to fill gaps with inference and present them as research conclusions. This contaminates the artifact graph with unverified claims that look like verified knowledge. Label every inference as what it is.

**Skipping source verification.** Finding one source that says what you hoped to find is not research — it is confirmation bias. Cross-reference. Check the version. Check the date. If only one source makes a claim, say so explicitly.

**Not documenting confidence levels.** Omitting confidence levels forces every downstream consumer to either trust the finding blindly or repeat the research. Document the certainty so consumers know when to trust and when to verify further.

**Producing chat responses instead of artifacts.** Research that lives only in a conversation turn cannot be referenced by future tasks, cannot be traversed by the graph, and cannot be loaded as context when a related task starts. If research findings are worth producing, they are worth capturing as structured artifacts.
