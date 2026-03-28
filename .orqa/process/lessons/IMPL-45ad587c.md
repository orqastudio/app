---
id: IMPL-45ad587c
type: lesson
title: "Dashboard widgets need actionable context, not just metrics"
description: UAT revealed that showing counts and status labels without reasoning or suggested actions creates noise rather than insight. Metrics without actionability are decoration.
status: active
created: 2026-03-13
updated: 2026-03-14
recurrence: 1
maturity: understanding
relationships:

  - target: EPIC-f7055984

    type: cautions
    rationale: "Multiple UAT findings (#2, #8, #13) converge on this pattern"
---

## Observation

The pipeline health dashboard surfaced counts (66 orphans, 197 warnings) and labels (stuck, bottleneck) without explaining why or what to do about them. Users see numbers that feel wrong but have no path to action. This pattern appeared across three independent findings:

1. Orphan count included documentation artifacts that aren't actually orphans — inflated metric
2. Pipeline stages labeled "stuck" with no reasoning for the assessment
3. No inferred actions box telling users what's needed to advance artifacts

## Principle

Every dashboard metric should answer three questions: **What is it? Why is it this value? What should I do about it?** A metric that only answers the first question is noise.
