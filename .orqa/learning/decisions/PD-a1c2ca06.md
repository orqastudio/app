---
id: PD-a1c2ca06
type: principle-decision
title: "Four enforcement layers — app, scanners, integrity, git hooks"
description: "Enforcement is mechanical, not behavioral. Four layers cover all modification paths: app enforcement prevents invalid states at action time, automated scanners apply objective transitions on file changes, integrity checks flag judgment-required issues, and git hooks catch edits that bypassed the app. All layers enforce both canonical and project-configured constraints."
status: active
created: 2026-03-15
updated: 2026-03-15
relationships:
  - type: drives
    target: EPIC-a60f5b6b
    rationale: "Four enforcement layers decision drove the principle enforcement foundations epic"
---

## Decision

### Enforcement Is Mechanical

If enforcement depends on AI remembering rules, enforcement fails under pressure. This is not behavioral guidance — it's the app refusing to accept invalid state, like a database rejecting bad SQL.

The app enforces BOTH canonical and project-configured constraints with the same rigour. Project relationships defined in `project.json` get the same mechanical enforcement as canonical ones. Configuration doesn't mean optional enforcement.

### What Gets Enforced

| Constraint | Scope | Layer |
| --- | --- | --- |
| Relationships must be bidirectional | Canonical + project | App |
| Relationships must use valid types | Canonical + project | App |
| Statuses must be from allowed vocabulary | Canonical + project | App |
| Status transitions must be valid | Canonical + project | App |
| Type-specific status subsets | Canonical + project | App |
| Delivery artifact must connect to parent | Project-configured | App |
| Child can't be further along than parent | Project-configured | App |
| Layer 1 artifacts are immutable | Canonical | App + Git hooks |
| Layer 2 artifacts must exist | Canonical | App |
| Required relationships must exist | Canonical | App + Integrity |

### Four Layers

| Layer | When | Human Needed? | What It Does |
| --- | --- | --- | --- |
| **App enforcement** | At action time | No | Prevents invalid states mechanically |
| **Automated scanners** | On file change (watcher) | No | Applies objective transitions automatically |
| **Integrity checks** | On scan/refresh | Yes | Flags judgment-required issues |
| **Git hooks** | At commit time | Yes | Last line of defence for out-of-app edits |

### Layer 1: App Enforcement

Makes invalid states impossible. You can't create them.

Covers both canonical constraints (relationship bidirectionality, status vocabulary) and project-configured constraints (delivery hierarchy, type-specific rules). The enforcement engine merges canonical + project config and enforces uniformly.

### Layer 2: Automated Scanners

Run on file watcher events. When an artifact changes on disk:

1. Read the changed file's relationships
2. Evaluate connected nodes for objective state changes
3. Auto-apply transitions that are unambiguous (no human judgment needed)

These are the configurable auto_rules from the state machine, triggered by file watcher events. Auto-rules can reference both canonical and project relationships.

Examples:

- All tasks on an epic are `completed` → auto-move epic to `review`
- A dependency completes → auto-unblock the waiting task
- A lesson hits recurrence threshold → auto-move to `review`

### Layer 3: Integrity Checks

Detect states that are technically valid but semantically wrong — situations where code can identify the problem but only a human can decide the fix.

Examples:

- Body text references an artifact with no corresponding relationship
- Lesson has recurred 3 times (promote to rule?)
- All tasks completed on an epic (move to review?)
- Child artifact further along than parent (advance parent or move child?)

### Layer 4: Git Hooks

Last line of defence for edits that bypassed the app — text editors, CLI agents, manual file editing. Run the same integrity checks at commit time.

Additionally enforce Layer 1 immutability — blocking commits that modify app-fixed artifacts (canonical type definitions, platform docs/skills).

## Rationale

Each layer covers a different modification path:

- **App**: the primary path — all in-app actions
- **Scanners**: reactive automation — when graph state changes objectively
- **Integrity**: human-judgment issues that can't be auto-resolved
- **Git hooks**: safety net for everything outside the app

The four layers ensure enforcement is complete regardless of how artifacts are modified. No path bypasses enforcement.

## Consequences

- App enforcement engine processes canonical + project config uniformly
- File watcher triggers scanner evaluations on artifact changes
- Integrity check results surface in the UI for human resolution
- Git hooks run integrity checks + Layer 1 immutability protection at commit time
- All four layers implemented as part of the migration epic
