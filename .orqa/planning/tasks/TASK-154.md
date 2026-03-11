---
id: TASK-154
title: Populate empty Related Rules sections on 5 rules
description: Fill in the empty Related Rules sections on RULE-002, RULE-013, RULE-021, RULE-025, and RULE-028 with accurate cross-references.
status: done
created: "2026-03-11"
updated: "2026-03-11"
epic: EPIC-049
depends-on: []
scope:
  - Review each of the 5 rules and identify which other rules are genuinely related
  - Populate Related Rules sections with accurate cross-references
  - Remove empty heading if no related rules exist (unlikely for these)
acceptance:
  - RULE-002, RULE-013, RULE-021, RULE-025, RULE-028 all have populated Related Rules sections
  - Each cross-reference is bidirectional where appropriate
  - No spurious references added just to fill the section
---
## What

Five rules have a "Related Rules" heading with no content: [RULE-002](RULE-002), [RULE-013](RULE-013), [RULE-021](RULE-021), [RULE-025](RULE-025), [RULE-028](RULE-028). These should either be populated with genuine cross-references or have the empty heading removed.

## How

1. Read each of the 5 rules and understand their scope
2. Search other rules for references to these 5 rules (they may already be referenced from the other direction)
3. Identify logical relationships (rules that enforce complementary constraints, rules that agents need together)
4. Add Related Rules entries with the standard format: `- [RULE-NNN](RULE-NNN) (slug) — brief description of relationship`
5. Check if the referenced rules also link back — add bidirectional refs where missing

## Verification

- [ ] All 5 rules have non-empty Related Rules sections (or heading removed if genuinely none)
- [ ] Cross-references are accurate and meaningful
- [ ] Bidirectional references added where appropriate
