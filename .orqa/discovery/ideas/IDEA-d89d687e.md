---
id: IDEA-d89d687e
type: discovery-idea
title: Cross-project classification learning with opt-in telemetry and AI-driven semantic merge
description: Projects that opt in share classification corrections to improve the global starting prompt. New version upgrades use AI-driven semantic merge to combine global improvements with local refinements without overwriting project-specific learnings.
status: captured
created: 2026-03-21
updated: 2026-03-21
relationships:
  - target: PILLAR-a6a4bbbb
    type: grounded
    rationale: Purpose Through Continuity — local learnings persist across upgrades
  - target: PILLAR-c9e0a695
    type: grounded
    rationale: Clarity Through Structure — classification improvements are visible and governed
  - target: PERSONA-2721ae35
    type: benefits
    rationale: Jordan (Independent) — solo users benefit from collective learning without being forced to share
---

## The Idea

Three capabilities that build on the thinking mode classification system:

### 1. Opt-In Telemetry

Projects that choose to share their classification corrections contribute to improving the global starting prompt. Strictly opt-in, anonymised, user-controlled.

- When the learning loop corrects a classification (user says "that was feedback not implementation"), the correction is logged locally
- If telemetry is enabled, the anonymised correction (prompt pattern → expected mode) is shared
- Users can opt out completely with zero impact on their local experience
- This is about democratising AI tooling, not data extraction

### 2. Global Prompt Evolution

Across consenting projects, classification corrections aggregate into improved starting descriptions for each thinking mode.

- "Users who expected learning-loop mode but got implementation when saying X" refines the learning-loop mode description
- Same governance pipeline (observation → lesson → rule) applied cross-project
- Improvements ship in new versions as updated thinking mode knowledge artifacts

### 3. AI-Driven Semantic Merge for Upgrades

When a new version ships with improved classification prompts, it cannot overwrite local project refinements.

An AI merge process must:
- Take the new global prompt (improved from cross-project learning)
- Take the local project's refinements (lessons that corrected classifications locally)
- Merge so local learnings are preserved but global improvements are incorporated
- Flag conflicts for human review rather than silently overwriting

This is a semantic merge — understanding the intent of both versions, not just diffing text. The separate-artifacts-per-mode architecture makes this tractable: each mode description is independently mergeable.

### Privacy Principle

The system must be as useful for projects that share nothing as for those that share everything. Opt-in telemetry is an enhancement, not a requirement. Local learning always works.

### 4. Tiered Learning Architecture

Learning operates at three distinct tiers. Each tier is fully independent — a project gains the full benefit of Tier 1 without any cloud dependency. Higher tiers are additive, not required.

**Tier 1 — Local**

The standard governance pipeline within a single project:

- Observation → lesson → rule, all within one project
- Always works with no external dependency and no sharing
- This is the baseline. Every project gets this regardless of cloud or telemetry

**Tier 2 — Organisation** (via Orqa Cloud)

An intermediary learning loop stage across an organisation's projects:

- Lessons from all of an org's projects are aggregated at the Cloud instance
- Patterns that recur across multiple projects are candidates for cross-project rules
- Org admins review and approve what propagates down to all org projects
- User-controlled at the org tier: admins decide what rules cascade, teams can opt their project out
- Cloud provides the aggregation and rollout mechanism — not the data capture mechanism
- Data capture still requires analytics tooling (hooks, telemetry, observation events) configured on each project. Cloud receives and aggregates what the local tools produce

**Tier 3 — Global** (via Orqa Cloud + analytics)

Anonymised pattern learning across all consenting organisations:

- Patterns observed across multiple consenting orgs feed into improved default starting artifacts
- New projects and version upgrades ship with starting artifacts informed by real cross-org usage
- Opt-in at two levels: org level (org admin decides whether the org participates) and project level (teams can opt individual projects out even if the org participates)
- Same principle as Tier 2: Cloud is the aggregation/rollout mechanism; analytics tooling on each project is the capture mechanism
- Anonymisation is non-negotiable: no org or project identity propagates upward

**What Cloud provides vs. what analytics tools provide:**

| Concern | Provided by |
|---------|-------------|
| Capturing observations locally (hook events, telemetry) | Analytics/observability tooling on each project |
| Transmitting aggregated lessons upward | Orqa Cloud management API |
| Cross-project pattern detection | Orqa Cloud |
| Conflict detection and merge review | Orqa Cloud |
| Controlled rollout to downstream projects | Orqa Cloud |
| User consent and propagation control | Orqa Cloud admin UI |

Cloud without analytics tooling means no data flows up. Analytics tooling without Cloud means all learning stays local (Tier 1). The two are complementary, not substitutes.

**First-party hosted Cloud** opens a path to offering Tier 2 and Tier 3 as a managed service — organisations that don't want to run their own Cloud instance can subscribe to a hosted tier and get cross-project learning with zero infrastructure overhead.