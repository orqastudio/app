---
name: warn-perpetual-bg-commands
enabled: true
event: bash
action: warn
pattern: (npm\s+run\s+dev|cargo\s+tauri\s+dev|cargo\s+watch|tauri\s+dev)
---

**Perpetual command detected — do NOT run as a background task in a worktree.**

These commands never terminate and will hold file locks, preventing worktree cleanup on Windows. If you need to see output:

- Run in foreground with a timeout
- Use `cargo build` or `npm run build` for one-shot verification
- Check compilation with `cargo check` instead of `cargo watch`

See: `.claude/rules/git-workflow.md` — "Background Process Discipline (NON-NEGOTIABLE)"
