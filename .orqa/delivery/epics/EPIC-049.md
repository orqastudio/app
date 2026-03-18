---
id: EPIC-049
title: "Agent, Skill, and Enforcement Artifact Audit"
description: "Comprehensive audit of all agent definitions, skill definitions, and enforcement artifacts (rules, hooks) to verify accuracy against the current codebase, fix stale references, remove deprecated content, and ensure internal consistency."
status: completed
priority: P1
created: 2026-03-11
updated: 2026-03-11
deadline: null
horizon: null
scoring:
  impact: 4
  urgency: 4
  complexity: 3
  dependencies: 3
relationships:
  - target: RES-035
    type: guided-by
    rationale: Auto-generated inverse of informed-by relationship from RES-035
  - target: RES-037
    type: guided-by
    rationale: Auto-generated inverse of informed-by relationship from RES-037
  - target: RES-036
    type: guided-by
    rationale: Auto-generated inverse of informed-by relationship from RES-036
  - target: MS-001
    type: fulfils
    rationale: Epic belongs to this milestone
  - target: TASK-084
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-085
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-086
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-087
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-088
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-089
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-093
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-139
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-140
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-141
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-142
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-143
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-144
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-145
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-146
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-147
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-148
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-149
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-150
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-151
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-152
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-153
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-154
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-155
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-156
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-157
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-158
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-159
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-160
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-163
    type: delivered-by
    rationale: Epic contains this task
  - target: TASK-339
    type: delivered-by
    rationale: Epic contains this task
  - target: RES-035
    type: guided-by
  - target: RES-036
    type: guided-by
  - target: RES-037
    type: guided-by
---
## Context

The comprehensive planning artifact audit (EPIC-048 review cycle) revealed that while planning artifacts, milestones, decisions, and documentation pages have been audited and fixed, the **team layer** (agents and skills) and **enforcement layer** (rules and hooks) have not been systematically verified against the current codebase state.

Agent definitions may reference outdated required reading paths, list skills that no longer exist, or describe workflows that have evolved. Skill definitions may contain stale code patterns, reference removed modules, or describe APIs that have changed. Rules may enforce patterns that are no longer relevant or miss patterns that should be enforced. Hooks may reference stale paths or commands.

This epic applies the same audit methodology used for planning artifacts to the team and enforcement layers.

## Implementation Design

### Phase 1: Agent Definition Audit

For each agent in `.orqa/process/agents/`:
1. Read the agent definition
2. Verify `skills:` list — do all referenced skills exist in `.orqa/process/skills/`?
3. Verify Required Reading paths — do all referenced documents exist?
4. Verify role description matches current universal role model (AD-029)
5. Check for stale references (old app names, deprecated concepts, wrong paths)
6. Check that delegation instructions match current subagent types available

### Phase 2: Skill Definition Audit

For each skill in `.orqa/process/skills/`:
1. Read the SKILL.md and any supporting files
2. Verify code patterns described match the actual codebase
3. Verify file paths referenced in examples exist
4. Check for stale module names, function signatures, or type definitions
5. Verify the skill's `layer` field is correct (core/project/plugin/community/user)
6. Check that Related Skills references point to existing skills

### Phase 3: Rule Enforcement Audit

For each rule in `.orqa/process/rules/`:
1. Verify the rule's enforcement mechanisms still apply
2. Check for stale path references (already partially done in prior audit)
3. Verify Related Rules cross-references point to existing rules
4. Check that any code patterns described in FORBIDDEN sections match reality
5. Identify rules that may need updating based on codebase evolution

### Phase 4: Hook Audit

For each hook in `.orqa/process/hooks/`:
1. Verify the hook script exists and is executable
2. Check that paths referenced in the script are correct
3. Verify the hook's trigger event is still valid
4. Test that the hook runs successfully

### Phase 5: Cross-Layer Consistency

1. Verify orchestrator agent (CLAUDE.md source) is consistent with all rules
2. Verify skill injection table in orchestrator matches available skills
3. Verify agent-to-subagent mapping is current
4. Check for orphaned skills (skills that no agent references)
5. Check for orphaned rules (rules referenced nowhere)

### Phase 6: Missing and Miscategorised Artifacts

Audit the boundaries between governance artifact types:
1. Rules vs skills — is a rule actually domain knowledge (skill)? Does a skill encode a binary constraint (rule)?
2. Where both are needed, ensure both exist with appropriate framing
3. Identify implicit conventions enforced in practice but not captured in any artifact
4. Identify lessons that should have been promoted
5. Check for missing hooks where automated enforcement would help

### Phase 7: Create Artifact Audit Skill

Encode the audit methodology from [EPIC-048](EPIC-048) and [EPIC-049](EPIC-049) into a reusable skill:
1. Per-artifact-type checklists (status, cross-references, paths, codebase alignment)
2. Systemic pattern grouping approach
3. Cross-layer consistency checks
4. Evidence requirements for each check

## Tasks

- [TASK-084](TASK-084): Audit all agent definitions for accuracy
- [TASK-085](TASK-085): Audit all skill definitions against codebase
- [TASK-086](TASK-086): Audit all rules for enforcement accuracy
- [TASK-087](TASK-087): Audit hooks for correctness
- [TASK-088](TASK-088): Cross-layer consistency verification
- [TASK-089](TASK-089): Create artifact audit skill
- [TASK-093](TASK-093): Audit for missing and miscategorised governance artifacts

## Out of Scope

- Rewriting skills from scratch (fix what's wrong, don't redesign)
- Adding new rules or skills (except the audit skill — TASK-089)
- Changes to the rule/skill/agent schema (schema changes are a separate epic)
