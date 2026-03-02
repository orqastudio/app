---
name: block-delete-source-dirs
enabled: true
event: bash
action: block
pattern: (rm\s+-rf?|git\s+rm\s+-r)\s+.*(docs/|src-tauri/|src/|tests/)
---

**BLOCKED: Deleting source-of-truth directories requires explicit user approval.**

These directories contain critical project content. Before deleting:

1. Verify replacement content exists and is COMMITTED: `git ls-tree HEAD -- <destination>`
2. Check `git stash list` for hidden work
3. Get explicit user confirmation

See: `.claude/rules/git-workflow.md` — Data Loss Prevention (CRITICAL)
