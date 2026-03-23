---
id: "EPIC-4a7aeacb"
type: epic
title: "Agent, Skill, and Enforcement Artifact Audit"
description: "Comprehensive audit of all agent definitions, skill definitions, and enforcement artifacts (rules, hooks) to verify accuracy against the current codebase, fix stale references, remove deprecated content, and ensure internal consistency."
status: "completed"
priority: "P1"
created: "2026-03-11"
updated: "2026-03-11"
deadline: null
horizon: null
scoring:
  impact: 4
  urgency: 4
  complexity: 3
  dependencies: 3
relationships:
  - target: "RES-63578440"
    type: "guided-by"
    rationale: "Auto-generated inverse of informed-by relationship from RES-63578440"
  - target: "RES-5484edbd"
    type: "guided-by"
    rationale: "Auto-generated inverse of informed-by relationship from RES-5484edbd"
  - target: "RES-a6311b1b"
    type: "guided-by"
    rationale: "Auto-generated inverse of informed-by relationship from RES-a6311b1b"
  - target: "TASK-b6e9df91"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-69b7753b"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-f32f3eba"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-b743b819"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-1ac4d16f"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-77b6e5bd"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-4a8c3c6a"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-cca73736"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-a5f1b36d"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-809a14cc"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-241f1480"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-732d8f12"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-50906c0c"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-42d570dc"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-a958f2d2"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-e752886d"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-1033a8ed"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-8b70b4c1"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-ab739ac3"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-8120e247"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-448102a7"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-22b0ab76"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-59ff2cfe"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-58d6a5ca"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-77f6948d"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-786b1168"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-7cbdca2a"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-03c8cc85"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-44de1cec"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-61934d01"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: "TASK-ec136ce9"
    type: "delivered-by"
    rationale: "Epic contains this task"
  - target: MS-654badde
    type: fulfils
    rationale: "Epic fulfils this milestone"
---
## Context

The comprehensive planning artifact audit (EPIC-0a8a5e72 review cycle) revealed that while planning artifacts, milestones, decisions, and documentation pages have been audited and fixed, the **team layer** (agents and skills) and **enforcement layer** (rules and hooks) have not been systematically verified against the current codebase state.

Agent definitions may reference outdated required reading paths, list skills that no longer exist, or describe workflows that have evolved. Skill definitions may contain stale code patterns, reference removed modules, or describe APIs that have changed. Rules may enforce patterns that are no longer relevant or miss patterns that should be enforced. Hooks may reference stale paths or commands.

This epic applies the same audit methodology used for planning artifacts to the team and enforcement layers.

## Implementation Design

### Phase 1: Agent Definition Audit

For each agent in `.orqa/process/agents/`:
1. Read the agent definition
2. Verify `skills:` list — do all referenced skills exist in `.orqa/process/skills/`?
3. Verify Required Reading paths — do all referenced documents exist?
4. Verify role description matches current universal role model (AD-774cc3d0)
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

Encode the audit methodology from [EPIC-0a8a5e72](EPIC-0a8a5e72) and [EPIC-4a7aeacb](EPIC-4a7aeacb) into a reusable skill:
1. Per-artifact-type checklists (status, cross-references, paths, codebase alignment)
2. Systemic pattern grouping approach
3. Cross-layer consistency checks
4. Evidence requirements for each check

## Tasks

- [TASK-b6e9df91](TASK-b6e9df91): Audit all agent definitions for accuracy
- [TASK-69b7753b](TASK-69b7753b): Audit all skill definitions against codebase
- [TASK-f32f3eba](TASK-f32f3eba): Audit all rules for enforcement accuracy
- [TASK-b743b819](TASK-b743b819): Audit hooks for correctness
- [TASK-1ac4d16f](TASK-1ac4d16f): Cross-layer consistency verification
- [TASK-77b6e5bd](TASK-77b6e5bd): Create artifact audit skill
- [TASK-4a8c3c6a](TASK-4a8c3c6a): Audit for missing and miscategorised governance artifacts

## Out of Scope

- Rewriting skills from scratch (fix what's wrong, don't redesign)
- Adding new rules or skills (except the audit skill — TASK-77b6e5bd)
- Changes to the rule/skill/agent schema (schema changes are a separate epic)