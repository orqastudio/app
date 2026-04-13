---
id: "EPIC-1f0fcb54"
type: epic
title: "Semantic layout lego blocks — strip primitives, enforce design system via components"
description: "Remove all visual escape hatches (padding, border, background, rounded, margin, overflow, style) from layout primitives (Stack, HStack, Box, Center). Replace with purpose-built semantic components: Panel (padded container), SectionHeader/SectionFooter (horizontal bars with baked-in padding + border), Callout (inline tonal banner). Add indent structural prop to HStack for tree-depth rows. Migrate ~100 consumer call sites across app, devtools, and plugins. Zero raw-HTML tolerance in app code."
status: delivered
priority: P0
created: "2026-04-05"
updated: "2026-04-13"
horizon: active
relationships:
  - target: "PILLAR-c9e0a695"
    type: "grounded"
    rationale: "Clarity Through Structure — a locked-down component API enforces visual consistency and makes it impossible for AI agents or developers to create inconsistent UI."
  - target: "MS-b1ac0a20"
    type: "fulfils"
    rationale: "Semantic component lockdown enables consistent plugin and devtools UI during dogfooding"
---

# Semantic Layout Lego Blocks

## Problem

Layout primitives (Stack, HStack, Box, Center) exposed free-form visual props — padding, paddingX, paddingY, paddingTop, paddingBottom, marginTop, border, borderTop, borderBottom, rounded, background, overflow, style. This made every call site a design decision, resulting in inconsistent spacing, arbitrary padding values, and an unpredictable visual system. AI-driven plugin UI development amplifies this — agents will use whatever props are available, creating visual drift.

## Solution

Strip layout primitives to structural-only (gap, align, justify, flex, height, width, minHeight, position, indent, wiring). Move all visual concerns to closed-set semantic components:

- **Panel** — padded container. Props: `padding: none|tight|normal|loose`, `background: none|card|muted|surface`, `border: none|all|top|bottom`, `rounded: none|sm|md|lg|xl`.
- **SectionHeader** — horizontal header bar. Props: `variant: section|subsection|compact`, `start`/`end`/`children` slots.
- **SectionFooter** — horizontal footer bar. Same API as SectionHeader, border-top instead of border-bottom.
- **Callout** — inline tonal banner. Props: `tone: info|warning|success|destructive|muted`, `density: compact|normal`, `border: solid|dashed`, optional icon.
- **indent** prop on HStack — closed-set left margin (0–8 levels, 8px step) for tree-depth rows.

## Outcome

- Layout primitives: 4 components stripped (Stack, HStack, Box, Center)
- New semantic components: 4 created (Panel, SectionHeader, SectionFooter, Callout)
- Consumer migration: ~100 files across app/src, devtools/src, plugins/workflows/software-kanban, libs/svelte-components stories
- Verification: grep for dropped props → zero matches. svelte-check → 0 errors on app, devtools, software-kanban.
- No raw HTML created. All visual styling locked behind closed-set variant props.
