---
id: "TASK-59ff2cfe"
type: task
title: "Populate empty Related Rules sections on 5 rules"
description: "Fill in the empty Related Rules sections on RULE-65973a88, RULE-633e636d, RULE-39169bcd, RULE-1f30904a, and RULE-d90112d9 with accurate cross-references."
status: "completed"
created: "2026-03-11"
updated: "2026-03-11"
acceptance:
  - "RULE-65973a88, RULE-633e636d, RULE-39169bcd, RULE-1f30904a, RULE-d90112d9 all have populated Related Rules sections"
  - "Each cross-reference is bidirectional where appropriate"
  - "No spurious references added just to fill the section"
relationships:
  - target: "EPIC-4a7aeacb"
    type: "delivers"
    rationale: "Task belongs to this epic"
  - target: "TASK-ec136ce9"
    type: "depended-on-by"
---
## What

Five rules have a "Related Rules" heading with no content: [RULE-65973a88](RULE-65973a88), [RULE-633e636d](RULE-633e636d), [RULE-39169bcd](RULE-39169bcd), [RULE-1f30904a](RULE-1f30904a), [RULE-d90112d9](RULE-d90112d9). These should either be populated with genuine cross-references or have the empty heading removed.

## How

1. Read each of the 5 rules and understand their scope
2. Search other rules for references to these 5 rules (they may already be referenced from the other direction)
3. Identify logical relationships (rules that enforce complementary constraints, rules that agents need together)
4. Add Related Rules entries with the standard format: `- RULE-NNN (slug) — brief description of relationship`
5. Check if the referenced rules also link back — add bidirectional refs where missing

## Verification

- [ ] All 5 rules have non-empty Related Rules sections (or heading removed if genuinely none)
- [ ] Cross-references are accurate and meaningful
- [ ] Bidirectional references added where appropriate