---
id: SKILL-007
title: Governance Maintenance
description: |
  Governance framework custodianship: auditing agent definitions, maintaining
  rules and skills, processing lesson promotions, verifying reading list currency,
  and ensuring internal consistency across the governance layer.
  Use when: Auditing governance artifacts, promoting lessons to rules, maintaining
  agent/skill/rule consistency, or reviewing the governance framework.
status: active
created: "2026-03-01"
updated: "2026-03-10"
layer: canon
version: 1.0.0
user-invocable: true
---


Methodology for maintaining a governance framework's internal consistency. This skill teaches the *custodianship process* — the specific artifact types and directory structures come from the project's governance skill.

## Responsibilities

1. **Agent Content Auditing** — Verify agent definitions have accurate tool lists, correct model assignments, current required reading sections, and bodies that reflect the project's actual tech stack.
2. **Skill Currency** — Ensure skills are up to date, relevant, and accurately reflect current best practices.
3. **Rule File Currency** — Audit rules for outdated, contradictory, or inapplicable content.
4. **Reading List Maintenance** — Verify all documents referenced in required reading sections actually exist. Flag stale references.
5. **Learning Loop Coordination** — Process implementation lessons. When recurrence reaches the promotion threshold, promote to a rule, coding standard, or skill update. Update the lesson's promotion metadata after promotion.
6. **Cross-Reference Integrity** — Ensure all artifact cross-references (depends-on, promoted-to, supersedes, etc.) point to existing artifacts.

## Audit Protocol

### Step 1: Agent Content Audit

- List all agent definitions
- For each agent: verify frontmatter fields (name, description, tools, model, skills)
- Verify required reading references resolve to real files
- Confirm model assignments are intentional
- Check that skill lists match the agent's domain

### Step 2: Skill Audit

- List all skill directories
- Verify each skill has valid frontmatter
- Check for hardcoded paths or project-specific assumptions in portable skills
- Confirm skills are referenced by at least one agent's skills list

### Step 3: Rule Audit

- List all rule files
- Check for contradictions between rules
- Flag rules that reference deprecated patterns or removed code
- Verify rule applicability to current architecture

### Step 4: Reading List Currency

- Collect all file paths from required reading sections across all agents
- Verify each path resolves to an existing file
- Flag any broken references with the agent that references them

### Step 5: Lesson Promotion Pipeline

- Check lessons for recurrence counts at or above threshold
- For each promotable lesson:
  1. Validate the learning against project history
  2. Determine target: new rule, coding standard addition, or skill update
  3. Apply the change to the appropriate file
  4. Update the lesson's promotion metadata
  5. Cross-reference with other agents that may be affected

## Change Processes

### Adding a New Agent

1. Verify no existing agent covers the responsibility
2. Create definition following the standard template
3. Include appropriate tool lists and skills
4. Add required reading references to existing docs

### Modifying an Existing Agent

1. Read the current definition completely
2. Make targeted changes — do not rewrite unchanged sections
3. Run a reading list check on the modified agent

### Promoting a Lesson

1. Read the lesson file
2. Validate the learning against recent project history
3. Determine target: rule, coding standard, or skill update
4. Apply the change to the appropriate file
5. Update the lesson's promotion metadata
6. Cross-reference with other agents that may be affected

## Critical Rules

- NEVER delete an agent definition without explicit user approval
- NEVER modify tool lists speculatively — only update when verified
- Always preserve frontmatter structure exactly
- When in doubt about a change, document it as a recommendation rather than applying it
- All governance changes must be traceable — include rationale in commit messages
