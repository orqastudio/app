---
id: "AD-efe10e1d"
type: discovery-decision
title: "CLI Rule Loading — Accept Full Injection Until Selective Loading Exists"
description: "Accept that .claude/rules/ symlink loads all 44 rules into every CLI session. The graph-based injection model requires selective loading that Claude Code does not yet support."
status: completed
created: "2026-03-12"
updated: "2026-03-12"
relationships: []
---
## Decision

Keep the `.claude/rules/` → `.orqa/process/rules/` symlink as-is. All rules load into every CLI session. This is a known tension with the graph-based injection model but is the pragmatic choice given current tool constraints.

## Rationale

**The tension:** The graph-based injection model says "inject IDs, not content — agents resolve on demand." But Claude Code automatically loads all `.md` files from `.claude/rules/` into the system prompt. There is no mechanism for selective loading based on file paths, task context, or rule relevance.

**Why we accept it:**

1. **Claude Code has no selective rule loading.** All files in `.claude/rules/` are injected. We cannot filter by `layer`, `scope`, or `status` fields. Until Claude Code supports metadata-driven filtering, the only alternative is removing rules from the directory — which loses enforcement.

2. **Rules are now portable.** After [TASK-1c15bc9a](TASK-1c15bc9a), core rules contain only universal principles (no project-specific content). The context cost per rule is lower than before.

3. **The cost is manageable.** 44 rules at ~50-200 lines each is significant but not session-killing. The orchestrator prompt + rules fit within typical context budgets.

4. **The app solves this differently.** OrqaStudio's native app uses the enforcement engine to inject rules selectively based on file paths and tool events. The full-body loading problem is CLI-specific.

**When to revisit:**

- If Claude Code adds metadata-driven rule filtering (e.g., frontmatter conditions)
- If rule count exceeds 60+ and context pressure becomes measurable
- If the OrqaStudio CLI plugin gains the ability to intercept and filter rule loading

## Consequences

- CLI sessions carry ~8-15k tokens of rule context regardless of task relevance
- Core rules must stay concise to minimize context cost (reinforced by [TASK-1c15bc9a](TASK-1c15bc9a) cleanup)
- The plugin's `UserPromptSubmit` hook can inject additional context on top of the baseline rules
- The app's enforcement engine remains the target architecture for selective injection