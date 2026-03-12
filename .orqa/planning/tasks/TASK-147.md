---
id: TASK-147
title: Audit frontend for repeat patterns and create component reuse skill and enforcement rule
description: Audit ui/lib/components/ for repeated patterns, create a component extraction skill, and update RULE-024 enforcement to mandate component reuse as a composability requirement.
status: done
created: "2026-03-11"
updated: "2026-03-11"
epic: EPIC-049
depends-on:
  - TASK-139
acceptance:
  - Audit document listing all repeated patterns with occurrence counts
  - New skill for component extraction methodology created
  - RULE-024 updated or companion rule created for enforcement
  - Follow-up tasks created for each shared component that should be extracted
  - Depends on TASK-139 (component inventory) completing first
---
## What

Component reuse is part of the composability principle (PILLAR-001). Audit the entire frontend (`ui/lib/components/`) to identify repeated patterns that should be extracted into shared components.

Three deliverables:

### 1. Frontend Pattern Audit
- Scan all components for repeated markup/logic patterns
- Identify inline implementations of things that should be shared (loading states, error displays, empty states, status indicators, toolbars, etc.)
- Quantify: how many times is each pattern repeated?

### 2. Component Creation Skill
- Create a skill that teaches agents how to identify candidates for shared components
- Covers: when to extract, naming conventions, prop design, variant patterns, where to place shared components
- Practical checklist agents use before creating any new UI element

### 3. Enforcement Rule Update
- Update [RULE-024](RULE-024) or create a companion rule that enforces component reuse as a composability requirement
- Include the "search before creating" mandate with specific search patterns
- FORBIDDEN section for inline implementations of things that have shared equivalents

## How

1. Use `search_semantic` to find all loading states, error displays, empty states, and toolbar patterns across `ui/lib/components/`
2. For each pattern, count occurrences and note the files containing inline implementations
3. Create the component extraction skill in `.orqa/team/skills/` covering when/how to extract, naming, prop design, and a pre-creation checklist
4. Edit [RULE-024](RULE-024) to add the search-before-creating mandate and FORBIDDEN section for known inline patterns
5. Create TASK-NNN follow-up artifacts for the top N patterns worth extracting

## Verification

- [ ] Audit document listing all repeated patterns with occurrence counts
- [ ] New skill for component extraction methodology created
- [ ] [RULE-024](RULE-024) updated or companion rule created for enforcement
- [ ] Follow-up tasks created for each shared component that should be extracted
- [ ] Depends on [TASK-139](TASK-139) (component inventory) completing first
