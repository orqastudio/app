---

id: IMPL-f7201eb5
type: lesson
title: Blank lines in YAML frontmatter corrupt artifact parsing
description: EPIC-c96c9f12 had 40 blank lines between the opening --- delimiter and the actual YAML fields. This breaks strict YAML parsers and can cause the artifact scanner to skip or misparse the file.
status: captured
created: 2026-03-15
updated: 2026-03-15
maturity: observation
recurrence: 1
relationships: []
---

## Pattern

Manual editing or agent output introduced blank lines between `---` and the first YAML key. Parsers that expect `---` followed immediately by key-value pairs fail silently.

## Fix

The pre-commit hook's schema validator should check for blank lines between frontmatter delimiters. The graph-guardian PostToolUse hook should also validate frontmatter structure on write.
