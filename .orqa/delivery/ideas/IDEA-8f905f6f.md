---
id: IDEA-8f905f6f
type: discovery-idea
title: "Rich CLI experience — make orqa commands beautiful"
description: "The orqa CLI outputs plain text. Invest in a polished terminal experience with colors, spinners, progress bars, tables, and branded output that makes the tool feel premium and informative."
status: captured
priority: P2
created: 2026-03-24
updated: 2026-03-24
horizon: active
relationships:
  - target: PILLAR-c9e0a695
    type: grounded
    rationale: "Clarity Through Structure — rich output makes CLI state and progress visible at a glance"
  - target: PERSONA-477971bf
    type: benefits
    rationale: "Practitioners spend significant time in the terminal — quality UX matters here too"
---

## What

The orqa CLI currently uses basic console.log/console.error output. A polished CLI experience would include:

### Output formatting

- Colored status indicators (green checkmarks, red errors, yellow warnings)
- Consistent prefixed output (already started in `orqa dev` with `[ctrl]`, `[vite]` etc.)
- Table output for `orqa graph stats`, `orqa daemon status`, `orqa dev status`
- Tree output for `orqa graph --tree`

### Progress indicators

- Spinners for long operations (daemon start, cargo builds, plugin refresh)
- Progress bars for multi-file operations (validation, content sync)
- Elapsed time display for builds

### Branding

- OrqaStudio logo/banner on `orqa dev` startup (already has a basic box)
- Consistent color palette across all commands
- Version display in headers

### Interactive features

- `orqa dev` dashboard mode (process status, log tailing, restart shortcuts)
- `orqa graph` interactive exploration
- Fuzzy search for artifact navigation

### Libraries to consider

- `chalk` or `picocolors` for colors (picocolors is smaller)
- `ora` for spinners
- `cli-table3` or `tty-table` for tables
- `boxen` for boxes/banners
- `ink` for React-like terminal UI (heavier, but powerful)
