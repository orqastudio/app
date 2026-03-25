---
id: TASK-c9fa5dd0
type: task
name: "Verify knowledge artifact structure standards"
status: active
description: "Verify all knowledge artifacts meet 500-2000 token range with 100-150 token summaries per RES-d6e8ab11 section 5"
relationships:
  - target: EPIC-2451d1a9
    type: delivers
    rationale: "Phase 3 — Agent Prompt Generation Verification"
  - target: TASK-c298a900
    type: depends-on
    rationale: "Phase 1 complete — knowledge artifacts moved to correct plugins before measuring"
acceptance:
  - "Every knowledge artifact measured and within 500-2,000 token range — zero exceptions without a user-approved AD artifact"
  - "Every knowledge artifact has a summary field of 100-150 tokens"
  - "Plugin manifests declare knowledge with injection tier, roles, paths, and tags"
  - "npx tsc --noEmit passes for libs/cli"
---

# TASK-c9fa5dd0: Verify knowledge artifact structure standards

## What to do

Verify all knowledge artifacts meet the standards from RES-d6e8ab11 section 5:

1. **Inventory all knowledge artifacts:**
   - List all files in `.orqa/process/knowledge/KNOW-*.md`
   - List knowledge files in each plugin: `plugins/*/knowledge/`
   - Count total knowledge artifacts

2. **Measure token counts:**
   - For each knowledge artifact, measure token count (words * 1.3 approximation)
   - Flag any artifact outside the 500-2,000 token range
   - For over-2,000 artifacts: identify natural split points for atomic decomposition
   - For under-500 artifacts: determine if they should be merged with related content

3. **Check summary fields:**
   - For each knowledge artifact, check for a `summary` field in the frontmatter
   - Measure summary token count — must be 100-150 tokens
   - Flag artifacts missing summaries
   - Flag summaries outside the 100-150 range

4. **Check plugin manifest declarations:**
   - Read each plugin's `orqa-plugin.json`
   - Find knowledge declarations in the manifest
   - Verify each knowledge entry includes:
     - `tier`: injection tier (always, stage-triggered, on-demand)
     - `roles`: which agent roles receive this knowledge
     - `paths`: file path patterns for scope matching
     - `tags`: semantic tags for search matching

5. **Produce a report:**
   - Table: artifact ID, token count, has-summary, summary-token-count, pass/fail
   - List of violations with remediation (split, merge, add summary)
   - Plugin manifest compliance per plugin

## Knowledge needed

- `.orqa/process/knowledge/KNOW-*.md` — project knowledge artifacts
- `plugins/*/knowledge/` — plugin knowledge artifacts
- `plugins/*/orqa-plugin.json` — plugin manifests with knowledge declarations
- RES-d6e8ab11 section 5 — knowledge artifact standards

## Agent role

Researcher — reads artifacts, measures, produces findings. No code changes.

## Verification

- Total artifact count documented
- Every artifact has a measured token count in findings
- Pass/fail per artifact against 500-2,000 range
- Summary field presence and size checked for every artifact
- Plugin manifest declarations audited for tier/roles/paths/tags
- `npx tsc --noEmit` in `libs/cli/`
