---
id: "RES-93c7bb99"
type: discovery-research
title: "Knowledge Maturity Pipeline — Self-Enforcing Learning Loop Architecture"
description: "Investigation into how OrqaStudio's governance artifacts should be restructured around a knowledge maturity pipeline to create a genuinely self-enforcing learning loop."
status: completed
created: "2026-03-12"
updated: "2026-03-12"
sources: []
relationships:
  - target: "EPIC-3e6cad90"
    type: "guides"
    rationale: "Research findings informed epic design"
  - target: "EPIC-88f359b0"
    type: "guides"
    rationale: "Research findings informed epic design"
  - target: "AD-430829f1"
    type: "informs"
---

## Problem Statement

OrqaStudio's mission is to turn messy situations into structured understanding through agile thinking and continuous retrospection. The governance layer is supposed to embody this — a self-enforcing learning loop where the system gets smarter by being used.

In practice, the loop is broken. We discovered this by experiencing the exact failure the system is supposed to prevent: we spent weeks building enforcement mechanisms (44 rules, hooks, skills, injection pipelines) and lost sight of whether any of it actually serves the learning loop. The implementation artifacts became the focus. The loop stopped being the focus.

This is not a bug in any specific artifact. It's an architectural problem: **knowledge is organized by category (decision, rule, lesson, skill) rather than by its role in the learning lifecycle.**

## The Drift Pattern (What Happened To Us)

1. Clear purpose: "build a self-enforcing learning loop"
2. Started implementing: rules, hooks, skills, enforcement entries
3. Each implementation decision spawned more implementation decisions
4. Accumulated 44 rules, 60 ideas, 57 epics, dozens of skills
5. The artifacts became the focus. The loop stopped being the focus.
6. Nobody asked: "Is the loop actually looping?"

No rule prevented this because the drift isn't a violation — it's a gradual shift in attention. You can't regex-match "losing sight of the bigger picture."

## The Learning Loop (What We're Actually Building)

From the user's perspective, the loop is simple:

```text
You're working. You make a mistake — or discover something that works.
You want the system to remember that, so next time the outcome is better.
```

That's the entire loop. Everything else is mechanism.

### Six Stages

| Stage | Question | What Happens |
| ------- | ---------- | ------------- |
| **Observation** | "What happened?" | Raw capture — something was noticed |
| **Understanding** | "Why did it happen?" | Root cause identified, context mapped |
| **Principle** | "What's the general rule?" | Abstract, transferable insight distilled |
| **Practice** | "How do you follow the rule?" | Concrete guidance, examples, anti-patterns |
| **Enforcement** | "How does the system prevent violation?" | Automated — fires without human action |
| **Verification** | "Is it working?" | Evidence of effectiveness, trend data |

### The Self-Enforcing Property

The loop self-enforces not through more hooks or regex, but through **structural visibility into whether knowledge is flowing through the pipeline**.

When you can see:

- "We have 16 observations that never became principles — learning is stuck at stage 2"
- "We have 30 enforcement entries with no connected principle — enforcement is disconnected from reasoning"
- "This principle has been violated 5 times since enforcement was added — the practice isn't working"
- "We've created 12 new enforcement entries this week and zero observations — we're generating constraints without learning"

That last signal is exactly what would have caught our drift. We were running the pipeline backwards — generating enforcement (stage 5) without observations feeding it (stage 1).

## Current State Mapping

| Current Artifact | Pipeline Stage | Problem |
| ----------------- | --------------- | --------- |
| Lesson (IMPL-NNN) | Observation + Understanding | Stuck — write-only, never flows to Principle |
| Decision (AD-NNN) | Principle | Disconnected — no link to Enforcement or Practice |
| Skill (SKILL.md) | Practice | Disconnected — no link to the learning that created it |
| Rule (RULE-NNN) | Enforcement | Disconnected — no link to the Principle it serves |
| ??? | Verification | Doesn't exist |

### Where the Loop Breaks

1. **Observation → Understanding**: Lessons capture both but don't distinguish them. A lesson that says "component purity was violated" (observation) is the same artifact type as one that says "display components accumulate data fetching because invoke() is convenient" (understanding). There's no structural signal that understanding has been achieved.

2. **Understanding → Principle**: Lessons have a `evolves-into` field, but promotion is entirely manual. No mechanism detects that a lesson maps to an existing principle, or that a recurring lesson should become a new principle. The `recurrence` counter exists but nothing reads it.

3. **Principle → Practice**: Decisions exist independently of skills. [AD-48b310f9](AD-48b310f9) says "component purity" but there's no structural link to the `component-extraction` skill that teaches how to achieve it. The practice was written separately and happens to cover the same ground.

