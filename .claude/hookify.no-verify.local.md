---
name: block-no-verify
enabled: true
event: bash
action: block
pattern: --no-verify
---

**BLOCKED: --no-verify is forbidden in this project.**

Fix the errors instead of bypassing hooks. Pre-commit hooks exist to catch real issues.

See: `.claude/rules/git-workflow.md` — "NEVER use `--no-verify` on commits."
