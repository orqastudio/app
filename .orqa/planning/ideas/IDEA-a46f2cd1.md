---
id: IDEA-a46f2cd1
type: planning-idea
title: "Evaluate Unlegacy.ai for automated codebase scanning"
description: "Research unlegacy.ai as a candidate integration for the OrqaStudio audit and validation surface. The goal is to compare its automated codebase-scanning approach against the current orqa validate pipeline and identify gaps we should cover."
status: captured
priority: P3
created: 2026-04-11
updated: 2026-04-11
horizon: later
relationships:
  - target: PILLAR-c9e0a695
    type: grounded
    rationale: "Codebase audit is the bridge from hidden technical debt to visible governance signal. Any tool that formalises the scan pass should be compared against the current validator."
---

## Source

User research lead, captured 2026-04-11 in conversation: "look into https://www.unlegacy.ai/ for codebase scanning".

## What

[Unlegacy.ai](https://www.unlegacy.ai/) is described as a codebase-scanning / legacy-modernisation tool. The lead is: evaluate its scan approach and see whether OrqaStudio's `orqa validate` / `orqa enforce` pipeline should borrow primitives, integrate with it directly, or treat it as a reference target for what our scan surface should detect.

## Why it's relevant

OrqaStudio already has three related scanning surfaces:

1. `orqa validate` — structural validation of `.orqa/` artifacts against plugin schemas.
2. `orqa enforce --check` — runs installed enforcement engines (clippy, eslint, tsc, cargo test, etc.) over the project.
3. `scan_governance` (in `engine/enforcement/src/scanner.rs`) — coverage scan across governance areas.

What these don't cover is the *source code* itself — architectural drift, circular dependencies, God-objects, stale APIs, deprecated patterns, migration opportunities. That's the gap Unlegacy.ai seems to target. If its model maps onto OrqaStudio's plugin-composed enforcement architecture, we could expose the same signal through the existing dashboards.

## What to investigate

- **Scan types**: what categories does Unlegacy detect? Architectural smells, dependency rot, deprecated APIs, dead code, test coverage gaps, language-version drift?
- **Language coverage**: which languages / ecosystems? OrqaStudio itself is Rust + TypeScript + Svelte; a scanner that only covers one is still useful but partial.
- **Delivery model**: CLI, local daemon, hosted SaaS, source code? A local CLI is the only shape that fits OrqaStudio's offline-first posture.
- **Output format**: structured (JSON) vs narrative (markdown report)? Structured output slots into the enforcement findings pipeline; narrative output feeds the `orqa audit` summary.
- **Licensing and cost**: free, paid, open source, commercial?
- **Integration point**: does it have a programmatic API the daemon can call, or is it invocation-only?

## Decision criteria

- Must produce structured findings the enforcement engine can consume (or be wrappable in a generator-plugin that does).
- Must run locally without requiring project source to be uploaded to a third-party service.
- Must degrade gracefully on projects it doesn't fully understand — a scanner that hard-fails on Svelte 5 isn't acceptable for the OrqaStudio dogfood case.

## Relationship to existing work

- Slots next to the enforcement generator plugins (`plugins/knowledge/rust`, `plugins/knowledge/typescript`) — if adopted, Unlegacy becomes a new generator-plugin that registers scan rules.
- Feeds the future `orqa audit` command.
- Could drive content for the devtools Issues tab — architectural drift shows up as long-lived issues, not transient errors.
- Complements the architecture-audit work tracked under `.orqa/documentation/file-audit/` (per `project_session_20260326_status.md` user memory).

## Not in scope

- Uploading OrqaStudio source to any third-party service.
- Replacing the existing clippy / eslint / tsc enforcement stack — Unlegacy would complement those, not replace them.
- Commercial-license adoption without an offline / self-hosted story.
