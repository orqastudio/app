---
name: No stubs or placeholders
description: Real implementations only — no deferred deliverables, no TODO placeholders
type: feedback
---

All implementations must be real and complete. No stubs, no placeholders, no "TODO: implement later." If a feature touches the IPC boundary, it needs all four layers committed together (Rust command + IPC types + Svelte component + store binding).

**Why:** Stubs accumulate as hidden tech debt. They pass type checks but fail at runtime. Multiple lessons (IMPL series) showed that partial implementations create cascading problems in downstream work.

**How to apply:** Before marking any implementation task complete, verify there are no TODO comments, placeholder returns, or unimplemented branches. RULE-020 enforces this.
