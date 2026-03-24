---
id: IDEA-a5c7d9e1
type: idea
title: "orqa build — diff-aware incremental rebuilds using git server"
status: exploring
description: Centralised build commands that use git diffs from the local server to determine what needs rebuilding. Only rebuild affected packages instead of everything.
created: 2026-03-23
relationships:
  - target: PERSONA-cda6edd6
    type: benefits
    rationale: "Serves the primary developer persona"
  - target: PILLAR-569581e0
    type: grounded
    rationale: Structure — centralised build management instead of manual per-package rebuilds
---

## The Idea

`orqa build` queries the git server for what changed since the last build marker, maps changed files to workspace packages, and rebuilds only what's needed.

```
orqa build              # Diff-aware incremental rebuild
orqa build --all        # Full rebuild (ignore diff)
orqa build --status     # Show what needs rebuilding
```

## How It Works

1. Read last build commit SHA from `.orqa/build-marker` (or similar)
2. Query git diff: `git diff <last-build>..<HEAD> --name-only`
3. Map changed files to workspace packages (libs/cli/ → @orqastudio/cli)
4. Include downstream dependents (if types changed, rebuild cli, sdk, etc.)
5. Rebuild in topological order
6. Update build marker

## Benefits

- No more manual `cd libs/cli && npx tsc` per package
- Downstream dependencies automatically included
- Build marker survives sessions — next session only rebuilds what changed
- Could extend to Rust workspace too (cargo already handles this, but the marker tracks it)