4. **Practice → Enforcement**: Skills are injected by file path patterns (RULE-e1f1afc1). Rules have enforcement entries. But there's no link from "this skill teaches X" to "this rule enforces X." They're parallel systems.

5. **Enforcement → Verification**: Nothing measures whether enforcement is effective. We don't know if a rule has fired 100 times (suggesting the practice isn't working) or zero times (suggesting either the practice is perfect or the enforcement pattern is wrong).

## Design Options

### Option A: Knowledge Maturity Pipeline (Single Artifact Type)

Replace all knowledge artifacts (lessons, decisions, rules, skills) with a single **Knowledge** artifact that carries a `maturity` field tracking its pipeline stage.

```yaml
id: K-042
title: "Display components must not call invoke()"
maturity: enforcement    # observation → understanding → principle → practice → enforcement → verified
upstream: [K-038]        # the observation that led here
downstream: [K-045]      # the verification that measures this
```

**Pros**: Cleanest model. The pipeline IS the structure. Maturity progression is visible by definition.

**Cons**: Loses semantic distinction between "why we decided this" (decision rationale) and "how to do this" (skill guidance). A single artifact carrying observation, principle, practice, AND enforcement would be very long.

### Option B: Current Types with Mandatory Pipeline Edges

Keep decisions, lessons, rules, and skills as separate types. Add mandatory `upstream`/`downstream` edges that connect them through the pipeline. Add Verification as a new stage.

```yaml
# On a Rule
id: RULE-9814ec3c
principle: AD-48b310f9        # the decision this enforces
practice: [component-extraction]  # the skill that teaches compliance
observations: [IMPL-ffb199b5, IMPL-42dd183e]  # the lessons that motivated this
verification: VER-006    # effectiveness measurement
```

**Pros**: Incremental — works with existing artifacts. Preserves semantic types.

**Cons**: Edges are additional fields that can be forgotten (the same problem we have now). The pipeline is implicit in the edges rather than explicit in the structure.

### Option C: Hybrid — Pipeline Groups

Group related artifacts into a **Knowledge Thread** — a named pipeline that connects one piece of learning from observation through verification.

```text
Thread: "Component Purity"
  Observation: IMPL-ffb199b5 ("invoke() found in display component")
  Understanding: IMPL-ffb199b5 body ("convenience leads to coupling")
  Principle: AD-48b310f9 ("display components receive props only")
  Practice: component-extraction skill
  Enforcement: RULE-9814ec3c enforcement entry (path match on $lib/components/)
  Verification: VER-xxx (violation trend: 5 → 2 → 0 over 3 months)
```

The thread is the first-class object. Individual artifacts are stages within it.

**Pros**: Makes the pipeline explicit AND preserves artifact types. You can see the full journey of a piece of learning. Bottleneck detection is natural ("threads stuck at Understanding").

**Cons**: New concept to maintain. Threads could become stale. Requires a new artifact type or a grouping mechanism.

## Recommendation

**Option C (Pipeline Groups / Knowledge Threads)** best serves the self-enforcing property because:

1. The thread itself is the visibility mechanism — you can see at a glance where knowledge is in its journey
2. Individual artifact types retain their semantic meaning (decisions have rationale, skills have guidance, rules have enforcement)
3. Bottleneck detection falls out naturally: count threads by their most mature stage
4. The "are we running the pipeline backwards?" signal becomes trivial: threads with Enforcement but no Observation are constraints without learning

The verification stage (currently missing) completes the loop. Without it, you never know if enforcement is working — you just assume it is.

## Open Questions

1. **Should threads be explicit artifacts** (a `THREAD-NNN.md` that lists its stages) or **implicit** (derived from edge relationships between existing artifacts)?

2. **What triggers maturity progression?** Manual promotion? Recurrence threshold? AI-suggested? The current manual promotion doesn't work — what would?

3. **What does verification look like in practice?** Violation count trends? Enforcement hit rates? User satisfaction signals? Build metrics?

4. **How does this interact with the pillar framework?** Pillars are purpose. Threads are learning. Does each thread serve a pillar? Or are pillars a separate concern?

5. **Does this change the enforcement mechanism?** Currently enforcement = regex on tool calls. If enforcement is a stage in a knowledge pipeline, does it need to be more nuanced?

6. **What about knowledge that doesn't come from mistakes?** The pipeline starts with Observation, but some principles come from design (not failure). Decisions like "use Tauri" weren't learned from a mistake — they were deliberate choices. How do deliberate design principles enter the pipeline?
