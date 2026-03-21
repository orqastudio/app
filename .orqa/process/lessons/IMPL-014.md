---
id: "IMPL-1bd6ed0c"
type: "lesson"
title: "Epic titles should describe outcomes, not process"
description: "Epic titles like \"UAT Round 1\" or \"Phase 3 Implementation\" describe process activities. Titles should describe what is achieved, not how the work is organised.\n"
status: "completed"
created: "2026-03-07"
updated: "2026-03-13"
maturity: "understanding"
recurrence: 1
relationships: []
---
## What Happened

During UAT Round 2 [EPIC-a2fa3068](EPIC-a2fa3068), epics were found with process-oriented titles like "UAT Round 1 — Dogfood Readiness Verification". The title describes the testing activity rather than the outcome being verified.

## The Correct Approach

Epic titles should answer "what does this achieve?" not "what process does this follow?":

- Bad: "UAT Round 1 — Dogfood Readiness Verification"
- Good: "Dogfood Readiness Verification"
- Bad: "Phase 3: Implementation Sprint"
- Good: "Streaming Pipeline Reliability"

Process words to avoid in titles: UAT, Phase, Sprint, Round, Audit, Review (when describing the activity rather than the outcome).

## Pattern

See description in frontmatter.

## Fix

Fix approach documented at time of lesson capture.
