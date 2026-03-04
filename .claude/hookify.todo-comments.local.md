---
name: block-todo-comments
enabled: true
event: file
action: block
conditions:
  - field: file_path
    operator: regex_match
    pattern: (ui/|src-tauri/src/)/.*\.(ts|svelte|rs)$
  - field: new_text
    operator: regex_match
    pattern: (#|//)\s*(TODO|FIXME|HACK|XXX|TEMP)\b
---

**BLOCKED: TODO/FIXME/HACK comments are forbidden in production code.**

Either implement the functionality now or track it in `TODO.md`. No placeholder comments in committed code.

See: `.claude/rules/coding-standards.md` — "No TODO comments: If something isn't done, it's tracked in TODO.md"
