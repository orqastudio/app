---
id: SKILL-040
title: Systems Thinking
description: Practical methodology for applying systems thinking to codebases — identifying systems, mapping boundaries, tracing relationships, finding patterns, and discovering the uniform base.
status: active
created: 2026-03-11
updated: 2026-03-11
layer: core
category: methodology
user-invocable: true
relationships:
  - target: PILLAR-001
    type: grounded
    rationale: Mapping boundaries, relationships, and the uniform base before changing code prevents special-case proliferation and makes architecture intentional
  - type: scoped-to
    target: AGENT-001
    rationale: Migrated from scope field
  - type: scoped-to
    target: AGENT-002
    rationale: Migrated from scope field
  - type: scoped-to
    target: AGENT-003
    rationale: Migrated from scope field
  - type: scoped-to
    target: AGENT-004
    rationale: Migrated from scope field
  - type: scoped-to
    target: AGENT-005
    rationale: Migrated from scope field
  - type: scoped-to
    target: AGENT-006
    rationale: Migrated from scope field
  - type: scoped-to
    target: AGENT-007
    rationale: Migrated from scope field
---

# Systems Thinking

Companion skill for [RULE-028](RULE-028) (Systems Thinking First). The rule defines the constraint — "think in systems before touching code." This skill provides the methodology — how to actually do it.

## 1. Identify the System

Before changing anything, name the system you are working within.

**Questions to ask:**
- What is the larger structure this change lives in?
- Is this a data pipeline, a rendering system, a state machine, a configuration tree?
- What are the inputs, outputs, and invariants?

**Example:** You are fixing a bug where a status badge shows the wrong color.

- Bad: "The badge component has a wrong CSS class"
- Good: "The status rendering system transforms backend status values → display properties (color, label, icon) for all artifact types. The badge is one output of this system."

Naming the system prevents you from treating a symptom as the whole problem.

## 2. Map the Boundaries

Every system has edges — where it starts, where it ends, and what crosses those edges.

**Questions to ask:**
- Where does data enter this system? (API response, user input, file read, config)
- Where does data leave? (UI render, file write, API call, database insert)
- What transformations happen between entry and exit?
- Who owns each boundary? (Which module, which layer, which team?)

**Technique — Boundary Walk:**

```
Entry point → First transformation → Intermediate state → Second transformation → Exit point
```

Trace this path for the specific data involved in your change. If you cannot trace the path, you don't understand the system well enough to change it safely.

## 3. Trace the Relationships

Systems are connected. A change in one system affects others.

**Questions to ask:**
- What depends on the thing I'm changing? (downstream consumers)
- What does the thing I'm changing depend on? (upstream producers)
- Are there parallel systems that follow the same pattern? (siblings)
- Are there systems that share infrastructure? (cousins)

**Technique — Impact Radius:**

1. Find all callers of the function/component you're modifying
2. For each caller, check if it makes assumptions about the current behavior
3. If yes, those callers must be updated in the same change
4. Repeat one level out — check callers of callers if the interface changed

## 4. Recognize the Pattern

Most systems follow patterns. Identifying the pattern tells you the correct shape of your change.

**Questions to ask:**
- Is there an existing pattern for how similar things work in this codebase?
- Are there other instances of this pattern? (other status types, other artifact types, other panel types)
- Does my change follow the pattern, extend it, or break it?

**The Pattern Test:**

If your change handles one case, search for all cases. If there are 8 artifact types and you're fixing status display, check all 8 — not just the one the bug was reported on. A fix that handles 1 of 8 cases is incomplete.

**Anti-pattern signals:**
- `if (type === "X") { special handling }` — You're adding a special case instead of fixing the general pattern
- "This only affects X" — Verify by searching for sibling cases
- "I'll do the others later" — The others will be forgotten

## 5. Find the Uniform Base

Every well-designed system has ONE default behavior applied uniformly. Variations are optional layers on top.

**Questions to ask:**
- What is the one behavior that should apply to ALL members of this system?
- Which variations are intentional enhancements?
- Which variations are accidental inconsistencies?

**Example:**

A rendering pipeline should have:
- **Base:** Every artifact type renders with title, status badge, and body content (uniform)
- **Enhancement:** Epic types additionally show a progress bar (intentional variation)
- **Bug:** Task types don't show the status badge (accidental inconsistency)

The fix is to ensure the base applies everywhere, then verify each enhancement is intentional.

**Technique — Uniform Base Discovery:**

1. List all members of the system (all types, all routes, all components)
2. For each member, list what behaviors it has
3. Find the common denominator — what every member should do
4. Differences from the common denominator are either intentional (documented) or bugs (fix them)

## 6. Apply — Before/After

### Before (Symptom Patching)

"The Epic status badge shows 'draft' in gray instead of blue."

Fix: Change the color mapping for 'draft' in the Epic badge component.

**Problems:** Only fixes Epic. Other types may have the same issue. The root cause (inconsistent color mapping) is unaddressed.

### After (Systems Thinking)

1. **System:** Status rendering pipeline — transforms status values to visual properties for all artifact types
2. **Boundaries:** Backend status enum → status-to-display mapping → component rendering
3. **Relationships:** 9 artifact types share this pipeline. Badge, list item, and detail view all consume it.
4. **Pattern:** All types should use the same status-to-display mapping. Check all 9 types × 3 consumers = 27 combinations.
5. **Uniform base:** One canonical status-to-display map used by all types. Type-specific overrides only where intentionally different.

Fix: Update the canonical status-to-display mapping. Verify all 9 types render correctly in all 3 consumers.

## When to Stop

Systems thinking has diminishing returns. Stop when:

- You can trace the full data path for your change
- You've verified all sibling cases follow the pattern
- You've confirmed the uniform base is intact
- The impact radius is contained

You don't need to redesign the entire architecture for a bug fix. You need to understand enough of the system to fix the root cause instead of a symptom.
