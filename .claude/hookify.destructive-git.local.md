---
name: warn-destructive-git
enabled: true
event: bash
action: warn
pattern: git\s+(reset\s+--hard|checkout\s+\.|clean\s+-fd|push\s+--force|stash\s+drop)
---

**Destructive git operation detected.**

Before proceeding:

1. Run `git status` and `git diff` to understand what will be lost
2. Check `git stash list` for hidden work
3. Confirm with the user before executing

See: `.claude/rules/git-workflow.md` — Data Loss Prevention
